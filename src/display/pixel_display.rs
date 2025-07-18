use core::array;
use std::fmt::Display;

use crate::{
    console_display::{
        DynamicConsoleDisplay,
        StaticConsoleDisplay,
    },
    constraint,
    impl_display_for_dynamic_widget,
    pixel::Pixel,
    widget::{
        DataCell,
        DynamicWidget,
        StaticWidget,
    },
};

pub struct DynamicPixelDisplay<T: Pixel> {
    data: Box<[T]>,
    width: usize,
    height: usize,
}

impl<T: Pixel> DynamicPixelDisplay<T> {
    /// Convenience method to build a blank display struct with specified dimensions.
    ///
    /// # Panics
    ///
    /// This function panics if the data generated from the fill does not match the dimensions of the display.
    /// This should not happen and is subject to change in the future.
    pub fn new(width: usize, height: usize, fill: T::U) -> Self
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
        if !width.is_multiple_of(T::WIDTH) ||
            !height.is_multiple_of(T::HEIGHT)
        {
            return Err(format!(
                "Width and height must be multiples of multipixel dimensions. Got width = {width}, height = {height}."
            ));
        }
        if data.len() != width * height {
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

impl<T: Pixel> DynamicConsoleDisplay<T> for DynamicPixelDisplay<T> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn data(&self) -> &[T] {
        &self.data
    }

    fn data_mut(&mut self) -> &mut Box<[T]> {
        &mut self.data
    }
}

impl<T: Pixel> DynamicWidget for DynamicPixelDisplay<T> {
    fn width_characters(&self) -> usize {
        self.width / T::WIDTH
    }

    fn height_characters(&self) -> usize {
        self.height / T::HEIGHT
    }

    fn string_data(&self) -> Vec<Vec<DataCell>> {
        self.data
            .chunks(self.width_characters())
            .map(|chunk| chunk.iter().map(|x| (*x).into()).collect())
            .collect()
    }
}

impl<T: Pixel> Display for DynamicPixelDisplay<T> {
    impl_display_for_dynamic_widget!();
}

/// Represents a console display with a width and height in pixels.
pub struct StaticPixelDisplay<
    T: Pixel,
    const WIDTH: usize,
    const HEIGHT: usize,
> {
    data: Box<[T]>,
}

impl<T: Pixel, const WIDTH: usize, const HEIGHT: usize>
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
    ///     console_display::DynamicConsoleDisplay,
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
    /// let pixel = disp.pixel(3, 2);
    ///
    /// assert_eq!(pixel, Ok(false));
    ///
    /// let pixel = disp.pixel(5, 6);
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
    pub fn pixel_static<const X: usize, const Y: usize>(&self) -> T::U
    where
        constraint!(X <= WIDTH):,
        constraint!(Y <= HEIGHT):,
        constraint!(X % T::WIDTH < T::WIDTH):,
        constraint!(Y % T::HEIGHT < T::HEIGHT):,
        [(); T::WIDTH * T::HEIGHT]:,
    {
        let pixel = &self.data()
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
        let pixel = &mut self.data_mut()
            [X / T::WIDTH + Y / T::HEIGHT * Self::WIDTH_CHARACTERS];
        pixel.set_subpixel_static::<{ X % T::WIDTH }, { Y % T::HEIGHT }>(
            value,
        );
    }
}

impl<T: Pixel, const WIDTH: usize, const HEIGHT: usize>
    StaticConsoleDisplay<T> for StaticPixelDisplay<T, WIDTH, HEIGHT>
{
    const WIDTH: usize = WIDTH;

    const HEIGHT: usize = HEIGHT;
}

impl<T: Pixel, const WIDTH: usize, const HEIGHT: usize>
    DynamicConsoleDisplay<T> for StaticPixelDisplay<T, WIDTH, HEIGHT>
{
    fn width(&self) -> usize {
        WIDTH
    }

    fn height(&self) -> usize {
        HEIGHT
    }

    fn data(&self) -> &[T] {
        &self.data
    }

    fn data_mut(&mut self) -> &mut Box<[T]> {
        &mut self.data
    }
}

impl<T: Pixel, const WIDTH: usize, const HEIGHT: usize> StaticWidget
    for StaticPixelDisplay<T, WIDTH, HEIGHT>
{
    const WIDTH_CHARACTERS: usize = WIDTH / T::WIDTH;

    const HEIGHT_CHARACTERS: usize = HEIGHT / T::HEIGHT;
}

impl<T: Pixel, const WIDTH: usize, const HEIGHT: usize> DynamicWidget
    for StaticPixelDisplay<T, WIDTH, HEIGHT>
{
    fn width_characters(&self) -> usize {
        Self::WIDTH_CHARACTERS
    }

    fn height_characters(&self) -> usize {
        Self::HEIGHT_CHARACTERS
    }

    fn string_data(&self) -> Vec<Vec<DataCell>> {
        self.data
            .chunks(Self::WIDTH_CHARACTERS)
            .map(|chunk| chunk.iter().map(|x| (*x).into()).collect())
            .collect()
    }
}

impl<T: Pixel, const WIDTH: usize, const HEIGHT: usize> Display
    for StaticPixelDisplay<T, WIDTH, HEIGHT>
{
    impl_display_for_dynamic_widget!();
}

#[cfg(test)]
mod tests {
    use crate::pixel::monochrome_pixel::SinglePixel;

    use super::*;

    mod dynamic_pixel_display {
        use super::*;

        #[test]
        fn build_from_data_success() {
            let dynamic_pixel_display =
                DynamicPixelDisplay::<SinglePixel>::build_from_data(
                    1,
                    1,
                    &[false],
                );
            assert!(dynamic_pixel_display.is_ok());
        }

        #[test]
        fn build_from_data_failure_less() {
            let dynamic_pixel_display =
                DynamicPixelDisplay::<SinglePixel>::build_from_data(
                    23,
                    1,
                    &[false; 22],
                );
            assert!(dynamic_pixel_display.is_err());
        }

        #[test]
        fn build_from_data_failure_more() {
            let dynamic_pixel_display =
                DynamicPixelDisplay::<SinglePixel>::build_from_data(
                    23,
                    1,
                    &[false; 24],
                );
            assert!(dynamic_pixel_display.is_err());
        }

        #[test]
        fn pixel_success() {
            let dynamic_pixel_display =
                DynamicPixelDisplay::<SinglePixel>::new(2, 1, false);
            let pixel = dynamic_pixel_display.pixel(1, 0);
            assert!(pixel.is_ok());
        }

        #[test]
        fn pixel_failure() {
            let dynamic_pixel_display =
                DynamicPixelDisplay::<SinglePixel>::new(2, 1, false);
            let pixel = dynamic_pixel_display.pixel(0, 1);
            assert!(pixel.is_err());
        }

        #[test]
        fn set_pixel_success() {
            let mut dynamic_pixel_display =
                DynamicPixelDisplay::<SinglePixel>::new(2, 1, false);
            let res = dynamic_pixel_display.set_pixel(1, 0, true);
            let pixel = dynamic_pixel_display.pixel(1, 0);
            assert!(res.is_ok());
            assert_eq!(pixel, Ok(true));
        }

        #[test]
        fn set_pixel_failure() {
            let mut dynamic_pixel_display =
                DynamicPixelDisplay::<SinglePixel>::new(2, 1, false);
            let res = dynamic_pixel_display.set_pixel(0, 1, true);
            assert!(res.is_err());
        }
    }
}
