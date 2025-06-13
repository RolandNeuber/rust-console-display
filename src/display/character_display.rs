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
    data: Box<[CharacterPixel]>,
}

impl<const WIDTH: usize, const HEIGHT: usize>
    ConsoleDisplay<CharacterPixel>
    for CharacterDisplay<CharacterPixel, WIDTH, HEIGHT>
{
    fn get_width(&self) -> usize {
        WIDTH
    }

    fn get_height(&self) -> usize {
        HEIGHT
    }

    fn get_data(&self) -> &[CharacterPixel] {
        &self.data
    }

    fn get_data_mut(&mut self) -> &mut Box<[CharacterPixel]> {
        &mut self.data
    }
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
    ///
    /// # Errors
    /// Returns an error when the fill cannot match the dimensions of the display.
    pub fn build(fill: CharacterPixel) -> Result<Self, String>
    where
        [(); WIDTH * HEIGHT]:,
    {
        let character_width = fill.get_width();
        let full_columns = WIDTH / character_width;
        let padding = WIDTH % character_width;
        let total_columns = full_columns + padding;

        let mut data: Vec<CharacterPixel> =
            vec![fill; total_columns * HEIGHT];

        for i in 0..padding {
            for y in 0..HEIGHT {
                let x = full_columns + i;
                data[x + y * total_columns] = CharacterPixel::build(' ', fill.get_foreground(), fill.get_background())
                    .expect("Invariant violated. Should not be a control character.");
            }
        }

        Self::build_from_data(data)
    }

    // TODO: Handle double-width characters properly
    /// Builds a display struct with the specified dimensions from the given data.
    ///
    /// # Errors
    /// Returns an error when the data length does not match the dimensions of the display.
    /// This also applies if double-width characters, like あ, are used and exceed the dimensions of the display.
    pub fn build_from_data(
        data: Vec<CharacterPixel>,
    ) -> Result<Self, String> {
        let mut new_data = Vec::with_capacity(data.capacity());
        for i in data {
            new_data.push(i);
            for _ in 1..i.get_width() {
                new_data.push(i.make_copy());
            }
        }

        if new_data.len() / WIDTH / HEIGHT != 1 {
            return Err(format!(
                "Data does not match specified dimensions. Expected length of {}, got {}.",
                WIDTH * HEIGHT,
                new_data.len()
            ));
        }

        Ok(Self {
            data: new_data.into_boxed_slice(),
        })
    }

    #[must_use] pub const fn get_width(&self) -> usize {
        WIDTH
    }

    #[must_use] pub const fn get_height(&self) -> usize {
        HEIGHT
    }

    /// Returns the pixel value at the specified coordinates.
    ///
    /// # Errors
    ///
    /// If the coordinates are out of bounds, an error is returned.    
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

        Ok(&self.get_data()[x + y * self.get_width()])
    }

    /// Sets a character pixel at a specific x and y coordinate.
    /// Also works with characters wider than one column extending to the right.
    /// If a wide character is partially replaced, the rest of the affected character is overwritten with a default narrow character.
    ///
    /// # Errors
    ///
    /// Returns an error if the coordinates are out of bounds. This also applies for wide characters partially out of bounds.
    ///
    /// # Panics
    ///
    /// If the default character pixel could not be constructed.
    /// This should never happen and is subject to change in the future.
    pub fn set_pixel(
        &mut self,
        x: usize,
        y: usize,
        value: &CharacterPixel,
    ) -> Result<(), String> {
        if x > self.get_width() + value.get_width() ||
            y >= self.get_height()
        {
            return Err(format!(
                "Pixel coordinates out of bounds. Got x = {x}, y = {y}."
            ));
        }

        self.data[x + y * WIDTH] = *value;
        Ok(())
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

        let mut iter = self.get_data().iter();
        let mut current_width = 0;
        while let Some(i) = iter.next() {
            if current_width >= WIDTH {
                string_repr.push_str("\r\n");
                current_width = 0;
            }
            if i.is_copy() {
                string_repr.push_str(&Color::color(
                    " ",
                    &i.get_foreground(),
                    &i.get_background(),
                ));
                current_width += 1;
                continue;
            }
            string_repr.push_str(i.to_string().as_str());
            current_width += i.get_width();
            for _ in 0..i.get_width() - 1 {
                iter.next();
            }
        }

        write!(f, "{}", string_repr.trim_end())
    }
}
