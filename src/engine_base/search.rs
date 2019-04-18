use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::marker::PhantomData;

use chess::{Board, ChessMove, MoveGen};

use super::eval::Eval;
use super::evaluate::Evaluate;
//use super::tt_entry::TtEntry;


pub trait Search<E: Eval> {
   fn search(&mut self, board: Board, alpha: E, beta: E, depth: u16) -> E;
   fn get_pv(&self) -> Vec<ChessMove>;
}

pub struct DefaultSearch<E: Eval, V: Evaluate<E>> {
    evaluator: V,
    stopping: Arc<AtomicBool>,
    phantom: PhantomData<E>,
}

impl<E: Eval, V: Evaluate<E>> DefaultSearch<E, V> {
    pub fn new(stopping: Arc<AtomicBool>,
               evaluator: V) -> Self {
        DefaultSearch {
            evaluator: evaluator,
            stopping: stopping,
            phantom: PhantomData
        }
    }

    pub fn qsearch(&mut self, board: Board, mut alpha: E, beta: E, depth: u16) -> E {
        let stand_pat = self.evaluator.evaluate(board, alpha, beta);

        if stand_pat >= beta {
            return beta;
        }

        if alpha < stand_pat {
            alpha = stand_pat;
        }

        let mut movegen = MoveGen::new_legal(&board);
        let targets = board.color_combined(!board.side_to_move());
        movegen.set_iterator_mask(*targets);

        for m in movegen {
            let score = -self.qsearch(board.make_move_new(m), -beta.add_depth(-1), -alpha.add_depth(-1), depth - 1).add_depth(1);
            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }

        return alpha;
    }
}

impl<E: Eval, V: Evaluate<E>> Search<E> for DefaultSearch<E, V> {
    fn search(&mut self, board: Board, mut alpha: E, beta: E, depth: u16) -> E {
        if depth <= 0 {
            return self.qsearch(board, alpha, beta, depth);
        }


        let mut movegen = MoveGen::new_legal(&board);

        let mut best_score;
        if let Some(first_move) = movegen.next() {
            best_score = -self.search(board.make_move_new(first_move), -beta.add_depth(-1), -alpha.add_depth(-1), depth - 1).add_depth(1);
            if best_score > alpha {
                if best_score >= beta {
                    return best_score;
                }
                alpha = best_score;
            }
        } else {
            return E::new_mate(0, !board.side_to_move());
        }

        for m in movegen {
            let new_board = board.make_move_new(m);
            let mut score = -self.search(new_board, (-alpha - E::one()).add_depth(-1), -alpha.add_depth(-1), depth - 1).add_depth(1);
            if score > alpha && score < beta {
                score = -self.search(new_board, -beta.add_depth(-1), -alpha.add_depth(-1), depth - 1).add_depth(1);
                if score > alpha {
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

    fn get_pv(&self) -> Vec<ChessMove> {
        vec!()
    }
}
