use num_traits::NumCast;

use crate::{
    drawing::{
        DynamicCanvas,
        GetPixel,
        SetPixel,
    },
    error::{
        COULD_NOT_CAST_X_COORD,
        COULD_NOT_CAST_Y_COORD,
        DisplayError,
        PIXEL_INDEX_OUT_OF_RANGE,
    },
    pixel::Pixel,
};

// TODO: Check if this can be const
#[deprecated]
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
        konst::for_range! { y in 0..self.height() =>
            konst::for_range! { x in 0..self.width() =>
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
        konst::for_range! { y in 0..self.height() =>
            konst::for_range! { x in 0..self.width() =>
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

pub const trait DynamicWidth {
    /// Returns the width of the display in a display specific, individually addressable unit (e.g. pixels, characters).
    #[must_use]
    fn width(&self) -> usize;
}

pub const trait DynamicHeight {
    /// Returns the height of the display in a display specific, individually addressable unit (e.g. pixels, characters).
    #[must_use]
    fn height(&self) -> usize;
}

pub trait SetPixels<T: Pixel>:
    DynamicWidth + DynamicHeight + SetPixel<T>
{
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
        konst::for_range! { y in 0..self.height() =>
            konst::for_range! { x in 0..self.width() =>
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
}

pub trait GetPixels<T: Pixel>:
    DynamicWidth + DynamicHeight + GetPixel<T>
{
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
        konst::for_range! { y in 0..self.height() =>
            konst::for_range! { x in 0..self.width() =>
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
}

pub const trait GetData<T: Pixel> {
    #[must_use]
    fn data(&self) -> &[T];
}

pub const trait GetDataMut<T: Pixel> {
    fn data_mut(&mut self) -> &mut Box<[T]>;
}
