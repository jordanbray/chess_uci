use error::Error;
use std::fmt;
use std::str::FromStr;

use engine::option_type::{parse_option_type, OptionType};
use parsers::*;

use nom::combinator::map;
use nom::bytes::complete::{tag, take_until};
use nom::sequence::tuple;
use nom::IResult;

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct EngineOption {
    name: String,
    option_type: OptionType,
}

pub fn parse_engine_option(input: &str) -> IResult<&str, EngineOption> {
    map(
        tuple((
            tag("option"),
            space,
            tag("name"),
            space,
            take_until("type"),
            parse_option_type,
        )),
        |(_, _, _, _, name, option_type)| EngineOption { name: name.trim().to_string(), option_type }
    )(input)
}

impl EngineOption {
    pub fn new(name: String, option_type: OptionType) -> EngineOption {
        EngineOption {
            name,
            option_type,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_option_type(&self) -> &OptionType {
        &self.option_type
    }
}

impl FromStr for EngineOption {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_engine_option(s)?.1)
    }
}

impl fmt::Display for EngineOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "option name {} {}", self.name, self.option_type)
    }
}

#[cfg(test)]
fn test_engine_option(s: &str, e: EngineOption) {
    let parsed = EngineOption::from_str(s);
    let text = e.to_string().trim().to_string();

    assert_eq!(parsed, Ok(e));
    assert_eq!(text, s.trim().to_string());
}

#[test]
fn test_engine_option_contempt() {
    test_engine_option(
        "option name Contempt type spin default 0 min -100 max 100\n",
        EngineOption {
            name: "Contempt".to_string(),
            option_type: OptionType::Spin(0, -100, 100),
        },
    );
}

#[test]
fn test_engine_option_with_spaces() {
    test_engine_option(
        "option name Debug Log File type string default\n",
        EngineOption {
            name: "Debug Log File".to_string(),
            option_type: OptionType::Str("".to_string()),
        },
    );
}

#[test]
fn test_engine_button() {
    test_engine_option(
        "option name Clear Hash type button\n",
        EngineOption {
            name: "Clear Hash".to_string(),
            option_type: OptionType::Button,
        },
    );
}
