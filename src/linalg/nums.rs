use std::num::NumCast;
use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    Neg,
};

use linalg::ops::{
    Absolute,
    ApproxEq,
    Sqrt,
    One,
    Zero,
};

pub trait BaseNum : Copy + One + Zero +
                    Add<Output=Self> + Sub<Output=Self> +
                    Mul<Output=Self> + Div<Output=Self> +
                    Neg<Output=Self> + Absolute<Self> + Sqrt<Self> + ApproxEq<Self> +
                    NumCast + PartialEq + PartialOrd  {
}

mod impls {
    use super::*;

    impl BaseNum for f32 {}
    impl BaseNum for f64 {}
}
