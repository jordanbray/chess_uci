use std::fmt;
use std::str::FromStr;
use error::Error;

use parsers::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum Score {
    Cp(i64),
    Mate(i64),
    Lower(i64),
    Upper(i64)
}

named!(parse_score_cp<&str, Score>, do_parse!(
        tag!("cp") >>
        space >>
        v: parse_i64 >>
        (Score::Cp(v))
    )
);

named!(parse_score_mate<&str, Score>, do_parse!(
        tag!("mate") >>
        space >>
        v: parse_i64 >>
        (Score::Mate(v))
    )
);

named!(parse_score_lower<&str, Score>, do_parse!(
        tag!("lowerbound") >>
        space >>
        v: parse_i64 >>
        (Score::Lower(v))
    )
);

named!(parse_score_upper<&str, Score>, do_parse!(
        tag!("upperbound") >>
        space >>
        v: parse_i64 >>
        (Score::Upper(v))
    )
);

named!(pub parse_score<&str, Score>, do_parse!(
        tag!("score") >>
        space >>
        v: alt_complete!(parse_score_cp |
                         parse_score_mate |
                         parse_score_upper |
                         parse_score_lower) >>
        (v)
    )
);

impl FromStr for Score {
     type Err = Error;

     fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_score(s)?.1)
     }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Score::Cp(x) => writeln!(f, "score cp {}", x),
            Score::Mate(x) => writeln!(f, "score mate {}", x),
            Score::Lower(x) => writeln!(f, "score lowerbound {}", x),
            Score::Upper(x) => writeln!(f, "score upperbound {}", x),
        }
    }
}

#[cfg(test)]
fn test_parse(s: &str, score: Score) {
    let parsed = Score::from_str(s);
    let text = score.to_string().trim().to_string();

    assert_eq!(parsed, Ok(score));
    assert_eq!(text, s.trim().to_string());
}

#[test]
fn test_score_negative() {
    test_parse("score cp -100\n", Score::Cp(-100));
}
#[test]
fn test_score_zero() {
    test_parse("score cp 0\n", Score::Cp(0));
}

#[test]
fn test_score_cp() {
    test_parse("score cp 100\n", Score::Cp(100));
}

#[test]
fn test_score_mate() {
    test_parse("score mate 100\n", Score::Mate(100));
}

#[test]
fn test_score_upper() {
    test_parse("score upperbound 100\n", Score::Upper(100));
}

#[test]
fn test_score_lower() {
    test_parse("score lowerbound 100\n", Score::Lower(100));
}
