pub mod single_widget;
pub mod two_widget;

pub trait Widget: ToString {
    /// Width of the display in characters.
    const WIDTH_CHARACTERS: usize;
    /// Height of the display in characters.
    const HEIGHT_CHARACTERS: usize;
}
