use crate::{
    traits::{
        GetDataMut,
        Height,
        Width,
    },
    constraint,
    pixel::Pixel,
    widget::StaticCharacterWidth,
};

pub trait SetPixelStatic<T: Pixel>:
    StaticCharacterWidth + Width + Height + GetDataMut<T>
{
    // TODO: Update docs
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
    fn set_pixel_static<const X: usize, const Y: usize>(
        &mut self,
        value: T::U,
    ) where
        constraint!(X < Self::WIDTH):,
        constraint!(Y < Self::HEIGHT):,
        constraint!(X % T::WIDTH < T::WIDTH):,
        constraint!(Y % T::HEIGHT < T::HEIGHT):,
        [(); T::WIDTH * T::HEIGHT]:,
    {
        let pixel = &mut self.data_mut()
            [X / T::WIDTH + Y / T::HEIGHT * Self::WIDTH_CHARACTERS];
        pixel.set_subpixel_static::<{ X % T::WIDTH }, { Y % T::HEIGHT }>(
            value,
        );
    }
}
