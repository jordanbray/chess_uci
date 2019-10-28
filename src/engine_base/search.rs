use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use chess::{Board, Color, MoveGen};

use super::eval::Eval;
use super::evaluate::Evaluate;
use super::pv::Pv;
use super::search_window::{SearchParams, AlphaBetaSearchParams};

//use super::tt_entry::TtEntry;

pub trait Search<E: Eval> {
    fn search(&mut self, board: Board, alpha: E, beta: E, depth: i16) -> E;
    fn get_pv(&self) -> &Pv;
}

pub struct DefaultSearch<E: Eval, V: Evaluate<E>> {
    evaluator: V,
    stopping: Arc<AtomicBool>,
    phantom: PhantomData<E>,
    pv: Pv,
}

impl<E: Eval, V: Evaluate<E>> DefaultSearch<E, V> {
    pub fn new(stopping: Arc<AtomicBool>, evaluator: V) -> Self {
        DefaultSearch {
            evaluator: evaluator,
            stopping: stopping,
            phantom: PhantomData,
            pv: Pv::new(),
        }
    }

    pub fn qsearch(&mut self, sp: &mut impl SearchParams<E>) -> E {
        let stand_pat = if sp.board().side_to_move() == Color::White {
            E::one()
        } else {
            -E::one()
        } * self.evaluator.evaluate(sp);

        if stand_pat >= sp.beta() {
            return sp.beta().add_depth(1);
        }

        if stand_pat > sp.alpha() {
            sp.set_alpha(stand_pat);
        }

        let mut movegen = MoveGen::new_legal(sp.board());
        let targets = sp.board().color_combined(!sp.board().side_to_move());
        movegen.set_iterator_mask(*targets);

        for m in movegen {
            let mut child_search = sp.lower_depth(m);
            let score = -self.qsearch(&mut child_search);
            if score >= sp.beta() {
                return sp.beta().add_depth(1);
            }
            if score > sp.alpha() {
                sp.set_alpha(sp.alpha());
                sp.update_pv(m, child_search);
            }
        }

        return sp.alpha().add_depth(1);
    }

    fn search_line(&mut self, sp: &mut impl SearchParams<E>) -> E {
        if sp.depth() <= 0 {
            return self.qsearch(sp);
        }

        let mut movegen = MoveGen::new_legal(sp.board());
        let mut best_score;
        if let Some(first_move) = movegen.next() {
            let mut child_search = sp.lower_depth(first_move);
            best_score = -self.search_line(&mut child_search);
            if best_score > sp.alpha() {
                sp.update_pv(first_move, child_search);

                if best_score >= sp.beta() {
                    return best_score.add_depth(1);
                }
                sp.set_alpha(best_score);
            }
        } else {
            return E::new_mate(0, Color::White);
        }

        for m in movegen {
            let mut child_search_zw = sp.lower_depth_into_null_window(m);
            let mut score = -self.search_line(&mut child_search_zw);

            if score > sp.alpha() && score < sp.beta() {
                let mut child_search = sp.lower_depth(m);
                score = -self.search_line(&mut child_search);
                if score > sp.alpha() {
                    sp.update_pv(m, child_search);
                    sp.set_alpha(score);
                }
            }

            if self.stopping.load(Ordering::Relaxed) {
                return E::null();
            }

            if score > best_score {
                if score >= sp.beta() {
                    return score.add_depth(1);
                }
                best_score = score;
            }
        }

        return best_score.add_depth(1);
    }
}

impl<E: Eval, V: Evaluate<E>> Search<E> for DefaultSearch<E, V> {
    fn search(&mut self, board: Board, alpha: E, beta: E, depth: i16) -> E {
        let mut sp = AlphaBetaSearchParams::new(board, alpha, beta, depth);
        let result = self.search_line(&mut sp);
        self.pv = sp.get_pv();
        result
    }

    fn get_pv(&self) -> &Pv {
        &self.pv
    }
}

#[cfg(test)]
use super::test_positions::{easy_tactic, super_easy_tactic};
#[cfg(test)]
use chess::ChessMove;
#[cfg(test)]
use super::evaluate::DefaultEvaluate;

#[cfg(test)]
fn find_move_qsearch(board: Board, m: ChessMove) {
    let mut searcher = DefaultSearch::new(
        Arc::<AtomicBool>::new(AtomicBool::new(false)),
        DefaultEvaluate::default(),
    );
    let mut search_params = AlphaBetaSearchParams::new(board, i32::min_value() + 20, i32::max_value() - 20, 0);
    searcher.qsearch(&mut search_params);
    let pv = search_params.get_pv();

    assert_eq!(pv[0], m);
}

#[cfg(test)]
fn find_move_search(board: Board, m: ChessMove) {
    let mut searcher = DefaultSearch::new(
        Arc::<AtomicBool>::new(AtomicBool::new(false)),
        DefaultEvaluate::default(),
    );

    searcher.search(board, i32::min_value() + 20, i32::max_value() - 20, 4);
    assert_eq!(searcher.get_pv()[0], m);
}

#[test]
fn test_qsearch() {
    let (board, best_move) = super_easy_tactic();
    find_move_qsearch(board, best_move);
}

#[test]
fn test_super_easy_search() {
    let (board, best_move) = super_easy_tactic();
    find_move_search(board, best_move);
}

#[test]
fn test_easy_search() {
    let (board, best_move) = easy_tactic();
    find_move_search(board, best_move);
}
