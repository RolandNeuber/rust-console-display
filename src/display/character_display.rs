use std::fmt::Display;

use crate::{
    console_display::ConsoleDisplay,
    impl_display_for_dynamic_widget,
    pixel::character_pixel::CharacterPixel,
    widget::{
        DataCell,
        DynamicWidget,
        StaticWidget,
    },
};

#[derive(Clone)]
pub struct CharacterDisplay<
    CharacterPixel,
    const WIDTH: usize,
    const HEIGHT: usize,
> {
    data: Box<[CharacterPixel]>,
}

impl<const WIDTH: usize, const HEIGHT: usize>
    ConsoleDisplay<CharacterPixel>
    for CharacterDisplay<CharacterPixel, WIDTH, HEIGHT>
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

impl<const WIDTH: usize, const HEIGHT: usize> DynamicWidget
    for CharacterDisplay<CharacterPixel, WIDTH, HEIGHT>
{
    fn width_characters(&self) -> usize {
        WIDTH
    }

    fn height_characters(&self) -> usize {
        HEIGHT
    }

    fn string_data(&self) -> Vec<Vec<DataCell>> {
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

        result
    }
}

impl<const WIDTH: usize, const HEIGHT: usize>
    CharacterDisplay<CharacterPixel, WIDTH, HEIGHT>
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

        Self::build_from_data(data).expect(
            "Invariant violated, data does not mach specified dimensions.",
        )
    }

    /// Builds a display struct with the specified dimensions from the given data.
    ///
    /// # Errors
    /// Returns an error when the data length does not match the dimensions of the display.
    /// This also applies if double-width characters, like あ, are used and exceed the dimensions of the display.
    pub fn build_from_data(
        data: Vec<CharacterPixel>,
    ) -> Result<Self, String> {
        let mut new_data = Vec::with_capacity(data.capacity());
        let mut row_length = 0;
        for i in data {
            new_data.push(i);
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
            data: new_data.into_boxed_slice(),
        })
    }

    #[must_use]
    pub const fn width(&self) -> usize {
        WIDTH
    }

    #[must_use]
    pub const fn height(&self) -> usize {
        HEIGHT
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> StaticWidget
    for CharacterDisplay<CharacterPixel, WIDTH, HEIGHT>
{
    const WIDTH_CHARACTERS: usize = WIDTH;

    const HEIGHT_CHARACTERS: usize = HEIGHT;
}

impl<const WIDTH: usize, const HEIGHT: usize> Display
    for CharacterDisplay<CharacterPixel, WIDTH, HEIGHT>
{
    impl_display_for_dynamic_widget!();
}

#[cfg(test)]
mod tests {
    use crate::pixel::color_pixel::TerminalColor;

    use super::*;

    #[test]
    fn build_from_data_success() {
        let character_display =
            CharacterDisplay::<CharacterPixel, 1, 1>::build_from_data(
                vec![
                    CharacterPixel::build(
                        ' ',
                        TerminalColor::Default,
                        TerminalColor::Default,
                    )
                    .unwrap(),
                ],
            );
        assert!(character_display.is_ok());
    }

    #[test]
    fn build_from_data_failure_dimensions() {
        let character_display =
            CharacterDisplay::<CharacterPixel, 8, 10>::build_from_data(
                vec![
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
    fn build_from_data_failure_fit() {
        let character_display =
            CharacterDisplay::<CharacterPixel, 9, 10>::build_from_data(
                vec![
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
