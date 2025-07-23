use crate::{
    constraint,
    pixel::Pixel,
    widget::{
        DynamicWidget,
        StaticWidget,
    },
};

pub trait DynamicConsoleDisplay<T: Pixel>: DynamicWidget {
    /// Returns the width of the display in a display specific, individually addressable unit (e.g. pixels, characters).
    fn width(&self) -> usize;
    /// Returns the height of the display in a display specific, individually addressable unit (e.g. pixels, characters).
    fn height(&self) -> usize;

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
    ///     pixel_display::DynamicPixelDisplay
    /// };
    ///
    /// let disp: DisplayDriver<DynamicPixelDisplay<SinglePixel>> = DisplayDriver::new(
    ///     DynamicPixelDisplay::<SinglePixel>::build_from_data(
    ///         6,
    ///         6,
    ///         &vec![
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
    fn pixel(&self, x: usize, y: usize) -> Result<T::U, String>
    where
        [(); T::WIDTH * T::HEIGHT]:,
    {
        if x >= self.width() || y >= self.height() {
            return Err(format!(
                "Pixel coordinates out of bounds. Got x = {x}, y = {y}."
            ));
        }

        let block_x: usize = x / T::WIDTH;
        let block_y: usize = y / T::HEIGHT;
        let offset_x: usize = x % T::WIDTH;
        let offset_y: usize = y % T::HEIGHT;

        let pixel =
            &self.data()[block_x + block_y * self.width_characters()];

        Ok(pixel
            .subpixel(offset_x, offset_y)
            .expect("Offset should be 0 or 1."))
    }

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
    fn set_pixel(
        &mut self,
        x: usize,
        y: usize,
        value: T::U,
    ) -> Result<(), String>
    where
        [(); T::WIDTH * T::HEIGHT]:,
    {
        if x >= self.width() || y >= self.height() {
            return Err(format!(
                "Pixel coordinates out of bounds. Got x = {x}, y = {y}."
            ));
        }

        let block_x: usize = x / T::WIDTH;
        let block_y: usize = y / T::HEIGHT;
        let offset_x: usize = x % T::WIDTH;
        let offset_y: usize = y % T::HEIGHT;

        let width_characters = self.width_characters();
        let pixel =
            &mut self.data_mut()[block_x + block_y * width_characters];
        pixel
            .set_subpixel(offset_x, offset_y, value)
            .expect("Offset should be 0 or 1.");

        Ok(())
    }

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
        for x in 0..self.width() {
            for y in 0..self.height() {
                pixels.push(self.pixel(x, y).expect(
                    "Invariant violated, pixel index out of range.",
                ));
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
    fn set_pixels(&mut self, data: &[T::U]) -> Result<(), String>
    where
        [(); T::WIDTH * T::HEIGHT]:,
    {
        if data.len() != self.width() * self.height() {
            return Err(format!(
                "Data does not match specified dimensions. Expected length of {}, got {}.",
                self.width() * self.height(),
                data.len()
            ));
        }
        for x in 0..self.width() {
            for y in 0..self.height() {
                self.set_pixel(x, y, data[x + y * self.width()]).expect(
                    "Invariant violated, pixel index out of range.",
                );
            }
        }
        Ok(())
    }

    #[must_use]
    fn data(&self) -> &[T];

    fn data_mut(&mut self) -> &mut Box<[T]>;

    fn draw_line(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        value: T::U,
    ) where
        [(); T::WIDTH * T::HEIGHT]:,
    {
        let dx = x2 - x1;
        let dy = y2 - y1;

        let steps = dx.abs().max(dy.abs());
        let x_inc = dx as f32 / steps as f32;
        let y_inc = dy as f32 / steps as f32;

        let mut x = x1 as f32;
        let mut y = y1 as f32;

        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        for _ in 0..=steps {
            if x >= 0. && y >= 0. {
                let _ = self.set_pixel(x as usize, y as usize, value);
            }
            x += x_inc;
            y += y_inc;
        }
    }

    fn draw_line_f32(
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

        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        for _ in 0..=steps.round() as usize {
            if x > -0.5 && y > -0.5 {
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
