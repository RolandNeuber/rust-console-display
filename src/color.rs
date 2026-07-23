//! Provides abstractions over colors that are used in terminal context.

pub mod traits;

pub use traits::*;

pub mod argb;
pub mod rgb;
pub mod terminal;

pub use argb::*;
pub use rgb::*;
pub use terminal::*;
