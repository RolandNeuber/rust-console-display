use std::ops::{
    Deref,
    DerefMut,
};

use console_display_macros::SingleWidget;

use crate::widget::DynamicWidget;

pub mod border_widget;
pub mod crt_widget;
pub mod double_buffer_widget;
pub mod inset_widget;
pub mod padding_widget;
pub mod uv_widget;

pub use border_widget::*;
pub use crt_widget::*;
pub use double_buffer_widget::*;
pub use inset_widget::*;
pub use padding_widget::*;
pub use uv_widget::*;

pub const trait SingleWidget<T: DynamicWidget>:
    DynamicWidget + Deref + DerefMut
{
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
