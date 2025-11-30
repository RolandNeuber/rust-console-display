use crate::{
    constraint,
    error::PixelError,
    widget::DataCell,
};

pub mod character_pixel;
pub mod color_pixel;
pub mod monochrome_pixel;

pub trait Pixel: Copy + Into<DataCell>
where
    Self: Sized,
{
    type U: Copy;

    /// The width of the block of pixels.
    const WIDTH: usize;
    /// The height of the block of pixels.
    const HEIGHT: usize;

    fn pixels(&self) -> &[Self::U; Self::WIDTH * Self::HEIGHT];

    fn pixels_mut(&mut self)
    -> &mut [Self::U; Self::WIDTH * Self::HEIGHT];

    /// Returns the value of the block at the specified coordinates.
    ///
    /// # Errors
    ///
    /// Returns an error, if the coordinates are out of bounds.
    fn subpixel(&self, x: usize, y: usize) -> Result<Self::U, PixelError>
    where
        [(); Self::WIDTH * Self::HEIGHT]:,
    {
        if x < Self::WIDTH || y < Self::HEIGHT {
            Ok(self.pixels()[x + y * Self::WIDTH])
        }
        else {
            Err(PixelError::CoordinatesOutOfBounds(
                x,
                Self::WIDTH,
                y,
                Self::HEIGHT,
            ))
        }
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
    ) -> Result<(), PixelError>
    where
        [(); Self::WIDTH * Self::HEIGHT]:,
    {
        if x < Self::WIDTH || y < Self::HEIGHT {
            self.pixels_mut()[x + y * Self::WIDTH] = value;
            Ok(())
        }
        else {
            Err(PixelError::CoordinatesOutOfBounds(
                x,
                Self::WIDTH,
                y,
                Self::HEIGHT,
            ))
        }
    }

    fn new(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]) -> Self;

    /// Builds a block of pixels from a slice of pixels.
    ///
    /// # Errors
    ///
    /// Returns an error, if the number of pixels does not match the dimensions of the block.
    fn build(args: &[Self::U]) -> Result<Self, PixelError>
    where
        [(); Self::WIDTH * Self::HEIGHT]:,
    {
        <[Self::U; Self::WIDTH * Self::HEIGHT]>::try_from(args)
            .map_or_else(
                |_| {
                    Err(PixelError::InvalidNumberOfArguments(
                        Self::WIDTH * Self::HEIGHT,
                        args.len(),
                    ))
                },
                |pixels| Ok(Self::new(pixels)),
            )
    }

    fn subpixel_static<const X: usize, const Y: usize>(&self) -> Self::U
    where
        [(); Self::WIDTH * Self::HEIGHT]:,
        constraint!(X < Self::WIDTH):,
        constraint!(Y < Self::HEIGHT):,
    {
        self.pixels()[X + Y * Self::WIDTH]
    }

    fn set_subpixel_static<const X: usize, const Y: usize>(
        &mut self,
        value: Self::U,
    ) where
        [(); Self::WIDTH * Self::HEIGHT]:,
        constraint!(X < Self::WIDTH):,
        constraint!(Y < Self::HEIGHT):,
    {
        self.pixels_mut()[X + Y * Self::WIDTH] = value;
    }
}
