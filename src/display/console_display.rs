use crate::{
    pixel::Pixel,
    widget::DynamicWidget,
};

pub trait ConsoleDisplay<T: Pixel>: DynamicWidget {
    /// Returns the width of the display in a display specific, individually addressable unit (e.g. pixels, characters).
    fn get_width(&self) -> usize;
    /// Returns the height of the display in a display specific, individually addressable unit (e.g. pixels, characters).
    fn get_height(&self) -> usize;

    // TODO: Add proper double buffering.
    /// Returns a vector containing all the pixels in the display.
    ///
    /// # Panics
    ///
    /// This function panics if the index of a pixel is out of bounds.
    /// This should not happen and is subject to change in the future.
    #[must_use]
    fn get_pixels(&self) -> Vec<T::U>
    where
        [(); T::WIDTH * T::HEIGHT]:,
    {
        let mut pixels =
            Vec::with_capacity(self.get_width() * self.get_height());
        for x in 0..self.get_width() {
            for y in 0..self.get_height() {
                pixels.push(self.get_pixel(x, y).expect(
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
        if data.len() != self.get_width() * self.get_height() {
            return Err(format!(
                "Data does not match specified dimensions. Expected length of {}, got {}.",
                self.get_width() * self.get_height(),
                data.len()
            ));
        }
        for x in 0..self.get_width() {
            for y in 0..self.get_height() {
                self.set_pixel(x, y, data[x + y * self.get_width()])
                    .expect(
                        "Invariant violated, pixel index out of range.",
                    );
            }
        }
        Ok(())
    }

    #[must_use]
    fn get_data(&self) -> &[T];

    fn get_data_mut(&mut self) -> &mut [T];

    /// Returns a bool representing the state of the pixel at the specified coordinate.
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
    fn get_pixel(&self, x: usize, y: usize) -> Result<T::U, String>
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

        Ok(pixel
            .get_subpixel(offset_x, offset_y)
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
        pixel
            .set_subpixel(offset_x, offset_y, value)
            .expect("Offset should be 0 or 1.");

        Ok(())
    }

    fn draw_line(
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
