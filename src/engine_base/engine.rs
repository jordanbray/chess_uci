use super::engine_options::EngineOptions;
use engine::Id;
use super::search::Search;
use chess_uci::timer::timer::Timer;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use crate::gui::gui_command;
use std::io::{Read, BufReader, Write};

struct EngineBase<E: Eval, T: TimeManager<E>, S: Search<E>> {
    options: EngineOptions,
    id: Id,
    searcher: S,
    timer: T,
    sent: HashMap<Instant, EngineCommand>,
    received: HashMap<Instant, GuiCommand>,
    _eval: PhantomData<E>,
}

impl<E: Eval, T: TimeManager<E>, S: Search<E>, W: Write> EngineBase<E, T, S, W> {
    pub fn new<R: BufReader>(time_manager: TimeManager, id: Id, options: EngineOptions) {
        EngineBase {
            options,
            id,
            timer,
            _eval: PhantomData,
        }
    }

    pub fn send<W: Write>(&mut self, command: EngineCommand, mut writer: W) {
        self.sent[Instant::now()] = command.clone();
        write!(writer, "{}", command);
    }

    pub fn main_loop<R: BufReader, W: Write>(&mut self, mut reader: R, mut writer: W) {
        let (tx, rx): (Sender<GuiCommand>, Receiver<GuiCommand>) = mpsc::channel();

        thread::spawn(move || {
            EngineBase::monitor_stdin(reader, tx);
        });

        let mut board = Board::default();
        let mut timer = Timer::new_static_move_time(Duration::from_secs(5));
        let mut moves_made = 0;

        for message in rx.iter() {
            received[Instant::now()] = message.clone();

            match message {
                GuiCommand::Uci => {
                    self.send(EngineCommand::UciOk, writer);
                },
                GuiCommand::IsReady => {
                    self.send(EngineCommand::ReadyOk, writer);
                },
                GuiCommand::Position(position, moves) => {
                    board = position;
                    moves_made = moves.len();
                    for x in moves {
                        board.make_move(*x);
                    }
                },
                GuiCommand::Go(go) => {
                    timer = Timer::new_from_go(go, board.side_to_move());
                    self.id_search(board, 1000, moves_made, writer);
                },
            }
        }
    }

    fn monitor_stdin(mut reader: BufReader, sender: Sender<GuiCommand>) {
        let mut handle = reader.lock();
        for line in handle.lines() {
            match GuiCommand::parse_str(line + "\n") {
                Ok(command) => sender.send(command).expect("Unable to send the GuiCommand to the main thread."),
                Err(_) => {}, // The standard states you should do nothing here.  Should we still do something?
            }
        }
    }
}


