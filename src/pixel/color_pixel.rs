use std::fmt::Display;

use crate::{
    impl_getters,
    impl_getters_mut,
    impl_new,
    pixel::{
        Pixel,
        monochrome_pixel::{
            HexPixel,
            MultiPixel,
            OctPixel,
            QuadPixel,
        },
    },
};

#[derive(Clone, Copy, Default)]
pub enum Color {
    #[default]
    Default,
    Color(RGBColor),
}

impl Color {
    #[must_use]
    pub fn color(
        text: &str,
        foreground_color: &Self,
        background_color: &Self,
    ) -> String {
        let mut output = String::new();

        if let Self::Color(foreground_color) = foreground_color {
            output = format!(
                "{}\x1b[38;2;{};{};{}m",
                output,
                foreground_color.r,
                foreground_color.g,
                foreground_color.b, // foreground color
            );
        }
        if let Self::Color(background_color) = background_color {
            output = format!(
                "{}\x1b[48;2;{};{};{}m",
                output,
                background_color.r,
                background_color.g,
                background_color.b, // background color
            );
        }
        format!("{output}{text}\x1b[0m")
    }
}

impl From<RGBColor> for Color {
    fn from(value: RGBColor) -> Self {
        Self::Color(value)
    }
}

#[derive(Clone, Copy)]
pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGBColor {
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
    };
    pub const RED: Self = Self { r: 255, g: 0, b: 0 };
    pub const GREEN: Self = Self { r: 0, g: 255, b: 0 };
    pub const BLUE: Self = Self { r: 0, g: 0, b: 255 };
    pub const YELLOW: Self = Self {
        r: 255,
        g: 255,
        b: 0,
    };
    pub const CYAN: Self = Self {
        r: 0,
        g: 255,
        b: 255,
    };
    pub const MAGENTA: Self = Self {
        r: 255,
        g: 0,
        b: 255,
    };

    #[rustfmt::skip]
    #[allow(clippy::cast_possible_truncation)]
    fn distance(color1: Self, color2: Self) -> f32 {
        (
            f64::from((i32::from(color1.r) - i32::from(color2.r)).pow(2)) +
            f64::from((i32::from(color1.g) - i32::from(color2.g)).pow(2)) +
            f64::from((i32::from(color1.b) - i32::from(color2.b)).pow(2))
        )
        .sqrt() as f32
    }

    fn mix(colors: &[Self]) -> Result<Self, &str> {
        if colors.is_empty() {
            return Err("colors must contain at least one element");
        }
        let mut sum = (0, 0, 0);
        for color in colors {
            sum.0 += u32::from(color.r);
            sum.1 += u32::from(color.g);
            sum.2 += u32::from(color.b);
        }
        let Ok(colors_len) = u32::try_from(colors.len())
        else {
            return Err("colors contains too many elements");
        };

        Ok(Self {
            r: (sum.0 / colors_len).clamp(0, 255) as u8,
            g: (sum.1 / colors_len).clamp(0, 255) as u8,
            b: (sum.2 / colors_len).clamp(0, 255) as u8,
        })
    }

    fn group<const N: usize>(colors: &[Self; N]) -> [bool; N] {
        let mut max = 0f32;
        let mut col1 = 0;
        let mut col2 = 0;
        for i in 0..N {
            for j in (i + 1)..N {
                let dist = Self::distance(colors[i], colors[j]);
                if dist > max {
                    max = dist;
                    col1 = i;
                    col2 = j;
                }
            }
        }
        let mut groups = [false; N];
        for i in 0..N {
            if Self::distance(colors[col1], colors[i]) >
                Self::distance(colors[col2], colors[i])
            {
                groups[i] = true;
            }
        }
        groups
    }

    #[must_use]
    pub fn color(
        text: &str,
        foreground_color: &Self,
        background_color: &Self,
    ) -> String {
        format!(
            "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m{}\x1b[0m",
            foreground_color.r,
            foreground_color.g,
            foreground_color.b, // foreground color
            background_color.r,
            background_color.g,
            background_color.b, // background color
            text
        )
    }
}

/// Represents a singular pixel implementing the [`MultiPixel`] trait.
#[derive(Clone, Copy)]
pub struct ColorSinglePixel {
    pixels: [RGBColor; 1],
}

impl Pixel for ColorSinglePixel {
    type U = RGBColor;

    const WIDTH: usize = 1;

    const HEIGHT: usize = 1;

    impl_getters!(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]);

    impl_getters_mut!(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]);
}

impl MultiPixel for ColorSinglePixel {
    impl_new!(Self, pixels: [Self::U; 1]);
}

impl Display for ColorSinglePixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            RGBColor::color("█", &self.pixels[0], &self.pixels[0])
        )
    }
}

#[derive(Clone, Copy)]
pub struct ColorDualPixel {
    pixels: [RGBColor; 2],
}

impl Pixel for ColorDualPixel {
    type U = RGBColor;

    const WIDTH: usize = 1;

    const HEIGHT: usize = 2;

    impl_getters!(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]);

    impl_getters_mut!(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]);
}

impl MultiPixel for ColorDualPixel {
    impl_new!(Self, pixels: [Self::U; 2]);
}

impl Display for ColorDualPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            RGBColor::color("▀", &self.pixels[0], &self.pixels[1])
        )
    }
}

#[derive(Clone, Copy)]
pub struct ColorQuadPixel {
    pixels: [RGBColor; 4],
}

impl Pixel for ColorQuadPixel {
    type U = RGBColor;

    const WIDTH: usize = 2;

    const HEIGHT: usize = 2;

    impl_getters!(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]);

    impl_getters_mut!(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]);
}

impl MultiPixel for ColorQuadPixel {
    impl_new!(Self, pixels: [Self::U; 4]);
}

impl Display for ColorQuadPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let colors = self.pixels;
        let grouping = RGBColor::group(&colors);
        let symb =
            QuadPixel::new(grouping).to_string().chars().next().unwrap();

        let mut col1 = vec![];
        let mut col2 = vec![];
        for i in 0..grouping.len() {
            if grouping[i] {
                col1.push(colors[i]);
            }
            else {
                col2.push(colors[i]);
            }
        }
        let col1 =
            RGBColor::mix(&col1).unwrap_or(RGBColor { r: 0, g: 0, b: 0 });
        let col2 =
            RGBColor::mix(&col2).unwrap_or(RGBColor { r: 0, g: 0, b: 0 });

        write!(
            f,
            "{}",
            RGBColor::color(symb.to_string().as_str(), &col1, &col2)
        )
    }
}

#[derive(Clone, Copy)]
pub struct ColorHexPixel {
    pixels: [RGBColor; 6],
}

impl Pixel for ColorHexPixel {
    type U = RGBColor;

    const WIDTH: usize = 2;

    const HEIGHT: usize = 3;

    impl_getters!(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]);

    impl_getters_mut!(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]);
}

impl MultiPixel for ColorHexPixel {
    impl_new!(Self, pixels: [Self::U; 6]);
}

impl Display for ColorHexPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let colors = self.pixels;
        let grouping = RGBColor::group(&colors);
        let symb =
            HexPixel::new(grouping).to_string().chars().next().unwrap();

        let mut col1 = vec![];
        let mut col2 = vec![];
        for i in 0..grouping.len() {
            if grouping[i] {
                col1.push(colors[i]);
            }
            else {
                col2.push(colors[i]);
            }
        }
        let col1 =
            RGBColor::mix(&col1).unwrap_or(RGBColor { r: 0, g: 0, b: 0 });
        let col2 =
            RGBColor::mix(&col2).unwrap_or(RGBColor { r: 0, g: 0, b: 0 });

        write!(
            f,
            "{}",
            RGBColor::color(symb.to_string().as_str(), &col1, &col2)
        )
    }
}

#[derive(Clone, Copy)]
pub struct ColorOctPixel {
    pixels: [RGBColor; 8],
}

impl Pixel for ColorOctPixel {
    type U = RGBColor;

    const WIDTH: usize = 2;

    const HEIGHT: usize = 4;

    impl_getters!(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]);

    impl_getters_mut!(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]);
}

impl MultiPixel for ColorOctPixel {
    impl_new!(Self, pixels: [Self::U; 8]);
}

impl Display for ColorOctPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let colors = self.pixels;
        let grouping = RGBColor::group(&colors);
        let symb =
            OctPixel::new(grouping).to_string().chars().next().unwrap();

        let mut col1 = vec![];
        let mut col2 = vec![];
        for i in 0..grouping.len() {
            if grouping[i] {
                col1.push(colors[i]);
            }
            else {
                col2.push(colors[i]);
            }
        }
        let col1 =
            RGBColor::mix(&col1).unwrap_or(RGBColor { r: 0, g: 0, b: 0 });
        let col2 =
            RGBColor::mix(&col2).unwrap_or(RGBColor { r: 0, g: 0, b: 0 });

        write!(
            f,
            "{}",
            RGBColor::color(symb.to_string().as_str(), &col1, &col2)
        )
    }
}
