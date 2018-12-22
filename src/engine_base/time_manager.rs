use std::time::{Instant, Duration};
use chess_uci::timer::Timer;

trait TimeManager {
    pub fn get_timer(&mut self) -> Timer;
    pub fn optimum(&mut self) -> Duration;
    pub fn maximum(&mut self) -> Duration;
}
