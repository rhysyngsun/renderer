#![feature(core)]
#![feature(plugin)]
#![feature(custom_attribute)]

#![cfg_attr(test, plugin(quickcheck_macros))]

#![allow(dead_code)]

#[cfg(test)]
extern crate quickcheck;

mod film;
mod linalg;
mod integrator;
mod light;
mod primative;
mod renderer;
mod sampler;
mod scene;

pub use linalg::{
    Absolute,
    ApproxEq,
};

pub fn abs<N: Absolute<Result>, Result>(n: &N) -> Result {
    Absolute::abs(n)
}

pub fn approx_eq<N: ApproxEq<M>, M>(a: &N, b: &N, eps: &M) -> bool {
    ApproxEq::approx_eq(a, b, eps)
}
