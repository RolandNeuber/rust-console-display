use core::array;
use std::marker::PhantomData;

use num_traits::NumCast;

use crate::{
    console_display::{
        DynamicHeight,
        DynamicWidth,
        GetData,
        GetDataMut,
        GetPixelStatic,
        GetPixels,
        Height,
        SetPixelStatic,
        SetPixels,
        Width,
    },
    drawing::{
        GetPixel,
        SetPixel,
    },
    error::{
        DATA_DOES_NOT_MATCH_DIMENSIONS,
        DisplayError,
        DrawingError,
        OFFSET_SHOULD_BE_0_OR_1,
    },
    optional_const_generics::{
        CompileTime,
        Dimension,
        RunTime,
    },
    pixel::Pixel,
    widget::{
        DynamicCharacterHeight,
        DynamicCharacterWidth,
        StaticCharacterHeight,
        StaticCharacterWidth,
        StringData,
        ToStringData,
    },
};

pub type DynamicPixelDisplay<T: Pixel> = PixelDisplay<RunTime, RunTime, T>;
pub type StaticPixelDisplay<
    T: Pixel,
    const WIDTH: usize,
    const HEIGHT: usize,
> = PixelDisplay<CompileTime<WIDTH>, CompileTime<HEIGHT>, T>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PixelDisplay<W: Dimension, H: Dimension, T: Pixel> {
    _w: PhantomData<W>,
    _h: PhantomData<H>,
    data: Box<[T]>,
    width: usize,
    height: usize,
}

impl<T: Pixel> PixelDisplay<RunTime, RunTime, T> {
    /// Convenience method to build a blank display struct with specified dimensions.
    ///
    /// # Panics
    ///
    /// This function panics if the data generated from the fill does not match the dimensions of the display.
    /// This should not happen and is subject to change in the future.
    // TODO: Check if this fn can be const
    pub fn new(width: usize, height: usize, fill: T::U) -> Self
    where
        Self: Sized,
        [(); T::WIDTH * T::HEIGHT]:,
        // Vec<<T as Pixel>::U>: [const] Deref,
        // T: [const] Pixel
    {
        let data: Vec<T::U> = vec![fill; width * height];
        Self::build_from_data(width, height, data.as_slice())
            .expect(DATA_DOES_NOT_MATCH_DIMENSIONS)
    }

    /// Builds a display struct from the given data with the specified dimensions.
    ///
    /// # Errors
    ///
    /// Returns an error when the length of the data does not match the dimensions of the display.
    // TODO: Check if this fn can be const
    pub fn build_from_data(
        width: usize,
        height: usize,
        data: &[T::U],
    ) -> Result<Self, DisplayError>
    where
        Self: Sized,
        [(); T::WIDTH * T::HEIGHT]:,
        // Vec<<T as Pixel>::U>: [const] Deref,
        // T: [const] Pixel
    {
        if !width.is_multiple_of(T::WIDTH) ||
            !height.is_multiple_of(T::HEIGHT)
        {
            return Err(DisplayError::DisplayDimensionsNotMultipleOfPixelDimensions(width, T::WIDTH, height, T::HEIGHT));
        }
        if data.len() != width * height {
            return Err(DisplayError::MismatchedDimensions(width, height));
        }

        let block_count_x = width / T::WIDTH;
        let block_count_y = height / T::HEIGHT;

        let mut multi_pixels =
            Vec::with_capacity(block_count_x * block_count_y);

        for row in 0..block_count_y {
            for col in 0..block_count_x {
                let block_x: usize = col * T::WIDTH;
                let block_y: usize = row * T::HEIGHT;

                let mut args: Vec<T::U> =
                    Vec::with_capacity(T::WIDTH * T::HEIGHT);

                for y in 0..T::HEIGHT {
                    for x in 0..T::WIDTH {
                        args.push(
                            data[block_x + x + (block_y + y) * width],
                        );
                    }
                }

                multi_pixels.push(T::build(args.as_slice())?);
            }
        }

        Ok(Self {
            _w: PhantomData,
            _h: PhantomData,
            width,
            height,
            data: multi_pixels.into_boxed_slice(),
        })
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, T: Pixel>
    PixelDisplay<CompileTime<WIDTH>, CompileTime<HEIGHT>, T>
{
    /// Convenience method to create a blank display struct with specified dimensions known at compile time.
    // TODO: Check if this fn can be const
    pub fn new(fill: T::U) -> Self
    where
        [(); T::WIDTH * T::HEIGHT]:,
        [(); WIDTH * HEIGHT]:,
        [(); 0 - WIDTH % T::WIDTH]:,
        [(); 0 - HEIGHT % T::HEIGHT]:,
        // T: [const] Pixel,
    {
        let data: [T::U; WIDTH * HEIGHT] = [fill; WIDTH * HEIGHT];
        Self::new_from_data(&data)
    }

    /// Creates a display struct from the given data with the specified dimensions known at compile time.
    // TODO: Check if this fn can be const
    pub fn new_from_data(data: &[T::U; WIDTH * HEIGHT]) -> Self
    where
        [(); T::WIDTH * T::HEIGHT]:,
        [(); 0 - WIDTH % T::WIDTH]:,
        [(); 0 - HEIGHT % T::HEIGHT]:,
        // T: [const] Pixel,
    {
        let mut multi_pixels = Vec::with_capacity(
            <Self as StaticCharacterWidth>::WIDTH_CHARACTERS *
                <Self as StaticCharacterHeight>::HEIGHT_CHARACTERS,
        );

        konst::for_range! { row in 0..<Self as StaticCharacterHeight>::HEIGHT_CHARACTERS =>
            konst::for_range! { col in 0..<Self as StaticCharacterWidth>::WIDTH_CHARACTERS =>
                let block_x: usize = col * T::WIDTH;
                let block_y: usize = row * T::HEIGHT;

                let args: [T::U; T::WIDTH * T::HEIGHT] =
                    array::from_fn(|i| {
                        let x = i % T::WIDTH;
                        let y = i / T::WIDTH;
                        data[block_x + x + (block_y + y) * WIDTH]
                    });

                multi_pixels.push(T::new(args));
            }
        }

        Self {
            _w: PhantomData,
            _h: PhantomData,
            width: WIDTH,
            height: HEIGHT,
            data: multi_pixels.into_boxed_slice(),
        }
    }
}

impl<W: Dimension, H: Dimension, T: Pixel> DynamicWidth
    for PixelDisplay<W, H, T>
{
    default fn width(&self) -> usize {
        self.width
    }
}

impl<W: Dimension, H: Dimension, T: Pixel> DynamicHeight
    for PixelDisplay<W, H, T>
{
    default fn height(&self) -> usize {
        self.height
    }
}

impl<W: Dimension, H: Dimension, T: Pixel> GetData<T>
    for PixelDisplay<W, H, T>
{
    default fn data(&self) -> &[T] {
        &self.data
    }
}

impl<W: Dimension, H: Dimension, T: Pixel> GetDataMut<T>
    for PixelDisplay<W, H, T>
{
    default fn data_mut(&mut self) -> &mut Box<[T]> {
        &mut self.data
    }
}

impl<T: Pixel, const WIDTH: usize, const HEIGHT: usize> DynamicWidth
    for PixelDisplay<CompileTime<WIDTH>, CompileTime<HEIGHT>, T>
{
    fn width(&self) -> usize {
        WIDTH
    }
}

impl<T: Pixel, const WIDTH: usize, const HEIGHT: usize> DynamicHeight
    for PixelDisplay<CompileTime<WIDTH>, CompileTime<HEIGHT>, T>
{
    fn height(&self) -> usize {
        HEIGHT
    }
}

impl<T: Pixel, const WIDTH: usize, const HEIGHT: usize> GetData<T>
    for PixelDisplay<CompileTime<WIDTH>, CompileTime<HEIGHT>, T>
{
    fn data(&self) -> &[T] {
        &self.data
    }
}

impl<T: Pixel, const WIDTH: usize, const HEIGHT: usize> GetDataMut<T>
    for PixelDisplay<CompileTime<WIDTH>, CompileTime<HEIGHT>, T>
{
    fn data_mut(&mut self) -> &mut Box<[T]> {
        &mut self.data
    }
}

impl<W: Dimension, H: Dimension, T: Pixel> DynamicCharacterWidth
    for PixelDisplay<W, H, T>
{
    default fn width_characters(&self) -> usize {
        self.width / T::WIDTH
    }
}

impl<W: Dimension, H: Dimension, T: Pixel> DynamicCharacterHeight
    for PixelDisplay<W, H, T>
{
    default fn height_characters(&self) -> usize {
        self.height / T::HEIGHT
    }
}

impl<W: Dimension, H: Dimension, T: Pixel> ToStringData
    for PixelDisplay<W, H, T>
{
    default fn string_data(&self) -> StringData {
        StringData {
            data: self
                .data
                .chunks(DynamicCharacterWidth::width_characters(self))
                .map(|chunk| chunk.iter().map(|x| (*x).into()).collect())
                .collect(),
        }
    }
}

default impl<T: Pixel, const WIDTH: usize, const HEIGHT: usize>
    StaticCharacterWidth
    for PixelDisplay<CompileTime<WIDTH>, CompileTime<HEIGHT>, T>
{
    const WIDTH_CHARACTERS: usize =
        <Self as StaticCharacterWidth>::WIDTH_CHARACTERS;
}

default impl<T: Pixel, const WIDTH: usize, const HEIGHT: usize>
    StaticCharacterHeight
    for PixelDisplay<CompileTime<WIDTH>, CompileTime<HEIGHT>, T>
{
    const HEIGHT_CHARACTERS: usize =
        <Self as StaticCharacterHeight>::HEIGHT_CHARACTERS;
}

impl<T: Pixel, const WIDTH: usize, const HEIGHT: usize> ToStringData
    for PixelDisplay<CompileTime<WIDTH>, CompileTime<HEIGHT>, T>
{
    default fn string_data(&self) -> StringData {
        StringData {
            data: self
                .data
                .chunks(<Self as StaticCharacterWidth>::WIDTH_CHARACTERS)
                .map(|chunk| chunk.iter().map(|x| (*x).into()).collect())
                .collect(),
        }
    }
}

impl<W: Dimension, H: Dimension, S: Pixel> GetPixel<S>
    for PixelDisplay<W, H, S>
{
    type A = usize;

    fn pixel(&self, x: Self::A, y: Self::A) -> Result<S::U, DrawingError>
    where
        [(); S::WIDTH * S::HEIGHT]:,
    {
        let x: Option<usize> = NumCast::from(x);
        let y: Option<usize> = NumCast::from(y);
        if let Some(x) = x &&
            let Some(y) = y
        {
            if x >= DynamicWidth::width(self) ||
                y >= DynamicHeight::height(self)
            {
                return Err(DisplayError::CoordinatesOutOfBounds(
                    x,
                    DynamicWidth::width(self),
                    y,
                    DynamicHeight::height(self),
                ))?;
            }

            let block_x: usize = x / S::WIDTH;
            let block_y: usize = y / S::HEIGHT;
            let offset_x: usize = x % S::WIDTH;
            let offset_y: usize = y % S::HEIGHT;

            let pixel = &GetData::data(self)[block_x +
                block_y * DynamicCharacterWidth::width_characters(self)];

            Ok(pixel
                .subpixel(offset_x, offset_y)
                .expect(OFFSET_SHOULD_BE_0_OR_1))
        }
        else {
            Err(DisplayError::CoordinatesToUsizeConversionFailed)?
        }
    }
}

impl<W: Dimension, H: Dimension, S: Pixel> SetPixel<S>
    for PixelDisplay<W, H, S>
{
    type A = usize;

    fn set_pixel(
        &mut self,
        x: Self::A,
        y: Self::A,
        value: S::U,
    ) -> Result<(), DrawingError>
    where
        [(); S::WIDTH * S::HEIGHT]:,
    {
        let x: Option<usize> = NumCast::from(x);
        let y: Option<usize> = NumCast::from(y);
        if let Some(x) = x &&
            let Some(y) = y
        {
            if x >= DynamicWidth::width(self) ||
                y >= DynamicHeight::height(self)
            {
                return Err(DisplayError::CoordinatesOutOfBounds(
                    x,
                    DynamicWidth::width(self),
                    y,
                    DynamicHeight::height(self),
                ))?;
            }

            let block_x: usize = x / S::WIDTH;
            let block_y: usize = y / S::HEIGHT;
            let offset_x: usize = x % S::WIDTH;
            let offset_y: usize = y % S::HEIGHT;

            let width_characters =
                DynamicCharacterWidth::width_characters(self);
            let pixel = &mut GetDataMut::data_mut(self)
                [block_x + block_y * width_characters];
            pixel
                .set_subpixel(offset_x, offset_y, value)
                .expect(OFFSET_SHOULD_BE_0_OR_1);

            Ok(())
        }
        else {
            Err(DisplayError::CoordinatesToUsizeConversionFailed)?
        }
    }
}

impl<W: Dimension, H: Dimension, S: Pixel> SetPixelStatic<S>
    for PixelDisplay<W, H, S>
where
    Self: Width + Height + StaticCharacterWidth,
{
}

impl<W: Dimension, H: Dimension, S: Pixel> GetPixelStatic<S>
    for PixelDisplay<W, H, S>
where
    Self: Width + Height + StaticCharacterWidth,
{
}

impl<W: Dimension, H: Dimension, S: Pixel> SetPixels<S>
    for PixelDisplay<W, H, S>
{
}

impl<W: Dimension, H: Dimension, S: Pixel> GetPixels<S>
    for PixelDisplay<W, H, S>
{
}

impl<T: Pixel, const WIDTH: usize, const HEIGHT: usize> Width
    for PixelDisplay<CompileTime<WIDTH>, CompileTime<HEIGHT>, T>
{
    const WIDTH: usize = WIDTH;
}

impl<T: Pixel, const WIDTH: usize, const HEIGHT: usize> Height
    for PixelDisplay<CompileTime<WIDTH>, CompileTime<HEIGHT>, T>
{
    const HEIGHT: usize = HEIGHT;
}

impl<T: Pixel, const WIDTH: usize, const HEIGHT: usize> const
    StaticCharacterWidth
    for PixelDisplay<CompileTime<WIDTH>, CompileTime<HEIGHT>, T>
{
    const WIDTH_CHARACTERS: usize = WIDTH / T::WIDTH;
}

impl<T: Pixel, const WIDTH: usize, const HEIGHT: usize> const
    StaticCharacterHeight
    for PixelDisplay<CompileTime<WIDTH>, CompileTime<HEIGHT>, T>
{
    const HEIGHT_CHARACTERS: usize = HEIGHT / T::HEIGHT;
}

#[cfg(test)]
mod tests {
    use crate::pixel::monochrome_pixel::SinglePixel;

    use super::*;

    mod dynamic_pixel_display {
        use super::*;

        #[test]
        fn build_from_data_success() {
            let dynamic_pixel_display =
                DynamicPixelDisplay::<SinglePixel>::build_from_data(
                    1,
                    1,
                    &[false],
                );
            assert!(dynamic_pixel_display.is_ok());
        }

        #[test]
        fn build_from_data_failure_less() {
            let dynamic_pixel_display =
                DynamicPixelDisplay::<SinglePixel>::build_from_data(
                    23,
                    1,
                    &[false; 22],
                );
            assert!(dynamic_pixel_display.is_err());
        }

        #[test]
        fn build_from_data_failure_more() {
            let dynamic_pixel_display =
                DynamicPixelDisplay::<SinglePixel>::build_from_data(
                    23,
                    1,
                    &[false; 24],
                );
            assert!(dynamic_pixel_display.is_err());
        }

        #[test]
        fn pixel_success() {
            let dynamic_pixel_display =
                DynamicPixelDisplay::<SinglePixel>::new(2, 1, false);
            let pixel = GetPixel::pixel(&dynamic_pixel_display, 1, 0);
            assert!(pixel.is_ok());
        }

        #[test]
        fn pixel_failure() {
            let dynamic_pixel_display =
                DynamicPixelDisplay::<SinglePixel>::new(2, 1, false);
            let pixel = GetPixel::pixel(&dynamic_pixel_display, 0, 1);
            assert!(pixel.is_err());
        }

        #[test]
        fn set_pixel_success() {
            let mut dynamic_pixel_display =
                DynamicPixelDisplay::<SinglePixel>::new(2, 1, false);
            let res = SetPixel::set_pixel(
                &mut dynamic_pixel_display,
                1,
                0,
                true,
            );
            let pixel = GetPixel::pixel(&dynamic_pixel_display, 1, 0);
            assert!(res.is_ok());
            assert_eq!(pixel, Ok(true));
        }

        #[test]
        fn set_pixel_failure() {
            let mut dynamic_pixel_display =
                DynamicPixelDisplay::<SinglePixel>::new(2, 1, false);
            let res = SetPixel::set_pixel(
                &mut dynamic_pixel_display,
                0,
                1,
                true,
            );
            assert!(res.is_err());
        }
    }
}
