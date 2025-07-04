use std::fmt::Display;

pub mod single_widget;
pub mod two_widget;

pub trait StaticWidget: DynamicWidget {
    /// Width of the display in characters.
    const WIDTH_CHARACTERS: usize;
    /// Height of the display in characters.
    const HEIGHT_CHARACTERS: usize;
}

pub trait DynamicWidget: Display {
    /// Returns the width of the display in characters.
    fn get_width_characters(&self) -> usize;
    /// Returns the height of the display in characters.
    fn get_height_characters(&self) -> usize;
}
