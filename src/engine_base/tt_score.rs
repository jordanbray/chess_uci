use super::eval::Eval;

pub enum TtScore<T: Eval> {
    Min(T),
    Max(T),
    Exact(T)
}

impl<T: Eval> TtScore<T> {
    pub fn min(&self) -> T {
        match self {
            TtScore::Min(x) => *x,
            TtScore::Max(_) => T::min_value(),
            TtScore::Exact(x) => *x
        }
    }

    pub fn max(&self) -> T {
        match self {
            TtScore::Min(_) => T::max_value(),
            TtScore::Max(x) => *x,
            TtScore::Exact(x) => *x,
        }
    }

    pub fn skip_search(&self, alpha: T, beta: T) -> Option<T> {
        match self {
            TtScore::Exact(x) => Some(*x),
            TtScore::Min(x) => {
                if *x >= beta {
                    Some(*x)
                } else {
                    None
                }
            }, TtScore::Max(x) => {
                if *x <= alpha {
                    Some(*x)
                } else {
                    None
                }
            }
        }
    }

    pub fn update_alpha_beta(&self, alpha: T, beta: T) -> (T, T) {
        match self {
            TtScore::Exact(x) => (*x, *x),
            TtScore::Min(x) => {
                if alpha > *x {
                    (alpha, beta)
                } else if beta > *x {
                    (*x, beta)
                } else {
                    (*x, *x) // Skip search must trigger before this?
                }
            },
            TtScore::Max(x) => {
                if beta < *x {
                    (alpha, beta)
                } else if alpha < *x {
                    (alpha, *x)
                } else {
                    (*x, *x) // Skip search must trigger before this?
                }
            }
        }
    }
}

#[test]
fn min_max() {
    let min_score = TtScore::Min(16i32);
    let max_score = TtScore::Max(16i32);
    let exact_score = TtScore::Exact(16i32);

    assert_eq!(min_score.max(), i32::max_value());
    assert_eq!(min_score.min(), 16i32);

    assert_eq!(max_score.max(), 16i32);
    assert_eq!(max_score.min(), i32::min_value());

    assert_eq!(exact_score.min(), 16i32);
    assert_eq!(exact_score.max(), 16i32);
}

#[test]
fn skip_searching() {
    let min_score = TtScore::Min(16i32);
    let max_score = TtScore::Max(16i32);
    let exact_score = TtScore::Exact(16i32);

    let result = Some(16i32);
    
    assert_eq!(exact_score.skip_search(-100, 100), result);
    assert_eq!(exact_score.skip_search(-100, 0), result);
    assert_eq!(exact_score.skip_search(100, 200), result);

    assert_eq!(min_score.skip_search(-100, 100), None);
    assert_eq!(min_score.skip_search(-100, 0), result);
    assert_eq!(min_score.skip_search(100, 200), None);

    assert_eq!(max_score.skip_search(-100, 00), None);
    assert_eq!(max_score.skip_search(-100, 0), None);
    assert_eq!(max_score.skip_search(100, 200), result);
}

#[test]
fn update_alpha_beta() {
    let min_score = TtScore::Min(16i32);
    let max_score = TtScore::Max(16i32);
    let exact_score = TtScore::Exact(16i32);

    let small_alpha = -100;
    let small_beta = 0;

    let middle_alpha = -100;
    let middle_beta = 100;

    let large_alpha = 100;
    let large_beta = 200;

    assert_eq!(exact_score.update_alpha_beta(small_alpha, small_beta), (16i32, 16i32));
    assert_eq!(exact_score.update_alpha_beta(middle_alpha, middle_beta), (16i32, 16i32));
    assert_eq!(exact_score.update_alpha_beta(large_alpha, large_beta), (16i32, 16i32));

    assert_eq!(min_score.update_alpha_beta(small_alpha, small_beta), (16, 16)); // ??? Invalid Operation?
    assert_eq!(min_score.update_alpha_beta(middle_alpha, middle_beta), (16, 100));
    assert_eq!(min_score.update_alpha_beta(large_alpha, large_beta), (100, 200));

    assert_eq!(max_score.update_alpha_beta(small_alpha, small_beta), (-100, 0));
    assert_eq!(max_score.update_alpha_beta(middle_alpha, middle_beta), (-100, 16));
    assert_eq!(max_score.update_alpha_beta(large_alpha, large_beta), (16, 16)); // ??? Invalid operation?
}
