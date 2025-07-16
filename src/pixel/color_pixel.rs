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
    widget::DataCell,
};
use std::fmt::Display;

pub trait Color
where
    Self: Sized,
{
    #[must_use]
    fn distance(color1: &Self, color2: &Self) -> f32;

    #[must_use]
    fn mix(colors: &[Self]) -> Self;

    #[must_use]
    fn group<const N: usize>(colors: &[Self; N]) -> [bool; N] {
        let mut max = 0f32;
        let mut col1 = 0;
        let mut col2 = 0;
        for i in 0..N {
            for j in (i + 1)..N {
                let dist = Self::distance(&colors[i], &colors[j]);
                if dist > max {
                    max = dist;
                    col1 = i;
                    col2 = j;
                }
            }
        }
        let mut groups = [false; N];
        for i in 0..N {
            if Self::distance(&colors[col1], &colors[i]) >
                Self::distance(&colors[col2], &colors[i])
            {
                groups[i] = true;
            }
        }
        groups
    }
}

/// Defines a color, usually used for foreground and background.
///
/// `Default` - Uses the default color provided by the terminal for foreground or background respectively.\
/// `ARGBColor` - Displays an actual opaque color made of RGB components and an alpha/opacity channel.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum TerminalColor {
    #[default]
    Default,
    ARGBColor(ARGBColor),
}

impl TerminalColor {
    #[must_use]
    pub fn color<'a>(
        text: &str,
        foreground_color: &'a Self,
        background_color: &'a Self,
    ) -> String {
        let mut codes = Vec::new();

        if let Self::ARGBColor(top_color) = foreground_color {
            if let Self::ARGBColor(bottom_color) = *background_color {
                let top_color = ARGBColor::blend(top_color, &bottom_color);
                codes.push(format!(
                    "\x1b[38;2;{};{};{}m",
                    top_color.color.r,
                    top_color.color.g,
                    top_color.color.b, // foreground color
                ));
            }
            codes.push(format!(
                "\x1b[38;2;{};{};{}m",
                top_color.color.r,
                top_color.color.g,
                top_color.color.b, // foreground color
            ));
        }
        if let Self::ARGBColor(background_color) = background_color {
            codes.push(format!(
                "\x1b[48;2;{};{};{}m",
                background_color.color.r,
                background_color.color.g,
                background_color.color.b, // background color
            ));
        }
        if codes.is_empty() {
            return text.to_owned();
        }
        format!("{}{text}{}", codes.join(""), "\x1b[0m")
    }

    #[must_use]
    pub fn blend(color_top: &Self, color_bottom: &Self) -> Self {
        if let Self::ARGBColor(color_top) = color_top &&
            let Self::ARGBColor(color_bottom) = color_bottom
        {
            return Self::ARGBColor(ARGBColor::blend(
                color_top,
                color_bottom,
            ));
        }
        if let Self::ARGBColor(_) = color_top {
            *color_top
        }
        else {
            *color_bottom
        }
    }
}

impl Color for TerminalColor {
    fn distance(color1: &Self, color2: &Self) -> f32 {
        if let Self::ARGBColor(col1) = color1 &&
            let Self::ARGBColor(col2) = color2
        {
            return ARGBColor::distance(col1, col2);
        }
        0.
    }

    fn mix(colors: &[Self]) -> Self {
        let mut argb_colors = Vec::with_capacity(colors.len());
        for color in colors {
            argb_colors.push(match color {
                Self::Default => continue,
                Self::ARGBColor(argbcolor) => *argbcolor,
            });
        }
        if argb_colors.is_empty() {
            return Self::Default;
        }

        Self::ARGBColor(ARGBColor::mix(argb_colors.as_slice()))
    }
}

impl From<RGBColor> for TerminalColor {
    fn from(value: RGBColor) -> Self {
        Self::ARGBColor(value.into())
    }
}

impl From<ARGBColor> for TerminalColor {
    fn from(value: ARGBColor) -> Self {
        Self::ARGBColor(value)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
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
}

impl RGBColor {
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

impl Color for RGBColor {
    #[rustfmt::skip]
    #[allow(clippy::suboptimal_flops)]
    fn distance(color1: &Self, color2: &Self) -> f32 {
        (
            ((f32::from(color1.r) - f32::from(color2.r)) / 255.).powi(2) +
            ((f32::from(color1.g) - f32::from(color2.g)) / 255.).powi(2) +
            ((f32::from(color1.b) - f32::from(color2.b)) / 255.).powi(2)
        )
        .sqrt()
    }

    fn mix(colors: &[Self]) -> Self {
        let mut sum = (0, 0, 0);
        for color in colors {
            sum.0 += u32::from(color.r);
            sum.1 += u32::from(color.g);
            sum.2 += u32::from(color.b);
        }
        let Ok(colors_len) = u32::try_from(colors.len())
        else {
            panic!("colors contains too many elements");
        };

        Self {
            r: (sum.0 / colors_len).clamp(0, 255) as u8,
            g: (sum.1 / colors_len).clamp(0, 255) as u8,
            b: (sum.2 / colors_len).clamp(0, 255) as u8,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ARGBColor {
    pub opacity: u8,
    pub color: RGBColor,
}

impl Color for ARGBColor {
    fn distance(color1: &Self, color2: &Self) -> f32 {
        // Equivalent to d = sqrt(r²+g²+b²+a²)
        RGBColor::distance(&color1.color, &color2.color).hypot(
            (f32::from(color1.opacity) - f32::from(color2.opacity)) / 255.,
        )
    }

    fn mix(colors: &[Self]) -> Self {
        let mut sum_opacity = 0;
        for color in colors {
            sum_opacity += u32::from(color.opacity);
        }
        let Ok(colors_len) = u32::try_from(colors.len())
        else {
            panic!("colors contains too many elements");
        };

        Self {
            opacity: (sum_opacity / colors_len).clamp(0, 255) as u8,
            color: RGBColor::mix(
                &colors.iter().map(|x| x.color).collect::<Vec<_>>(),
            ),
        }
    }
}

impl ARGBColor {
    #[must_use]
    pub fn color(
        text: &str,
        foreground_color: &Self,
        background_color: &Self,
    ) -> String {
        format!(
            "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m{}\x1b[0m",
            foreground_color.color.r,
            foreground_color.color.g,
            foreground_color.color.b, // foreground color
            background_color.color.r,
            background_color.color.g,
            background_color.color.b, // background color
            text
        )
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn blend(color_top: &Self, color_bottom: &Self) -> Self {
        let opacity_top = f32::from(color_top.opacity) / 255.;
        let opacity_bottom = f32::from(color_bottom.opacity) / 255.;
        let opacity_res =
            opacity_bottom.mul_add(1. - opacity_top, opacity_top);
        let red = f32::from(color_top.color.r).mul_add(
            opacity_top,
            f32::from(color_bottom.color.r) *
                (1. - opacity_top) *
                opacity_bottom,
        ) / opacity_res;
        let green = f32::from(color_top.color.g).mul_add(
            opacity_top,
            f32::from(color_bottom.color.g) *
                (1. - opacity_top) *
                opacity_bottom,
        ) / opacity_res;
        let blue = f32::from(color_top.color.b).mul_add(
            opacity_top,
            f32::from(color_bottom.color.b) *
                (1. - opacity_top) *
                opacity_bottom,
        ) / opacity_res;
        Self {
            opacity: (opacity_res * 255.).clamp(0., 255.) as u8,
            color: RGBColor {
                r: red.clamp(0., 255.) as u8,
                g: green.clamp(0., 255.) as u8,
                b: blue.clamp(0., 255.) as u8,
            },
        }
    }
}

impl From<RGBColor> for ARGBColor {
    fn from(value: RGBColor) -> Self {
        Self {
            opacity: u8::MAX,
            color: value,
        }
    }
}

/// Represents a singular pixel implementing the [`MultiPixel`] trait.
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
}

impl MultiPixel for ColorSinglePixel {
    impl_new!(Self, pixels: [Self::U; 1]);
}

impl Display for ColorSinglePixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            TerminalColor::color("█", &self.pixels[0], &self.pixels[0])
        )
    }
}

impl From<ColorSinglePixel> for DataCell {
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
}

impl MultiPixel for ColorDualPixel {
    impl_new!(Self, pixels: [Self::U; 2]);
}

impl Display for ColorDualPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            TerminalColor::color("▀", &self.pixels[0], &self.pixels[1])
        )
    }
}

impl From<ColorDualPixel> for DataCell {
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
}

impl MultiPixel for ColorQuadPixel {
    impl_new!(Self, pixels: [Self::U; 4]);
}

impl Display for ColorQuadPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let colors = self.pixels;
        let grouping = TerminalColor::group(&colors);
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
        let col1 = TerminalColor::mix(&col1);
        let col2 = TerminalColor::mix(&col2);

        write!(
            f,
            "{}",
            TerminalColor::color(symb.to_string().as_str(), &col1, &col2)
        )
    }
}

impl From<ColorQuadPixel> for DataCell {
    fn from(val: ColorQuadPixel) -> Self {
        let colors = val.pixels;
        let grouping = TerminalColor::group(&colors);
        let symb = QuadPixel::new(grouping).character();

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
        let col1 = TerminalColor::mix(&col1);
        let col2 = TerminalColor::mix(&col2);

        Self {
            character: symb,
            foreground: col1,
            background: col2,
        }
    }
}

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
}

impl MultiPixel for ColorHexPixel {
    impl_new!(Self, pixels: [Self::U; 6]);
}

impl Display for ColorHexPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let colors = self.pixels;
        let grouping = TerminalColor::group(&colors);
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
        let col1 = TerminalColor::mix(&col1);
        let col2 = TerminalColor::mix(&col2);

        write!(
            f,
            "{}",
            TerminalColor::color(symb.to_string().as_str(), &col1, &col2)
        )
    }
}

impl From<ColorHexPixel> for DataCell {
    fn from(val: ColorHexPixel) -> Self {
        let colors = val.pixels;
        let grouping = TerminalColor::group(&colors);
        let symb = HexPixel::new(grouping).character();

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
        let col1 = TerminalColor::mix(&col1);
        let col2 = TerminalColor::mix(&col2);

        Self {
            character: symb,
            foreground: col1,
            background: col2,
        }
    }
}

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
}

impl MultiPixel for ColorOctPixel {
    impl_new!(Self, pixels: [Self::U; 8]);
}

impl Display for ColorOctPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let colors = self.pixels;
        let grouping = TerminalColor::group(&colors);
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
        let col1 = TerminalColor::mix(&col1);
        let col2 = TerminalColor::mix(&col2);

        write!(
            f,
            "{}",
            TerminalColor::color(symb.to_string().as_str(), &col1, &col2)
        )
    }
}

impl From<ColorOctPixel> for DataCell {
    fn from(val: ColorOctPixel) -> Self {
        let colors = val.pixels;
        let grouping = TerminalColor::group(&colors);
        let symb = OctPixel::new(grouping).character();

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
        let col1 = TerminalColor::mix(&col1);
        let col2 = TerminalColor::mix(&col2);

        Self {
            character: symb,
            foreground: col1,
            background: col2,
        }
    }
}
