use core::array;
use std::fmt::Display;

use crate::{
    console_display::ConsoleDisplay,
    constraint,
    pixel::monochrome_pixel::MultiPixel,
    widget::{
        DynamicWidget,
        StaticWidget,
    },
};

pub struct DynamicPixelDisplay<T: MultiPixel> {
    data: Box<[T]>,
    width: usize,
    height: usize,
}

impl<T: MultiPixel> DynamicPixelDisplay<T> {
    /// Convenience method to build a blank display struct with specified dimensions.
    ///
    /// # Panics
    ///
    /// This function panics if the data generated from the fill does not match the dimensions of the display.
    /// This should not happen and is subject to change in the future.
    pub fn build(width: usize, height: usize, fill: T::U) -> Self
    where
        Self: Sized,
        [(); T::WIDTH * T::HEIGHT]:,
    {
        let data: Vec<T::U> = vec![fill; width * height];
        Self::build_from_data(width, height, &data).expect(
            "Invariant violated, data does not mach specified dimensions.",
        )
    }

    /// Builds a display struct from the given data with the specified dimensions.
    ///
    /// # Errors
    ///
    /// Returns an error when the length of the data does not match the dimensions of the display.
    pub fn build_from_data(
        width: usize,
        height: usize,
        data: &[T::U],
    ) -> Result<Self, String>
    where
        Self: Sized,
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
            data: multi_pixels.into_boxed_slice(),
        })
    }
}

impl<T: MultiPixel> ConsoleDisplay<T> for DynamicPixelDisplay<T> {
    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn get_data(&self) -> &[T] {
        &self.data
    }

    fn get_data_mut(&mut self) -> &mut Box<[T]> {
        &mut self.data
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

        write!(f, "{}", string_repr.trim_end_matches("\r\n"))
    }
}

/// Represents a console display with a width and height in pixels.
pub struct StaticPixelDisplay<
    T: MultiPixel,
    const WIDTH: usize,
    const HEIGHT: usize,
> {
    data: Box<[T]>,
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
        Self::new_from_data(&data)
    }

    /// Creates a display struct from the given data with the specified dimensions known at compile time.
    pub fn new_from_data(data: &[T::U; WIDTH * HEIGHT]) -> Self
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
                    array::from_fn(|i| {
                        let x = i % T::WIDTH;
                        let y = i / T::WIDTH;
                        data[block_x + x + (block_y + y) * WIDTH]
                    });

                multi_pixels.push(T::new(args));
            }
        }

        Self {
            data: multi_pixels.into_boxed_slice(),
        }
    }

    // TODO: Add proper double buffering.
    /// Returns a vector containing all the pixels in the display.
    ///
    /// # Panics
    ///
    /// This function panics if the index of a pixel is out of bounds.
    /// This should not happen and is subject to change in the future.
    #[must_use]
    pub fn get_pixels(&self) -> [T::U; WIDTH * HEIGHT]
    where
        [(); T::WIDTH * T::HEIGHT]:,
    {
        array::from_fn(|i| {
            let x = i % self.get_width();
            let y = i / self.get_width();
            self.get_pixel(x, y)
                .expect("Invariant violated, pixel index out of range.")
        })
    }

    /// Sets the pixels of the display to the provided data.
    ///
    /// # Panics
    ///
    /// This function panics if the index of a pixel is out of bounds.
    /// This should not happen and is subject to change in the future.
    pub fn set_pixels(&mut self, data: &[T::U; WIDTH * HEIGHT])
    where
        [(); T::WIDTH * T::HEIGHT]:,
    {
        for x in 0..self.get_width() {
            for y in 0..self.get_height() {
                self.set_pixel(x, y, data[x + y * self.get_width()])
                    .expect(
                        "Invariant violated, pixel index out of range.",
                    );
            }
        }
    }

    // TODO: Update docs
    /// Returns a bool representing the state of the pixel at the specified coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// #![allow(incomplete_features)]
    /// #![feature(generic_const_exprs)]
    ///
    /// use console_display::{
    ///     console_display::ConsoleDisplay,
    ///     display_driver::DisplayDriver,
    ///     pixel::monochrome_pixel::SinglePixel,
    ///     pixel_display::StaticPixelDisplay
    /// };
    ///
    /// let disp: DisplayDriver<StaticPixelDisplay<SinglePixel, 6, 6>> = DisplayDriver::new(
    ///     StaticPixelDisplay::<SinglePixel, 6, 6>::new_from_data(
    ///         &[
    ///             true, true, true, true,  true, true, // 0
    ///             true, true, true, true,  true, true, // 1
    ///             true, true, true, false, true, true, //-2-
    ///             true, true, true, true,  true, true, // 3
    ///             true, true, true, true,  true, true, // 4
    ///             true, true, true, true,  true, true, // 5
    ///         ] //  0     1     2   --3--    4     5
    ///     )
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
    ///
    /// # Errors
    ///
    /// Returns an error if the pixel coordinates are out of bounds.
    ///
    /// # Panics
    ///
    /// If the index of a subpixel is out of bounds.
    /// This should not happen and is subject to change in the future.
    #[must_use]
    pub fn get_pixel_static<const X: usize, const Y: usize>(&self) -> T::U
    where
        constraint!(X <= WIDTH):,
        constraint!(Y <= HEIGHT):,
        constraint!(X % T::WIDTH < T::WIDTH):,
        constraint!(Y % T::HEIGHT < T::HEIGHT):,
        [(); T::WIDTH * T::HEIGHT]:,
    {
        let pixel = &self.get_data()
            [X / T::WIDTH + Y / T::HEIGHT * Self::WIDTH_CHARACTERS];
        pixel.subpixel_static::<{ X % T::WIDTH }, { Y % T::HEIGHT }>()
    }

    // TODO: Update docs
    /// Set a pixel at the specified coordinate with a given value.
    ///
    /// # Errors
    ///
    /// Returns an error if the pixel coordinates are out of bounds.
    ///
    /// # Panics
    ///
    /// If the index of a subpixel is out of bounds.
    /// This should not happen and is subject to change in the future.
    pub fn set_pixel_static<const X: usize, const Y: usize>(
        &mut self,
        value: T::U,
    ) where
        constraint!(X <= WIDTH):,
        constraint!(Y <= HEIGHT):,
        constraint!(X % T::WIDTH < T::WIDTH):,
        constraint!(Y % T::HEIGHT < T::HEIGHT):,
        [(); T::WIDTH * T::HEIGHT]:,
    {
        let pixel = &mut self.get_data_mut()
            [X / T::WIDTH + Y / T::HEIGHT * Self::WIDTH_CHARACTERS];
        pixel.set_subpixel_static::<{ X % T::WIDTH }, { Y % T::HEIGHT }>(
            value,
        );
    }
}

impl<T: MultiPixel, const WIDTH: usize, const HEIGHT: usize>
    ConsoleDisplay<T> for StaticPixelDisplay<T, WIDTH, HEIGHT>
{
    fn get_width(&self) -> usize {
        WIDTH
    }

    fn get_height(&self) -> usize {
        HEIGHT
    }

    fn get_data(&self) -> &[T] {
        &self.data
    }

    fn get_data_mut(&mut self) -> &mut Box<[T]> {
        &mut self.data
    }
}

impl<T: MultiPixel, const WIDTH: usize, const HEIGHT: usize> DynamicWidget
    for StaticPixelDisplay<T, WIDTH, HEIGHT>
{
    fn get_width_characters(&self) -> usize {
        Self::WIDTH_CHARACTERS
    }

    fn get_height_characters(&self) -> usize {
        Self::HEIGHT_CHARACTERS
    }
}

impl<T: MultiPixel, const WIDTH: usize, const HEIGHT: usize> StaticWidget
    for StaticPixelDisplay<T, WIDTH, HEIGHT>
{
    const WIDTH_CHARACTERS: usize = WIDTH / T::WIDTH;

    const HEIGHT_CHARACTERS: usize = HEIGHT / T::HEIGHT;
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

        write!(f, "{}", string_repr.trim_end_matches("\r\n"))
    }
}
