use nom::Err;
use std::convert::From;
use std::fmt;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum Error {
    ParseError,
    SpawnError,
    SendError,
    RecvError,
    CommandError,
}

impl<'a> From<Err<&'a str>> for Error {
    fn from(_: Err<&'a str>) -> Error {
        Error::ParseError
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ParseError => writeln!(f, "Parse Error"),
            Error::SpawnError => writeln!(f, "Spawn Error"),
            Error::SendError => writeln!(f, "Send Error"),
            Error::RecvError => writeln!(f, "Recv Error"),
            Error::CommandError => writeln!(f, "Command Error"),
        }
    }
}
