use crate::pixel::color_pixel::Color;

pub struct CharacterPixel {
    character: char,
    foreground: Color,
    background: Color,
}

impl CharacterPixel {
    pub fn new(character: char, foreground: Color, background: Color) -> CharacterPixel {
        CharacterPixel {
            character,
            foreground,
            background
        }
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
}

impl ToString for CharacterPixel {
    fn to_string(&self) -> String {
        Color::color(&self.character.to_string().as_str(), &self.foreground, &self.background)
    }
}