use chess::{Board, ChessMove, Square};
use std::str::FromStr;

pub fn super_easy_tactic() -> (Board, ChessMove) {
    (
        Board::from_str("3q1k2/8/8/8/8/8/8/3QK3 w - - 0 1").unwrap(),
        ChessMove::new(Square::D1, Square::D8, None),
    )
}

pub fn easy_tactic() -> (Board, ChessMove) {
    (
        Board::from_str("r5k1/p1p3bp/1p2q1p1/5p2/8/P1P4P/1P2BPP1/3QR1K1 w - - 0 1").unwrap(),
        ChessMove::new(Square::E2, Square::F3, None),
    )
}
