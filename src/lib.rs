#![feature(core)]
#![feature(plugin)]
#![feature(custom_attribute)]

#![plugin(quickcheck_macros)]

#![allow(dead_code)]

extern crate quickcheck;
extern crate quickcheck_macros;

mod film;
mod linalg;
mod integrator;
mod light;
mod primative;
mod renderer;
mod sampler;
mod scene;

