#![feature(plugin)]
#![feature(custom_attribute)]
#![feature(zero_one)]
#![feature(box_syntax)]
#![feature(box_patterns)]

#![cfg_attr(test, plugin(quickcheck_macros))]

#![allow(dead_code)]

#[macro_use]
extern crate nom;

#[cfg(test)]
extern crate quickcheck;

pub mod cameras;
pub mod core;
pub mod film;
pub mod formats;
pub mod lights;
pub mod linalg;
pub mod integrator;
pub mod primative;
pub mod renderer;

pub use linalg::{Absolute, ApproxEq};

pub fn abs<N: Absolute<Result>, Result>(n: &N) -> Result {
    Absolute::abs(n)
}

pub fn approx_eq<N: ApproxEq<M>, M>(a: &N, b: &N, eps: &M) -> bool {
    ApproxEq::approx_eq(a, b, eps)
}
