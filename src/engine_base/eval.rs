use chess::Color;
use num_traits::{Num, NumAssign, NumAssignOps, NumOps, Signed, Bounded, NumCast, PrimInt};
use std::cmp::{PartialOrd, PartialEq};
use std::fmt::Debug;

pub trait Eval: Num + NumAssign + NumAssignOps + NumOps + Signed + Bounded + PartialEq + PartialOrd + NumCast + Debug + Copy  {
    fn max_supported_mates() -> Self;
    fn from_ply(ply: u16) -> Self;
    fn new_mate(ply: u16, color: Color) -> Self;
    fn depth_to_mate(&self) -> Option<i64>;
}

impl<T> Eval for T
where
    T: Num + NumAssign + NumAssignOps + NumOps + Signed + Bounded + PartialEq + PartialOrd + NumCast + Debug + Copy + PrimInt {
    fn max_supported_mates() -> Self {
        Self::from(800).expect("Failed to convert value 800 to Eval type.")
    }

    fn from_ply(ply: u16) -> Self {
        Self::from(ply).expect("Failed to convert u16 ply into Eval type.")
    }

    fn new_mate(ply: u16, color: Color) -> Self {
        match color {
            Color::White => Self::max_value() - (Self::from_ply(ply)),
            Color::Black => Self::min_value() + (Self::from_ply(ply)),
        }
    }

    fn depth_to_mate(&self) -> Option<i64> {
        if *self > Self::max_value() - Self::max_supported_mates() {
            Some((Self::max_value() - *self).to_i64().expect("Failed to convert mate depth to i64"))
        } else if *self < Self::min_value() + Self::max_supported_mates() {
            Some((Self::min_value() - *self).to_i64().expect("Failed to convert mate depth to i64"))
        } else {
            None
        }
    }
}

#[cfg(test)]
fn test_mates<T: Eval>() {
    assert_eq!(T::min_value(), T::new_mate(0, Color::Black));
    assert_eq!(T::max_value(), T::new_mate(0, Color::White));

    assert!(T::new_mate(0, Color::Black) < T::new_mate(1, Color::Black));
    assert!(T::new_mate(0, Color::White) > T::new_mate(1, Color::White));

    assert_eq!(T::new_mate(0, Color::Black).depth_to_mate(), Some(0));
    assert_eq!(T::new_mate(0, Color::White).depth_to_mate(), Some(0));

    assert_eq!(T::new_mate(10, Color::White).depth_to_mate(), Some(10));
    assert_eq!(T::new_mate(10, Color::Black).depth_to_mate(), Some(-10));
}


#[test]
fn test_mates_i16() {
    test_mates::<i16>();
}

#[test]
fn test_mates_i32() {
    test_mates::<i32>();
}
