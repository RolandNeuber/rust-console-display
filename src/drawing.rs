use core::f32;

use num_traits::NumCast;

use crate::{
    error::DrawingError,
    pixel::Pixel,
    widget::DynamicWidget,
};

pub mod ellipse;
pub mod line;
pub mod rectangle;

pub use ellipse::*;
pub use line::*;
pub use rectangle::*;

/// Defines a fill for a drawable.
pub const trait FillType {}

/// Defines no fill on a drawable, e.g. only outline.
#[derive(PartialEq, Eq, Debug)]
pub struct NoFill;
impl const FillType for NoFill {}

/// Defines flat fill on a drawable.
#[derive(PartialEq, Eq, Debug)]
pub struct Filled;
impl const FillType for Filled {}

/// Defines an object that you can draw on and query pixels from.
#[deprecated]
pub const trait DynamicCanvas<S: Pixel>: DynamicWidget {
    type A: NumCast;
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
    ///     pixel_display::DynamicPixelDisplay,
    ///     drawing::DynamicCanvas
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
    fn pixel(&self, x: Self::A, y: Self::A) -> Result<S::U, DrawingError>
    where
        [(); S::WIDTH * S::HEIGHT]:;

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
    fn draw<D: [const] DynamicDrawable<N>, const N: usize>(
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

pub const trait GetPixel<S: Pixel> {
    type A: NumCast;

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
    ///     pixel_display::DynamicPixelDisplay,
    ///     drawing::DynamicCanvas
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
    fn pixel(&self, x: Self::A, y: Self::A) -> Result<S::U, DrawingError>
    where
        [(); S::WIDTH * S::HEIGHT]:;
}

/// Defines an object that can be drawn onto a canvas.
#[deprecated]
pub const trait DynamicDrawable<const N: usize> {
    /// Draws the drawable onto a canvas with the specified pixel type/brush.
    fn draw<T: DynamicCanvas<S>, S: Pixel>(
        &self,
        display: &mut T,
        value: S::U,
    ) where
        [(); S::WIDTH * S::HEIGHT]:;

    /// Transforms the drawable by applying a function to all coordinate pairs that define it.
    /// Returns a new transformed drawable.
    #[must_use]
    fn transform<F: [const] Fn((f32, f32)) -> (f32, f32)>(
        &self,
        transform: F,
    ) -> Self;
}

/// Defines an object that can be drawn onto a canvas.
pub const trait Drawable<const N: usize> {
    /// Draws the drawable onto a canvas with the specified pixel type/brush.
    fn draw<T: SetPixel<S>, S: Pixel>(&self, display: &mut T, value: S::U)
    where
        [(); S::WIDTH * S::HEIGHT]:;
}

pub const trait Transformable {
    /// Transforms the drawable by applying a function to all coordinate pairs that define it.
    /// Returns a new transformed drawable.
    #[must_use]
    fn transform<F: [const] Fn((f32, f32)) -> (f32, f32)>(
        &self,
        transform: F,
    ) -> Self;
}
