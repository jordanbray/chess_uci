use error::Error;
use std::fmt;
use std::str::FromStr;

use parsers::*;

use nom::IResult;
use nom::combinator::{map, complete, value, rest};
use nom::bytes::streaming::tag;
use nom::branch::alt;
use nom::sequence::tuple;
use nom::multi::fold_many1;
use nom::character::complete::alphanumeric1;

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum OptionType {
    Check(bool),
    Spin(i64, i64, i64),
    Combo(String, Vec<String>),
    Button,
    Str(String),
}

fn parse_check(input: &str) -> IResult<&str, OptionType> {
    map(
        tuple((
            tag("check"),
            space,
            tag("default"),
            space,
            alt((
                complete(value(true, tag("true"))),
                complete(value(false, tag("false"))),
            )),
        )),
        |(_, _, _, _, v)| OptionType::Check(v)
    )(input)
}

fn parse_spin(input: &str) -> IResult<&str, OptionType> {
    map(
        tuple((
            tag("spin"),
            space,
            tag("default"),
            space,
            parse_i64,
            space,
            tag("min"),
            space,
            parse_i64,
            space,
            tag("max"),
            space,
            parse_i64
        )),
        |(_, _, _, _, def, _, _, _, min, _, _, _, max)| OptionType::Spin(def, min, max)
    )(input)
}

fn parse_combo_var(input: &str) -> IResult<&str, &str> {
    map(
        tuple((
            tag("var"),
            space,
            alphanumeric1
        )),
        |(_, _, x)| x
    )(input)
}

fn parse_combo_var_space(input: &str) -> IResult<&str, &str> {
    map(
        tuple((
            parse_combo_var,
            space
        )),
        |(x, _)| x
    )(input)
}

fn parse_combo(input: &str) -> IResult<&str, OptionType> {
    map(
        tuple((
            tag("combo"),
            space,
            tag("default"),
            space,
            alphanumeric1,
            space,
            fold_many1(
                alt((
                    complete(parse_combo_var_space),
                    complete(parse_combo_var)
                )),
                Vec::new(),
                |mut acc: Vec<String>, item: &str| {
                    acc.push(item.to_string());
                    acc
                }
            ),
        )),
        |(_, _, _, _, def, _, options)| OptionType::Combo(def.to_string(), options)
    )(input)
}

fn parse_button(input: &str) -> IResult<&str, OptionType> {
    value(OptionType::Button, tag("button"))(input)
}

fn parse_nostring(input: &str) -> IResult<&str, OptionType> {
    Ok((input, OptionType::Str(String::new())))
}

fn parse_somestring(input: &str) -> IResult<&str, OptionType> {
    map(
        tuple((
            space,
            rest
        )),
        |(_, v)| OptionType::Str(v.trim().to_string())
    )(input)
}

fn parse_string(input: &str) -> IResult<&str, OptionType> {
    map(
        tuple((
            tag("string"),
            space,
            tag("default"),
            alt((
                complete(parse_somestring),
                complete(parse_nostring),
            )),
        )),
        |(_, _, _, res)| res
    )(input)
}

pub fn parse_option_type(input: &str) -> IResult<&str, OptionType> {
    map(
        tuple((
            tag("type"),
            space,
            alt((
                complete(parse_check),
                complete(parse_spin),
                complete(parse_combo),
                complete(parse_button),
                complete(parse_string)
            )),
        )),
        |(_, _, v)| v
    )(input)
}

impl FromStr for OptionType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_option_type(s)?.1)
    }
}

impl fmt::Display for OptionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "type ")?;
        match self {
            OptionType::Check(x) => writeln!(f, "check default {}", x),
            OptionType::Spin(x, y, z) => writeln!(f, "spin default {} min {} max {}", x, y, z),
            OptionType::Combo(x, y) => {
                write!(f, "combo default {}", x)?;
                for z in y.into_iter() {
                    write!(f, " var {}", z)?;
                }
                writeln!(f, "")
            }
            OptionType::Button => writeln!(f, "button"),
            OptionType::Str(x) => writeln!(f, "string default {}", x),
        }
    }
}

#[cfg(test)]
fn test_option_type(s: &str, o: OptionType) {
    let parsed = OptionType::from_str(s);
    let text = o.to_string().trim().to_string();

    assert_eq!(parsed, Ok(o));
    assert_eq!(text, s.trim().to_string());
}

#[test]
fn test_option_type_check() {
    test_option_type("type check default true\n", OptionType::Check(true));
}

#[test]
fn test_option_type_spin() {
    test_option_type(
        "type spin default 89 min 10 max 1000\n",
        OptionType::Spin(89, 10, 1000),
    );
}

#[test]
fn test_option_type_combo() {
    test_option_type(
        "type combo default Normal var Solid var Normal var Risky\n",
        OptionType::Combo(
            "Normal".to_string(),
            vec![
                "Solid".to_string(),
                "Normal".to_string(),
                "Risky".to_string(),
            ],
        ),
    );
}

#[test]
fn test_option_type_button() {
    test_option_type("type button\n", OptionType::Button);
}

#[test]
fn test_option_type_string_empty() {
    test_option_type("type string default\n", OptionType::Str("".to_string()));
}

#[test]
fn test_option_type_string_full() {
    test_option_type(
        "type string default Jordan Bray\n",
        OptionType::Str("Jordan Bray".to_string()),
    );
}
