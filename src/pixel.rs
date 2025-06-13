use std::fmt::Display;

pub mod character_pixel;
pub mod color_pixel;
pub mod monochrome_pixel;

pub trait Pixel: Display + Copy
where
    Self: Sized,
{
    type U: Copy;

    /// The width of the block of pixels.
    const WIDTH: usize;
    /// The height of the block of pixels.
    const HEIGHT: usize;

    fn get_pixels(&self) -> &[Self::U; Self::WIDTH * Self::HEIGHT];

    fn get_pixels_mut(
        &mut self,
    ) -> &mut [Self::U; Self::WIDTH * Self::HEIGHT];

    /// Returns the value of the block at the specified coordinates.
    ///
    /// # Errors
    ///
    /// Returns an error, if the coordinates are out of bounds.
    fn get_subpixel(&self, x: usize, y: usize) -> Result<Self::U, String>
    where
        [(); Self::WIDTH * Self::HEIGHT]:,
    {
        self.get_pixels().get(x + y * Self::WIDTH).map_or_else(
            || Err("Coordinates out of range.".to_string()),
            |subpixel| Ok(*subpixel),
        )
    }

    /// Returns the value of the block at the specified coordinates.
    ///
    /// # Errors
    ///
    /// Returns an error, if the coordinates are out of bounds.
    fn set_subpixel(
        &mut self,
        x: usize,
        y: usize,
        value: Self::U,
    ) -> Result<(), String>
    where
        [(); Self::WIDTH * Self::HEIGHT]:,
    {
        let index = x + y * Self::WIDTH;
        if index < self.get_pixels().len() {
            self.get_pixels_mut()[index] = value;
            Ok(())
        }
        else {
            Err("Coordinates out of range.".to_string())
        }
    }
}
