use nom::Err;
use std::convert::From;
use std::fmt;
use std::io::Error as IoError;
use std::sync::mpsc::TryRecvError;

#[derive(Debug, PartialEq)]
pub enum Error {
    ParseError,
    SpawnError,
    SendError,
    RecvError,
    CommandError,
    IoError,
    EngineDeadError,
    NoCommandError,
    Timeout,
}

impl<'a> From<Err<&'a str>> for Error {
    fn from(_: Err<&'a str>) -> Error {
        Error::ParseError
    }
}

impl From<IoError> for Error {
    fn from(_: IoError) -> Error {
        Error::IoError
    }
}

impl From<TryRecvError> for Error {
    fn from(x: TryRecvError) -> Error {
        match x {
            TryRecvError::Empty => Error::NoCommandError,
            TryRecvError::Disconnected => Error::EngineDeadError,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ParseError => write!(f, "Parse Error"),
            Error::SpawnError => write!(f, "Spawn Error"),
            Error::SendError => write!(f, "Send Error"),
            Error::RecvError => write!(f, "Recv Error"),
            Error::CommandError => write!(f, "Command Error"),
            Error::IoError => write!(f, "IO Error"),
            Error::NoCommandError => write!(f, "No comand could be read"),
            Error::EngineDeadError => write!(f, "Engine Dead"),
            Error::Timeout => write!(f, "Timeout"),
        }
    }
}
