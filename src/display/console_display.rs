use num_traits::NumCast;

use crate::{
    constraint,
    drawing::DynamicCanvas,
    error::{
        COULD_NOT_CAST_X_COORD,
        COULD_NOT_CAST_Y_COORD,
        DisplayError,
        PIXEL_INDEX_OUT_OF_RANGE,
    },
    pixel::Pixel,
    widget::StaticWidget,
};

// TODO: Check if this can be const
pub trait DynamicConsoleDisplay<T: Pixel>: DynamicCanvas<T> {
    /// Returns the width of the display in a display specific, individually addressable unit (e.g. pixels, characters).
    fn width(&self) -> usize;
    /// Returns the height of the display in a display specific, individually addressable unit (e.g. pixels, characters).
    fn height(&self) -> usize;

    /// Returns a vector containing all the pixels in the display.
    ///
    /// # Panics
    ///
    /// This function panics if the index of a pixel is out of bounds.
    /// This should not happen and is subject to change in the future.
    #[must_use]
    fn pixels(&self) -> Vec<T::U>
    where
        [(); T::WIDTH * T::HEIGHT]:,
    {
        let mut pixels = Vec::with_capacity(self.width() * self.height());
        for y in 0..self.height() {
            for x in 0..self.width() {
                pixels.push(
                    self.pixel(
                        NumCast::from(x).expect(COULD_NOT_CAST_X_COORD),
                        NumCast::from(y).expect(COULD_NOT_CAST_Y_COORD),
                    )
                    .expect(PIXEL_INDEX_OUT_OF_RANGE),
                );
            }
        }
        pixels
    }

    /// Sets the pixels of the display to the provided data.
    ///
    /// # Errors
    ///
    /// Returns an error if the provided data does not match the dimensions of the display.
    ///
    /// # Panics
    ///
    /// This function panics if the index of a pixel is out of bounds.
    /// This should not happen and is subject to change in the future.
    fn set_pixels(&mut self, data: &[T::U]) -> Result<(), DisplayError>
    where
        [(); T::WIDTH * T::HEIGHT]:,
    {
        if data.len() != self.width() * self.height() {
            return Err(DisplayError::MismatchedDimensions(
                self.width(),
                self.height(),
            ));
        }
        for y in 0..self.height() {
            for x in 0..self.width() {
                self.set_pixel(
                    NumCast::from(x).expect(COULD_NOT_CAST_X_COORD),
                    NumCast::from(y).expect(COULD_NOT_CAST_Y_COORD),
                    data[x + y * self.width()],
                )
                .expect(PIXEL_INDEX_OUT_OF_RANGE);
            }
        }
        Ok(())
    }

    #[must_use]
    fn data(&self) -> &[T];

    fn data_mut(&mut self) -> &mut Box<[T]>;
}

// TODO: Check if this can be const
pub trait StaticConsoleDisplay<T: Pixel>:
    DynamicConsoleDisplay<T> + StaticWidget
{
    const WIDTH: usize;
    const HEIGHT: usize;
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
    ///     pixel_display::StaticPixelDisplay,
    ///     drawing::DynamicCanvas
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
    fn pixel_static<const X: usize, const Y: usize>(&self) -> T::U
    where
        constraint!(X <= Self::WIDTH):,
        constraint!(Y <= Self::HEIGHT):,
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
    fn set_pixel_static<const X: usize, const Y: usize>(
        &mut self,
        value: T::U,
    ) where
        constraint!(X <= Self::WIDTH):,
        constraint!(Y <= Self::HEIGHT):,
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
