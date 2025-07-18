use std::{
    fmt::Display,
    ops::{
        Deref,
        DerefMut,
    },
};

use crate::pixel::color_pixel::TerminalColor;

pub mod single_widget;
pub mod two_widget;

pub trait StaticWidget: DynamicWidget {
    /// Width of the display in characters.
    const WIDTH_CHARACTERS: usize;
    /// Height of the display in characters.
    const HEIGHT_CHARACTERS: usize;
}

pub trait DynamicWidget {
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
    fn string_data(&self) -> StringData;
}

pub struct StringData {
    pub data: Vec<Vec<DataCell>>,
}

impl Display for StringData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str_repr = String::new();
        for row in &self.data {
            for cell in row {
                str_repr.push_str(&cell.to_string());
            }
            str_repr.push_str("\r\n");
        }
        str_repr = str_repr.trim_end_matches("\r\n").to_string();
        write!(f, "{str_repr}")
    }
}

impl Deref for StringData {
    type Target = Vec<Vec<DataCell>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for StringData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
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
