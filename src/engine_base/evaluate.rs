use super::eval::Eval;
use chess::{Board, Piece, Color};
use std::default::Default;

pub trait Evaluate<E: Eval> {
    fn evaluate(&mut self, board: Board, alpha: E, beta: E) -> E;
}

pub struct DefaultEvaluate {
    pawn: i32,
    knight: i32,
    bishop: i32,
    rook: i32,
    queen: i32,
}

impl Evaluate<i32> for DefaultEvaluate {
    fn evaluate(&mut self, board: Board, _alpha: i32, _beta: i32) -> i32 {
        let white = board.color_combined(Color::White);
        let black = board.color_combined(Color::Black);

        let pawns = board.pieces(Piece::Pawn);
        let knights = board.pieces(Piece::Knight);
        let bishops = board.pieces(Piece::Bishop);
        let rooks = board.pieces(Piece::Rook);
        let queens = board.pieces(Piece::Queen);

        let white_pawns = (white & pawns).popcnt() as i32;
        let black_pawns = (black & pawns).popcnt() as i32;
        
        let white_knights = (white & knights).popcnt() as i32;
        let black_knights = (black & knights).popcnt() as i32;

        let white_bishops = (white & bishops).popcnt() as i32;
        let black_bishops = (black & bishops).popcnt() as i32;

        let white_rooks = (white & rooks).popcnt() as i32;
        let black_rooks = (black & rooks).popcnt() as i32;

        let white_queens = (white & queens).popcnt() as i32;
        let black_queens = (black & queens).popcnt() as i32;

        self.pawn * (white_pawns - black_pawns) +
            self.knight * (white_knights - black_knights) +
            self.bishop * (white_bishops - black_bishops) +
            self.rook * (white_rooks - black_rooks) +
            self.queen * (white_queens - black_queens)
    }
}

impl Default for DefaultEvaluate {
    fn default() -> Self {
        DefaultEvaluate {
            pawn: 100,
            knight: 295,
            bishop: 330,
            rook: 500,
            queen: 900,
        }
    }
}

#[test]
fn should_be_equal() {
    let mut evaluator = DefaultEvaluate::default();
    assert_eq!(evaluator.evaluate(Board::default(), -100, 100), 0);
}

