use num_traits::NumCast;

use crate::{
    drawing::SetPixel,
    error::{
        COULD_NOT_CAST_X_COORD,
        COULD_NOT_CAST_Y_COORD,
        DisplayError,
        PIXEL_INDEX_OUT_OF_RANGE,
    },
    pixel::Pixel,
    traits::{
        DynamicHeight,
        DynamicWidth,
    },
};

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

#[cfg(test)]
mod tests {
    use crate::{
        drawing::GetPixel,
        pixel::monochrome::SinglePixel,
        pixel_display::DynamicPixelDisplay,
    };

    use super::*;

    #[test]
    fn set_pixels() {
        let mut dynamic_pixel_display =
            DynamicPixelDisplay::<SinglePixel>::new(2, 1, false);
        dynamic_pixel_display.set_pixels(&[true, false]).unwrap();
        assert!(dynamic_pixel_display.pixel(0, 0).unwrap());
        assert!(!dynamic_pixel_display.pixel(1, 0).unwrap());
    }
}
