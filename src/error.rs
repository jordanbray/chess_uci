use nom::error::ErrorKind;
use nom::Err;
use std::convert::From;
use std::fmt;
use std::io::Error as IoError;
use std::sync::mpsc::TryRecvError;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    SpawnError,
    SendError,
    RecvError,
    CommandError,
    IoError,
    EngineDeadError,
    NoCommandError,
    Timeout,
    IncompleteParseError,
    ParseError { text: String, error: ErrorKind },
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

impl From<Err<(&str, ErrorKind)>> for Error {
    fn from(x: Err<(&str, ErrorKind)>) -> Error {
        match x {
            Err::Incomplete(_) => Error::IncompleteParseError,
            Err::Error(y) => Error::ParseError {
                text: y.0.to_string(),
                error: y.1.clone(),
            },
            Err::Failure(y) => Error::ParseError {
                text: y.0.to_string(),
                error: y.1.clone(),
            },
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::SpawnError => write!(f, "Spawn Error"),
            Error::SendError => write!(f, "Send Error"),
            Error::RecvError => write!(f, "Recv Error"),
            Error::CommandError => write!(f, "Command Error"),
            Error::IoError => write!(f, "IO Error"),
            Error::NoCommandError => write!(f, "No comand could be read"),
            Error::EngineDeadError => write!(f, "Engine Dead"),
            Error::Timeout => write!(f, "Timeout"),
            Error::ParseError { text, error } => {
                write!(f, "Parse Error: {:?} on \"{}\"", error, text)
            }
            Error::IncompleteParseError => write!(f, "Incomplete Data - Parse Error"),
        }
    }
}
