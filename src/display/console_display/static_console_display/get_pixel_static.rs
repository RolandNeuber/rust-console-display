use crate::{
    console_display::{
        GetData,
        Height,
        Width,
    },
    constraint,
    pixel::Pixel,
    widget::StaticCharacterWidth,
};

pub trait GetPixelStatic<T: Pixel>:
    StaticCharacterWidth + Width + Height + GetData<T>
{
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
    ///     pixel_display::StaticPixelDisplay,
    ///     drawing::DynamicCanvas
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
}
