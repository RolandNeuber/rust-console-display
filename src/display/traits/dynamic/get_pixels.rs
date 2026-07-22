use num_traits::NumCast;

use crate::{
    drawing::GetPixel,
    error::{
        COULD_NOT_CAST_X_COORD,
        COULD_NOT_CAST_Y_COORD,
        PIXEL_INDEX_OUT_OF_RANGE,
    },
    pixel::Pixel,
    traits::{
        DynamicHeight,
        DynamicWidth,
    },
};

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
