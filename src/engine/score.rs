use crate::engine_base::eval::Eval;
use error::Error;
use num_traits::NumCast;
use parsers::*;
use std::convert::From;
use std::fmt;
use std::str::FromStr;

use nom::IResult;
use nom::combinator::{map, complete};
use nom::bytes::streaming::tag;
use nom::branch::alt;
use nom::sequence::tuple;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum Score {
    Cp(i64),
    Mate(i64),
    Lower(i64),
    Upper(i64),
}

fn parse_score_cp(input: &str) -> IResult<&str, Score> {
    map(
        tuple((
            tag("cp"),
            space,
            parse_i64,
        )),
        |(_, _, v)| Score::Cp(v)
    )(input)
}

fn parse_score_mate(input: &str) -> IResult<&str, Score> {
    map(
        tuple((
            tag("mate"),
            space,
            parse_i64,
        )),
        |(_, _, v)| Score::Mate(v)
    )(input)
}

fn parse_score_lower(input: &str) -> IResult<&str, Score> {
    map(
        tuple((
            tag("lowerbound"),
            space,
            parse_i64
        )),
        |(_, _, v)| Score::Lower(v)
    )(input)
}

fn parse_score_upper(input: &str) -> IResult<&str, Score> {
    map(
        tuple((
            tag("upperbound"),
            space,
            parse_i64,
        )),
        |(_, _, v)| Score::Upper(v)
    )(input)
}

pub fn parse_score(input: &str) -> IResult<&str, Score> {
    map(
        tuple((
            tag("score"),
            space,
            alt((
                complete(parse_score_cp),
                complete(parse_score_mate),
                complete(parse_score_upper),
                complete(parse_score_lower)
            )),
        )),
        |(_, _, score)| score
    )(input)
}

impl FromStr for Score {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_score(s)?.1)
    }
}

impl<E: Eval> From<E> for Score {
    fn from(eval: E) -> Score {
        if let Some(mate) = eval.depth_to_mate() {
            Score::Mate(mate)
        } else {
            Score::Cp(NumCast::from::<E>(eval).expect("eval is in the i64 range."))
        }
    }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Score::Cp(x) => writeln!(f, "score cp {}", x),
            Score::Mate(x) => writeln!(f, "score mate {}", x),
            Score::Lower(x) => writeln!(f, "score lowerbound {}", x),
            Score::Upper(x) => writeln!(f, "score upperbound {}", x),
        }
    }
}

#[cfg(test)]
fn test_parse(s: &str, score: Score) {
    let parsed = Score::from_str(s);
    let text = score.to_string().trim().to_string();

    assert_eq!(parsed, Ok(score));
    assert_eq!(text, s.trim().to_string());
}

#[test]
fn test_score_negative() {
    test_parse("score cp -100\n", Score::Cp(-100));
}
#[test]
fn test_score_zero() {
    test_parse("score cp 0\n", Score::Cp(0));
}

#[test]
fn test_score_cp() {
    test_parse("score cp 100\n", Score::Cp(100));
}

#[test]
fn test_score_mate() {
    test_parse("score mate 100\n", Score::Mate(100));
}

#[test]
fn test_score_upper() {
    test_parse("score upperbound 100\n", Score::Upper(100));
}

#[test]
fn test_score_lower() {
    test_parse("score lowerbound 100\n", Score::Lower(100));
}
