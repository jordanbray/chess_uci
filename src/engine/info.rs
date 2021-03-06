use error::Error;
use std::fmt;
use std::str::FromStr;

use chess::ChessMove;
use engine::score::{parse_score, Score};
use parsers::*;

#[cfg(test)]
use chess::{File, Rank, Square};

use nom::IResult;
use nom::combinator::{map, complete};
use nom::bytes::streaming::tag;
use nom::multi::fold_many1;
use nom::branch::alt;
use nom::sequence::tuple;


#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Info {
    depth: Option<u64>,
    seldepth: Option<u64>,
    time: Option<u64>,
    nodes: Option<u64>,
    pv: Vec<ChessMove>,
    multi_pv: Option<u64>,
    score: Option<Score>,
    cur_move: Option<ChessMove>,
    cur_move_number: Option<u64>,
    hash_full: Option<f32>,
    nps: Option<u64>,
    tb_hits: Option<u64>,
    cpu_load: Option<f32>,
    engine_string: Option<String>,
    refutation: Vec<ChessMove>,
    cur_line: Vec<ChessMove>,
}

impl Info {
    pub fn get_depth(&self) -> Option<u64> {
        self.depth
    }

    pub fn get_seldepth(&self) -> Option<u64> {
        self.seldepth
    }

    pub fn get_time(&self) -> Option<u64> {
        self.time
    }

    pub fn get_nodes(&self) -> Option<u64> {
        self.nodes
    }

    pub fn get_pv(&self) -> &Vec<ChessMove> {
        &self.pv
    }

    pub fn get_multi_pv(&self) -> Option<u64> {
        self.multi_pv
    }

    pub fn get_score(&self) -> Option<Score> {
        self.score
    }

    pub fn cur_get_move(&self) -> Option<ChessMove> {
        self.cur_move
    }

    pub fn get_cur_move_number(&self) -> Option<u64> {
        self.cur_move_number
    }

    pub fn get_hash_full(&self) -> Option<f32> {
        self.hash_full
    }

    pub fn get_nps(&self) -> Option<u64> {
        self.nps
    }

    pub fn get_tbhits(&self) -> Option<u64> {
        self.tb_hits
    }

    pub fn get_cpu_load(&self) -> Option<f32> {
        self.cpu_load
    }

    pub fn get_engine_string(&self) -> &Option<String> {
        &self.engine_string
    }

    pub fn get_refutation(&self) -> &Vec<ChessMove> {
        &self.refutation
    }

    pub fn get_cur_line(&self) -> &Vec<ChessMove> {
        &self.cur_line
    }
}

macro_rules! set_non_default {
    ($result:ident, $a:ident, $b:ident, $val:ident) => {
        if $result.$val == $b.$val {
            $result.$val = $a.$val.clone();
        } else {
            $result.$val = $b.$val.clone();
        }
    };
}

macro_rules! add_builder {
    ($name:ident, $type:ty) => {
        pub fn $name(a: $type) -> Info {
            let mut result = Info::default();
            result.$name = a.clone();
            result
        }
    };
}

macro_rules! add_builder_option {
    ($name:ident, $type:ty) => {
        pub fn $name(a: $type) -> Info {
            let mut result = Info::default();
            result.$name = Some(a.clone());
            result
        }
    };
}

impl Info {
    add_builder!(pv, Vec<ChessMove>);
    add_builder!(refutation, Vec<ChessMove>);
    add_builder!(cur_line, Vec<ChessMove>);
    add_builder_option!(depth, u64);
    add_builder_option!(seldepth, u64);
    add_builder_option!(time, u64);
    add_builder_option!(nodes, u64);
    add_builder_option!(multi_pv, u64);
    add_builder_option!(score, Score);
    add_builder_option!(cur_move, ChessMove);
    add_builder_option!(cur_move_number, u64);
    add_builder_option!(hash_full, f32);
    add_builder_option!(nps, u64);
    add_builder_option!(tb_hits, u64);
    add_builder_option!(cpu_load, f32);
    add_builder_option!(engine_string, String);

    pub fn combine(&self, b: &Info) -> Info {
        let mut result = Info::default();

        set_non_default!(result, self, b, pv); // done
        set_non_default!(result, self, b, refutation);
        set_non_default!(result, self, b, cur_line);
        set_non_default!(result, self, b, depth); // done
        set_non_default!(result, self, b, seldepth); // done
        set_non_default!(result, self, b, time); // done
        set_non_default!(result, self, b, nodes); // done
        set_non_default!(result, self, b, multi_pv); // done
        set_non_default!(result, self, b, score); // done
        set_non_default!(result, self, b, cur_move); // done
        set_non_default!(result, self, b, cur_move_number); // done
        set_non_default!(result, self, b, hash_full);
        set_non_default!(result, self, b, nps); // done
        set_non_default!(result, self, b, tb_hits); // done
        set_non_default!(result, self, b, cpu_load);
        set_non_default!(result, self, b, engine_string);

        result
    }
}

fn parse_info_pv(input: &str) -> IResult<&str, Info> {
    map(
        tuple((
            space,
            tag("pv"),
            space,
            parse_movelist
        )),
        |(_, _, _, moves)| Info::pv(moves)
    )(input)
}

fn parse_info_depth(input: &str) -> IResult<&str, Info> {
    map(
        tuple((
            space,
            tag("depth"),
            space,
            integer,
        )),
        |(_, _, _, depth)| Info::depth(depth)
    )(input)
}

fn parse_info_seldepth(input: &str) -> IResult<&str, Info> {
    map(
        tuple((
            space,
            tag("seldepth"),
            space,
            integer,
        )),
        |(_, _, _, seldepth)| Info::seldepth(seldepth)
    )(input)
}

fn parse_info_time(input: &str) -> IResult<&str, Info> {
    map(
        tuple((
            space,
            tag("time"),
            space,
            integer
        )),
        |(_, _, _, time)| Info::time(time)
    )(input)
}

fn parse_info_nodes(input: &str) -> IResult<&str, Info> {
    map(
        tuple((
            space,
            tag("nodes"),
            space,
            integer,
        )),
        |(_, _, _, nodes)| Info::nodes(nodes)
    )(input)
}

fn parse_info_multi_pv(input: &str) -> IResult<&str, Info> {
    map(
        tuple((
            space,
            tag("multipv"),
            space,
            integer,
        )),
        |(_, _, _, mpv)| Info::multi_pv(mpv)
    )(input)
}

fn parse_info_score(input: &str) -> IResult<&str, Info> {
    map(
        tuple((
            space,
            parse_score
        )),
        |(_, score)| Info::score(score)
    )(input)
}

fn parse_info_cur_move(input: &str) -> IResult<&str, Info> {
    map(
        tuple((
            space,
            tag("currmove"),
            space,
            parse_move
        )),
        |(_, _, _, m)| Info::cur_move(m)
    )(input)
}

fn parse_info_cur_move_number(input: &str) -> IResult<&str, Info> {
    map(
        tuple((
            space,
            tag("currmovenumber"),
            space,
            integer
        )),
        |(_, _, _, i)| Info::cur_move_number(i)
    )(input)
}

fn parse_info_nps(input: &str) -> IResult<&str, Info> {
    map(
        tuple((
            space,
            tag("nps"),
            space,
            integer
        )),
        |(_, _, _, nps)| Info::nps(nps)
    )(input)
}

fn parse_info_tb_hits(input: &str) -> IResult<&str, Info> {
    map(
        tuple((
            space,
            tag("tbhits"),
            space,
            integer
        )),
        |(_, _, _, tb_hits)| Info::tb_hits(tb_hits)
    )(input)
}

pub fn parse_info(input: &str) -> IResult<&str, Info> {
    map(
        tuple((
            tag("info"),
            fold_many1(
                alt((
                    complete(parse_info_pv),
                    complete(parse_info_depth),
                    complete(parse_info_seldepth),
                    complete(parse_info_time),
                    complete(parse_info_nodes),
                    complete(parse_info_multi_pv),
                    complete(parse_info_score),
                    complete(parse_info_cur_move),
                    complete(parse_info_cur_move_number),
                    complete(parse_info_nps),
                    complete(parse_info_tb_hits)
                )),
                Info::default(),
                |acc: Info, next: Info| acc.combine(&next)
            ),
        )),
        |(_, info)| info
    )(input)
}

impl FromStr for Info {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_info(s)?.1)
    }
}

impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "info")?;

        if let Some(depth) = self.depth {
            write!(f, " depth {}", depth)?;
        }

        if let Some(seldepth) = self.seldepth {
            write!(f, " seldepth {}", seldepth)?;
        }

        if let Some(mpv) = self.multi_pv {
            write!(f, " multipv {}", mpv)?;
        }

        if let Some(score) = self.score {
            write!(f, " {}", score.to_string().trim())?;
        }

        if let Some(nodes) = self.nodes {
            write!(f, " nodes {}", nodes)?;
        }

        if let Some(time) = self.time {
            write!(f, " time {}", time)?;
        }

        if let Some(nps) = self.nps {
            write!(f, " nps {}", nps)?;
        }

        if let Some(cur_move) = self.cur_move {
            write!(f, " currmove {}", cur_move)?;
        }

        if let Some(cur_move_number) = self.cur_move_number {
            write!(f, " currmovenumber {}", cur_move_number)?;
        }

        if let Some(tb_hits) = self.tb_hits {
            write!(f, " tbhits {}", tb_hits)?;
        }

        if self.pv.len() > 0 {
            write!(f, " pv")?;
            for x in self.pv.iter() {
                write!(f, " {}", x)?;
            }
        }
        writeln!(f, "")
    }
}

#[cfg(test)]
fn test_info(s: &str, i: Info) {
    let parsed = Info::from_str(s);
    let text = i.to_string().trim().to_string();

    assert_eq!(parsed, Ok(i));
    assert_eq!(text, s.trim().to_string());
}

#[test]
fn test_normal_info() {
    let e2e4 = ChessMove::new(
        Square::make_square(Rank::Second, File::E),
        Square::make_square(Rank::Fourth, File::E),
        None,
    );
    let e7e5 = ChessMove::new(
        Square::make_square(Rank::Seventh, File::E),
        Square::make_square(Rank::Fifth, File::E),
        None,
    );

    test_info("info depth 2 seldepth 3 multipv 1 score cp 6 nodes 100 time 1 nps 1000 currmove e2e4 currmovenumber 1 tbhits 0 pv e2e4 e7e5\n",
              Info::pv(vec![e2e4, e7e5])
              .combine(&Info::depth(2))
              .combine(&Info::seldepth(3))
              .combine(&Info::multi_pv(1))
              .combine(&Info::nodes(100))
              .combine(&Info::time(1))
              .combine(&Info::score(Score::Cp(6)))
              .combine(&Info::cur_move(e2e4))
              .combine(&Info::cur_move_number(1))
              .combine(&Info::nps(1000))
              .combine(&Info::tb_hits(0)));
}
