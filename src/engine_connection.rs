use std::io::{BufRead, BufReader, Write};
use std::process::{ChildStdin, Stdio, self};
use std::str::FromStr;
use std::time::{Duration, Instant};
use std::sync::mpsc::{Receiver, sync_channel, TryRecvError};
use std::thread::{sleep, spawn};

use chess::{Board, ChessMove};

use command::Command;
use error::Error;
use engine::best_move::BestMove;
use engine::engine_command::EngineCommand;
use gui::gui_command::GuiCommand;
use gui::go::Go;
use timer::timer::Timer;

pub struct EngineConnection<'a> {
    history: Vec<Command>,
    stdin: ChildStdin,
    receiver: Receiver<Command>,
    timer: Option<&'a mut Timer>,
}

impl<'a> EngineConnection<'a> {
    pub fn new(path: &str) -> Result<EngineConnection, Error> {
        let process = process::Command::new(path)
                                            .stdin(Stdio::piped())
                                            .stdout(Stdio::piped())
                                            .spawn()?;

        let (tx, rx) = sync_channel(1024);

        let mut reader = BufReader::new(process.stdout.unwrap());

        spawn(move || {
            let mut s = String::new();
            while let Ok(_) = reader.read_line(&mut s) {
                if let Ok(command) = Command::from_str(&s) {
                    if let Err(_) = tx.send(command.clone()) {
                        break;
                    }
                } else {
                    break;
                }
                s = String::new();
            }
        });

        let mut ec = EngineConnection {
            stdin: process.stdin.unwrap(),
            history: vec!(),
            receiver: rx,
            timer: None,
        };

        ec.send_uci()?;
        ec.send_isready()?;

        Ok(ec)
    }

    pub fn set_timer(&mut self, timer: &'a mut Timer) {
        self.timer = Some(timer);
    }

    pub fn send_position(&mut self,
                         position: Board,
                         moves: Vec<ChessMove>) -> Result<(), Error> {
        self.send(GuiCommand::Position(position, moves))
    }

    pub fn send_go(&mut self) -> Result<(), Error> {
        let mut go = Go::default();
        if let Some(ref timer) = self.timer {
            go = go.combine(&((**timer).into()))
        }

        self.send(GuiCommand::Go(go))?;
        if let Some(ref mut timer) = self.timer {
            timer.start();
        }
        Ok(())
    }

    pub fn recv_best_move(&mut self) -> Result<BestMove, Error> {
        loop {
            match self.recv(Instant::now(), Duration::new(0, 0)) {
                Ok(EngineCommand::BestMove(x)) => return Ok(x),
                Ok(_) => {},
                Err(e) => return Err(e),
            }
        }
    }

    pub fn history(&self) -> &Vec<Command> {
        &self.history
    }

    pub fn recv_best_move_using_timer(&mut self) -> Result<BestMove, Error> {
        // check to make sure there is a timer, and that it was started
        if let Some(ref mut timer) = self.timer {
            if !timer.started() {
                timer.start();
            }
        } else {
            return Err(Error::CommandError);
        }

        // recv(..) until we get a best_move from the engine, or until
        // the engine times out.
        //
        // Return any unexpected errors as well, such as if the engine
        // crashes.
        let mut best_move: Option<BestMove> = None;
        while best_move.is_none() {
            match self.recv_best_move() {
                Ok(x) => {
                    best_move = Some(x);
                },
                Err(Error::NoCommandError) => { },
                Err(x) => return Err(x)
            };

            if let Some(ref timer) = self.timer {
                if timer.timeout_for(timer.get_player()) {
                    break;
                }
            }

            // Don't peg the CPU.  The engine *is* using it to think,
            // afer all...
            sleep(Duration::from_millis(1));
        }

        // tell the timer the engine made its move.  Additionally,
        // confirm with the timer that it didn't timeout after making
        // the move.
        if let Some(ref mut timer) = self.timer {
            if let Some(best_move) = best_move {
                timer.made_move();
                if timer.timeout_for(!timer.get_player()) {
                    return Err(Error::Timeout);
                }
                return Ok(best_move);
            } else {
                return Err(Error::Timeout);
            }
        }

        // We can't hit this point, because we already know
        // that self.timer is not None.
        unreachable!();
    }

    fn send(&mut self, command: GuiCommand) -> Result<(), Error> {
        self.stdin.write_all(command.to_string().as_bytes())?;
        self.history.push(Command::new_from_gui(command));
        Ok(())
    }

    fn send_uci(&mut self) -> Result<(), Error> {
        self.send(GuiCommand::Uci)?;
        self.recv_uci_ok()
    }

    fn recv(&mut self, start: Instant, timeout: Duration) -> Result<EngineCommand, Error> {
        loop {
            match self.receiver.try_recv() {
                Ok(Command::Engine(c)) => {
                    self.history.push(Command::Engine(c.clone()));
                    return Ok(c);
                },

                Ok(c) => {
                    self.history.push(c);
                },

                Err(TryRecvError::Disconnected) =>
                    return Err(Error::EngineDeadError),

                Err(TryRecvError::Empty) => {
                    if start.elapsed() < timeout {
                        sleep(Duration::from_millis(1));
                    } else {
                        break;
                    }
                }
            }
        }

        Err(Error::NoCommandError)
    }

    fn recv_uci_ok(&mut self) -> Result<(), Error> {
        let start = Instant::now();

        loop {
            match self.recv(start, Duration::new(5, 0)) {
                Ok(EngineCommand::UciOk) => return Ok(()),
                Ok(_) => {},
                Err(e) => return Err(e),
            }
        }
    }

    fn send_isready(&mut self) -> Result<(), Error> {
        self.send(GuiCommand::IsReady)?;
        self.recv_ready_ok()
    }

    fn recv_ready_ok(&mut self) -> Result<(), Error> {
        let start = Instant::now();
        loop {
            match self.recv(start, Duration::new(1, 0)) {
                Ok(EngineCommand::ReadyOk) => return Ok(()),
                Ok(_) => {},
                Err(e) => return Err(e),
            }
        }
    }
}

#[test]
fn open_stockfish() {
    let mut timer = Timer::new_with_increment(Duration::new(5, 0), Duration::new(1, 0));
    let mut e = EngineConnection::new("/usr/bin/stockfish").unwrap();

    e.set_timer(&mut timer);
    e.send_position(Board::default(), vec!()).unwrap();
    e.send_go().unwrap();
    e.recv_best_move_using_timer().unwrap();
}

