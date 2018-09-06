use std::fmt;
use std::str::FromStr;
use error::Error;

use parsers::*;

use engine::{EngineId, EngineBestMove, EngineCopyProtection};

enum EngineCommand {
    Id(EngineId),
    UciOk,
    ReadyOk,
    BestMove(EngineBestMove),
    CopyProtection(EngineCopyProtection),
    Registration(EngineCopyProtection),
}

