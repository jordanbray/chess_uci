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

pub struct EngineConnection {
    commands: Vec<Command>,
    stdin: ChildStdin,
    receiver: Receiver<Command>,
    timer: Option<Timer>,
    last_received_index: usize,
}

impl EngineConnection {
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
            commands: vec!(),
            receiver: rx,
            timer: None,
            last_received_index: 0,
        };

        ec.send_uci()?;
        ec.send_isready()?;

        Ok(ec)
    }

    pub fn set_timer(&mut self, timer: Timer) {
        self.timer = Some(timer);
    }

    pub fn send_position(&mut self,
                         position: Board,
                         moves: Vec<ChessMove>) -> Result<(), Error> {
        self.send(GuiCommand::Position(position, moves))
    }

    pub fn send_go(&mut self) -> Result<(), Error> {
        let mut go = Go::default();
        if let Some(timer) = self.timer {
            go = go.combine(&timer.into())
        }

        self.send(GuiCommand::Go(go))?;
        if let Some(mut timer) = self.timer {
            timer.start();
        }
        Ok(())
    }

    pub fn recv_best_move(&mut self) -> Result<BestMove, Error> {
        while match self.recv(Instant::now(), Duration::new(0, 0)) {
            Ok(EngineCommand::BestMove(x)) => return Ok(x),
            Err(e) => return Err(e),
            _ => true,
        } { }
        unreachable!();
    }

    fn send(&mut self, command: GuiCommand) -> Result<(), Error> {
        self.stdin.write_all(command.to_string().as_bytes())?;
        self.commands.push(Command::new_from_gui(command));
        Ok(())
    }

    fn send_uci(&mut self) -> Result<(), Error> {
        self.send(GuiCommand::Uci)?;
       self.recv_uci_ok()
    }

    fn recv(&mut self, start: Instant, timeout: Duration) -> Result<EngineCommand, Error> {
        let mut finished = false;
        let mut received_one = false;
        while !finished {
            let command = self.receiver.try_recv();
            match command {
                Ok(c) => {
                    self.commands.push(c.clone());
                    received_one = true;
                },
                Err(err) => match err {
                    TryRecvError::Disconnected => {
                        return Err(err.into());
                    },
                    TryRecvError::Empty => {
                        finished = start.elapsed() > timeout ||
                                   received_one;
                    }
                }
            }

            if !finished {
                sleep(Duration::from_millis(10));
            }
        }

        for i in self.last_received_index..self.commands.len() {
            match self.commands[i] {
                Command::Engine(ref x) => {
                    self.last_received_index = i + 1;
                    return Ok(x.clone());
                },
                _ => { }
            };
        }

        Err(Error::NoCommandError)
    }

    fn recv_uci_ok(&mut self) -> Result<(), Error> {
        let start = Instant::now();

        while match self.recv(start, Duration::new(1, 0)) {
            Ok(ref c) => c != &EngineCommand::UciOk,
            Err(Error::NoCommandError) => true,
            Err(e) => return Err(e),
        } {}
        Ok(())
    }

    fn send_isready(&mut self) -> Result<(), Error> {
        self.send(GuiCommand::IsReady)?;
        self.recv_ready_ok()
    }

    fn recv_ready_ok(&mut self) -> Result<(), Error> {
        let start = Instant::now();

        while match self.recv(start, Duration::new(1, 0)) {
            Ok(ref c) => c != &EngineCommand::ReadyOk,
            Err(Error::NoCommandError) => true,
            Err(e) => return Err(e),
        } {}
        Ok(())
    }
}

#[test]
fn open_stockfish() {
    let mut e = EngineConnection::new("/usr/bin/stockfish").unwrap();
    let timer = Timer::new_with_increment(Duration::new(5, 0), Duration::new(1, 0));
    e.set_timer(timer);
    e.send_position(Board::default(), vec!()).unwrap();
    e.send_go().unwrap();
    loop {
        match e.recv_best_move() {
            Ok(x) => break,
            Err(Error::NoCommandError) => {},
            Err(_) => panic!("Error receiving best move."),
        };
        sleep(Duration::from_millis(10));
    }
}
