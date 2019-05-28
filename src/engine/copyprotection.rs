use error::Error;
use std::fmt;
use std::str::FromStr;

use parsers::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum CopyProtection {
    Good,
    Checking,
    Error,
}

named!(pub parse_copyprotection<&str, CopyProtection>, do_parse!(
        tag!("copyprotection") >>
        space >>
        val: alt_complete!(
                value!(CopyProtection::Good, tag!("ok")) |
                value!(CopyProtection::Checking, tag!("checking")) |
                value!(CopyProtection::Error, tag!("error"))
            ) >>
        (val)
    )
);

impl FromStr for CopyProtection {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_copyprotection(s)?.1)
    }
}

impl fmt::Display for CopyProtection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CopyProtection::Good => writeln!(f, "copyprotection ok"),
            CopyProtection::Checking => writeln!(f, "copyprotection checking"),
            CopyProtection::Error => writeln!(f, "copyprotection error"),
        }
    }
}

#[cfg(test)]
fn parse_copy(s: &str, c: CopyProtection) {
    let parsed = CopyProtection::from_str(s);
    let text = c.to_string().trim().to_string();

    assert_eq!(parsed, Ok(c));
    assert_eq!(text, s.trim().to_string());
}

#[test]
fn test_copyprotection_ok() {
    parse_copy("copyprotection ok\n", CopyProtection::Good);
}

#[test]
fn test_copyprotection_checking() {
    parse_copy("copyprotection checking\n", CopyProtection::Checking);
}

#[test]
fn test_copyprotection_error() {
    parse_copy("copyprotection error\n", CopyProtection::Error);
}
