use std::marker::PhantomData;

use num_traits::NumCast;

use crate::{
    console_display::{
        DynamicConsoleDisplay,
        DynamicHeight,
        DynamicWidth,
        GetData,
        GetDataMut,
        GetPixelStatic,
        GetPixels,
        Height,
        SetPixelStatic,
        SetPixels,
        StaticConsoleDisplay,
        Width,
    },
    drawing::{
        DynamicCanvas,
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
    pixel::{
        Pixel,
        character_pixel::{
            CharacterPixel,
            CharacterPixelData,
        },
    },
    widget::{
        DynamicCharacterHeight,
        DynamicCharacterWidth,
        DynamicWidget,
        StaticCharacterHeight,
        StaticCharacterWidth,
        StaticWidget,
        StringData,
        ToStringData,
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

#[derive(Debug, Clone, PartialEq, Eq)]
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
                data[x + y * total_columns] = CharacterPixel::new::<' '>(
                    fill.foreground(),
                    fill.background(),
                );
            }
        }

        Self::build_from_data(width, height, &data)
            .expect(DATA_DOES_NOT_MATCH_DIMENSIONS)
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
    ) -> Result<Self, DisplayError> {
        let mut new_data = Vec::with_capacity(data.len());
        let mut row_length = 0;
        for i in data {
            new_data.push(*i);
            for _ in 1..i.width() {
                new_data.push(i.make_copy());
            }
            row_length += i.width();
            if row_length > width && row_length - i.width() < width {
                return Err(DisplayError::MalformedCharacterData(
                    i.character(),
                ));
            }
            if row_length >= width {
                row_length = 0;
            }
        }

        if new_data.len() != width * height {
            return Err(DisplayError::MismatchedDimensions(
                width * height,
                new_data.len(),
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
                data[x + y * total_columns] = CharacterPixel::new::<' '>(
                    fill.foreground(),
                    fill.background(),
                );
            }
        }

        Self::build_from_data(&data).expect(DATA_DOES_NOT_MATCH_DIMENSIONS)
    }

    /// Builds a display struct with the specified dimensions from the given data.
    ///
    /// # Errors
    /// Returns an error when the data length does not match the dimensions of the display.
    /// This also applies if double-width characters, like あ, are used and exceed the dimensions of the display.
    pub fn build_from_data(
        data: &[CharacterPixel],
    ) -> Result<Self, DisplayError> {
        let mut new_data = Vec::with_capacity(data.len());
        let mut row_length = 0;
        for i in data {
            new_data.push(*i);
            for _ in 1..i.width() {
                new_data.push(i.make_copy());
            }
            row_length += i.width();
            if row_length > WIDTH && row_length - i.width() < WIDTH {
                return Err(DisplayError::MalformedCharacterData(
                    i.character(),
                ));
            }
            if row_length >= WIDTH {
                row_length = 0;
            }
        }

        if new_data.len() != WIDTH * HEIGHT {
            return Err(DisplayError::MismatchedDimensions(
                WIDTH * HEIGHT,
                new_data.len(),
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

impl<W: Dimension, H: Dimension> DynamicWidth
    for CharacterDisplay<W, H, CharacterPixel>
{
    default fn width(&self) -> usize {
        self.width
    }
}

impl<W: Dimension, H: Dimension> DynamicHeight
    for CharacterDisplay<W, H, CharacterPixel>
{
    default fn height(&self) -> usize {
        self.height
    }
}

impl<W: Dimension, H: Dimension> GetData<CharacterPixel>
    for CharacterDisplay<W, H, CharacterPixel>
{
    default fn data(&self) -> &[CharacterPixel] {
        &self.data
    }
}

impl<W: Dimension, H: Dimension> GetDataMut<CharacterPixel>
    for CharacterDisplay<W, H, CharacterPixel>
{
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

impl<const WIDTH: usize, const HEIGHT: usize> DynamicWidth
    for CharacterDisplay<
        CompileTime<WIDTH>,
        CompileTime<HEIGHT>,
        CharacterPixel,
    >
{
    default fn width(&self) -> usize {
        WIDTH
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> DynamicHeight
    for CharacterDisplay<
        CompileTime<WIDTH>,
        CompileTime<HEIGHT>,
        CharacterPixel,
    >
{
    default fn height(&self) -> usize {
        HEIGHT
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> GetData<CharacterPixel>
    for CharacterDisplay<
        CompileTime<WIDTH>,
        CompileTime<HEIGHT>,
        CharacterPixel,
    >
{
    default fn data(&self) -> &[CharacterPixel] {
        &self.data
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> GetDataMut<CharacterPixel>
    for CharacterDisplay<
        CompileTime<WIDTH>,
        CompileTime<HEIGHT>,
        CharacterPixel,
    >
{
    default fn data_mut(&mut self) -> &mut Box<[CharacterPixel]> {
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
            if width >= DynamicWidget::width_characters(self) {
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

impl<W: Dimension, H: Dimension> DynamicCharacterWidth
    for CharacterDisplay<W, H, CharacterPixel>
{
    default fn width_characters(&self) -> usize {
        self.width / CharacterPixel::WIDTH
    }
}

impl<W: Dimension, H: Dimension> DynamicCharacterHeight
    for CharacterDisplay<W, H, CharacterPixel>
{
    default fn height_characters(&self) -> usize {
        self.height / CharacterPixel::HEIGHT
    }
}

impl<W: Dimension, H: Dimension> ToStringData
    for CharacterDisplay<W, H, CharacterPixel>
{
    default fn string_data(&self) -> StringData {
        let mut result = Vec::new();
        let mut row = Vec::new();
        let mut width = 0;

        let mut iter = self.data.iter();
        while let Some(cell) = iter.next() {
            if width >= DynamicCharacterWidth::width_characters(self) {
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
            if width >= <Self as StaticWidget>::WIDTH_CHARACTERS {
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

impl<const WIDTH: usize, const HEIGHT: usize> DynamicCharacterWidth
    for CharacterDisplay<
        CompileTime<WIDTH>,
        CompileTime<HEIGHT>,
        CharacterPixel,
    >
{
    fn width_characters(&self) -> usize {
        WIDTH
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> DynamicCharacterHeight
    for CharacterDisplay<
        CompileTime<WIDTH>,
        CompileTime<HEIGHT>,
        CharacterPixel,
    >
{
    fn height_characters(&self) -> usize {
        HEIGHT
    }
}

impl<const WIDTH: usize, const HEIGHT: usize>
    From<
        CharacterDisplay<
            CompileTime<WIDTH>,
            CompileTime<HEIGHT>,
            CharacterPixel,
        >,
    > for StringData
{
    fn from(
        val: CharacterDisplay<
            CompileTime<WIDTH>,
            CompileTime<HEIGHT>,
            CharacterPixel,
        >,
    ) -> Self {
        let mut result = Vec::new();
        let mut row = Vec::new();
        let mut width = 0;

        let mut iter = val.data.iter();
        while let Some(cell) = iter.next() {
            if width >=
                <CharacterDisplay<
                    CompileTime<WIDTH>,
                    CompileTime<HEIGHT>,
                    CharacterPixel,
                > as StaticCharacterWidth>::WIDTH_CHARACTERS
            {
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

        Self { data: result }
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
    ) -> Result<CharacterPixelData, DrawingError>
    where
        [(); S::WIDTH * S::HEIGHT]:,
    {
        let x: Option<usize> = NumCast::from(x);
        let y: Option<usize> = NumCast::from(y);
        if let Some(x) = x &&
            let Some(y) = y
        {
            if x >= DynamicConsoleDisplay::width(self) ||
                y >= DynamicConsoleDisplay::height(self)
            {
                Err(DisplayError::CoordinatesOutOfBounds(
                    x,
                    DynamicConsoleDisplay::width(self),
                    y,
                    DynamicConsoleDisplay::height(self),
                ))?;
            }

            let block_x: usize = x / S::WIDTH;
            let block_y: usize = y / S::HEIGHT;
            let offset_x: usize = x % S::WIDTH;
            let offset_y: usize = y % S::HEIGHT;

            let pixel = &DynamicConsoleDisplay::data(self)[block_x +
                block_y * DynamicWidget::width_characters(self)];

            Ok(pixel
                .subpixel(offset_x, offset_y)
                .expect(OFFSET_SHOULD_BE_0_OR_1))
        }
        else {
            Err(DisplayError::CoordinatesToUsizeConversionFailed)?
        }
    }

    default fn set_pixel(
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
            if x >= DynamicConsoleDisplay::width(self) ||
                y >= DynamicConsoleDisplay::height(self)
            {
                Err(DisplayError::CoordinatesOutOfBounds(
                    x,
                    DynamicConsoleDisplay::width(self),
                    y,
                    DynamicConsoleDisplay::height(self),
                ))?;
            }

            let block_x: usize = x / S::WIDTH;
            let block_y: usize = y / S::HEIGHT;
            let offset_x: usize = x % S::WIDTH;
            let offset_y: usize = y % S::HEIGHT;

            let width_characters = DynamicWidget::width_characters(self);
            let pixel = &mut DynamicConsoleDisplay::data_mut(self)
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

impl<W: Dimension, H: Dimension, S: Pixel<U = CharacterPixelData>>
    GetPixel<S> for CharacterDisplay<W, H, CharacterPixel>
{
    default type A = usize;

    default fn pixel(
        &self,
        x: Self::A,
        y: Self::A,
    ) -> Result<CharacterPixelData, DrawingError>
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
                Err(DisplayError::CoordinatesOutOfBounds(
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

impl<W: Dimension, H: Dimension, S: Pixel<U = CharacterPixelData>>
    SetPixel<S> for CharacterDisplay<W, H, CharacterPixel>
{
    default type A = usize;

    default fn set_pixel(
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
                Err(DisplayError::CoordinatesOutOfBounds(
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

impl<W: Dimension, H: Dimension, S: Pixel<U = CharacterPixelData>>
    SetPixelStatic<S> for CharacterDisplay<W, H, CharacterPixel>
where
    Self: GetDataMut<S> + Height + Width + StaticCharacterWidth,
{
}

impl<W: Dimension, H: Dimension, S: Pixel<U = CharacterPixelData>>
    GetPixelStatic<S> for CharacterDisplay<W, H, CharacterPixel>
where
    Self: GetData<S> + Height + Width + StaticCharacterWidth,
{
}

impl<W: Dimension, H: Dimension, S: Pixel<U = CharacterPixelData>>
    SetPixels<S> for CharacterDisplay<W, H, CharacterPixel>
{
}

impl<W: Dimension, H: Dimension, S: Pixel<U = CharacterPixelData>>
    GetPixels<S> for CharacterDisplay<W, H, CharacterPixel>
{
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
    ) -> Result<CharacterPixelData, DrawingError>
    where
        [(); CharacterPixel::WIDTH * CharacterPixel::HEIGHT]:,
    {
        let x: Option<usize> = NumCast::from(x);
        let y: Option<usize> = NumCast::from(y);
        if let Some(x) = x &&
            let Some(y) = y
        {
            if x >= DynamicConsoleDisplay::width(self) ||
                y >= DynamicConsoleDisplay::height(self)
            {
                return Err(DisplayError::CoordinatesOutOfBounds(
                    x,
                    DynamicConsoleDisplay::width(self),
                    y,
                    DynamicConsoleDisplay::height(self),
                ))?;
            }

            let block_x: usize = x / CharacterPixel::WIDTH;
            let block_y: usize = y / CharacterPixel::HEIGHT;

            let pixel = &DynamicConsoleDisplay::data(self)[block_x +
                block_y * DynamicWidget::width_characters(self)];

            Ok(pixel.subpixel_static::<0, 0>())
        }
        else {
            Err(DisplayError::CoordinatesToUsizeConversionFailed)?
        }
    }

    fn set_pixel(
        &mut self,
        x: Self::A,
        y: Self::A,
        value: CharacterPixelData,
    ) -> Result<(), DrawingError>
    where
        [(); CharacterPixel::WIDTH * CharacterPixel::HEIGHT]:,
    {
        let x: Option<usize> = NumCast::from(x);
        let y: Option<usize> = NumCast::from(y);
        if let Some(x) = x &&
            let Some(y) = y
        {
            if x >= DynamicConsoleDisplay::width(self) ||
                y >= DynamicConsoleDisplay::height(self)
            {
                return Err(DisplayError::CoordinatesOutOfBounds(
                    x,
                    DynamicConsoleDisplay::width(self),
                    y,
                    DynamicConsoleDisplay::height(self),
                ))?;
            }

            let block_x: usize = x / CharacterPixel::WIDTH;
            let block_y: usize = y / CharacterPixel::HEIGHT;

            let width_characters =
                DynamicCharacterWidth::width_characters(self);
            let pixel = &mut DynamicConsoleDisplay::data_mut(self)
                [block_x + block_y * width_characters];
            pixel.set_subpixel_static::<0, 0>(value);

            Ok(())
        }
        else {
            Err(DisplayError::CoordinatesToUsizeConversionFailed)?
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> GetPixel<CharacterPixel>
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
    ) -> Result<CharacterPixelData, DrawingError>
    where
        [(); CharacterPixel::WIDTH * CharacterPixel::HEIGHT]:,
    {
        let x: Option<usize> = NumCast::from(x);
        let y: Option<usize> = NumCast::from(y);
        if let Some(x) = x &&
            let Some(y) = y
        {
            if x >= DynamicWidth::width(self) ||
                y >= DynamicHeight::height(self)
            {
                Err(DisplayError::CoordinatesOutOfBounds(
                    x,
                    DynamicWidth::width(self),
                    y,
                    DynamicHeight::height(self),
                ))?;
            }

            let block_x: usize = x / CharacterPixel::WIDTH;
            let block_y: usize = y / CharacterPixel::HEIGHT;

            let pixel = &GetData::data(self)[block_x +
                block_y * DynamicCharacterWidth::width_characters(self)];

            Ok(pixel.subpixel_static::<0, 0>())
        }
        else {
            Err(DisplayError::CoordinatesToUsizeConversionFailed)?
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> SetPixel<CharacterPixel>
    for CharacterDisplay<
        CompileTime<WIDTH>,
        CompileTime<HEIGHT>,
        CharacterPixel,
    >
{
    type A = usize;

    fn set_pixel(
        &mut self,
        x: Self::A,
        y: Self::A,
        value: CharacterPixelData,
    ) -> Result<(), DrawingError>
    where
        [(); CharacterPixel::WIDTH * CharacterPixel::HEIGHT]:,
    {
        let x: Option<usize> = NumCast::from(x);
        let y: Option<usize> = NumCast::from(y);
        if let Some(x) = x &&
            let Some(y) = y
        {
            if x >= DynamicWidth::width(self) ||
                y >= DynamicHeight::height(self)
            {
                Err(DisplayError::CoordinatesOutOfBounds(
                    x,
                    DynamicWidth::width(self),
                    y,
                    DynamicHeight::height(self),
                ))?;
            }

            let block_x: usize = x / CharacterPixel::WIDTH;
            let block_y: usize = y / CharacterPixel::HEIGHT;

            let width_characters =
                DynamicCharacterWidth::width_characters(self);
            let pixel = &mut GetDataMut::data_mut(self)
                [block_x + block_y * width_characters];
            pixel.set_subpixel_static::<0, 0>(value);

            Ok(())
        }
        else {
            Err(DisplayError::CoordinatesToUsizeConversionFailed)?
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

impl<const WIDTH: usize, const HEIGHT: usize> Width
    for CharacterDisplay<
        CompileTime<WIDTH>,
        CompileTime<HEIGHT>,
        CharacterPixel,
    >
{
    const WIDTH: usize = WIDTH;
}

impl<const WIDTH: usize, const HEIGHT: usize> Height
    for CharacterDisplay<
        CompileTime<WIDTH>,
        CompileTime<HEIGHT>,
        CharacterPixel,
    >
{
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

impl<const WIDTH: usize, const HEIGHT: usize> const StaticCharacterWidth
    for CharacterDisplay<
        CompileTime<WIDTH>,
        CompileTime<HEIGHT>,
        CharacterPixel,
    >
{
    const WIDTH_CHARACTERS: usize = WIDTH;
}

impl<const WIDTH: usize, const HEIGHT: usize> const StaticCharacterHeight
    for CharacterDisplay<
        CompileTime<WIDTH>,
        CompileTime<HEIGHT>,
        CharacterPixel,
    >
{
    const HEIGHT_CHARACTERS: usize = HEIGHT;
}

#[cfg(test)]
mod tests {
    use crate::color::TerminalColor;

    use super::*;

    #[test]
    fn compile_time_build_from_data_success() {
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
    fn compile_time_build_from_data_failure_dimensions() {
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
    fn compile_time_build_from_data_failure_fit() {
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

    #[test]
    fn run_time_build_from_data_success() {
        let character_display =
            DynamicCharacterDisplay::<CharacterPixel>::build_from_data(
                1,
                1,
                &[CharacterPixel::build(
                    ' ',
                    TerminalColor::Default,
                    TerminalColor::Default,
                )
                .unwrap()],
            );
        assert!(character_display.is_ok());
    }

    #[test]
    fn run_time_build_from_data_failure_dimensions() {
        let character_display =
            DynamicCharacterDisplay::<CharacterPixel>::build_from_data(
                8,
                10,
                &vec![
                    CharacterPixel::build(
                        ' ',
                        TerminalColor::Default,
                        TerminalColor::Default,
                    )
                    .unwrap();
                    8 * 10 - 1
                ],
            );
        assert!(character_display.is_err());
    }

    #[test]
    fn run_time_build_from_data_failure_fit() {
        let character_display =
            DynamicCharacterDisplay::<CharacterPixel>::build_from_data(
                9,
                10,
                &vec![
                    CharacterPixel::build(
                        'あ',
                        TerminalColor::Default,
                        TerminalColor::Default,
                    )
                    .unwrap();
                    9 * 10 / 2
                ],
            );
        assert!(character_display.is_err());
    }
}
