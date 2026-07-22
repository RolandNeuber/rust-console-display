//! Provides abstractions over colors that are used in terminal context.

pub mod traits;

pub use traits::*;

pub mod argb_color;
pub mod rgb_color;
pub mod terminal_color;

pub use argb_color::*;
pub use rgb_color::*;
pub use terminal_color::*;
