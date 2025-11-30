use crate::{
    color::{
        Color,
        TerminalColor,
    },
    impl_from_color_pixel_for_datacell,
    impl_getters,
    impl_getters_mut,
    impl_new,
    pixel::{
        Pixel,
        monochrome_pixel::{
            HexPixel,
            OctPixel,
            QuadPixel,
        },
    },
    widget::DataCell,
};

/// Represents a singular pixel implementing the [`Pixel`] trait.
#[derive(Clone, Copy)]
pub struct ColorSinglePixel {
    pixels: [TerminalColor; 1],
}

impl Pixel for ColorSinglePixel {
    type U = TerminalColor;

    const WIDTH: usize = 1;

    const HEIGHT: usize = 1;

    impl_getters!(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]);

    impl_getters_mut!(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]);

    impl_new!(Self, pixels: [Self::U; 1]);
}

impl const From<ColorSinglePixel> for DataCell {
    fn from(val: ColorSinglePixel) -> Self {
        Self {
            character: '█',
            foreground: val.pixels[0],
            background: val.pixels[0],
        }
    }
}

#[derive(Clone, Copy)]
pub struct ColorDualPixel {
    pixels: [TerminalColor; 2],
}

impl Pixel for ColorDualPixel {
    type U = TerminalColor;

    const WIDTH: usize = 1;

    const HEIGHT: usize = 2;

    impl_getters!(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]);

    impl_getters_mut!(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]);

    impl_new!(Self, pixels: [Self::U; 2]);
}

impl const From<ColorDualPixel> for DataCell {
    fn from(val: ColorDualPixel) -> Self {
        Self {
            character: '▀',
            foreground: val.pixels[0],
            background: val.pixels[1],
        }
    }
}

#[derive(Clone, Copy)]
pub struct ColorQuadPixel {
    pixels: [TerminalColor; 4],
}

impl Pixel for ColorQuadPixel {
    type U = TerminalColor;

    const WIDTH: usize = 2;

    const HEIGHT: usize = 2;

    impl_getters!(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]);

    impl_getters_mut!(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]);

    impl_new!(Self, pixels: [Self::U; 4]);
}

impl_from_color_pixel_for_datacell!(ColorQuadPixel, QuadPixel);

#[derive(Clone, Copy)]
pub struct ColorHexPixel {
    pixels: [TerminalColor; 6],
}

impl Pixel for ColorHexPixel {
    type U = TerminalColor;

    const WIDTH: usize = 2;

    const HEIGHT: usize = 3;

    impl_getters!(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]);

    impl_getters_mut!(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]);

    impl_new!(Self, pixels: [Self::U; 6]);
}

impl_from_color_pixel_for_datacell!(ColorHexPixel, HexPixel);

#[derive(Clone, Copy)]
pub struct ColorOctPixel {
    pixels: [TerminalColor; 8],
}

impl Pixel for ColorOctPixel {
    type U = TerminalColor;

    const WIDTH: usize = 2;

    const HEIGHT: usize = 4;

    impl_getters!(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]);

    impl_getters_mut!(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]);

    impl_new!(Self, pixels: [Self::U; 8]);
}

impl_from_color_pixel_for_datacell!(ColorOctPixel, OctPixel);
