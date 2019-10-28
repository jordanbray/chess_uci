use error::Error;
use std::fmt;
use std::str::FromStr;

use chess::ChessMove;
use parsers::*;

#[cfg(test)]
use chess::{File, Rank, Square};

#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct BestMove {
    chess_move: ChessMove,
    ponder_move: Option<ChessMove>,
}

impl BestMove {
    pub fn new(m: ChessMove) -> BestMove {
        BestMove {
            chess_move: m,
            ponder_move: None,
        }
    }

    pub fn new_with_ponder(m: ChessMove, ponder: ChessMove) -> BestMove {
        BestMove {
            chess_move: m,
            ponder_move: Some(ponder),
        }
    }

    pub fn get_move(&self) -> ChessMove {
        self.chess_move
    }

    pub fn get_ponder(&self) -> Option<ChessMove> {
        self.ponder_move
    }
}

named!(parse_best_move_noponder<&str, BestMove>, do_parse!(
        tag!("bestmove") >>
        space >>
        m: parse_move >>
        (BestMove::new(m))
    )
);

named!(parse_best_move_ponder<&str, BestMove>, do_parse!(
        tag!("bestmove") >>
        space >>
        m: parse_move >>
        space >>
        tag!("ponder") >>
        space >>
        p: parse_move >>
        (BestMove::new_with_ponder(m, p))
    )
);

named!(pub parse_best_move<&str, BestMove>, do_parse!(
        val: alt_complete!(parse_best_move_ponder | parse_best_move_noponder) >>
        (val)
    )
);

impl FromStr for BestMove {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_best_move(s)?.1)
    }
}

impl fmt::Display for BestMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bestmove {}", self.chess_move)?;
        match self.ponder_move {
            Some(x) => write!(f, " ponder {}", x)?,
            None => {}
        };

        writeln!(f, "")
    }
}

#[cfg(test)]
fn test_parse(s: &str, bm: BestMove) {
    let parsed = BestMove::from_str(s);
    let text = bm.to_string().trim().to_string();

    assert_eq!(parsed, Ok(bm));
    assert_eq!(text, s.trim().to_string());
}

#[test]
fn test_bestmove_ponder() {
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

    test_parse(
        "bestmove e2e4 ponder e7e5\n",
        BestMove::new_with_ponder(e2e4, e7e5),
    );
}

#[test]
fn test_bestmove_noponder() {
    let e2e4 = ChessMove::new(
        Square::make_square(Rank::Second, File::E),
        Square::make_square(Rank::Fourth, File::E),
        None,
    );

    test_parse("bestmove e2e4\n", BestMove::new(e2e4));
}
