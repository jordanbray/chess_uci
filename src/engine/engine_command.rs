use error::Error;
use std::fmt;
use std::str::FromStr;

use engine::best_move::{parse_best_move, BestMove};
use engine::copyprotection::{parse_copyprotection, CopyProtection};
use engine::engine_option::{parse_engine_option, EngineOption};
use engine::id::{parse_engine_id, Id};
use engine::info::{parse_info, Info};
use engine::registration::{parse_registration, Registration};

use nom::IResult;
use nom::combinator::{map, value, complete};
use nom::bytes::streaming::tag;
use nom::branch::alt;

#[cfg(test)]
use chess::{ChessMove, File, Rank, Square};
#[cfg(test)]
use engine::option_type::OptionType;
#[cfg(test)]
use engine::score::Score;

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

fn parse_engine_command_id(input: &str) -> IResult<&str, EngineCommand> {
    map(parse_engine_id,
        |value| EngineCommand::Id(value)
    )(input)
}

fn parse_engine_command_uciok(input: &str) -> IResult<&str, EngineCommand> {
    value(EngineCommand::UciOk, tag("uciok"))(input)
}

fn parse_engine_command_readyok(input: &str) -> IResult<&str, EngineCommand> {
    value(EngineCommand::ReadyOk, tag("readyok"))(input)
}

fn parse_engine_command_best_move(input: &str) -> IResult<&str, EngineCommand> {
    map(parse_best_move,
        |m| EngineCommand::BestMove(m)
    )(input)
}

fn parse_engine_command_copy_protection(input: &str) -> IResult<&str, EngineCommand> {
    map(parse_copyprotection,
        |c| EngineCommand::CopyProtection(c)
    )(input)
}

fn parse_engine_command_registration(input: &str) -> IResult<&str, EngineCommand> {
    map(parse_registration,
        |r| EngineCommand::Registration(r)
    )(input)
}

fn parse_engine_command_info(input: &str) -> IResult<&str, EngineCommand> {
    map(parse_info,
        |i| EngineCommand::Info(i)
    )(input)
}

fn parse_engine_command_engine_option(input: &str) -> IResult<&str, EngineCommand> {
    map(parse_engine_option,
        |o| EngineCommand::EngineOption(o)
    )(input)
}

fn parse_engine_command(input: &str) -> IResult<&str, EngineCommand> {
    alt((
        complete(parse_engine_command_id),
        complete(parse_engine_command_uciok),
        complete(parse_engine_command_readyok),
        complete(parse_engine_command_best_move),
        complete(parse_engine_command_copy_protection),
        complete(parse_engine_command_registration),
        complete(parse_engine_command_info),
        complete(parse_engine_command_engine_option),
    ))(input)
}

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
    test_parse(
        "id name test engine\n",
        EngineCommand::Id(Id::name("test engine")),
    );
}

#[test]
fn test_engine_command_id_author() {
    test_parse(
        "id author Jordan Bray\n",
        EngineCommand::Id(Id::author("Jordan Bray")),
    );
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
    let e2e4 = ChessMove::new(
        Square::make_square(Rank::Second, File::E),
        Square::make_square(Rank::Fourth, File::E),
        None,
    );

    test_parse(
        "bestmove e2e4\n",
        EngineCommand::BestMove(BestMove::new(e2e4)),
    );
}

#[test]
fn test_engine_command_copy_protection() {
    test_parse(
        "copyprotection ok\n",
        EngineCommand::CopyProtection(CopyProtection::Good),
    );
}

#[test]
fn test_engine_command_registration() {
    test_parse(
        "registration ok\n",
        EngineCommand::Registration(Registration::Good),
    );
}

#[test]
fn test_engine_command_info() {
    let e2e4 = ChessMove::new(
        Square::make_square(Rank::Second, File::E),
        Square::make_square(Rank::Fourth, File::E),
        None,
    );
    let e7e5 = ChessMove::new(
        Square::make_square(Rank::Seventh, File::E),
        Square::make_square(Rank::Fifth, File::E),
        None,
    );

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
    test_parse(
        "option name Contempt type spin default 0 min -100 max 100\n",
        EngineCommand::EngineOption(EngineOption::new(
            "Contempt".to_string(),
            OptionType::Spin(0, -100, 100),
        )),
    );
}
