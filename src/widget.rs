pub mod single_widget;
pub mod two_widget;

pub trait Widget: ToString {
    /// Returns the width of the display in characters.
    fn get_width_characters(&self) -> usize;
    /// Returns the height of the display in characters.
    fn get_height_characters(&self) -> usize;
}