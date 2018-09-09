use std::time::{Duration, Instant};
use chess::Color;

use gui::go::Go;
use std::convert::Into;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
struct PlayerTimer {
    time: Duration,
    increment: Duration,
}

fn duration_to_millis(duration: Duration) -> u64 {
    duration.as_secs() * 1000 + (duration.subsec_millis() as u64)
}

fn remaining_or_zero(optional_start: Option<Instant>, time: Duration) -> Duration {
    if let Some(start) = optional_start {
        let elapsed = start.elapsed();
        if elapsed > time {
            Duration::new(0, 0)
        } else {
            time - elapsed
        }
    } else {
        time
    }
}

impl PlayerTimer {
    pub fn remaining(&self, start: Option<Instant>, playing: bool) -> Duration {
        if !playing {
            self.time
        } else {
            remaining_or_zero(start, self.time)
        }
    }

    pub fn get_increment(&self) -> Duration {
        self.increment
    }

    pub fn made_move(&mut self, start: Option<Instant>, extra_time: Duration) {
        self.time = self.remaining(start, true);
        self.time += self.increment;
        self.time += extra_time;
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Timer {
    white: Option<PlayerTimer>,
    black: Option<PlayerTimer>,
    move_time: Option<Duration>,
    player: Color,
    start: Option<Instant>,
    moves_to_go: u64,
    start_moves_to_go: u64,
    add_time_on_move_n: Duration,
}

impl Into<Go> for Timer {
    fn into(self) -> Go {
        let mut go = Go::default();

        let zero = Duration::new(0, 0);

        if let Some(white) = self.white {
            go = go.combine(
                    &Go::wtime(
                        duration_to_millis(
                            white.remaining(self.start, self.player == Color::White)
                        )
                    )
                );
            if white.increment != zero {
                go = go.combine(&Go::winc(duration_to_millis(white.get_increment())));
            }
        }
        if let Some(black) = self.black {
            go = go.combine(
                    &Go::btime(
                        duration_to_millis(
                            black.remaining(self.start, self.player == Color::Black)
                        )
                    )
                );
            if black.increment != zero {
                go = go.combine(&Go::binc(duration_to_millis(black.get_increment())));
            }
        }

        if let Some(move_time) = self.move_time {
            go = go.combine(&Go::movetime(duration_to_millis(move_time)));
        }

        if self.moves_to_go != 0 {
            go = go.combine(&Go::movestogo(self.moves_to_go));
        }

        if ((self.player == Color::White && self.white.is_none()) ||
            (self.player == Color::Black && self.black.is_none())) &&
           self.move_time.is_none() {
            go = go.combine(&Go::infinite(true));    
        }

        go
    }
}

impl Timer {
    pub fn remaining_for(&self, player: Color) -> Option<Duration> {
        let timer = if player == Color::White { self.white } else { self.black };

        if let Some(t) = timer {
            Some(t.remaining(self.start, self.player == player))
        } else if let Some(move_time) = self.move_time {
            if self.player == player {
                Some(move_time)
            } else {
                Some(remaining_or_zero(self.start, move_time))
            }
        } else {
            None
        }
    }

    pub fn set_add_time_on_move_n(&mut self, add: Duration) {
        self.add_time_on_move_n = add;
    }

    pub fn made_move(&mut self) {
        if self.player == Color::Black {
            self.moves_to_go -= 1;
        }
        
        let add_time = if self.moves_to_go == 0 {
            self.moves_to_go = self.start_moves_to_go;
            self.add_time_on_move_n
        } else {
            Duration::new(0, 0)
        };
        {
            let clock = if self.player == Color::White { &mut self.white } else { &mut self.black };
            if let Some(player_clock) = clock {
                player_clock.made_move(self.start, add_time);
            }
        }
        
        self.player = !self.player;
        self.start();
    }

    pub fn white_remaining(&self) -> Option<Duration> {
        self.remaining_for(Color::White)
    }

    pub fn black_remaining(&self) -> Option<Duration> {
        self.remaining_for(Color::Black)
    }

    pub fn update_from_go(&mut self, go: &Go) {
        let copy_from = Timer::new_from_go(go, self.player);

        self.white = copy_from.white;
        self.black = copy_from.black;
        self.move_time = copy_from.move_time;
        self.moves_to_go = copy_from.moves_to_go;
    }

    pub fn new_from_go(go: &Go, player: Color) -> Timer {
        Timer::new_from_durations(
            go.get_wtime()
              .map(Duration::from_millis),

            go.get_winc()
              .map(Duration::from_millis)
              .unwrap_or(Duration::from_millis(0)),

            go.get_btime()
              .map(Duration::from_millis),

            go.get_binc()
              .map(Duration::from_millis)
              .unwrap_or(Duration::from_millis(0)),

            go.get_movetime()
              .map(Duration::from_millis),

            go.get_movestogo()
              .unwrap_or(0),

            go.get_movestogo()
              .unwrap_or(0),

            if player == Color::White {
                go.get_wtime()
                  .map(Duration::from_millis)
                  .unwrap_or(Duration::new(0, 0))
            } else {
                go.get_btime()
                  .map(Duration::from_millis)
                  .unwrap_or(Duration::new(0, 0))
            },

            player,

            Some(Instant::now())
        )
    }

    pub fn start(&mut self) {
        self.start = Some(Instant::now());
    }

    pub fn new_without_increment(time: Duration) -> Timer {
        Timer::new_from_durations(
            Some(time),
            Duration::new(0, 0),
            Some(time),
            Duration::new(0, 0),
            None,
            0,
            0,
            Duration::new(0, 0),
            Color::White,
            None)
    }

    pub fn new_with_increment(time: Duration, inc: Duration) -> Timer {
        Timer::new_from_durations(
            Some(time),
            inc,
            Some(time),
            inc,
            None,
            0,
            0,
            Duration::new(0, 0),
            Color::White,
            None)
    }

    pub fn new_static_move_time(time: Duration) -> Timer {
        Timer::new_from_durations(
            None,
            Duration::new(0, 0),
            None,
            Duration::new(0, 0),
            Some(time),
            0,
            0,
            Duration::new(0, 0),
            Color::White,
            None)
    }

    pub fn new_from_durations(
        wtime: Option<Duration>,
        winc: Duration,
        btime: Option<Duration>,
        binc: Duration,
        move_time: Option<Duration>,
        moves_to_go: u64,
        start_moves_to_go: u64,
        add_time_on_move_n: Duration,
        player: Color,
        start: Option<Instant>) -> Timer {
        Timer {
            white: wtime.map(|x| PlayerTimer {
                time: x,
                increment: winc
            }),
            black: btime.map(|x| PlayerTimer {
                time: x,
                increment: binc
            }),
            move_time: move_time,
            moves_to_go: moves_to_go,
            start_moves_to_go: start_moves_to_go,
            add_time_on_move_n: add_time_on_move_n,
            player: player,
            start: start
        }
    }
}

#[test]
fn test_with_increment_into_go() {
    let timer = Timer::new_from_durations(
        Some(Duration::new(5, 0)),
        Duration::new(1, 0),
        Some(Duration::new(7, 0)),
        Duration::new(2, 0),
        None,
        0,
        0,
        Duration::new(0, 0),
        Color::White,
        None);

    let go = Go::default()
        .combine(&Go::wtime(5000))
        .combine(&Go::winc(1000))
        .combine(&Go::btime(7000))
        .combine(&Go::binc(2000));

    assert_eq!(go, timer.into());
}

#[test]
fn test_without_increment_into_go() {
    let timer = Timer::new_without_increment(Duration::new(5, 0));

    let go = Go::default()
         .combine(&Go::wtime(5000))
         .combine(&Go::btime(5000));

    assert_eq!(go, timer.into());
}


#[cfg(test)]
use std::thread::sleep;

#[cfg(test)]
fn durations_within_5ms(x: Duration, y: Duration) -> bool {
    let delta = if x > y {
        x - y
    } else {
        y - x
    };

    delta < Duration::from_millis(50)
}

#[test]
fn test_make_move_with_inc() {
    let mut timer = Timer::new_with_increment(Duration::new(5, 0), Duration::new(1, 0));
    timer.start();

    sleep(Duration::new(3, 0));
    timer.made_move();
    assert!(durations_within_5ms(timer.white_remaining().unwrap(), Duration::new(3, 0)));

    sleep(Duration::new(2, 0));
    timer.made_move();
    assert!(durations_within_5ms(timer.black_remaining().unwrap(), Duration::new(4, 0)));

    sleep(Duration::new(2, 0));
    timer.made_move();
    assert!(durations_within_5ms(timer.white_remaining().unwrap(), Duration::new(2, 0)));
}
