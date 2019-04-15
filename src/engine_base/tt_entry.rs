use super::eval::Eval;
use super::tt_score::TtScore;
use chess::ChessMove;

pub struct TtEntry<T: Eval> {
    score: TtScore<T>,
    depth: i16,
    chess_move: ChessMove,
}

impl<T: Eval> TtEntry<T> {
    pub fn new_min(eval: T, depth: i16, chess_move: ChessMove) -> TtEntry<T> {
        TtEntry { score: TtScore::Min(eval), depth: depth, chess_move: chess_move }
    }

    pub fn new_max(eval: T, depth: i16, chess_move: ChessMove) -> TtEntry<T> {
        TtEntry { score: TtScore::Max(eval), depth: depth, chess_move: chess_move }
    }
    
    pub fn new_exact(eval: T, depth: i16, chess_move: ChessMove) -> TtEntry<T> {
        TtEntry { score: TtScore::Exact(eval), depth: depth, chess_move: chess_move }
    }

    pub fn skip_search(&self, depth: i16, alpha: T, beta: T) -> Option<(T, ChessMove)> {
        if depth <= self.depth {
            match self.score.skip_search(alpha, beta) {
                None => None,
                Some(x) => Some((x, self.chess_move))
            }
        } else {
            None
        }
    }

    pub fn update_alpha_beta(&self, depth: i16, alpha: T, beta: T) -> (T, T) {
        if depth <= self.depth {
            self.score.update_alpha_beta(alpha, beta)
        } else {
            (alpha, beta)
        }
    }

    pub fn get_move(&self) -> ChessMove {
        self.chess_move
    }
}

#[test]
fn test_skip_search() {
    let entry = TtEntry::new_min(16i32, 10, ChessMove::default());

    assert_eq!(entry.skip_search(15, -100, 0), None);
    assert_eq!(entry.skip_search(10, -100, 100), None);
    assert_eq!(entry.skip_search(10, -100, 0), Some((16i32, ChessMove::default())));
}

#[test]
fn test_update_alpha_beta() {
    let entry = TtEntry::new_min(16i32, 10, ChessMove::default());

    assert_eq!(entry.update_alpha_beta(15, -100, 100), (-100, 100));
    assert_eq!(entry.update_alpha_beta(10, -100, 100), (16i32, 100));
    assert_eq!(entry.update_alpha_beta(10, -100, 0), (16, 16));
}

