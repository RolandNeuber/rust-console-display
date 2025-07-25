#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![doc = include_str!("../README.md")]

pub mod color;
mod display;
mod macros;
pub mod pixel;
pub mod widget;

pub use display::*;
