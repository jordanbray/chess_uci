use std::time::{Instant, Duration};
use crate::timer::timer::Timer;
use super::eval::Eval;

pub trait TimeManager<E: Eval> {
    fn continue_id(&mut self, last_eval: E, timer: &Timer, moves: u16) -> bool;
    fn continue_search(&mut self, alpha: E, beta: E, timer: &Timer, moves: u16) -> bool;
}

pub struct DefaultTimeManager;

impl TimeManager<i32> for DefaultTimeManager {
    fn continue_id(&mut self, _last_eval: i32, timer: &Timer, moves: u16) -> bool {
        if let Some(start) = timer.get_start() {
            let more_time = timer.get_add_time_on_move_n();
            let time_for_move = timer.get_time();

            let moves_to_go = if more_time != Duration::new(0, 0) {
                timer.get_moves_to_go() as u32
            } else if moves <= 90 {
                (100 - moves) as u32
            } else {
                10u32
            };

            let mut time_to_use = time_for_move;
            if moves_to_go > 0 {
                time_to_use = time_for_move / moves_to_go;
            }

            let now = Instant::now();
            
            start + time_to_use < now
        } else {
            true
        }
    }

    fn continue_search(&mut self, _alpha: i32, _beta: i32, timer: &Timer, moves: u16) -> bool {
        self.continue_id(0, timer, moves)
    }
}

// This is hard to test.  Perhaps Timer needs to be a trait to enable testing...
