use std::fmt::Display;

use crate::pixel::color_pixel::{Color, TerminalColor};

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
    fn string_data(&self) -> Vec<Vec<DataCell>>;
}

#[derive(Clone, Copy)]
pub struct DataCell {
    pub character: char,
    pub foreground: TerminalColor,
    pub background: TerminalColor,
}

impl Display for DataCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            TerminalColor::color(
                &self.character.to_string(),
                &self.foreground,
                &self.background
            )
        )
    }
}
