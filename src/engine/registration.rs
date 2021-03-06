use error::Error;
use std::fmt;
use std::str::FromStr;

use nom::IResult;
use nom::combinator::{map, complete, value};
use nom::bytes::streaming::tag;
use nom::branch::alt;
use nom::sequence::tuple;

use parsers::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum Registration {
    Good,
    Checking,
    Error,
}

pub fn parse_registration(input: &str) -> IResult<&str, Registration> {
    map(
        tuple((
            tag("registration"),
            space,
            alt((
                complete(value(Registration::Good, tag("ok"))),
                complete(value(Registration::Checking, tag("checking"))),
                complete(value(Registration::Error, tag("error"))),
            )),
        )),
        |(_, _, reg)| reg
    )(input)
}

impl FromStr for Registration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_registration(s)?.1)
    }
}

impl fmt::Display for Registration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Registration::Good => writeln!(f, "registration ok"),
            Registration::Checking => writeln!(f, "registration checking"),
            Registration::Error => writeln!(f, "registration error"),
        }
    }
}

#[cfg(test)]
fn test_registration(s: &str, c: Registration) {
    let parsed = Registration::from_str(s);
    let text = c.to_string().trim().to_string();

    assert_eq!(parsed, Ok(c));
    assert_eq!(text, s.trim().to_string());
}

#[test]
fn test_registration_ok() {
    test_registration("registration ok\n", Registration::Good);
}

#[test]
fn test_registration_checking() {
    test_registration("registration checking\n", Registration::Checking);
}

#[test]
fn test_registration_error() {
    test_registration("registration error\n", Registration::Error);
}
