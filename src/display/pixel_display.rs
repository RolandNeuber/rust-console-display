use std::fmt::Display;

use crate::{
    console_display::ConsoleDisplay,
    pixel::monochrome_pixel::MultiPixel,
    widget::{
        DynamicWidget,
        StaticWidget,
    },
};

pub struct DynamicPixelDisplay<T: MultiPixel> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: MultiPixel> ConsoleDisplay for DynamicPixelDisplay<T> {
    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }
}

impl<T: MultiPixel> DynamicWidget for DynamicPixelDisplay<T> {
    fn get_width_characters(&self) -> usize {
        self.width / T::WIDTH
    }

    fn get_height_characters(&self) -> usize {
        self.height / T::HEIGHT
    }
}

impl<T: MultiPixel> DynamicPixelDisplay<T> {
    /// Convenience method to build a blank display struct with specified dimensions.
    pub fn build(
        width: usize,
        height: usize,
        fill: T::U,
    ) -> Result<Self, String>
    where
        [(); T::WIDTH * T::HEIGHT]:,
    {
        let data: Vec<T::U> = vec![fill; width * height];
        Self::build_from_data(width, height, &data)
    }

    /// Builds a display struct from the given data with the specified dimensions.
    pub fn build_from_data(
        width: usize,
        height: usize,
        data: &[T::U],
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

        let pixel = &self.get_data()
            [block_x + block_y * self.get_width_characters()];
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

        let width_characters = self.get_width_characters();
        let pixel =
            &mut self.get_data_mut()[block_x + block_y * width_characters];
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

impl<T: MultiPixel> Display for DynamicPixelDisplay<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string_repr = String::new();
        for y in 0..self.get_height_characters() {
            for x in 0..self.get_width_characters() {
                string_repr.push_str(
                    self.get_data()[x + y * self.get_width_characters()]
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

/// Represents a console display with a width and height in pixels.
pub struct StaticPixelDisplay<
    T: MultiPixel,
    const WIDTH: usize,
    const HEIGHT: usize,
> {
    data: Vec<T>,
}

impl<T: MultiPixel, const WIDTH: usize, const HEIGHT: usize> DynamicWidget
    for StaticPixelDisplay<T, WIDTH, HEIGHT>
{
    fn get_width_characters(&self) -> usize {
        WIDTH / T::WIDTH
    }

    fn get_height_characters(&self) -> usize {
        HEIGHT / T::HEIGHT
    }
}

impl<T: MultiPixel, const WIDTH: usize, const HEIGHT: usize> ConsoleDisplay
    for StaticPixelDisplay<T, WIDTH, HEIGHT>
{
    fn get_width(&self) -> usize {
        WIDTH
    }

    fn get_height(&self) -> usize {
        HEIGHT
    }
}

impl<T: MultiPixel, const WIDTH: usize, const HEIGHT: usize> StaticWidget
    for StaticPixelDisplay<T, WIDTH, HEIGHT>
{
    const WIDTH_CHARACTERS: usize = WIDTH / T::WIDTH;

    const HEIGHT_CHARACTERS: usize = HEIGHT / T::HEIGHT;
}

impl<T: MultiPixel, const WIDTH: usize, const HEIGHT: usize>
    StaticPixelDisplay<T, WIDTH, HEIGHT>
{
    /// Convenience method to create a blank display struct with specified dimensions known at compile time.
    pub fn new(fill: T::U) -> Self
    where
        [(); T::WIDTH * T::HEIGHT]:,
        [(); WIDTH * HEIGHT]:,
        [(); 0 - WIDTH % T::WIDTH]:,
        [(); 0 - HEIGHT % T::HEIGHT]:,
    {
        let data: [T::U; WIDTH * HEIGHT] = [fill; WIDTH * HEIGHT];
        Self::new_from_data(data)
    }

    /// Creates a display struct from the given data with the specified dimensions known at compile time.
    pub fn new_from_data(data: [T::U; WIDTH * HEIGHT]) -> Self
    where
        [(); T::WIDTH * T::HEIGHT]:,
        [(); 0 - WIDTH % T::WIDTH]:,
        [(); 0 - HEIGHT % T::HEIGHT]:,
    {
        let mut multi_pixels = Vec::with_capacity(
            Self::WIDTH_CHARACTERS * Self::HEIGHT_CHARACTERS,
        );

        for row in 0..Self::HEIGHT_CHARACTERS {
            for col in 0..Self::WIDTH_CHARACTERS {
                let block_x: usize = col * T::WIDTH;
                let block_y: usize = row * T::HEIGHT;

                let args: [T::U; T::WIDTH * T::HEIGHT] =
                    core::array::from_fn(|i| {
                        let x = i % T::WIDTH;
                        let y = i / T::WIDTH;
                        data[block_x + x + (block_y + y) * WIDTH]
                    });

                multi_pixels.push(T::new(args));
            }
        }

        Self { data: multi_pixels }
    }

    #[must_use]
    pub const fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    const fn get_data_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
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
            &self.get_data()[block_x + block_y * Self::WIDTH_CHARACTERS];
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

        let pixel = &mut self.get_data_mut()
            [block_x + block_y * Self::WIDTH_CHARACTERS];
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

impl<T: MultiPixel, const WIDTH: usize, const HEIGHT: usize> Display
    for StaticPixelDisplay<T, WIDTH, HEIGHT>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string_repr = String::new();
        for y in 0..Self::HEIGHT_CHARACTERS {
            for x in 0..Self::WIDTH_CHARACTERS {
                string_repr.push_str(
                    self.get_data()[x + y * Self::WIDTH_CHARACTERS]
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
