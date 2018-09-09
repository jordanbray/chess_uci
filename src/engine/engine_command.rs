use std::fmt;
use std::str::FromStr;
use error::Error;

use engine::id::{Id, parse_engine_id};
use engine::best_move::{BestMove, parse_best_move};
use engine::copyprotection::{CopyProtection, parse_copyprotection};
use engine::registration::{Registration, parse_registration};
use engine::info::{Info, parse_info};
use engine::engine_option::{EngineOption, parse_engine_option};

#[cfg(test)]
use chess::{Square, Rank, File, ChessMove};
#[cfg(test)]
use engine::score::Score;
#[cfg(test)]
use engine::option_type::OptionType;

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum EngineCommand {
    Id(Id),
    UciOk,
    ReadyOk,
    BestMove(BestMove),
    CopyProtection(CopyProtection),
    Registration(Registration),
    Info(Info),
    EngineOption(EngineOption),
}

named!(parse_engine_command_id<&str, EngineCommand>, do_parse!(
        value: parse_engine_id >>
        (EngineCommand::Id(value))
    )
);

named!(parse_engine_command_uciok<&str, EngineCommand>, do_parse!(
        tag!("uciok") >>
        (EngineCommand::UciOk)
    )
);

named!(parse_engine_command_readyok<&str, EngineCommand>, do_parse!(
        tag!("readyok") >>
        (EngineCommand::ReadyOk)
    )
);

named!(parse_engine_command_best_move<&str, EngineCommand>, do_parse!(
        value: parse_best_move >>
        (EngineCommand::BestMove(value))
    )
);

named!(parse_engine_command_copy_protection<&str, EngineCommand>, do_parse!(
        value: parse_copyprotection >>
        (EngineCommand::CopyProtection(value))
    )
);

named!(parse_engine_command_registration<&str, EngineCommand>, do_parse!(
        value: parse_registration >>
        (EngineCommand::Registration(value))
    )
);

named!(parse_engine_command_info<&str, EngineCommand>, do_parse!(
        value: parse_info >>
        (EngineCommand::Info(value))
    )
);

named!(parse_engine_command_engine_option<&str, EngineCommand>, do_parse!(
        value: parse_engine_option >>
        (EngineCommand::EngineOption(value))
    )
);

named!(parse_engine_command<&str, EngineCommand>, do_parse!(
        value: alt_complete!(parse_engine_command_id |
                             parse_engine_command_uciok |
                             parse_engine_command_readyok |
                             parse_engine_command_best_move |
                             parse_engine_command_copy_protection |
                             parse_engine_command_registration |
                             parse_engine_command_info |
                             parse_engine_command_engine_option) >>
        (value)
    )
);

impl FromStr for EngineCommand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_engine_command(s)?.1)
    }
}


impl fmt::Display for EngineCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EngineCommand::Id(x) => write!(f, "{}", x),
            EngineCommand::UciOk => writeln!(f, "uciok"),
            EngineCommand::ReadyOk => writeln!(f, "readyok"),
            EngineCommand::BestMove(x) => write!(f, "{}", x),
            EngineCommand::CopyProtection(x) => write!(f, "{}", x),
            EngineCommand::Registration(x) => write!(f, "{}", x),
            EngineCommand::Info(x) => write!(f, "{}", x),
            EngineCommand::EngineOption(x) => write!(f, "{}", x),
        }
    }
}

#[cfg(test)]
fn test_parse(s: &str, e: EngineCommand) {
    let parsed = EngineCommand::from_str(s);
    let text = e.to_string().trim().to_string();

    assert_eq!(parsed, Ok(e));
    assert_eq!(text, s.trim().to_string());
}

#[test]
fn test_engine_command_id_name() {
    test_parse("id name test engine\n", EngineCommand::Id(Id::name("test engine")));
}

#[test]
fn test_engine_command_id_author() {
    test_parse("id author Jordan Bray\n", EngineCommand::Id(Id::author("Jordan Bray")));
}

#[test]
fn test_engine_command_uciok() {
    test_parse("uciok\n", EngineCommand::UciOk);
}

#[test]
fn test_engine_command_readyok() {
    test_parse("readyok\n", EngineCommand::ReadyOk);
}

#[test]
fn test_engine_command_best_move() {
    let e2e4 = ChessMove::new(Square::make_square(Rank::Second, File::E),
                              Square::make_square(Rank::Fourth, File::E), None);

    test_parse("bestmove e2e4\n", EngineCommand::BestMove(BestMove::new(e2e4)));
}

#[test]
fn test_engine_command_copy_protection() {
    test_parse("copyprotection ok\n", EngineCommand::CopyProtection(CopyProtection::Good));
}

#[test]
fn test_engine_command_registration() {
    test_parse("registration ok\n", EngineCommand::Registration(Registration::Good));
}

#[test]
fn test_engine_command_info() {
    let e2e4 = ChessMove::new(Square::make_square(Rank::Second, File::E),
                              Square::make_square(Rank::Fourth, File::E), None);
    let e7e5 = ChessMove::new(Square::make_square(Rank::Seventh, File::E),
                              Square::make_square(Rank::Fifth, File::E), None);


    test_parse("info depth 2 seldepth 3 multipv 1 score cp 6 nodes 100 time 1 nps 1000 currmove e2e4 currmovenumber 1 tbhits 0 pv e2e4 e7e5\n",
              EngineCommand::Info(Info::pv(vec![e2e4, e7e5])
                        .combine(&Info::depth(2))
                        .combine(&Info::seldepth(3))
                        .combine(&Info::multi_pv(1))
                        .combine(&Info::nodes(100))
                        .combine(&Info::time(1))
                        .combine(&Info::score(Score::Cp(6)))
                        .combine(&Info::cur_move(e2e4))
                        .combine(&Info::cur_move_number(1))
                        .combine(&Info::nps(1000))
                        .combine(&Info::tb_hits(0))));
}

#[test]
fn test_engine_command_engine_option() {
    test_parse("option name Contempt type spin default 0 min -100 max 100\n",
                       EngineCommand::EngineOption(
                           EngineOption::new("Contempt".to_string(),
                                             OptionType::Spin(0, -100, 100))));
}

