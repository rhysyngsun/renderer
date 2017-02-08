#![feature(plugin)]
#![feature(custom_attribute)]

// testing specific plugins (random data via quickcheck)
#![cfg_attr(test, plugin(quickcheck_macros))]

// conditionally enable clippy for linting
#![cfg_attr(feature="clippy", plugin(clippy))]


#![allow(dead_code)]

#[macro_use]
extern crate nom;
extern crate num;
extern crate regex;

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
