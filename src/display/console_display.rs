use crate::widget::DynamicWidget;

pub trait ConsoleDisplay: DynamicWidget {
    /// Returns the width of the display in a display specific, individually addressable unit (e.g. pixels, characters).
    fn get_width(&self) -> usize;
    /// Returns the height of the display in a display specific, individually addressable unit (e.g. pixels, characters).
    fn get_height(&self) -> usize;
}
