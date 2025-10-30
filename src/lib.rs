#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![feature(const_deref)]
#![doc = include_str!("../README.md")]

pub mod color;
mod display;
pub mod drawing;
mod macros;
pub mod pixel;
pub mod widget;

pub use display::*;
