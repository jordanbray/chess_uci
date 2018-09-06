use std::fmt;
use std::str::FromStr;
use error::Error;
use nom::rest;

use parsers::*;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Id {
    pub name: Option<String>,
    pub author: Option<String>,
}

impl Id {
    pub fn name(name: &str) -> Id {
        Id {
            name: Some(name.to_string()),
            author: None
        }
    }

    pub fn author(author: &str) -> Id {
        Id {
            name: None,
            author: Some(author.to_string())
        }
    }
}

named!(parse_engine_id_name<&str, Id>, do_parse!(
        tag!("id") >>
        space >>
        tag!("name") >>
        space >>
        val: rest >>
        (Id::name(val.trim()))
    )
);

named!(pub parse_engine_id_author<&str, Id>, do_parse!(
        tag!("id") >>
        space >>
        tag!("author") >>
        space >>
        val: rest >>
        (Id::author(val.trim()))
    )
);

named!(pub parse_engine_id<&str, Id>, do_parse!(
        val: alt_complete!(parse_engine_id_name | parse_engine_id_author) >>
        (val)
    )
);


impl FromStr for Id {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_engine_id(s)?.1)
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.name {
            Some(ref x) => try!(writeln!(f, "id name {}", x)),
            None => {}
        }
        match self.author {
            Some(ref x) => try!(writeln!(f, "id author {}", x)),
            None => {}
        }
        write!(f, "")
    }
}

#[cfg(test)]
fn test_parse(text: &str, id: Id) {
    let parsed = Id::from_str(text);
    let newstr = id.to_string().trim().to_string();

    assert_eq!(parsed, Ok(id));
    assert_eq!(newstr, text.trim().to_string());
}

#[test]
fn test_id_name() {
    test_parse("id name test engine\n", Id::name("test engine"));
}

#[test]
fn test_id_author() {
    test_parse("id author Jordan Bray\n", Id::author("Jordan Bray"));
}

