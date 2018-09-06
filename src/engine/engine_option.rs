use std::fmt;
use std::str::FromStr;
use error::Error;

use parsers::*;
use engine::option_type::{OptionType, parse_option_type};

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct EngineOption {
    name: String,
    option_type: OptionType
}

named!(parse_engine_option<&str, EngineOption>, do_parse!(
        tag!("option") >>
        space >>
        tag!("name") >>
        space >>
        name: take_until!("type") >>
        option_type: parse_option_type >>
        (EngineOption { name: name.trim().to_string(), option_type: option_type })
    )
);

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
    test_engine_option("option name Contempt type spin default 0 min -100 max 100\n",
                       EngineOption { name: "Contempt".to_string(),
                                      option_type: OptionType::Spin(0, -100, 100) });
}

#[test]
fn test_engine_option_with_spaces() {
    test_engine_option("option name Debug Log File type string default\n",
                       EngineOption { name: "Debug Log File".to_string(),
                                      option_type: OptionType::Str("".to_string()) });
}

#[test]
fn test_engine_button() {
    test_engine_option("option name Clear Hash type button\n",
                       EngineOption { name: "Clear Hash".to_string(),
                                      option_type: OptionType::Button });
}
