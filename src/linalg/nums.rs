#![macro_use]

use std::num::NumCast;
use std::ops::{Add,Sub,Mul,Div,Neg};

pub trait BaseNum : Copy + One + Zero +
                    Add<Output=Self> + Sub<Output=Self> +
                    Mul<Output=Self> + Div<Output=Self> +
                    Neg<Output=Self> + Absolute<Self> + Sqrt<Self> + NumCast + PartialEq + PartialOrd  {
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

// Bring back the Zero trait
pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

// Bring back the One trait
pub trait One {
    fn one() -> Self;
}

macro_rules! absolute_impl {
    ($n: ty) => {
        impl Absolute<$n> for $n {
            fn abs(n: &$n) -> $n {
                n.abs()
            }
        }
    }
}

macro_rules! absolute_id_impl {
    ($n: ty) => {
        impl Absolute<$n> for $n {
            fn abs(n: &$n) -> $n {
                *n
            }
        }
    }
}

absolute_impl!(f32);
absolute_impl!(f64);
absolute_impl!(isize);
absolute_impl!(i8);
absolute_impl!(i16);
absolute_impl!(i32);
absolute_impl!(i64);
absolute_id_impl!(usize);
absolute_id_impl!(u8);
absolute_id_impl!(u16);
absolute_id_impl!(u32);
absolute_id_impl!(u64);

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


impl BaseNum for f32 {}
impl BaseNum for f64 {}
