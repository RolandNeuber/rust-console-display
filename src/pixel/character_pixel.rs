use std::fmt::{
    Debug,
    Display,
};

use crate::{
    constraint,
    or,
    pixel::Pixel,
};

use super::color_pixel::Color;
use unicode_width::UnicodeWidthChar;

#[derive(Clone, Copy, Default)]
pub struct CharacterPixel {
    data: [CharacterPixelData; 1],
}

#[derive(Clone, Copy)]
pub struct CharacterPixelData {
    character: char,
    foreground: Color,
    background: Color,
    copy: bool,
    width: usize,
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
}

impl CharacterPixel {
    pub fn new<const CHARACTER: char>(
        foreground: Color,
        background: Color,
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
        foreground: Color,
        background: Color,
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
    pub const fn get_character(&self) -> char {
        self.data[0].character
    }

    #[must_use]
    pub const fn get_foreground(&self) -> Color {
        self.data[0].foreground
    }

    #[must_use]
    pub const fn get_background(&self) -> Color {
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
    pub const fn get_width(&self) -> usize {
        self.data[0].width
    }
}

impl Display for CharacterPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            Color::color(
                self.get_character().to_string().as_str(),
                &self.get_foreground(),
                &self.get_background()
            )
        )
    }
}

impl Debug for CharacterPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.get_character())
    }
}

impl TryFrom<char> for CharacterPixel {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        CharacterPixel::build(value, Color::Default, Color::Default)
    }
}

impl Default for CharacterPixelData {
    fn default() -> Self {
        Self {
            character: ' ',
            foreground: Default::default(),
            background: Default::default(),
            copy: false,
            width: 1,
        }
    }
}
