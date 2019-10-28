use super::eval::Eval;
use super::pv::Pv;
use super::search::Search;
use super::time_manager::TimeManager;
use crate::engine::info::Info;
use crate::timer::timer::Timer;
use std::convert::TryInto;
use std::io::Write;

use chess::Board;

use std::marker::PhantomData;

pub trait IterativeDeepening {
    fn id_search<W: Write>(
        &mut self,
        board: Board,
        max_depth: i16,
        moves_made: u16,
        writer: W,
    ) -> Pv;
}

pub struct DefaultIterativeDeepening<E: Eval, T: TimeManager<E>, S: Search<E>> {
    searcher: S,
    time_manager: T,
    timer: Timer,
    _eval: PhantomData<E>,
}

impl<E: Eval, T: TimeManager<E>, S: Search<E>> DefaultIterativeDeepening<E, T, S> {
    pub fn new(searcher: S, time_manager: T, timer: Timer) -> DefaultIterativeDeepening<E, T, S> {
        DefaultIterativeDeepening {
            searcher,
            time_manager,
            timer,
            _eval: PhantomData,
        }
    }
}

impl<E: Eval, T: TimeManager<E>, S: Search<E>> IterativeDeepening
    for DefaultIterativeDeepening<E, T, S>
{
    fn id_search<W: Write>(
        &mut self,
        board: Board,
        max_depth: i16,
        moves_made: u16,
        mut writer: W,
    ) -> Pv {
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

            let info = Info::default()
                .combine(&Info::depth(depth.try_into().unwrap()))
                .combine(&Info::score(eval.into()))
                .combine(&Info::pv(pv.clone().into_iter().collect()));
            write!(writer, "{}", info);

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
    let mut id = DefaultIterativeDeepening::new(
        DefaultSearch::new(
            Arc::<AtomicBool>::new(AtomicBool::new(false)),
            DefaultEvaluate::default(),
        ),
        DefaultTimeManager::new(),
        Timer::new_without_increment(Duration::from_secs(100000)),
    );

    assert_eq!(id.id_search(board, 4, 0, vec!())[0], best_move);
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
