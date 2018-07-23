use std::fmt;
use std::str::FromStr;
use error::Error;
use nom::{rest, digit};
use chess::{ChessMove, Square, Piece, Rank, File, Board};

#[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
pub struct Go {
    search_moves: Vec<ChessMove>,
    ponder: Option<ChessMove>,
    wtime: Option<u64>,
    btime: Option<u64>,
    winc: Option<u64>,
    binc: Option<u64>,
    movestogo: Option<u64>,
    depth: Option<u64>,
    nodes: Option<u64>,
    mate: Option<u64>,
    movetime: Option<u64>,
    infinite: bool
}

macro_rules! set_non_default {
    ($result:ident, $a:ident, $b:ident, $val:ident) => {
        if $result.$val == $b.$val {
            $result.$val = $a.$val.clone();
        } else {
            $result.$val = $b.$val.clone();
        }
    }
}

macro_rules! add_builder {
    ($name:ident, $type:ty) => {
        pub fn $name(a: $type) -> Go {
            let mut result = Go::default();
            result.$name = a.clone();
            result
        }
    }
}

macro_rules! add_builder_option {
    ($name:ident, $type:ty) => {
        pub fn $name(a: $type) -> Go {
            let mut result = Go::default();
            result.$name = Some(a.clone());
            result
        }
    }
}

impl Go {
    add_builder!(search_moves, Vec<ChessMove>);
    add_builder_option!(ponder, ChessMove);
    add_builder_option!(wtime, u64);
    add_builder_option!(btime, u64);
    add_builder_option!(winc, u64);
    add_builder_option!(binc, u64);
    add_builder_option!(movestogo, u64);
    add_builder_option!(depth, u64);
    add_builder_option!(nodes, u64);
    add_builder_option!(mate, u64);
    add_builder_option!(movetime, u64);
    add_builder!(infinite, bool);

    pub fn combine(&self, b: &Go) -> Go {
        let mut result = Go::default();

        set_non_default!(result, self, b, search_moves);
        set_non_default!(result, self, b, ponder);
        set_non_default!(result, self, b, wtime);
        set_non_default!(result, self, b, btime);
        set_non_default!(result, self, b, winc);
        set_non_default!(result, self, b, binc);
        set_non_default!(result, self, b, movestogo);
        set_non_default!(result, self, b, depth);
        set_non_default!(result, self, b, nodes);
        set_non_default!(result, self, b, mate);
        set_non_default!(result, self, b, movetime);
        set_non_default!(result, self, b, infinite);

        result
    }
}

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

named!(parse_rank<&str, Rank>, do_parse!(
        r: alt!(
            value!(Rank::First, tag!("1")) |
            value!(Rank::Second, tag!("2")) |
            value!(Rank::Third, tag!("3")) |
            value!(Rank::Fourth, tag!("4")) |
            value!(Rank::Fifth, tag!("5")) |
            value!(Rank::Sixth, tag!("6")) |
            value!(Rank::Seventh, tag!("7")) |
            value!(Rank::Eighth, tag!("8"))
        ) >>
        (r)
    )
);

named!(parse_file<&str, File>, do_parse!(
        f: alt!(
            value!(File::A, tag!("a")) |
            value!(File::B, tag!("b")) |
            value!(File::C, tag!("c")) |
            value!(File::D, tag!("d")) |
            value!(File::E, tag!("e")) |
            value!(File::F, tag!("f")) |
            value!(File::G, tag!("g")) |
            value!(File::H, tag!("h"))
        ) >>
        (f)
    )
);

named!(parse_square<&str, Square>, do_parse!(
        f: parse_file >>
        r: parse_rank >>
        (Square::make_square(r, f))
    )
);

named!(parse_promotion_piece<&str, Option<Piece>>, do_parse!(
        p: opt!(alt_complete!(
            value!(Piece::Knight, tag!("n")) |
            value!(Piece::Bishop, tag!("b")) |
            value!(Piece::Rook, tag!("r")) |
            value!(Piece::Queen, tag!("q"))
        )) >>
        (p)
    )
);

named!(parse_move<&str, ChessMove>, do_parse!(
        s1: parse_square >>
        s2: parse_square >>
        promotion: parse_promotion_piece >>
        (ChessMove::new(s1, s2, promotion))
    )
);

named!(space<&str, &str>, eat_separator!(" \t\r\n"));

named!(parse_uci<&str, GuiCommand>, do_parse!(
        tag!("uci") >>
        (GuiCommand::Uci)
    )
);

named!(parse_fen<&str, Board>, do_parse!(
        x: do_parse!(
            board: take_while_s!(|y| "pPnNbBrRqQkK12345678/".contains(y)) >>
            space >>
            player: alt!(tag!("w") | tag!("b")) >>
            space >>
            castle: take_while_s!(|y| "-kKqQ".contains(y)) >>
            space >>
            ep: take_while_s!(|y| "abcdefgh12345678-".contains(y)) >>
            space >>
            m1: take_while_s!(|y| "0123456789".contains(y)) >>
            space >>
            m2: take_while_s!(|y| "0123456789".contains(y)) >>
            board: expr_opt!(Board::from_fen(board.to_owned() +
                                             " " +
                                             player +
                                             " " +
                                             castle +
                                             " " +
                                             ep +
                                             " " +
                                             m1 +
                                             " " +
                                             m2)) >>
            (board)
        ) >>
        (x)
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

named!(
    integer<&str, u64>,
    map_res!(
        digit,
        u64::from_str
    )
);

named!(parse_go_wtime<&str, Go>, do_parse!(
        space >>
        tag!("wtime") >>
        space >>
        val: integer >>
        (Go::wtime(val))
    )
);

named!(parse_go_btime<&str, Go>, do_parse!(
        space >>
        tag!("btime") >>
        space >>
        val: integer >>
        (Go::btime(val))
    )
);

named!(parse_go_winc<&str, Go>, do_parse!(
        space >>
        tag!("winc") >>
        space >>
        val: integer >>
        (Go::winc(val))
    )
);

named!(parse_go_binc<&str, Go>, do_parse!(
        space >>
        tag!("binc") >>
        space >>
        val: integer >>
        (Go::binc(val))
    )
);

named!(parse_go_movestogo<&str, Go>, do_parse!(
        space >>
        tag!("movestogo") >>
        space >>
        val: integer >>
        (Go::movestogo(val))
    )
);

named!(parse_go_depth<&str, Go>, do_parse!(
        space >>
        tag!("depth") >>
        space >>
        val: integer >>
        (Go::depth(val))
    )
);

named!(parse_go_nodes<&str, Go>, do_parse!(
        space >>
        tag!("nodes") >>
        space >>
        val: integer >>
        (Go::nodes(val))
    )
);

named!(parse_go_mate<&str, Go>, do_parse!(
        space >>
        tag!("mate") >>
        space >>
        val: integer >>
        (Go::mate(val))
    )
);

named!(parse_go_movetime<&str, Go>, do_parse!(
        space >>
        tag!("movetime") >>
        space >>
        val: integer >>
        (Go::movetime(val))
    )
);

named!(parse_go_infinite<&str, Go>, do_parse!(
        space >>
        tag!("infinite") >>
        (Go::infinite(true))
    )
);

named!(parse_go_ponder<&str, Go>, do_parse!(
        space >>
        tag!("ponder") >>
        space >>
        m: parse_move >>
        (Go::ponder(m))
    )
);

named!(parse_movelist<&str, Vec<ChessMove> >, do_parse!(
        moves: fold_many1!(
            parse_move,
            Vec::new(),
            |mut acc: Vec<ChessMove>, item: ChessMove| {
                acc.push(item);
                acc
            }
        ) >>
        (moves.to_vec())
    )
);

named!(parse_go_searchmoves<&str, Go>, do_parse!(
        space >>
        tag!("searchmoves") >>
        space >>
        moves: parse_movelist >>
        (Go::search_moves(moves.to_vec()))
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
        tag!("moves") >>
        space >>
        moves: parse_movelist >>
        (moves.to_vec())
    )
);

named!(parse_position_moves_empty<&str, Vec<ChessMove>>, do_parse!(
        eof!() >>
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

#[cfg(test)]
fn test_parse(s: &str, c: GuiCommand) {
    let parsed = GuiCommand::from_str(s);
    assert_eq!(parsed, Ok(c));
}

#[test]
fn test_parse_gui() {
    test_parse("uci", GuiCommand::Uci);

    test_parse("debug on", GuiCommand::Debug(true));

    test_parse("debug off" , GuiCommand::Debug(false));

    test_parse("setoption name test", GuiCommand::SetOption("test".to_string(), None));
    
    test_parse("setoption name test value value",
               GuiCommand::SetOption("test".to_string(), Some("value".to_string())));
    
    test_parse("isready", GuiCommand::IsReady);

    test_parse("register code", GuiCommand::Register("code".to_string()));

    test_parse("ucinewgame", GuiCommand::UciNewGame);
   
    test_parse("stop", GuiCommand::Stop);

    test_parse("ponderhit", GuiCommand::PonderHit);

    test_parse("quit", GuiCommand::Quit);

    test_parse("go btime 100 wtime 100\n", GuiCommand::Go(Go::wtime(100).combine(&Go::btime(100))));
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

