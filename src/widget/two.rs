use console_display_macros::TwoWidget;

pub mod alternative;
pub mod horizontal_tiling;
pub mod overlay;
pub mod vertical_tiling;

pub use alternative::*;
pub use horizontal_tiling::*;
pub use overlay::*;
pub use vertical_tiling::*;

pub const trait TwoWidget<S, T> {
    fn children(&self) -> (&S, &T);
    fn children_mut(&mut self) -> (&mut S, &mut T);
}
