use std::ops::{
    Deref,
    DerefMut,
};

use console_display_macros::TwoWidget;

use crate::widget::DynamicWidget;

use super::StaticWidget;

pub mod alternative_widget;
pub mod horizontal_tiling_widget;
pub mod overlay_widget;
pub mod vertical_tiling_widget;

pub use alternative_widget::*;
pub use horizontal_tiling_widget::*;
pub use overlay_widget::*;
pub use vertical_tiling_widget::*;

#[deprecated]
pub const trait TwoWidgetOld<S: DynamicWidget, T: DynamicWidget>:
    DynamicWidget + Deref + DerefMut
{
    fn children(&self) -> (&S, &T);
    fn children_mut(&mut self) -> (&mut S, &mut T);
}

pub const trait TwoWidget<S, T> {
    fn children(&self) -> (&S, &T);
    fn children_mut(&mut self) -> (&mut S, &mut T);
}
