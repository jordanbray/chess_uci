use std;
use command::Command;
use timer::timer::Timer;
use chess::{Board, ChessMove};
use error::Error;
use std::io::{BufRead, BufReader};
use std::process::{ChildStdin, Stdio};
use engine::engine_command::EngineCommand;
use gui::gui_command::GuiCommand;
use gui::go::Go;
use std::io::Write;
use std::str::FromStr;
use std::time::{Duration, Instant};
use engine::best_move::BestMove;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;

pub struct EngineConnection {
    commands: Vec<Command>,
    stdin: ChildStdin,
    reader_thread: JoinHandle<()>,
    receiver: Receiver<Command>,
    timer: Option<Timer>,
    board: Option<Board>,
    moves: Vec<ChessMove>,
    last_received_index: usize,
}

impl EngineConnection {
    pub fn new(path: &str) -> Result<EngineConnection, Error> {
        let process = match std::process::Command::new(path)
                                                  .stdin(Stdio::piped())
                                                  .stdout(Stdio::piped())
                                                  .spawn() {
            Err(_) => return Err(Error::SpawnError),
            Ok(process) => process
        };

        let (tx, rx) = mpsc::sync_channel(1024);

        let stopping = Arc::new(AtomicBool::new(false));
        let stdout = process.stdout.unwrap();
        let thread_tx = tx.clone();

        let reader_thread = std::thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            while !stopping.load(Ordering::SeqCst) {
                let mut s = String::new();
                match reader.read_line(&mut s) {
                    Err(_) => stopping.store(true, Ordering::SeqCst),
                    Ok(_) => {
                        match Command::from_str(&s) {
                            Ok(command) => {
                                match thread_tx.send(command.clone()) {
                                    Err(e) => {
                                        println!("Error sending: {}", e);
                                        panic!();
                                    },
                                    _ => {}
                                }
                            },
                            Err(_) => {
                                stopping.store(true, Ordering::SeqCst);
                            }
                        }
                    }
                };
            }
        });

        let mut ec = EngineConnection {
            stdin: process.stdin.unwrap(),
            reader_thread: reader_thread,
            commands: vec!(),
            receiver: rx,
            timer: None,
            board: None,
            moves: vec!(),
            last_received_index: 0,
        };

        match ec.send_uci() {
            Err(x) => {
                println!("Error sending UCI {}", x);
                return Err(x);
            },
            Ok(()) => {}
        };

        match ec.send_isready() {
            Err(x) => {
                println!("Error sending isready {}", x);
                return Err(x);
            },
            Ok(()) => {}
        }

        Ok(ec)
    }

    pub fn set_timer(&mut self, timer: Timer) {
        self.timer = Some(timer);
    }

    pub fn send_position(&mut self,
                         position: Board,
                         moves: Vec<ChessMove>) -> Result<(), Error> {
        self.board = Some(position);
        self.moves = moves.clone();
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
        while match self.recv() {
            Some(EngineCommand::BestMove(x)) => {
                return Ok(x);
            },
            None => { return Err(Error::CommandError); },
            _ => true,
        } { }
        unreachable!();
    }

    fn send(&mut self, command: GuiCommand) -> Result<(), Error> {
        match self.stdin.write_all(command.to_string().as_bytes()) {
            Err(_) => {
                println!("Error Sending!!!");
                Err(Error::SendError)
            },
            Ok(_) => {
                self.commands.push(Command::new_from_gui(command));
                Ok(())
            }
        }
    }

    fn send_uci(&mut self) -> Result<(), Error> {
        self.send(GuiCommand::Uci)?;
        self.recv_uci_ok()
    }

    fn recv(&mut self) -> Option<EngineCommand> {
        let start = Instant::now();
        let mut finished = false;
        let mut received_one = false;
        while !finished {
            let command = self.receiver.try_recv();
            match command {
                Ok(c) => {
                    self.commands.push(c.clone());
                    received_one = true;
                },
                Err(x) => {
                    println!("Error Receiving: {}", x);   
                    finished = start.elapsed() > Duration::from_millis(100) ||
                               received_one;
                }
            };
            std::thread::sleep(Duration::from_millis(50));
        }

        for i in self.last_received_index..self.commands.len() {
            match self.commands[i] {
                Command::Engine(ref x) => {
                    self.last_received_index = i + 1;
                    println!("Returning {}", x);
                    return Some(x.clone());
                },
                ref y => {
                    println!("Not Sending {}", y);
                }
            }
        }
        None
    }

    fn recv_uci_ok(&mut self) -> Result<(), Error> {
        while match self.recv() {
            Some(ref c) => c != &EngineCommand::UciOk,
            None => return Err(Error::CommandError),
        } {}
        Ok(())
    }

    fn send_isready(&mut self) -> Result<(), Error> {
        self.send(GuiCommand::IsReady)?;
        self.recv_ready_ok()
    }

    fn recv_ready_ok(&mut self) -> Result<(), Error> {
        while match self.recv() {
            Some(ref c) => c != &EngineCommand::ReadyOk,
            None => return Err(Error::CommandError),
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
    e.recv_best_move().unwrap();
}
