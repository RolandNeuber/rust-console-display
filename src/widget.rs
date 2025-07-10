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
    #[must_use]
    fn width_characters(&self) -> usize;
    /// Returns the height of the display in characters.
    #[must_use]
    fn height_characters(&self) -> usize;
    /// Returns a string representation.
    /// The first vector contains rows.
    /// The vectors inside/rows contain individual characters.
    #[must_use]
    fn string_data(&self) -> Vec<Vec<String>>;
}
