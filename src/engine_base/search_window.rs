use super::eval::Eval;
use super::pv::Pv;
use chess::{Board, ChessMove};

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
    fn set_alpha(&mut self, alpha: E);
    fn beta(&self) -> E;
    fn depth(&self) -> i16;
    fn lower_depth(&self, chess_move: ChessMove) -> Self;
    fn board(&self) -> &Board;
    fn lower_depth_into_null_window(&self, chess_move: ChessMove) -> NullWindowSearchParams<E>;
    fn is_pv(&self) -> bool;
    fn update_pv(&mut self, _chess_move: ChessMove, _other: Self);
    fn clear_pv(&mut self);
}

impl<E: Eval> AlphaBetaSearchParams<E> {
    pub fn new(board: Board, alpha: E, beta: E, depth: i16) -> AlphaBetaSearchParams<E> {
        AlphaBetaSearchParams::<E> {
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

    fn set_alpha(&mut self, alpha: E) {
        self.alpha = alpha;
    }

    fn beta(&self) -> E {
        self.beta
    }

    fn board(&self) -> &Board {
        &self.board
    }

    fn clear_pv(&mut self) {
        self.pv.clear();
    }

    fn lower_depth(&self, chess_move: ChessMove) -> AlphaBetaSearchParams<E> {
        AlphaBetaSearchParams::<E> {
            board: self.board.make_move_new(chess_move),
            alpha: -self.beta.add_depth(-1),
            beta: -self.alpha.add_depth(-1),
            depth: self.depth - 1,
            pv: Pv::new(),
        }
    }

    fn lower_depth_into_null_window(&self, chess_move: ChessMove) -> NullWindowSearchParams<E> {
        NullWindowSearchParams::<E> {
            board: self.board.make_move_new(chess_move),
            score: -self.alpha.add_depth(-1),
            depth: self.depth - 1,
        }
    }

    fn is_pv(&self) -> bool {
        true
    }

    fn update_pv(&mut self, chess_move: ChessMove, other: AlphaBetaSearchParams<E>) {
        self.pv.update(chess_move, &other.pv);
    }

    fn depth(&self) -> i16 {
        self.depth
    }
}

impl<E: Eval> SearchParams<E> for NullWindowSearchParams<E> {
    fn alpha(&self) -> E {
        self.score - E::one()
    }

    fn set_alpha(&mut self, _alpha: E) {}

    fn beta(&self) -> E {
        self.score
    }

    fn board(&self) -> &Board {
        &self.board
    }

    fn lower_depth(&self, chess_move: ChessMove) -> NullWindowSearchParams<E> {
        NullWindowSearchParams::<E> {
            board: self.board.make_move_new(chess_move),
            score: E::one() - self.score.add_depth(-1),
            depth: self.depth - 1,
        }
    }

    fn update_pv(&mut self, _chess_move: ChessMove, _other: NullWindowSearchParams<E>) {}

    fn lower_depth_into_null_window(&self, chess_move: ChessMove) -> NullWindowSearchParams<E> {
        self.lower_depth(chess_move)
    }

    fn is_pv(&self) -> bool {
        false
    }

    fn depth(&self) -> i16 {
        self.depth
    }

    fn clear_pv(&mut self) {}
}

#[cfg(test)]
fn normal_window() -> AlphaBetaSearchParams<i32> {
    AlphaBetaSearchParams::new(Board::default(), -50, 100, 8)
}

#[test]
fn test_window() {
    let sp = normal_window();
    assert_eq!(sp.alpha(), -50);
    assert_eq!(sp.beta(), 100);
}
