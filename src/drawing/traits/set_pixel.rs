use num_traits::NumCast;

use crate::{
    drawing::{
        Drawable,
        Transformable,
    },
    error::DrawingError,
    pixel::Pixel,
};

pub const trait SetPixel<S: Pixel> {
    type A: NumCast;

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
        x: Self::A,
        y: Self::A,
        value: S::U,
    ) -> Result<(), DrawingError>
    where
        [(); S::WIDTH * S::HEIGHT]:;

    /// Draw a drawable/shape onto a canvas with the specified pixel type/brush.
    /// Convenience method for inversing `DynamicDrawable::draw` by using double dispatch.
    fn draw<
        D: [const] Drawable<N> + [const] Transformable,
        const N: usize,
    >(
        &mut self,
        drawable: &D,
        value: S::U,
    ) where
        Self: Sized,
        [(); S::WIDTH * S::HEIGHT]:,
    {
        drawable.draw(self, value);
    }
}
