use arrayvec::ArrayVec;
use chess::ChessMove;
use nodrop::NoDrop;
use std::iter::IntoIterator;
use std::ops::Index;

const MAX_PLY: usize = 512;

pub struct Pv {
    pv: NoDrop<ArrayVec<[ChessMove; MAX_PLY]>>,
}

impl Pv {
    pub fn new() -> Pv {
        Pv {
            pv: NoDrop::new(ArrayVec::new()),
        }
    }

    pub fn update(&mut self, chess_move: ChessMove, other: &Pv) {
        self.clear();
        self.push(chess_move);
        for x in other.pv.iter() {
            self.push(*x);
        }
    }

    pub fn clear(&mut self) {
        self.pv.clear();
    }

    pub fn push(&mut self, m: ChessMove) {
        self.pv.push(m);
    }

    pub fn len(&self) -> usize {
        self.pv.len()
    }
}

impl Index<usize> for Pv {
    type Output = ChessMove;

    fn index(&self, index: usize) -> &ChessMove {
        &self.pv[index]
    }
}

impl IntoIterator for Pv {
    type Item = ChessMove;
    type IntoIter = ::arrayvec::IntoIter<[ChessMove; MAX_PLY]>;

    fn into_iter(self) -> Self::IntoIter {
        self.pv.clone().into_iter()
    }
}

impl Clone for Pv {
    fn clone(&self) -> Self {
        Pv {
            pv: NoDrop::new(self.pv.clone()),
        }
    }
}

#[cfg(test)]
use chess::{File, Rank, Square};

#[test]
fn update() {
    let mut pv1 = Pv::new();
    let e2e4 = ChessMove::new(
        Square::make_square(Rank::Second, File::E),
        Square::make_square(Rank::Fourth, File::E),
        None,
    );
    let d7d5 = ChessMove::new(
        Square::make_square(Rank::Seventh, File::D),
        Square::make_square(Rank::Fifth, File::D),
        None,
    );
    let mut pv2 = Pv::new();
    pv2.push(d7d5);
    pv1.update(e2e4, &pv2);
    assert_eq!(pv1[0], e2e4);
    assert_eq!(pv1[1], d7d5);
}
