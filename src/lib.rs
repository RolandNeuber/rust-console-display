#![feature(generic_const_exprs)]

mod display;
mod macros;
pub mod pixel;
pub mod color_pixel;
pub mod widget;

pub use display::DisplayDriver;