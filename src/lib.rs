#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![feature(const_convert)]
#![feature(const_ref_cell)]
#![feature(adt_const_params)]
#![feature(lazy_type_alias)]
#![feature(specialization)]
#![feature(const_default)]
#![feature(derive_const)]
#![doc = include_str!("../README.md")]

pub mod color;
mod display;
pub mod drawing;
pub mod error;
mod macros;
pub mod optional_const_generics;
pub mod pixel;
mod public_api;
pub mod widget;

pub use display::*;
