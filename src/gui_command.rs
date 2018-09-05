use std::fmt;
use std::str::FromStr;
use error::Error;
use nom::rest;
use chess::{ChessMove, Board};

#[cfg(test)]
use chess::{Rank, File, Piece, Square};

use parsers::*;
use go::*;

#[derive(Debug, PartialEq, Clone)]
pub enum GuiCommand {
    Uci,
    Debug(bool),
    IsReady,
    SetOption(String, Option<String>),
    Register(String),
    UciNewGame,
    Position(Board, Vec<ChessMove>),
    Go(Go),
    Stop,
    PonderHit,
    Quit
}

named!(parse_uci<&str, GuiCommand>, do_parse!(
        tag!("uci") >>
        (GuiCommand::Uci)
    )
);

named!(parse_debug<&str, GuiCommand>, do_parse!(
        tag!("debug") >>
        space >>
        a: alt!(value!(true, tag!("on")) | value!(false, tag!("off"))) >>
        (GuiCommand::Debug(a))
    )
);

named!(parse_isready<&str, GuiCommand>, do_parse!(
        tag!("isready") >>
        (GuiCommand::IsReady)
    )
);

named!(parse_setoption_value<&str, GuiCommand>, do_parse!(
        tag!("setoption") >>
        space >>
        tag!("name") >>
        space >>
        a: take_until!("value") >>
        tag!("value") >>
        b: rest >>
        (GuiCommand::SetOption(a.trim().to_string(), Some(b.trim().to_string())))
    )
);

named!(parse_setoption_novalue<&str, GuiCommand>, do_parse!(
        tag!("setoption") >>
        space >>
        tag!("name") >>
        space >>
        a: rest >>
        (GuiCommand::SetOption(a.trim().to_string(), None))
    )
);

named!(parse_register<&str, GuiCommand>, do_parse!(
        tag!("register") >>
        space >>
        token: rest >>
        (GuiCommand::Register(token.to_string()))
    )
);

named!(parse_ucinewgame<&str, GuiCommand>, do_parse!(
        tag!("ucinewgame") >>
        (GuiCommand::UciNewGame)
    )
);

named!(parse_stop<&str, GuiCommand>, do_parse!(
        tag!("stop") >>
        (GuiCommand::Stop)
    )
);

named!(parse_ponderhit<&str, GuiCommand>, do_parse!(
        tag!("ponderhit") >>
        (GuiCommand::PonderHit)
    )
);

named!(parse_quit<&str, GuiCommand>, do_parse!(
        tag!("quit") >>
        (GuiCommand::Quit)
    )
);

named!(parse_go<&str, GuiCommand>, do_parse!(
        tag!("go") >>
        go: fold_many1!(
            alt_complete!(
                parse_go_wtime |
                parse_go_btime |
                parse_go_winc |
                parse_go_binc |
                parse_go_movestogo |
                parse_go_depth |
                parse_go_nodes |
                parse_go_mate |
                parse_go_movetime |
                parse_go_infinite |
                parse_go_ponder |
                parse_go_searchmoves
            ),
            Go::default(),
            |acc: Go, item: Go| acc.combine(&item)) >>
        (GuiCommand::Go(go))
    )
);

named!(parse_position_fen<&str, Board>, do_parse!(
        tag!("fen") >>
        space >>
        board: parse_fen >>
        (board)
    )
);

named!(parse_position_startpos<&str, Board>, do_parse!(
        tag!("startpos") >>
        (Board::default())
    )
);

named!(parse_position_moves<&str, Vec<ChessMove>>, do_parse!(
        space >>
        tag!("moves") >>
        space >>
        moves: parse_movelist >>
        (moves.to_vec())
    )
);

named!(parse_position_moves_empty<&str, Vec<ChessMove>>, do_parse!(
        non_newline_space >>
        tag!("\n") >>
        (Vec::new())
    )
);

named!(parse_position<&str, GuiCommand>, do_parse!(
        tag!("position") >>
        space >>
        board: alt_complete!(parse_position_fen | parse_position_startpos) >>
        moves: alt_complete!(parse_position_moves | parse_position_moves_empty) >>
        (GuiCommand::Position(board, moves))
    )
);

named!(parse_all<&str, GuiCommand>, alt_complete!(
        parse_ucinewgame |
        parse_uci |
        parse_debug |
        parse_isready |
        parse_setoption_value |
        parse_setoption_novalue |
        parse_register |
        parse_stop |
        parse_ponderhit |
        parse_quit |
        parse_go |
        parse_position
    )
);

impl FromStr for GuiCommand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_all(s)?.1)
    }
}
impl fmt::Display for GuiCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GuiCommand::Uci => writeln!(f, "uci"),
            GuiCommand::Debug(val) => writeln!(f, "debug {}", if *val { "on" } else { "off" }),
            GuiCommand::IsReady => writeln!(f, "isready"),
            GuiCommand::SetOption(name, value) => match value {
                None => writeln!(f, "setoption name {}", name),
                Some(v) => writeln!(f, "setoption name {} value {}", name, v)
            },
            GuiCommand::Register(code) => writeln!(f, "register {}", code),
            GuiCommand::UciNewGame => writeln!(f, "ucinewgame"),
            GuiCommand::Position(pos, moves) => {
                if pos == &Board::default() {
                    try!(write!(f, "position startpos"));
                } else {
                    try!(write!(f, "position fen {}", ""));
                }

                if moves.len() != 0 {
                    writeln!(f, "{}", moves.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "))
                } else {
                    writeln!(f, "")
                }
            },
            GuiCommand::Go(go) => {
                try!(write!(f, "go"));
                match go.ponder {
                    Some(ref p) => try!(write!(f, "ponder {}", p)),
                    None => {},
                };

                if go.wtime.is_some() {
                    try!(write!(f, " wtime {}", go.wtime.unwrap()));
                }
                if go.btime.is_some() {
                    try!(write!(f, " btime {}", go.btime.unwrap()));
                }
                if go.winc.is_some() {
                    try!(write!(f, " winc {}", go.winc.unwrap()));
                }
                if go.binc.is_some() {
                    try!(write!(f, " binc {}", go.binc.unwrap()));
                }
                if go.movestogo.is_some() {
                    try!(write!(f, " movestogo {}", go.movestogo.unwrap()));
                }
                if go.depth.is_some() {
                    try!(write!(f, " depth {}", go.depth.unwrap()));
                }
                if go.nodes.is_some() {
                    try!(write!(f, " nodes {}", go.nodes.unwrap()));
                }
                if go.mate.is_some() {
                    try!(write!(f, " mate {}", go.mate.unwrap()));
                }
                if go.movetime.is_some() {
                    try!(write!(f, " movetime {}", go.movetime.unwrap()));
                }
                if go.infinite {
                    try!(write!(f, " infinite"));
                }

                if go.search_moves.len() != 0 {
                    try!(write!(f, " searchmoves {}", go.search_moves.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")));
                }
                writeln!(f, "")
            }
            GuiCommand::Stop => writeln!(f, "stop"),
            GuiCommand::PonderHit => writeln!(f, "ponderhit"),
            GuiCommand::Quit => writeln!(f, "quit")
        }
    }
}

#[cfg(test)]
fn test_parse(s: &str, c: GuiCommand) {
    let parsed = GuiCommand::from_str(s);
    assert_eq!(parsed, Ok(c));
}

#[test]
fn test_parse_gui() {
    test_parse("uci", GuiCommand::Uci);
}

#[test]
fn test_parse_debug_on() {
    test_parse("debug on", GuiCommand::Debug(true));
}

#[test]
fn test_parse_debug_off() {
    test_parse("debug off" , GuiCommand::Debug(false));
}

#[test]
fn test_parse_setoption_noval() {
    test_parse("setoption name test",
               GuiCommand::SetOption("test".to_string(), None));
}

#[test]
fn test_parse_setoption_withval() {
    test_parse("setoption name test value value",
               GuiCommand::SetOption("test".to_string(),
                                     Some("value".to_string())));
}

#[test]
fn test_isready() {
    test_parse("isready", GuiCommand::IsReady);
}

#[test]
fn test_registration() {
    test_parse("register code", GuiCommand::Register("code".to_string()));
}

#[test]
fn test_ucinewgame() {
    test_parse("ucinewgame", GuiCommand::UciNewGame);
}

#[test]
fn test_stop() {
    test_parse("stop", GuiCommand::Stop);
}

#[test]
fn test_ponderhit() {
    test_parse("ponderhit", GuiCommand::PonderHit);
}

#[test]
fn test_quit() {
    test_parse("quit", GuiCommand::Quit);
}

#[test]
fn test_parse_go_times() {
    test_parse("go btime 100 wtime 100\n",
               GuiCommand::Go(Go::wtime(100).combine(
                             &Go::btime(100))));
}

#[test]
fn test_parse_startpos() {
    test_parse("position startpos\n",
               GuiCommand::Position(Board::default(), vec!()));
}
#[test]
fn test_parse_startpos_moves() {
    let e2e4 = ChessMove::new(Square::make_square(Rank::Second, File::E),
                              Square::make_square(Rank::Fourth, File::E), None);
    let e7e5 = ChessMove::new(Square::make_square(Rank::Seventh, File::E),
                              Square::make_square(Rank::Fifth, File::E), None);

    test_parse("position startpos moves e2e4 e7e5\n",
               GuiCommand::Position(Board::default(), vec!(e2e4, e7e5)));
}

#[test]
fn test_position_fen() {
    test_parse("position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1\n", GuiCommand::Position(Board::default(), vec!()));
}

#[test]
fn test_parse_position_fen_moves() {
    let e2e4 = ChessMove::new(Square::make_square(Rank::Second, File::E),
                              Square::make_square(Rank::Fourth, File::E), None);
    let e7e5 = ChessMove::new(Square::make_square(Rank::Seventh, File::E),
                              Square::make_square(Rank::Fifth, File::E), None);

    test_parse("position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 moves e2e4 e7e5\n",
               GuiCommand::Position(Board::default(), vec!(e2e4, e7e5)));
}

#[test]
fn test_parse_queening_move() {
    let queening = ChessMove::new(Square::make_square(Rank::Seventh, File::E),
                                  Square::make_square(Rank::Eighth, File::E),
                                  Some(Piece::Queen));

    test_parse("position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 moves e7e8q\n", GuiCommand::Position(Board::default(), vec!(queening)));
}

