use nom::Err;
use std::convert::From;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum Error {
    ParseError,
}

impl<'a> From<Err<&'a str>> for Error {
    fn from(_: Err<&'a str>) -> Error {
        Error::ParseError
    }
}
