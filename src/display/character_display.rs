use std::fmt::Display;

use crate::{
    console_display::ConsoleDisplay,
    pixel::{
        character_pixel::CharacterPixel,
        color_pixel::Color,
    },
    widget::{
        DynamicWidget,
        StaticWidget,
    },
};

#[derive(Clone)]
pub struct CharacterDisplay<
    CharacterPixel,
    const WIDTH: usize,
    const HEIGHT: usize,
> {
    data: Vec<Option<CharacterPixel>>,
}

impl<const WIDTH: usize, const HEIGHT: usize> DynamicWidget
    for CharacterDisplay<CharacterPixel, WIDTH, HEIGHT>
{
    fn get_width_characters(&self) -> usize {
        WIDTH
    }

    fn get_height_characters(&self) -> usize {
        HEIGHT
    }
}

impl<const WIDTH: usize, const HEIGHT: usize>
    CharacterDisplay<CharacterPixel, WIDTH, HEIGHT>
{
    /// Convenience method to build a blank display struct with specified dimensions
    pub fn build(fill: CharacterPixel) -> Result<Self, String> {
        let data: Vec<CharacterPixel> = vec![fill; WIDTH * HEIGHT];
        Self::build_from_data(data)
    }

    /// Builds a display struct with the specified dimensions from the given data.
    pub fn build_from_data(
        data: Vec<CharacterPixel>,
    ) -> Result<Self, String> {
        let mut new_data = Vec::with_capacity(data.capacity());
        for i in data {
            new_data.push(Some(i.clone()));
            for _ in 1..i.get_width() {
                new_data.push(None);
            }
        }

        if new_data.len() / WIDTH / HEIGHT != 1 {
            return Err(format!(
                "Data does not match specified dimensions. Expected length of {}, got {}.",
                WIDTH * HEIGHT,
                new_data.len()
            ));
        }

        Ok(Self { data: new_data })
    }

    #[must_use]
    pub const fn get_data(&self) -> &Vec<Option<CharacterPixel>> {
        &self.data
    }

    #[allow(dead_code)]
    const fn get_data_mut(&mut self) -> &mut Vec<Option<CharacterPixel>> {
        &mut self.data
    }

    pub fn get_pixel(
        &self,
        x: usize,
        y: usize,
    ) -> Result<&CharacterPixel, String> {
        if x >= self.get_width() || y >= self.get_height() {
            return Err(format!(
                "Pixel coordinates out of bounds. Got x = {x}, y = {y}."
            ));
        }

        let mut offset = 0;
        while self.get_data()[x + y * self.get_width() - offset].is_none()
        {
            offset += 1;
        }

        self.get_data()[x + y * self.get_width() - offset]
            .as_ref()
            .map_or_else(
                || Err("Data malformed, pixel was None.".to_string()),
                Ok,
            )
    }

    /// Sets a character pixel at a specific x and y coordinate.
    /// Also works with characters wider than one column extending to the right.
    /// If a wide character is partially replaced, the rest of the affected character is overwritten with a default narrow character.
    /// Returns an error if the coordinates are out of bounds. This also applies for wide characters partially out of bounds.
    pub fn set_pixel(
        &mut self,
        x: usize,
        y: usize,
        value: &CharacterPixel,
    ) -> Result<(), String> {
        let default_pixel = Some(
            CharacterPixel::build(' ', Color::Default, Color::Default)
                .unwrap(),
        );

        if x > self.get_width() + value.get_width() ||
            y >= self.get_height()
        {
            return Err(format!(
                "Pixel coordinates out of bounds. Got x = {x}, y = {y}."
            ));
        }

        let mut overlap = 0;
        while self.data[x + y * WIDTH - overlap].is_none() {
            self.data[x + y * WIDTH - overlap] = default_pixel.clone();
            overlap += 1;
        }
        self.data[x + y * WIDTH - overlap] = default_pixel.clone();

        self.data[x + y * WIDTH] = Some(value.clone());

        let overlap = value.get_width();
        let mut max_overlap = value.get_width();
        for i in 1..value.get_width() {
            if let Some(character) = &self.data[x + y * WIDTH + i] {
                max_overlap = max_overlap.max(i + character.get_width());
            }

            self.data[x + y * WIDTH + i] = None;
        }
        for i in 0..max_overlap - overlap {
            self.data[x + y * WIDTH + value.get_width() - 1 + i] =
                default_pixel.clone();
        }

        Ok(())
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> ConsoleDisplay
    for CharacterDisplay<CharacterPixel, WIDTH, HEIGHT>
{
    fn get_width(&self) -> usize {
        WIDTH
    }

    fn get_height(&self) -> usize {
        HEIGHT
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> StaticWidget
    for CharacterDisplay<CharacterPixel, WIDTH, HEIGHT>
{
    const WIDTH_CHARACTERS: usize = WIDTH;

    const HEIGHT_CHARACTERS: usize = HEIGHT;
}

impl<const WIDTH: usize, const HEIGHT: usize> Display
    for CharacterDisplay<CharacterPixel, WIDTH, HEIGHT>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string_repr = String::new();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let character = &self.get_data()[x + y * WIDTH];
                match character {
                    None => {}
                    Some(character) => string_repr
                        .push_str(character.to_string().as_str()),
                }
            }
            string_repr.push_str("\r\n");
        }
        string_repr.pop();
        write!(f, "{string_repr}")
    }
}
