use std::fmt::{
    Debug,
    Display,
};

use crate::{
    constraint,
    or,
    pixel::Pixel,
    widget::DataCell,
};

use crate::color::{
    Color,
    TerminalColor,
};
use unicode_width::UnicodeWidthChar;

#[derive(Clone, Copy, Default)]
pub struct CharacterPixel {
    data: [CharacterPixelData; 1],
}

#[derive(Clone, Copy)]
pub struct CharacterPixelData {
    character: char,
    foreground: TerminalColor,
    background: TerminalColor,
    copy: bool,
    width: usize,
}

impl From<CharacterPixel> for CharacterPixelData {
    fn from(value: CharacterPixel) -> Self {
        value.data[0]
    }
}

impl Pixel for CharacterPixel {
    type U = CharacterPixelData;

    const WIDTH: usize = 1;

    const HEIGHT: usize = 1;

    fn pixels(&self) -> &[Self::U; Self::WIDTH * Self::HEIGHT] {
        &self.data
    }

    fn pixels_mut(
        &mut self,
    ) -> &mut [Self::U; Self::WIDTH * Self::HEIGHT] {
        &mut self.data
    }

    fn new(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]) -> Self {
        Self { data: pixels }
    }
}

impl From<CharacterPixel> for DataCell {
    fn from(val: CharacterPixel) -> Self {
        Self {
            character: val.character(),
            foreground: val.foreground(),
            background: val.background(),
        }
    }
}

impl CharacterPixel {
    /// Constructs a character pixel from a char, a foreground and a background color.
    ///
    /// # Panics
    ///
    /// This function checks for control characters at compile time.
    /// Panics when the compile time checks miss a control character.
    /// This should not happen and is a implementation detail that is subject to change.
    #[must_use]
    pub fn new<const CHARACTER: char>(
        foreground: TerminalColor,
        background: TerminalColor,
    ) -> Self
    where
        constraint!(CHARACTER >= '\u{20}'):, // Exclude C0 control chars
        constraint!(or!(CHARACTER < '\u{7F}', CHARACTER >= '\u{A0}')):, // Exclude C1 control chars
    {
        Self {
            data: [CharacterPixelData {
                character: CHARACTER,
                foreground,
                background,
                copy: false,
                width: UnicodeWidthChar::width(CHARACTER).expect(
                    "Invariant violated, found control character.",
                ),
            }],
        }
    }

    /// Constructs a character pixel from a char, a foreground and a background color.
    ///
    /// # Errors
    ///
    /// Returns an error if the character is a control character.
    pub fn build(
        character: char,
        foreground: TerminalColor,
        background: TerminalColor,
    ) -> Result<Self, String> {
        Ok(Self {
            data: [CharacterPixelData {
                character,
                foreground,
                background,
                copy: false,
                width: match UnicodeWidthChar::width(character) {
                    Some(val) => val,
                    None => {
                        return Err("Control characters are not allowed."
                            .to_string());
                    }
                },
            }],
        })
    }

    #[must_use]
    pub const fn make_copy(&self) -> Self {
        let mut clone = *self;
        clone.set_copy(true);
        clone
    }

    #[must_use]
    pub const fn character(&self) -> char {
        self.data[0].character
    }

    #[must_use]
    pub const fn foreground(&self) -> TerminalColor {
        self.data[0].foreground
    }

    #[must_use]
    pub const fn background(&self) -> TerminalColor {
        self.data[0].background
    }

    #[must_use]
    pub const fn is_copy(&self) -> bool {
        self.data[0].copy
    }

    const fn set_copy(&mut self, copy: bool) {
        self.data[0].copy = copy;
    }

    #[must_use]
    pub const fn width(&self) -> usize {
        self.data[0].width
    }
}

impl Display for CharacterPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            TerminalColor::color(
                self.character().to_string().as_str(),
                &self.foreground(),
                &self.background()
            )
        )
    }
}

impl Debug for CharacterPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.character())
    }
}

impl TryFrom<char> for CharacterPixel {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Self::build(value, TerminalColor::Default, TerminalColor::Default)
    }
}

impl Default for CharacterPixelData {
    fn default() -> Self {
        Self {
            character: ' ',
            foreground: TerminalColor::default(),
            background: TerminalColor::default(),
            copy: false,
            width: 1,
        }
    }
}
