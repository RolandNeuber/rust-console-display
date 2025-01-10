use std::fmt::Debug;

use crate::pixel::color_pixel::Color;
use unicode_width::UnicodeWidthChar;

#[derive(Clone)]
pub struct CharacterPixel {
    character: char,
    foreground: Color,
    background: Color,
    width: usize,
}

impl CharacterPixel {
    pub fn build(character: char, foreground: Color, background: Color) -> Result<CharacterPixel, String> {
        Ok(CharacterPixel {
            character,
            foreground,
            background,
            width: match UnicodeWidthChar::width(character) {
                Some(val) => val,
                None => return Err("Control characters are not allowed.".to_string())
            },
        })
    }

    pub fn get_character(&self) -> char {
        self.character
    }

    pub fn get_foreground(&self) -> Color {
        self.foreground
    }

    pub fn get_background(&self) -> Color {
        self.background
    }

    pub fn get_width(&self) -> usize {
        self.width
    }
}

impl ToString for CharacterPixel {
    fn to_string(&self) -> String {
        Color::color(&self.character.to_string().as_str(), &self.foreground, &self.background)
    }
}

impl Debug for CharacterPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.character)
    }
}