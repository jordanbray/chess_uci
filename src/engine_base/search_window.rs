use super::eval::Eval;
use super::pv::Pv;
use chess::{ChessMove, Board};

pub struct AlphaBetaSearchParams<E: Eval> {
    board: Board,
    alpha: E,
    beta: E,
    depth: i16,
    pv: Pv,
}

pub struct NullWindowSearchParams<E: Eval> {
    board: Board,
    score: E,
    depth: i16,
}

pub trait SearchParams<E: Eval> {
    fn alpha(&self) -> E;
    fn beta(&self) -> E;
    fn depth(&self) -> i16;
    fn lower_depth(&self, chess_move: ChessMove) -> Self;
    fn lower_depth_into_null_window(&self, chess_move: ChessMove) -> NullWindowSearchParams;
    fn is_pv(&self) -> bool;
    fn update_pv(&mut self, _chess_move: ChessMove, _other: Self) {
    }
}

impl<E: Eval> AlphaBetaSearchParams<E> {
    pub fn new(board: Board, alpha: E, beta: E, depth: i16) {
        AlphaBetaSearchParams {
            board: board,
            alpha: alpha,
            beta: beta,
            depth: depth,
            pv: Pv::new(),
        }
    }

    pub fn get_pv(self) -> Pv {
        self.pv
    }
}

impl<E: Eval> SearchParams<E> for AlphaBetaSearchParams<E> {
    fn alpha(&self) -> E {
        self.alpha
    }

    fn beta(&self) -> E {
        self.beta
    }

    fn lower_depth(&self, chess_move: ChessMove) -> Self {
        AlphaBetaWindow {
            board: board.make_move_new(chess_move),
            alpha: -self.beta.add_depth(-1),
            beta: -self.alpha.add_depth(-1),
            depth: self.depth - 1,
            pv: Pv::new(),
        }
    }

    fn lower_depth_into_null_window(&self, chess_move: ChessMove) -> NullWindowSearchParams {
        NullWindowSearchParams {
            board: board.make_move_new(chess_move),
            score: -self.alpha.add_depth(-1),
            depth: self.depth - 1,
        }
    }

    fn is_pv(&self) -> bool {
        true
    }

    fn update_pv(&mut self, chess_move: ChessMove, other: Self) {
        self.pv.update(chess_move, other.pv);
    }
}

impl<E: Eval> SearchParams<E> for NullWindowSearchParams<E> {
    fn alpha(&self) -> E {
        self.score - 1
    }

    fn beta(&self) -> E {
        self.score
    }

    fn lower_depth(&self, chess_move: ChessMove) -> Self {
        NullWindow {
            board: board.make_move_new(chess_move),
            score: -self.score.add_depth(-1),
            depth: self.depth - 1,
        }
    }

    fn lower_depth_into_null_window(&self) -> NullWindowSearchParams {
        self.lower_depth()
    }

    fn is_pv(&self) -> bool {
        false
    }
}

#[test]



