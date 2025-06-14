use std::fmt::Display;

use crate::{
    pixel::{
        character_pixel::CharacterPixel,
        color_pixel::Color,
        monochrome_pixel::MultiPixel,
    },
    widget::Widget,
};

pub trait ConsoleDisplay: Widget {
    /// Returns the width of the display in a display specific, individually addressable unit (e.g. pixels, characters).
    fn get_width(&self) -> usize;
    /// Returns the height of the display in a display specific, individually addressable unit (e.g. pixels, characters).
    fn get_height(&self) -> usize;
}

/// Represents a console display with a width and height in pixels.
pub struct PixelDisplay<T: MultiPixel> {
    data: Vec<T>,
    width: usize,
    height: usize,
    block_count_x: usize,
    block_count_y: usize,
}

impl<T: MultiPixel> ConsoleDisplay for PixelDisplay<T> {
    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }
}

impl<T: MultiPixel> Widget for PixelDisplay<T> {
    fn get_width_characters(&self) -> usize {
        self.block_count_x
    }

    fn get_height_characters(&self) -> usize {
        self.block_count_y
    }
}

impl<T: MultiPixel> PixelDisplay<T> {
    /// Convenience method to build a blank display struct with specified dimensions
    pub fn build(
        width: usize,
        height: usize,
        fill: T::U,
    ) -> Result<Self, String>
    where
        [(); T::WIDTH * T::HEIGHT]:,
    {
        let data: Vec<T::U> = vec![fill; width * height];
        Self::build_from_data(width, height, data)
    }

    /// Builds a display struct with the specified dimensions from the given data.
    pub fn build_from_data(
        width: usize,
        height: usize,
        data: Vec<T::U>,
    ) -> Result<Self, String>
    where
        [(); T::WIDTH * T::HEIGHT]:,
    {
        if width % T::WIDTH != 0 || height % T::HEIGHT != 0 {
            return Err(format!(
                "Width and height must be multiples of multipixel dimensions. Got width = {width}, height = {height}."
            ));
        }
        if data.len() / width / height != 1 {
            return Err(format!(
                "Data does not match specified dimensions. Expected length of {}, got {}.",
                width * height,
                data.len()
            ));
        }

        let block_count_x = width / T::WIDTH;
        let block_count_y = height / T::HEIGHT;

        let mut multi_pixels =
            Vec::with_capacity(block_count_x * block_count_y);

        for row in 0..block_count_y {
            for col in 0..block_count_x {
                let block_x: usize = col * T::WIDTH;
                let block_y: usize = row * T::HEIGHT;

                let mut args: Vec<T::U> =
                    Vec::with_capacity(T::WIDTH * T::HEIGHT);

                for y in 0..T::HEIGHT {
                    for x in 0..T::WIDTH {
                        args.push(
                            data[block_x + x + (block_y + y) * width],
                        );
                    }
                }

                multi_pixels.push(T::build(&args)?);
            }
        }

        Ok(Self {
            width,
            height,
            block_count_x,
            block_count_y,
            data: multi_pixels,
        })
    }

    #[must_use]
    pub const fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    const fn get_data_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    #[must_use]
    pub const fn get_block_count_x(&self) -> &usize {
        &self.block_count_x
    }

    #[must_use]
    pub const fn get_block_count_y(&self) -> &usize {
        &self.block_count_y
    }

    /// Returns a bool representing the state of the pixel at the specified coordinate.
    /// Returns an error if the coordinates are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// #![allow(incomplete_features)]
    /// #![feature(generic_const_exprs)]
    ///
    /// use console_display::{
    ///     display_driver::DisplayDriver,
    ///     pixel::monochrome_pixel::SinglePixel,
    ///     console_display::PixelDisplay
    /// };
    ///
    /// let disp: DisplayDriver<PixelDisplay<SinglePixel>> = DisplayDriver::new(
    ///     PixelDisplay::<SinglePixel>::build_from_data(
    ///         6,
    ///         6,
    ///         vec![
    ///             true, true, true, true,  true, true, // 0
    ///             true, true, true, true,  true, true, // 1
    ///             true, true, true, false, true, true, //-2-
    ///             true, true, true, true,  true, true, // 3
    ///             true, true, true, true,  true, true, // 4
    ///             true, true, true, true,  true, true, // 5
    ///         ] //  0     1     2   --3--    4     5
    ///     ).expect("Could not construct display.")
    /// );
    /// // Replace with actual error handling
    ///
    /// let pixel = disp.get_pixel(3, 2);
    ///
    /// assert_eq!(pixel, Ok(false));
    ///
    /// let pixel = disp.get_pixel(5, 6);
    ///
    /// assert!(matches!(pixel, Err(_)));
    /// ```
    pub fn get_pixel(&self, x: usize, y: usize) -> Result<T::U, String>
    where
        [(); T::WIDTH * T::HEIGHT]:,
    {
        if x >= self.get_width() || y >= self.get_height() {
            return Err(format!(
                "Pixel coordinates out of bounds. Got x = {x}, y = {y}."
            ));
        }

        let block_x: usize = x / T::WIDTH;
        let block_y: usize = y / T::HEIGHT;
        let offset_x: usize = x % T::WIDTH;
        let offset_y: usize = y % T::HEIGHT;

        let pixel =
            &self.get_data()[block_x + block_y * self.get_block_count_x()];
        pixel.get_subpixel(offset_x, offset_y).map_or_else(
            |_| Err("Offset should be 0 or 1.".to_string()),
            Ok,
        )
    }

    pub fn set_pixel(
        &mut self,
        x: usize,
        y: usize,
        value: T::U,
    ) -> Result<(), String>
    where
        [(); T::WIDTH * T::HEIGHT]:,
    {
        if x >= self.get_width() || y >= self.get_height() {
            return Err(format!(
                "Pixel coordinates out of bounds. Got x = {x}, y = {y}."
            ));
        }

        let block_x: usize = x / T::WIDTH;
        let block_y: usize = y / T::HEIGHT;
        let offset_x: usize = x % T::WIDTH;
        let offset_y: usize = y % T::HEIGHT;

        let block_count_x = *self.get_block_count_x();

        let pixel =
            &mut self.get_data_mut()[block_x + block_y * block_count_x];
        pixel.set_subpixel(offset_x, offset_y, value).map_or_else(
            |_| Err("Offset should be 0 or 1.".to_string()),
            Ok,
        )
    }

    pub fn draw_line(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        value: T::U,
    ) where
        [(); T::WIDTH * T::HEIGHT]:,
    {
        let dx = x2 - x1;
        let dy = y2 - y1;

        let steps: f32 = dx.abs().max(dy.abs());
        let x_inc = dx / steps;
        let y_inc = dy / steps;

        let mut x = x1;
        let mut y = y1;

        for _ in 0..=steps.round() as usize {
            if x >= 0. && y >= 0. {
                let _ = self.set_pixel(
                    x.round() as usize,
                    y.round() as usize,
                    value,
                );
            }
            x += x_inc;
            y += y_inc;
        }
    }
}

impl<T: MultiPixel> Display for PixelDisplay<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string_repr = String::new();
        for y in 0..*self.get_block_count_y() {
            for x in 0..*self.get_block_count_x() {
                string_repr.push_str(
                    self.get_data()[x + y * self.get_block_count_x()]
                        .to_string()
                        .as_str(),
                );
            }
            string_repr.push_str("\r\n");
        }
        string_repr.pop();

        write!(f, "{string_repr}")
    }
}

#[derive(Clone)]
pub struct CharacterDisplay<CharacterPixel> {
    data: Vec<Option<CharacterPixel>>,
    width: usize,
    height: usize,
}

impl CharacterDisplay<CharacterPixel> {
    /// Convenience method to build a blank display struct with specified dimensions
    pub fn build(
        width: usize,
        height: usize,
        fill: CharacterPixel,
    ) -> Result<Self, String> {
        let data: Vec<CharacterPixel> = vec![fill; width * height];
        Self::build_from_data(width, height, data)
    }

    /// Builds a display struct with the specified dimensions from the given data.
    pub fn build_from_data(
        width: usize,
        height: usize,
        data: Vec<CharacterPixel>,
    ) -> Result<Self, String> {
        let mut new_data = Vec::with_capacity(data.capacity());
        for i in data {
            new_data.push(Some(i.clone()));
            for _ in 1..i.get_width() {
                new_data.push(None);
            }
        }

        if new_data.len() / width / height != 1 {
            return Err(format!(
                "Data does not match specified dimensions. Expected length of {}, got {}.",
                width * height,
                new_data.len()
            ));
        }

        Ok(Self {
            width,
            height,
            data: new_data,
        })
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
        value: CharacterPixel,
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
        while self.data[x + y * self.width - overlap].is_none() {
            self.data[x + y * self.width - overlap] =
                default_pixel.clone();
            overlap += 1;
        }
        self.data[x + y * self.width - overlap] = default_pixel.clone();

        self.data[x + y * self.width] = Some(value.clone());

        let overlap = value.get_width();
        let mut max_overlap = value.get_width();
        for i in 1..value.get_width() {
            if let Some(character) = &self.data[x + y * self.width + i] {
                max_overlap = max_overlap.max(i + character.get_width());
            }

            self.data[x + y * self.width + i] = None;
        }
        for i in 0..max_overlap - overlap {
            self.data[x + y * self.width + value.get_width() - 1 + i] =
                default_pixel.clone();
        }

        Ok(())
    }
}

impl ConsoleDisplay for CharacterDisplay<CharacterPixel> {
    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }
}

impl Widget for CharacterDisplay<CharacterPixel> {
    fn get_width_characters(&self) -> usize {
        self.width
    }

    fn get_height_characters(&self) -> usize {
        self.height
    }
}

impl Display for CharacterDisplay<CharacterPixel> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string_repr = String::new();
        for y in 0..self.get_height_characters() {
            for x in 0..self.get_width_characters() {
                let character =
                    &self.get_data()[x + y * self.get_width_characters()];
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
