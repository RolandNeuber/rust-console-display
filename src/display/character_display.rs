use std::marker::PhantomData;

use num_traits::NumCast;

use crate::{
    console_display::{
        DynamicConsoleDisplay,
        StaticConsoleDisplay,
    },
    drawing::DynamicCanvas,
    optional_const_generics::{
        CompileTime,
        Dimension,
        RunTime,
    },
    pixel::{
        Pixel,
        character_pixel::{
            CharacterPixel,
            CharacterPixelData,
        },
    },
    widget::{
        DynamicWidget,
        StaticWidget,
        StringData,
    },
};

pub type DynamicCharacterDisplay<CharacterPixel> =
    CharacterDisplay<RunTime, RunTime, CharacterPixel>;
pub type StaticCharacterDisplay<
    CharacterPixel,
    const WIDTH: usize,
    const HEIGHT: usize,
> = CharacterDisplay<
    CompileTime<WIDTH>,
    CompileTime<HEIGHT>,
    CharacterPixel,
>;

pub struct CharacterDisplay<W: Dimension, H: Dimension, CharacterPixel> {
    _w: PhantomData<W>,
    _h: PhantomData<H>,
    width: usize,
    height: usize,
    data: Box<[CharacterPixel]>,
}

impl CharacterDisplay<RunTime, RunTime, CharacterPixel> {
    /// Convenience method to build a blank display struct with specified dimensions
    ///
    /// # Panics
    ///
    /// This function panics if the data generated from the fill does not match the dimensions of the display.
    /// This should not happen and is subject to change in the future.
    #[must_use]
    pub fn new(width: usize, height: usize, fill: CharacterPixel) -> Self {
        let character_width = fill.width();
        let full_columns = width / character_width;
        let padding = width % character_width;
        let total_columns = full_columns + padding;

        let mut data: Vec<CharacterPixel> =
            vec![fill; total_columns * height];

        for i in 0..padding {
            for y in 0..height {
                let x = full_columns + i;
                data[x + y * total_columns] = CharacterPixel::build(' ', fill.foreground(), fill.background())
                    .expect("Invariant violated. Should not be a control character.");
            }
        }

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
        data: &[CharacterPixel],
    ) -> Result<Self, String> {
        let mut new_data = Vec::with_capacity(data.len());
        let mut row_length = 0;
        for i in data {
            new_data.push(*i);
            for _ in 1..i.width() {
                new_data.push(i.make_copy());
            }
            row_length += i.width();
            if row_length > width && row_length - i.width() < width {
                return Err(
                    "Data is malformed, character spans multiple rows."
                        .to_string(),
                );
            }
            if row_length >= width {
                row_length = 0;
            }
        }

        if new_data.len() != width * height {
            return Err(format!(
                "Data does not match specified dimensions. Expected length of {}, got {}.",
                width * height,
                new_data.len()
            ));
        }

        Ok(Self {
            _w: PhantomData,
            _h: PhantomData,
            width,
            height,
            data: new_data.into_boxed_slice(),
        })
    }
}

impl<const WIDTH: usize, const HEIGHT: usize>
    CharacterDisplay<
        CompileTime<WIDTH>,
        CompileTime<HEIGHT>,
        CharacterPixel,
    >
{
    /// Convenience method to build a blank display struct with specified dimensions
    ///
    /// # Panics
    ///
    /// This function panics if the data generated from the fill does not match the dimensions of the display.
    /// This should not happen and is subject to change in the future.
    #[must_use]
    pub fn new(fill: CharacterPixel) -> Self
    where
        [(); WIDTH * HEIGHT]:,
    {
        let character_width = fill.width();
        let full_columns = WIDTH / character_width;
        let padding = WIDTH % character_width;
        let total_columns = full_columns + padding;

        let mut data: Vec<CharacterPixel> =
            vec![fill; total_columns * HEIGHT];

        for i in 0..padding {
            for y in 0..HEIGHT {
                let x = full_columns + i;
                data[x + y * total_columns] = CharacterPixel::build(' ', fill.foreground(), fill.background())
                    .expect("Invariant violated. Should not be a control character.");
            }
        }

        Self::build_from_data(&data).expect(
            "Invariant violated, data does not mach specified dimensions.",
        )
    }

    /// Builds a display struct with the specified dimensions from the given data.
    ///
    /// # Errors
    /// Returns an error when the data length does not match the dimensions of the display.
    /// This also applies if double-width characters, like あ, are used and exceed the dimensions of the display.
    pub fn build_from_data(
        data: &[CharacterPixel],
    ) -> Result<Self, String> {
        let mut new_data = Vec::with_capacity(data.len());
        let mut row_length = 0;
        for i in data {
            new_data.push(*i);
            for _ in 1..i.width() {
                new_data.push(i.make_copy());
            }
            row_length += i.width();
            if row_length > WIDTH && row_length - i.width() < WIDTH {
                return Err(
                    "Data is malformed, character spans multiple rows."
                        .to_string(),
                );
            }
            if row_length >= WIDTH {
                row_length = 0;
            }
        }

        if new_data.len() != WIDTH * HEIGHT {
            return Err(format!(
                "Data does not match specified dimensions. Expected length of {}, got {}.",
                WIDTH * HEIGHT,
                new_data.len()
            ));
        }

        Ok(Self {
            _w: PhantomData,
            _h: PhantomData,
            width: WIDTH,
            height: HEIGHT,
            data: new_data.into_boxed_slice(),
        })
    }
}

impl<W: Dimension, H: Dimension> DynamicConsoleDisplay<CharacterPixel>
    for CharacterDisplay<W, H, CharacterPixel>
{
    default fn width(&self) -> usize {
        self.width
    }

    default fn height(&self) -> usize {
        self.height
    }

    default fn data(&self) -> &[CharacterPixel] {
        &self.data
    }

    default fn data_mut(&mut self) -> &mut Box<[CharacterPixel]> {
        &mut self.data
    }
}

impl<const WIDTH: usize, const HEIGHT: usize>
    DynamicConsoleDisplay<CharacterPixel>
    for CharacterDisplay<
        CompileTime<WIDTH>,
        CompileTime<HEIGHT>,
        CharacterPixel,
    >
{
    fn width(&self) -> usize {
        WIDTH
    }

    fn height(&self) -> usize {
        HEIGHT
    }

    fn data(&self) -> &[CharacterPixel] {
        &self.data
    }

    fn data_mut(&mut self) -> &mut Box<[CharacterPixel]> {
        &mut self.data
    }
}

impl<W: Dimension, H: Dimension> DynamicWidget
    for CharacterDisplay<W, H, CharacterPixel>
{
    default fn width_characters(&self) -> usize {
        self.width / CharacterPixel::WIDTH
    }

    default fn height_characters(&self) -> usize {
        self.height / CharacterPixel::HEIGHT
    }

    default fn string_data(&self) -> StringData {
        let mut result = Vec::new();
        let mut row = Vec::new();
        let mut width = 0;

        let mut iter = self.data.iter();
        while let Some(cell) = iter.next() {
            if width >= self.width_characters() {
                result.push(row);
                row = Vec::new();
                width = 0;
            }

            if cell.is_copy() {
                row.push(CharacterPixel::default().into());
                width += 1;
                continue;
            }

            row.push((*cell).into());
            width += cell.width();

            for _ in 1..cell.width() {
                iter.next();
            }
        }

        if !row.is_empty() {
            result.push(row);
        }

        StringData { data: result }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> DynamicWidget
    for CharacterDisplay<
        CompileTime<WIDTH>,
        CompileTime<HEIGHT>,
        CharacterPixel,
    >
{
    fn width_characters(&self) -> usize {
        WIDTH
    }

    fn height_characters(&self) -> usize {
        HEIGHT
    }

    fn string_data(&self) -> StringData {
        let mut result = Vec::new();
        let mut row = Vec::new();
        let mut width = 0;

        let mut iter = self.data.iter();
        while let Some(cell) = iter.next() {
            if width >= Self::WIDTH_CHARACTERS {
                result.push(row);
                row = Vec::new();
                width = 0;
            }

            if cell.is_copy() {
                row.push(CharacterPixel::default().into());
                width += 1;
                continue;
            }

            row.push((*cell).into());
            width += cell.width();

            for _ in 1..cell.width() {
                iter.next();
            }
        }

        if !row.is_empty() {
            result.push(row);
        }

        StringData { data: result }
    }
}

impl<W: Dimension, H: Dimension, S: Pixel<U = CharacterPixelData>>
    DynamicCanvas<S> for CharacterDisplay<W, H, CharacterPixel>
{
    default type A = usize;

    default fn pixel(
        &self,
        x: Self::A,
        y: Self::A,
    ) -> Result<CharacterPixelData, String>
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

    default fn set_pixel(
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

impl<const WIDTH: usize, const HEIGHT: usize> DynamicCanvas<CharacterPixel>
    for CharacterDisplay<
        CompileTime<WIDTH>,
        CompileTime<HEIGHT>,
        CharacterPixel,
    >
{
    type A = usize;

    fn pixel(
        &self,
        x: Self::A,
        y: Self::A,
    ) -> Result<CharacterPixelData, String>
    where
        [(); CharacterPixel::WIDTH * CharacterPixel::HEIGHT]:,
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

            let block_x: usize = x / CharacterPixel::WIDTH;
            let block_y: usize = y / CharacterPixel::HEIGHT;

            let pixel =
                &self.data()[block_x + block_y * self.width_characters()];

            Ok(pixel.subpixel(0, 0).expect("Offset should be 0."))
        }
        else {
            Err("Coordinates could not be converted to usize.".to_string())
        }
    }

    fn set_pixel(
        &mut self,
        x: Self::A,
        y: Self::A,
        value: CharacterPixelData,
    ) -> Result<(), String>
    where
        [(); CharacterPixel::WIDTH * CharacterPixel::HEIGHT]:,
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

            let block_x: usize = x / CharacterPixel::WIDTH;
            let block_y: usize = y / CharacterPixel::HEIGHT;

            let width_characters = self.width_characters();
            let pixel =
                &mut self.data_mut()[block_x + block_y * width_characters];
            pixel
                .set_subpixel(0, 0, value)
                .expect("Offset should be 0.");

            Ok(())
        }
        else {
            Err("Coordinates could not be converted to usize.".to_string())
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize>
    StaticConsoleDisplay<CharacterPixel>
    for CharacterDisplay<
        CompileTime<WIDTH>,
        CompileTime<HEIGHT>,
        CharacterPixel,
    >
{
    const WIDTH: usize = WIDTH;

    const HEIGHT: usize = HEIGHT;
}

impl<const WIDTH: usize, const HEIGHT: usize> const StaticWidget
    for CharacterDisplay<
        CompileTime<WIDTH>,
        CompileTime<HEIGHT>,
        CharacterPixel,
    >
{
    const WIDTH_CHARACTERS: usize = WIDTH;

    const HEIGHT_CHARACTERS: usize = HEIGHT;
}

#[cfg(test)]
mod tests {
    use crate::color::TerminalColor;

    use super::*;

    #[test]
    fn build_from_data_success() {
        let character_display = StaticCharacterDisplay::<
            CharacterPixel,
            1,
            1,
        >::build_from_data(&[
            CharacterPixel::build(
                ' ',
                TerminalColor::Default,
                TerminalColor::Default,
            )
            .unwrap(),
        ]);
        assert!(character_display.is_ok());
    }

    #[test]
    fn build_from_data_failure_dimensions() {
        let character_display = StaticCharacterDisplay::<
            CharacterPixel,
            8,
            10,
        >::build_from_data(&vec![
                    CharacterPixel::build(
                        ' ',
                        TerminalColor::Default,
                        TerminalColor::Default,
                    )
                    .unwrap();
                    8 * 10 - 1
                ]);
        assert!(character_display.is_err());
    }

    #[test]
    fn build_from_data_failure_fit() {
        let character_display = StaticCharacterDisplay::<
            CharacterPixel,
            9,
            10,
        >::build_from_data(&vec![
                    CharacterPixel::build(
                        'あ',
                        TerminalColor::Default,
                        TerminalColor::Default,
                    )
                    .unwrap();
                    9 * 10 / 2
                ]);
        assert!(character_display.is_err());
    }
}
