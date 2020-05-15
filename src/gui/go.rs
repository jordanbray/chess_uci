use chess::ChessMove;
use parsers::*;

use nom::IResult;
use nom::combinator::{map, complete, value};
use nom::bytes::streaming::tag;
use nom::multi::fold_many1;
use nom::branch::alt;
use nom::sequence::tuple;

#[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
pub struct Go {
    search_moves: Vec<ChessMove>,
    ponder: Option<ChessMove>,
    wtime: Option<u64>,
    btime: Option<u64>,
    winc: Option<u64>,
    binc: Option<u64>,
    movestogo: Option<u64>,
    depth: Option<u64>,
    nodes: Option<u64>,
    mate: Option<u64>,
    movetime: Option<u64>,
    infinite: bool,
}

impl Go {
    pub fn get_search_moves(&self) -> &Vec<ChessMove> {
        &self.search_moves
    }

    pub fn get_ponder(&self) -> Option<ChessMove> {
        self.ponder
    }

    pub fn get_wtime(&self) -> Option<u64> {
        self.wtime
    }

    pub fn get_btime(&self) -> Option<u64> {
        self.btime
    }

    pub fn get_winc(&self) -> Option<u64> {
        self.winc
    }

    pub fn get_binc(&self) -> Option<u64> {
        self.binc
    }

    pub fn get_movestogo(&self) -> Option<u64> {
        self.movestogo
    }

    pub fn get_depth(&self) -> Option<u64> {
        self.depth
    }

    pub fn get_nodes(&self) -> Option<u64> {
        self.nodes
    }

    pub fn get_mate(&self) -> Option<u64> {
        self.mate
    }

    pub fn get_movetime(&self) -> Option<u64> {
        self.movetime
    }

    pub fn get_infinite(&self) -> bool {
        self.infinite
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
        pub fn $name(a: $type) -> Go {
            let mut result = Go::default();
            result.$name = a.clone();
            result
        }
    };
}

macro_rules! add_builder_option {
    ($name:ident, $type:ty) => {
        pub fn $name(a: $type) -> Go {
            let mut result = Go::default();
            result.$name = Some(a.clone());
            result
        }
    };
}

impl Go {
    add_builder!(search_moves, Vec<ChessMove>);
    add_builder_option!(ponder, ChessMove);
    add_builder_option!(wtime, u64);
    add_builder_option!(btime, u64);
    add_builder_option!(winc, u64);
    add_builder_option!(binc, u64);
    add_builder_option!(movestogo, u64);
    add_builder_option!(depth, u64);
    add_builder_option!(nodes, u64);
    add_builder_option!(mate, u64);
    add_builder_option!(movetime, u64);
    add_builder!(infinite, bool);

    pub fn combine(&self, b: &Go) -> Go {
        let mut result = Go::default();

        set_non_default!(result, self, b, search_moves);
        set_non_default!(result, self, b, ponder);
        set_non_default!(result, self, b, wtime);
        set_non_default!(result, self, b, btime);
        set_non_default!(result, self, b, winc);
        set_non_default!(result, self, b, binc);
        set_non_default!(result, self, b, movestogo);
        set_non_default!(result, self, b, depth);
        set_non_default!(result, self, b, nodes);
        set_non_default!(result, self, b, mate);
        set_non_default!(result, self, b, movetime);
        set_non_default!(result, self, b, infinite);

        result
    }
}

fn parse_go_wtime(input: &str) -> IResult<&str, Go> {
    map(
        tuple((
            space,
            tag("wtime"),
            space,
            integer
        )),
        |(_, _, _, val)| Go::wtime(val)
    )(input)
}

fn parse_go_btime(input: &str) -> IResult<&str, Go> {
    map(
        tuple((
            space,
            tag("btime"),
            space,
            integer
        )),
        |(_, _, _, val)| Go::btime(val)
    )(input)
}

fn parse_go_winc(input: &str) -> IResult<&str, Go> {
    map(
        tuple((
            space,
            tag("btime"),
            space,
            integer
        )),
        |(_, _, _, winc)| Go::winc(winc)
    )(input)
}

fn parse_go_binc(input: &str) -> IResult<&str, Go> {
    map(
        tuple((
            space,
            tag("binc"),
            space,
            integer,
        )),
        |(_, _, _, binc)| Go::binc(binc)
    )(input)
}

fn parse_go_movestogo(input: &str) -> IResult<&str, Go> {
    map(
        tuple((
            space,
            tag("movestogo"),
            space,
            integer,
        )),
        |(_, _, _, movestogo)| Go::movestogo(movestogo)
    )(input)
}

fn parse_go_depth(input: &str) -> IResult<&str, Go> {
    map(
        tuple((
            space,
            tag("depth"),
            space,
            integer,
        )),
        |(_, _, _, depth)| Go::depth(depth)
    )(input)
}

fn parse_go_nodes(input: &str) -> IResult<&str, Go> {
    map(
        tuple((
            space,
            tag("nodes"),
            space,
            integer,
        )),
        |(_, _, _, nodes)| Go::nodes(nodes)
    )(input)
}

fn parse_go_mate(input: &str) -> IResult<&str, Go> {
    map(
        tuple((
            space,
            tag("mate"),
            space,
            integer,
        )),
        |(_, _, _, mate)| Go::mate(mate)
    )(input)
}

fn parse_go_movetime(input: &str) -> IResult<&str, Go> {
    map(
        tuple((
            space,
            tag("movetime"),
            space,
            integer,
        )),
        |(_, _, _, mate)| Go::movetime(mate)
    )(input)
}

fn parse_go_infinite(input: &str) -> IResult<&str, Go> {
    value(
        Go::infinite(true),
        tuple((
            space,
            tag("infinite")
        ))
    )(input)
}

fn parse_go_ponder(input: &str) -> IResult<&str, Go> {
    map(
        tuple((
            space,
            tag("ponder"),
            space,
            parse_move,
        )),
        |(_, _, _, m)| Go::ponder(m)
    )(input)
}

fn parse_go_searchmoves(input: &str) -> IResult<&str, Go> {
    map(
        tuple((
            space,
            tag("searchmoves"),
            space,
            parse_movelist
        )),
        |(_, _, _, moves)| Go::search_moves(moves.to_vec())
    )(input)
}

pub fn parse_go(input: &str) -> IResult<&str, Go> {
    map(
        tuple((
            tag("go"),
            fold_many1(
                alt((
                    complete(parse_go_wtime),
                    complete(parse_go_btime),
                    complete(parse_go_winc),
                    complete(parse_go_binc),
                    complete(parse_go_movestogo),
                    complete(parse_go_depth),
                    complete(parse_go_nodes),
                    complete(parse_go_mate),
                    complete(parse_go_movetime),
                    complete(parse_go_infinite),
                    complete(parse_go_ponder),
                    complete(parse_go_searchmoves),
                )),
                Go::default(),
                |acc: Go, next: Go| acc.combine(&next)
            ),
        )),
        |(_, go)| go
    )(input)
}
