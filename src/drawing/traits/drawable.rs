use crate::{
    drawing::SetPixel,
    pixel::Pixel,
};

/// Defines an object that can be drawn onto a canvas.
pub const trait Drawable<const N: usize> {
    /// Draws the drawable onto a canvas with the specified pixel type/brush.
    fn draw<T: SetPixel<S>, S: Pixel>(&self, display: &mut T, value: S::U)
    where
        [(); S::WIDTH * S::HEIGHT]:;
}
