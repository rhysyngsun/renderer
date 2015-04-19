#![feature(core)]
#![feature(plugin)]
#![feature(custom_attribute)]

#![cfg_attr(test, plugin(quickcheck_macros))]

#![allow(dead_code)]

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
extern crate quickcheck_macros;

mod film;
mod linalg;
mod integrator;
mod light;
mod primative;
mod renderer;
mod sampler;
mod scene;

