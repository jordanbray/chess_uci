use error::Error;
use std::fmt;
use std::str::FromStr;

use nom::combinator::rest;
use nom::character::complete::alphanumeric1;
use parsers::*;

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum OptionType {
    Check(bool),
    Spin(i64, i64, i64),
    Combo(String, Vec<String>),
    Button,
    Str(String),
}

named!(parse_check<&str, OptionType>, do_parse!(
        tag!("check") >>
        space >>
        tag!("default") >>
        space >>
        v: alt!(complete!(value!(true, tag!("true"))) |
                complete!(value!(false, tag!("false")))) >>
        (OptionType::Check(v))
    )
);

named!(parse_spin<&str, OptionType>, do_parse!(
        tag!("spin") >>
        space >>
        tag!("default") >>
        space >>
        default: parse_i64 >>
        space >>
        tag!("min") >>
        space >>
        min: parse_i64 >>
        space >>
        tag!("max") >>
        space >>
        max: parse_i64 >>
        (OptionType::Spin(default, min, max))
    )
);

named!(parse_combo_var<&str, &str>, do_parse!(
        tag!("var") >>
        space >>
        x: alphanumeric1 >>
        (x)
    )
);

named!(parse_combo_var_space<&str, &str>, do_parse!(
        v: parse_combo_var >>
        space >>
        (v)
    )
);

named!(parse_combo<&str, OptionType>, do_parse!(
        tag!("combo") >>
        space >>
        tag!("default") >>
        space >>
        v: alphanumeric1 >>
        space >>
        options: fold_many1!(
            alt!(complete!(parse_combo_var_space) | complete!(parse_combo_var)),
            Vec::new(),
            |mut acc: Vec<String>, item: &str| {
                acc.push(item.to_string());
                acc
            }
        ) >>
        (OptionType::Combo(v.to_string(), options))
    )
);

named!(parse_button<&str, OptionType>, do_parse!(
        tag!("button") >>
        (OptionType::Button)
    )
);

named!(parse_nostring<&str, OptionType>, do_parse!(
        (OptionType::Str("".to_string()))
    )
);

named!(parse_somestring<&str, OptionType>, do_parse!(
        space >>
        v: rest >>
        (OptionType::Str(v.trim().to_string()))
    )
);

named!(parse_string<&str, OptionType>, do_parse!(
        tag!("string") >>
        space >>
        tag!("default") >>
        v: alt!(complete!(parse_somestring) | complete!(parse_nostring)) >>
        (v)
    )
);

named!(pub parse_option_type<&str, OptionType>, do_parse!(
        tag!("type") >>
        space >>
        v: alt!(complete!(parse_check) |
                complete!(parse_spin) |
                complete!(parse_combo) |
                complete!(parse_button) |
                complete!(parse_string)) >>
        (v)
    )
);

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
