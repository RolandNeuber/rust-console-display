use core::array;
use std::marker::PhantomData;

use num_traits::NumCast;

use crate::{
    console_display::{
        DynamicConsoleDisplay,
        StaticConsoleDisplay,
    }, drawing::DynamicCanvas, optional_const_generics::{CompileTime, Dimension, RunTime}, pixel::Pixel, widget::{
        DynamicWidget,
        StaticWidget,
        StringData,
    }
};

pub type DynamicPixelDisplay<T: Pixel> = PixelDisplay<RunTime, RunTime, T>;
pub type StaticPixelDisplay<T: Pixel, const WIDTH: usize, const HEIGHT: usize> = PixelDisplay<CompileTime<WIDTH>, CompileTime<HEIGHT>, T>;

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
    pub fn new(width: usize, height: usize, fill: T::U) -> Self
    where
        Self: Sized,
        [(); T::WIDTH * T::HEIGHT]:,
    {
        let data: Vec<T::U> = vec![fill; width * height];
        Self::build_from_data(width, height, &data).expect(
            "Invariant violated, data does not mach specified dimensions.",
        )
    }

    /// Builds a display struct from the given data with the specified dimensions.
    ///
    /// # Errors
    ///
    /// Returns an error when the length of the data does not match the dimensions of the display.
    pub fn build_from_data(
        width: usize,
        height: usize,
        data: &[T::U],
    ) -> Result<Self, String>
    where
        Self: Sized,
        [(); T::WIDTH * T::HEIGHT]:,
    {
        if !width.is_multiple_of(T::WIDTH) ||
            !height.is_multiple_of(T::HEIGHT)
        {
            return Err(format!(
                "Width and height must be multiples of multipixel dimensions. Got width = {width}, height = {height}."
            ));
        }
        if data.len() != width * height {
            return Err(format!(
                "Data does not match specified dimensions. Expected length of {}, got {}.",
                width * height,
                data.len()
            ));
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

                multi_pixels.push(T::build(&args)?);
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

impl<const WIDTH: usize, const HEIGHT: usize, T: Pixel> PixelDisplay<CompileTime<WIDTH>, CompileTime<HEIGHT>, T> {
    /// Convenience method to create a blank display struct with specified dimensions known at compile time.
    pub fn new(fill: T::U) -> Self
    where
        [(); T::WIDTH * T::HEIGHT]:,
        [(); WIDTH * HEIGHT]:,
        [(); 0 - WIDTH % T::WIDTH]:,
        [(); 0 - HEIGHT % T::HEIGHT]:,
    {
        let data: [T::U; WIDTH * HEIGHT] = [fill; WIDTH * HEIGHT];
        Self::new_from_data(&data)
    }

    /// Creates a display struct from the given data with the specified dimensions known at compile time.
    pub fn new_from_data(data: &[T::U; WIDTH * HEIGHT]) -> Self
    where
        [(); T::WIDTH * T::HEIGHT]:,
        [(); 0 - WIDTH % T::WIDTH]:,
        [(); 0 - HEIGHT % T::HEIGHT]:,
    {
        let mut multi_pixels = Vec::with_capacity(
            Self::WIDTH_CHARACTERS * Self::HEIGHT_CHARACTERS,
        );

        for row in 0..Self::HEIGHT_CHARACTERS {
            for col in 0..Self::WIDTH_CHARACTERS {
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

impl<W: Dimension, H: Dimension, T: Pixel> DynamicConsoleDisplay<T> for PixelDisplay<W, H, T> {
    default fn width(&self) -> usize {
        self.width
    }

    default fn height(&self) -> usize {
        self.height
    }

    default fn data(&self) -> &[T] {
        &self.data
    }

    default fn data_mut(&mut self) -> &mut Box<[T]> {
        &mut self.data
    }
}

impl<T: Pixel, const WIDTH: usize, const HEIGHT: usize>
    DynamicConsoleDisplay<T> for PixelDisplay<CompileTime<WIDTH>, CompileTime<HEIGHT>, T>
{
    fn width(&self) -> usize {
        WIDTH
    }

    fn height(&self) -> usize {
        HEIGHT
    }

    fn data(&self) -> &[T] {
        &self.data
    }

    fn data_mut(&mut self) -> &mut Box<[T]> {
        &mut self.data
    }
}

impl<W: Dimension, H: Dimension, T: Pixel> DynamicWidget for PixelDisplay<W, H, T> {
    default fn width_characters(&self) -> usize {
        self.width / T::WIDTH
    }

    default fn height_characters(&self) -> usize {
        self.height / T::HEIGHT
    }

    default fn string_data(&self) -> StringData {
        StringData {
            data: self
                .data
                .chunks(self.width_characters())
                .map(|chunk| chunk.iter().map(|x| (*x).into()).collect())
                .collect(),
        }
    }
}

impl<T: Pixel, const WIDTH: usize, const HEIGHT: usize> DynamicWidget
    for PixelDisplay<CompileTime<WIDTH>, CompileTime<HEIGHT>, T>
{
    fn width_characters(&self) -> usize {
        Self::WIDTH_CHARACTERS
    }

    fn height_characters(&self) -> usize {
        Self::HEIGHT_CHARACTERS
    }

    fn string_data(&self) -> StringData {
        StringData {
            data: self
                .data
                .chunks(Self::WIDTH_CHARACTERS)
                .map(|chunk| chunk.iter().map(|x| (*x).into()).collect())
                .collect(),
        }
    }
}

impl<W: Dimension, H: Dimension, S: Pixel> DynamicCanvas<S> for PixelDisplay<W, H, S> {
    type A = usize;

    fn pixel(&self, x: Self::A, y: Self::A) -> Result<S::U, String>
    where
        [(); S::WIDTH * S::HEIGHT]:,
    {
        let x: Option<usize> = NumCast::from(x);
        let y: Option<usize> = NumCast::from(y);
        if let Some(x) = x &&
            let Some(y) = y
        {
            if x >= self.width() || y >= self.height() {
                return Err(format!(
                    "Pixel coordinates out of bounds. Got x = {x}, y = {y}."
                ));
            }

            let block_x: usize = x / S::WIDTH;
            let block_y: usize = y / S::HEIGHT;
            let offset_x: usize = x % S::WIDTH;
            let offset_y: usize = y % S::HEIGHT;

            let pixel =
                &self.data()[block_x + block_y * self.width_characters()];

            Ok(pixel
                .subpixel(offset_x, offset_y)
                .expect("Offset should be 0 or 1."))
        }
        else {
            Err("Coordinates could not be converted to usize.".to_string())
        }
    }

    fn set_pixel(
        &mut self,
        x: Self::A,
        y: Self::A,
        value: S::U,
    ) -> Result<(), String>
    where
        [(); S::WIDTH * S::HEIGHT]:,
    {
        let x: Option<usize> = NumCast::from(x);
        let y: Option<usize> = NumCast::from(y);
        if let Some(x) = x &&
            let Some(y) = y
        {
            if x >= self.width() || y >= self.height() {
                return Err(format!(
                    "Pixel coordinates out of bounds. Got x = {x}, y = {y}."
                ));
            }

            let block_x: usize = x / S::WIDTH;
            let block_y: usize = y / S::HEIGHT;
            let offset_x: usize = x % S::WIDTH;
            let offset_y: usize = y % S::HEIGHT;

            let width_characters = self.width_characters();
            let pixel =
                &mut self.data_mut()[block_x + block_y * width_characters];
            pixel
                .set_subpixel(offset_x, offset_y, value)
                .expect("Offset should be 0 or 1.");

            Ok(())
        }
        else {
            Err("Coordinates could not be converted to usize.".to_string())
        }
    }
}

impl<T: Pixel, const WIDTH: usize, const HEIGHT: usize>
    StaticConsoleDisplay<T> for PixelDisplay<CompileTime<WIDTH>, CompileTime<HEIGHT>, T>
{
    const WIDTH: usize = WIDTH;

    const HEIGHT: usize = HEIGHT;
}


impl<T: Pixel, const WIDTH: usize, const HEIGHT: usize> const StaticWidget
    for PixelDisplay<CompileTime<WIDTH>, CompileTime<HEIGHT>, T>
{
    const WIDTH_CHARACTERS: usize = WIDTH / T::WIDTH;

    const HEIGHT_CHARACTERS: usize = HEIGHT / T::HEIGHT;
}

#[cfg(test)]
mod tests {
    use crate::pixel::monochrome_pixel::SinglePixel;

    use super::*;

    mod dynamic_pixel_display {
        use crate::drawing::DynamicCanvas;

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
            let pixel = dynamic_pixel_display.pixel(1, 0);
            assert!(pixel.is_ok());
        }

        #[test]
        fn pixel_failure() {
            let dynamic_pixel_display =
                DynamicPixelDisplay::<SinglePixel>::new(2, 1, false);
            let pixel = dynamic_pixel_display.pixel(0, 1);
            assert!(pixel.is_err());
        }

        #[test]
        fn set_pixel_success() {
            let mut dynamic_pixel_display =
                DynamicPixelDisplay::<SinglePixel>::new(2, 1, false);
            let res = dynamic_pixel_display.set_pixel(1, 0, true);
            let pixel = dynamic_pixel_display.pixel(1, 0);
            assert!(res.is_ok());
            assert_eq!(pixel, Ok(true));
        }

        #[test]
        fn set_pixel_failure() {
            let mut dynamic_pixel_display =
                DynamicPixelDisplay::<SinglePixel>::new(2, 1, false);
            let res = dynamic_pixel_display.set_pixel(0, 1, true);
            assert!(res.is_err());
        }
    }
}
