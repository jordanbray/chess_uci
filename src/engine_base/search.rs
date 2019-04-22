use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::marker::PhantomData;

use chess::{Board, MoveGen, Color};

use super::eval::Eval;
use super::evaluate::Evaluate;
use super::pv::Pv;

//use super::tt_entry::TtEntry;


pub trait Search<E: Eval> {
   fn search(&mut self, board: Board, alpha: E, beta: E, depth: u16) -> E;
   fn get_pv(&self) -> &Pv;
}

pub struct DefaultSearch<E: Eval, V: Evaluate<E>> {
    evaluator: V,
    stopping: Arc<AtomicBool>,
    phantom: PhantomData<E>,
    pv: Pv,
}

impl<E: Eval, V: Evaluate<E>> DefaultSearch<E, V> {
    pub fn new(stopping: Arc<AtomicBool>,
               evaluator: V) -> Self {
        DefaultSearch {
            evaluator: evaluator,
            stopping: stopping,
            phantom: PhantomData,
            pv: Pv::new(),
        }
    }

    pub fn qsearch(&mut self, board: Board, mut alpha: E, mut beta: E, depth: u16, pv: &mut Pv) -> E {
        alpha = alpha.add_depth(-1);
        beta = beta.add_depth(-1);
        let stand_pat = if board.side_to_move() == Color::White {
            E::one()
        } else {
            -E::one()
        } * self.evaluator.evaluate(board, alpha, beta);

        if stand_pat >= beta {
            return beta;
        }

        if stand_pat > alpha {
            alpha = stand_pat;
        }

        let mut child_pv = Pv::new();

        let mut movegen = MoveGen::new_legal(&board);
        let targets = board.color_combined(!board.side_to_move());
        movegen.set_iterator_mask(*targets);

        for m in movegen {
            child_pv.clear();
            let score = -self.qsearch(board.make_move_new(m), -beta, -alpha, depth - 1, &mut child_pv).add_depth(1);
            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
                pv.update(m, &child_pv);
            }
        }

        return alpha;
    }

    fn search_line(&mut self, board: Board, mut alpha: E, mut beta: E, depth: u16, pv: &mut Pv) -> E {
        let mut child_pv = Pv::new();

        if depth <= 0 {
            return self.qsearch(board, alpha, beta, depth, &mut child_pv);
        }

        alpha = alpha.add_depth(-1);
        beta = beta.add_depth(-1);
 
        let mut movegen = MoveGen::new_legal(&board);
        let mut best_score;
        if let Some(first_move) = movegen.next() {
            best_score = -self.search_line(board.make_move_new(first_move), -beta, -alpha, depth - 1, &mut child_pv).add_depth(1);
            if best_score > alpha {
                pv.update(first_move, &child_pv);

                if best_score >= beta {
                    return best_score;
                }
                alpha = best_score;
            }
        } else {
            return E::new_mate(0, Color::White);
        }

        for m in movegen {
            child_pv.clear();
            let new_board = board.make_move_new(m);
            let mut score = -self.search_line(new_board, -alpha - E::one(), -alpha, depth - 1, &mut child_pv).add_depth(1);
            if score > alpha && score < beta {
                child_pv.clear();
                score = -self.search_line(new_board, -beta, -alpha, depth - 1, &mut child_pv).add_depth(1);
                if score > alpha {
                    pv.update(m, &child_pv);

                    alpha = score;
                }
            }

            if self.stopping.load(Ordering::Relaxed) {
                return E::zero();
            }

            if score > best_score {
                if score >= beta {
                    return score;
                }
                best_score = score;
            }

        }
        return best_score;
    }


}

impl<E: Eval, V: Evaluate<E>> Search<E> for DefaultSearch<E, V> {
    fn search(&mut self, board: Board, alpha: E, beta: E, depth: u16) -> E {
        let mut pv = Pv::new();
        let result = self.search_line(board, alpha, beta, depth, &mut pv);
        self.pv = pv;
        result
    }

    fn get_pv(&self) -> &Pv {
        &self.pv
    }
}

#[cfg(test)]
use chess::{ChessMove, Rank, File, Square};

#[cfg(test)]
fn easy_tactic() -> Board {
    Board::from_fen("r5k1/p1p3bp/1p2q1p1/5p2/8/P1P4P/1P2BPP1/3QR1K1 w - - 0 1".to_string()).unwrap()
}

#[cfg(test)]
fn super_easy_tactic() -> Board {
    Board::from_fen("3q1k2/8/8/8/8/8/8/3QK3 w - - 0 1".to_string()).unwrap()
}

#[cfg(test)]
use super::evaluate::DefaultEvaluate;

#[cfg(test)]
fn find_move_qsearch(board: Board, m: ChessMove) {
    let mut searcher = DefaultSearch::new(Arc::<AtomicBool>::new(AtomicBool::new(false)), DefaultEvaluate::default());
    let mut pv = Pv::new();
    searcher.qsearch(board, i32::min_value() + 20, i32::max_value() - 20, 0, &mut pv);

    assert_eq!(pv[0], m);
}

#[cfg(test)]
fn find_move_search(board: Board, m: ChessMove) {
    let mut searcher = DefaultSearch::new(Arc::<AtomicBool>::new(AtomicBool::new(false)), DefaultEvaluate::default());

    searcher.search(board, i32::min_value() + 20, i32::max_value() - 20, 4);
    assert_eq!(searcher.get_pv()[0], m);
}

#[test]
fn test_qsearch() {
    let best_move = ChessMove::new(Square::make_square(Rank::First, File::D), Square::make_square(Rank::Eighth, File::D), None);
    find_move_qsearch(super_easy_tactic(), best_move);
}

#[test]
fn test_super_easy_search() {
    let best_move = ChessMove::new(Square::make_square(Rank::First, File::D), Square::make_square(Rank::Eighth, File::D), None);
    find_move_search(super_easy_tactic(), best_move);
}

#[test]
fn test_easy_search() {
    let best_move = ChessMove::new(Square::make_square(Rank::Second, File::E), Square::make_square(Rank::Third, File::F), None);
    find_move_search(easy_tactic(), best_move);
}
