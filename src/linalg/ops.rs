#![macro_use]


pub trait One {
    fn one() -> Self;
}

pub trait Zero {
    fn zero() -> Self;
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
    fn approx_eq(&self, other: &Self) -> bool;
    fn approx_eps() -> Eps;
}

impl ApproxEq<f64> for f64 {
    fn approx_eq(&self, other: &f64) -> bool {
        f64::abs(*self - *other) < f64::approx_eps()
    }

    #[inline]
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

absolute_impl! { f64 }

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
