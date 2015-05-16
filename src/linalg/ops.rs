#![macro_use]

use std::num::{
    One,
    Zero,
};

use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Rem,
};

pub trait BaseNum : Add<Output=Self> + Sub<Output=Self> +
                Mul<Output=Self> + Div<Output=Self> +
                ApproxEq<Self> + Absolute<Self> + Sqrt<Self> +
                Neg<Output=Self> + Rem<Self> +
                Copy + Zero + One + PartialOrd {
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

pub trait Dot {
    fn dot(&self, other: &Self) -> Self;
}

pub trait ApproxEq<Eps> {
    fn approx_eq(&self, other: &Self, epsilon: &Eps) -> bool;
    fn approx_eps() -> Eps;
}

impl ApproxEq<f32> for f32 {
    fn approx_eq(&self, other: &f32, epsilon: &f32) -> bool {
        ::abs(&(*self - *other)) < *epsilon
    }

    fn approx_eps() -> f32 {
        1e-6
    }
}

impl ApproxEq<f64> for f64 {
    fn approx_eq(&self, other: &f64, epsilon: &f64) -> bool {
        ::abs(&(*self - *other)) < *epsilon
    }

    fn approx_eps() -> f64 {
        1e-6
    }
}

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

absolute_impl! { f32 f64 }

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
