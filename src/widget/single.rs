use std::ops::{
    Deref,
    DerefMut,
};

use console_display_macros::SingleWidget;

pub mod border;
pub mod crt;
pub mod double_buffer;
pub mod inset;
pub mod padding;
pub mod uv;

pub use border::*;
pub use crt::*;
pub use double_buffer::*;
pub use inset::*;
pub use padding::*;
pub use uv::*;

pub const trait SingleWidget<T> {
    type Borrowed<'a>: Deref<Target = T>
    where
        T: 'a,
        Self: 'a;
    type BorrowedMut<'a>: DerefMut<Target = T>
    where
        T: 'a,
        Self: 'a;

    fn child(&self) -> Self::Borrowed<'_>;
    fn child_mut(&mut self) -> Self::BorrowedMut<'_>;
}
