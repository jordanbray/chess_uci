use chess::Color;
use num_traits::{
    Bounded, Num, NumAssign, NumAssignOps, NumCast, NumOps, PrimInt, Signed, WrappingAdd,
    WrappingMul, WrappingSub,
};
use std::cmp::{PartialEq, PartialOrd};
use std::fmt::Debug;

pub trait Eval:
    Num
    + NumAssign
    + NumAssignOps
    + NumOps
    + Signed
    + Bounded
    + PartialEq
    + PartialOrd
    + NumCast
    + Debug
    + Copy
{
    fn max_supported_mates() -> Self;
    fn from_ply(ply: i16) -> Self;
    fn new_mate(ply: i16, color: Color) -> Self;
    fn depth_to_mate(&self) -> Option<i64>;
    fn add_depth(&self, amount: i16) -> Self;
    fn min_eval() -> Self;
    fn max_eval() -> Self;
    fn null() -> Self;
    fn bound(other: Self) -> Self;
}

// Number line for integers, used below:
// i??::min_value() == null evaluation,
// -i??::max_value() == -inf,
// -i??::max_value() + 1 == Black mates in 0
// -i??::max_value() + 2 == Black mates in 1
// ...
// -i??::max_value() + 801 = Black mates in 800
// -i??::max_value() + 802 = Really bad for white...
// ...
// i??::zero() = Equal
// ...
// i??::max_value() - 802 = Really bad for black...
// i??::max_value() - 801 = White mates in 800
// ...
// i??::max_value() - 2 == White mates in 1
// i??::max_value() - 1 == White mates in 0
// i??::max_value() == inf
//
// This line works on 1s complement and 2s complement.
// i??::min_value() is unused because (for example on 8-bit), -128 cannot be negated into +128,
// because +128 cannot be represented in 8 bits.
//
// Instead, all logic works on -T::max_value() instead of T::min_value() to ensure that the minimum
// value can be negated into a maximum value.
impl<T> Eval for T
where
    T: Num
        + NumAssign
        + NumAssignOps
        + NumOps
        + Signed
        + Bounded
        + PartialEq
        + PartialOrd
        + NumCast
        + Debug
        + Copy
        + PrimInt
        + WrappingAdd
        + WrappingSub
        + WrappingMul,
{
    fn max_supported_mates() -> Self {
        Self::from(800).expect("Failed to convert value 800 to Eval type.")
    }

    fn from_ply(ply: i16) -> Self {
        Self::from(ply).expect("Failed to convert u16 ply into Eval type.")
    }

    fn new_mate(ply: i16, color: Color) -> Self {
        match color {
            Color::White => Self::max_eval() - Self::from_ply(ply) - Self::one(),
            Color::Black => Self::min_eval() + Self::from_ply(ply) + Self::one(),
        }
    }

    fn add_depth(&self, amount: i16) -> Self {
        if let Some(depth) = self.depth_to_mate() {
            if depth < 0 {
                Self::new_mate(((-depth) as i16) + amount, Color::Black)
            } else {
                Self::new_mate((depth as i16) + amount, Color::White)
            }
        } else {
            *self
        }
    }

    fn min_eval() -> Self {
        -T::max_value()
    }

    fn max_eval() -> Self {
        T::max_value()
    }

    fn null() -> Self {
        T::min_value()
    }

    fn bound(other: Self) -> Self {
        if other < Self::min_eval() {
            Self::min_eval()
        } else {
            other
        }
    }

    fn depth_to_mate(&self) -> Option<i64> {
        if *self <= Self::min_eval() || *self >= Self::max_eval() {
            None
        } else if *self > Self::max_eval() - Self::max_supported_mates() - Self::one() {
            Some(
                (Self::max_eval() - *self - Self::one())
                    .to_i64()
                    .expect("Failed to convert mate depth to i64"),
            )
        } else if *self < Self::min_eval() + Self::max_supported_mates() + Self::one() {
            Some(
                (Self::min_eval() - *self + Self::one())
                    .to_i64()
                    .expect("Failed to convert mate depth to i64"),
            )
        } else {
            None
        }
    }
}

#[cfg(test)]
fn test_mates<T: Eval>() {
    assert_eq!(
        T::min_value() + T::one() + T::one(),
        T::new_mate(0, Color::Black)
    );
    assert_eq!(T::max_value() - T::one(), T::new_mate(0, Color::White));

    assert!(T::new_mate(0, Color::Black) < T::new_mate(1, Color::Black));
    assert!(T::new_mate(0, Color::White) > T::new_mate(1, Color::White));

    assert_eq!(T::new_mate(0, Color::Black).depth_to_mate(), Some(0));
    assert_eq!(T::new_mate(0, Color::White).depth_to_mate(), Some(0));

    assert_eq!(T::new_mate(10, Color::White).depth_to_mate(), Some(10));
    assert_eq!(T::new_mate(10, Color::Black).depth_to_mate(), Some(-10));

    assert_eq!(
        -T::new_mate(10, Color::Black),
        T::new_mate(10, Color::White)
    );

    assert_eq!(T::min_value().add_depth(1), T::min_value());
    assert_eq!(T::max_value().add_depth(1), T::max_value());
}

#[cfg(test)]
fn test_add_depth<E: Eval>() {
    let e1: E = E::from(200).expect("200 in range.");
    assert_eq!(e1.add_depth(1), e1);
    assert_eq!(e1.add_depth(-1), e1);

    let e2: E = E::from(-200).expect("-200 in range.");
    assert_eq!(e2.add_depth(1), e2);
    assert_eq!(e2.add_depth(-1), e2);

    let e3 = E::new_mate(5, Color::Black);
    assert_eq!(e3.add_depth(1), E::new_mate(6, Color::Black));
    assert_eq!(e3.add_depth(-1), E::new_mate(4, Color::Black));

    let e4 = E::new_mate(5, Color::White);
    assert_eq!(e4.add_depth(1), E::new_mate(6, Color::White));
    assert_eq!(e4.add_depth(-1), E::new_mate(4, Color::White));

    assert!(e1 < e4);
    assert!(e1 > e2);
    assert!(e1 > e3);

    assert!(e2 > e3);
    assert!(e2 < e4);

    assert!(e3 < e4);
}

#[test]
fn test_mates_i16() {
    test_mates::<i16>();
}

#[test]
fn test_add_depth_i16() {
    test_add_depth::<i16>();
}

#[test]
fn test_mates_i32() {
    test_mates::<i32>();
}

#[test]
fn test_add_depth_i32() {
    test_add_depth::<i32>();
}
