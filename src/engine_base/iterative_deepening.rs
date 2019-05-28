use super::eval::Eval;
use super::pv::Pv;
use super::search::Search;
use super::time_manager::TimeManager;
use crate::timer::timer::Timer;

use chess::Board;

use std::marker::PhantomData;

pub struct IterativeDeepening<E: Eval, T: TimeManager<E>, S: Search<E>> {
    searcher: S,
    time_manager: T,
    timer: Timer,
    _eval: PhantomData<E>,
}

impl<E: Eval, T: TimeManager<E>, S: Search<E>> IterativeDeepening<E, T, S> {
    pub fn new(searcher: S, time_manager: T, timer: Timer) -> IterativeDeepening<E, T, S> {
        IterativeDeepening {
            searcher,
            time_manager,
            timer,
            _eval: PhantomData,
        }
    }

    pub fn id_search(&mut self, board: Board, max_depth: i16, moves_made: u16) -> Pv {
        let alpha = E::min_eval();
        let beta = E::max_eval();
        let mut pv = Pv::new();

        for depth in 1..max_depth {
            let eval = self.searcher.search(board, alpha, beta, depth);
            if eval != E::null() {
                pv = (*self.searcher.get_pv()).clone();
            } else {
                break;
            }

            if !self.time_manager.continue_id(eval, &self.timer, moves_made) {
                break;
            }
        }

        pv
    }
}

#[cfg(test)]
use super::evaluate::DefaultEvaluate;
#[cfg(test)]
use super::search::DefaultSearch;
#[cfg(test)]
use super::test_positions::{easy_tactic, super_easy_tactic};
#[cfg(test)]
use super::time_manager::DefaultTimeManager;
#[cfg(test)]
use chess::ChessMove;
#[cfg(test)]
use std::sync::atomic::AtomicBool;
#[cfg(test)]
use std::sync::Arc;
#[cfg(test)]
use std::time::Duration;

#[cfg(test)]
fn perform_id_search(board: Board, best_move: ChessMove) {
    let mut id = IterativeDeepening::new(
        DefaultSearch::new(
            Arc::<AtomicBool>::new(AtomicBool::new(false)),
            DefaultEvaluate::default(),
        ),
        DefaultTimeManager::new(),
        Timer::new_without_increment(Duration::from_secs(100000)),
    );

    assert_eq!(id.id_search(board, 4, 0)[0], best_move);
}

#[test]
fn test_super_easy_tactic() {
    let (board, best_move) = super_easy_tactic();
    perform_id_search(board, best_move);
}

#[test]
fn test_easy_tactic() {
    let (board, best_move) = easy_tactic();
    perform_id_search(board, best_move);
}
