#![macro_use]

use std::num::NumCast;
use std::ops::{Add,Sub,Mul,Div,Neg};

pub trait BaseNum : Copy + One + Zero +
                    Add<Output=Self> + Sub<Output=Self> +
                    Mul<Output=Self> + Div<Output=Self> +
                    Neg<Output=Self> + Absolute<Self> + Sqrt<Self> + ApproxEq<Self> +
                    NumCast + PartialEq + PartialOrd  {
}

pub trait Sqrt<N> {
    fn sqrt(&Self) -> N;
}

pub trait Absolute<N> {
    fn abs(&Self) -> N;
}

pub trait Cross {
    type Output;

    fn cross(&self, other: &Self) -> Self::Output;
}

pub trait ApproxEq<Eps> {
    fn approx_eq(&self, other: &Self, epsilon: &Eps) -> bool;
}

pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

pub trait One {
    fn one() -> Self;
}

mod impls {
    #![macro_use]

    use super::*;

    macro_rules! approx_eq_impl {
        ($($n: ty)*) => ($(
            impl ApproxEq<$n> for $n {
                fn approx_eq(&self, other: &$n, epsilon: &$n) -> bool {
                    ::abs(&(*self - *other)) < *epsilon
                }
            }
        )*)

    }

    approx_eq_impl! { f32 f64 }

    macro_rules! absolute_impl {
        ($($n: ty)*) => ($(
            impl Absolute<$n> for $n {
                fn abs(n: &$n) -> $n {
                    n.abs()
                }
            }
        )*)
    }

    macro_rules! absolute_id_impl {
        ($($n: ty)*) => ($(
            impl Absolute<$n> for $n {
                fn abs(n: &$n) -> $n {
                    *n
                }
            }
        )*)
    }

    absolute_impl! { f32 f64 isize i8 i16 i32 i64 }
    absolute_id_impl! { usize u8 u16 u32 u64 }

    macro_rules! one_and_zero_impl {
        ($n: ty, $one: expr, $zero: expr) => {
            impl Zero for $n {
                fn zero() -> $n {
                    $one
                }
                fn is_zero(&self) -> bool {
                    *self == $zero
                }
            }

            impl One for $n {
                fn one() -> $n {
                    $one
                }
            }
        }
    }

    one_and_zero_impl!(f32, 1.0, 0.0);
    one_and_zero_impl!(f64, 1.0, 0.0);
    one_and_zero_impl!(isize, 1, 0);
    one_and_zero_impl!(i8, 1, 0);
    one_and_zero_impl!(i16, 1, 0);
    one_and_zero_impl!(i32, 1, 0);
    one_and_zero_impl!(i64, 1, 0);
    one_and_zero_impl!(usize, 1, 0);
    one_and_zero_impl!(u8, 1, 0);
    one_and_zero_impl!(u16, 1, 0);
    one_and_zero_impl!(u32, 1, 0);
    one_and_zero_impl!(u64, 1, 0);

    macro_rules! impl_float_sqrt {
        ($n: ty) => {
            impl Sqrt<$n> for $n {
                fn sqrt(n: &$n) -> $n {
                    n.sqrt()
                }
            }
        }
    }

    impl_float_sqrt!(f32);
    impl_float_sqrt!(f64);
}
