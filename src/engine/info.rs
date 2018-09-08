use std::fmt;
use std::str::FromStr;
use error::Error;

use parsers::*;
use chess::ChessMove;
use engine::score::{Score, parse_score};

#[cfg(test)]
use chess::{Square, Rank, File};

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

macro_rules! set_non_default {
    ($result:ident, $a:ident, $b:ident, $val:ident) => {
        if $result.$val == $b.$val {
            $result.$val = $a.$val.clone();
        } else {
            $result.$val = $b.$val.clone();
        }
    }
}

macro_rules! add_builder {
    ($name:ident, $type:ty) => {
        pub fn $name(a: $type) -> Info {
            let mut result = Info::default();
            result.$name = a.clone();
            result
        }
    }
}

macro_rules! add_builder_option {
    ($name:ident, $type:ty) => {
        pub fn $name(a: $type) -> Info {
            let mut result = Info::default();
            result.$name = Some(a.clone());
            result
        }
    }
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

named!(parse_info_pv<&str, Info>, do_parse!(
        space >>
        tag!("pv") >>
        space >>
        moves: parse_movelist >>
        (Info::pv(moves))
    )
);

named!(parse_info_depth<&str, Info>, do_parse!(
        space >>
        tag!("depth") >>
        space >>
        depth: integer >>
        (Info::depth(depth))
    )
);

named!(parse_info_seldepth<&str, Info>, do_parse!(
        space >>
        tag!("seldepth") >>
        space >>
        seldepth: integer >>
        (Info::seldepth(seldepth))
    )
);

named!(parse_info_time<&str, Info>, do_parse!(
        space >>
        tag!("time") >>
        space >>
        time: integer >>
        (Info::time(time))
    )
);

named!(parse_info_nodes<&str, Info>, do_parse!(
        space >>
        tag!("nodes") >>
        space >>
        nodes: integer >>
        (Info::nodes(nodes))
    )
);

named!(parse_info_multi_pv<&str, Info>, do_parse!(
        space >>
        tag!("multipv") >>
        space >>
        mpv: integer >>
        (Info::multi_pv(mpv))
    )
);

named!(parse_info_score<&str, Info>, do_parse!(
        space >>
        score: parse_score >>
        (Info::score(score))
    )
);

named!(parse_info_cur_move<&str, Info>, do_parse!(
        space >>
        tag!("currmove") >>
        space >>
        m: parse_move >>
        (Info::cur_move(m))
    )
);

named!(parse_info_cur_move_number<&str, Info>, do_parse!(
        space >>
        tag!("currmovenumber") >>
        space >>
        i: integer >>
        (Info::cur_move_number(i))
    )
);

named!(parse_info_nps<&str, Info>, do_parse!(
        space >>
        tag!("nps") >>
        space >>
        nps: integer >>
        (Info::nps(nps))
    )
);

named!(parse_info_tb_hits<&str, Info>, do_parse!(
        space >>
        tag!("tbhits") >>
        space >>
        tb_hits: integer >>
        (Info::tb_hits(tb_hits))
    )
);


named!(pub parse_info<&str, Info>, do_parse!(
        tag!("info") >>
        info: fold_many1!(
            alt_complete!(parse_info_pv |
                          parse_info_depth |
                          parse_info_seldepth |
                          parse_info_time |
                          parse_info_nodes |
                          parse_info_multi_pv |
                          parse_info_score |
                          parse_info_cur_move |
                          parse_info_cur_move_number |
                          parse_info_nps |
                          parse_info_tb_hits),
            Info::default(),
            |acc: Info, next: Info| acc.combine(&next)) >>
        (info)
    )
);

impl FromStr for Info {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_info(s)?.1)
    }
}

impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "info"));

        if let Some(depth) = self.depth {
            try!(write!(f, " depth {}", depth));
        }

        if let Some(seldepth) = self.seldepth {
            try!(write!(f, " seldepth {}", seldepth));
        }

        if let Some(mpv) = self.multi_pv {
            try!(write!(f, " multipv {}", mpv));
        }

        if let Some(score) = self.score {
            try!(write!(f, " {}", score.to_string().trim()));
        }
    
        if let Some(nodes) = self.nodes {
            try!(write!(f, " nodes {}", nodes));
        }

        if let Some(time) = self.time {
            try!(write!(f, " time {}", time));
        }

        if let Some(nps) = self.nps {
            write!(f, " nps {}", nps)?;
        }

        if let Some(cur_move) = self.cur_move {
            try!(write!(f, " currmove {}", cur_move));
        }

        if let Some(cur_move_number) = self.cur_move_number {
            try!(write!(f, " currmovenumber {}", cur_move_number));
        }

        if let Some(tb_hits) = self.tb_hits {
            write!(f, " tbhits {}", tb_hits)?;
        }

        if self.pv.len() > 0 {
            try!(write!(f, " pv"));
            for x in self.pv.iter() {
                try!(write!(f, " {}", x));
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
    let e2e4 = ChessMove::new(Square::make_square(Rank::Second, File::E),
                              Square::make_square(Rank::Fourth, File::E), None);
    let e7e5 = ChessMove::new(Square::make_square(Rank::Seventh, File::E),
                              Square::make_square(Rank::Fifth, File::E), None);


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
