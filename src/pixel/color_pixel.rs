use crate::{impl_getters, pixel::monochrome_pixel::{HexPixel, MultiPixel, OctPixel, QuadPixel}};

#[derive(Clone, Copy)]
pub enum Color {
    Default,
    Color(RGBColor)
}

impl Color {
    pub fn color(text: &str, foreground_color: &Color, background_color: &Color) -> String {
        let mut output = String::new();

        if let Color::Color(foreground_color) = foreground_color {
            output = format!(
                "{}\x1b[38;2;{};{};{}m",
                output, foreground_color.r, foreground_color.g, foreground_color.b, // foreground color
            );
        }
        if let Color::Color(background_color) = background_color {
            output = format!(
                "{}\x1b[48;2;{};{};{}m",
                output, background_color.r, background_color.g, background_color.b, // background color
            );
        }
        format!(
            "{}{}\x1b[0m",
            output, text
        )
    }
}

pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl RGBColor {
    fn distance(color1: RGBColor, color2: RGBColor) -> f32 {
        (
            (color1.r as i32 - color2.r as i32).pow(2) as f32 +
            (color1.g as i32 - color2.g as i32).pow(2) as f32 +
            (color1.b as i32 - color2.b as i32).pow(2) as f32
        ).sqrt()
    }

    fn mix(colors: &[RGBColor]) -> Result<RGBColor, &str> {
        if colors.len() == 0 {
            return Err("colors must contain at least one element");
        }
        let mut sum = (0, 0, 0);
        for color in colors {
            sum.0 += color.r as u32;
            sum.1 += color.g as u32;
            sum.2 += color.b as u32;
        }
        Ok(RGBColor {
            r: (sum.0 / colors.len() as u32) as u8,
            g: (sum.1 / colors.len() as u32) as u8,
            b: (sum.2 / colors.len() as u32) as u8,
        })
    }

    fn group<const N: usize>(colors: &[RGBColor; N]) -> [bool; N] {
        let mut max = 0f32;
        let mut col1 = 0;
        let mut col2 = 0;
        for i in 0..N {
            for j in (i+1)..N {
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
            if 
                Self::distance(colors[col1], colors[i]) >
                Self::distance(colors[col2], colors[i]) {
                groups[i] = true;
            }
        }
        groups
    }

    pub fn color(text: &str, foreground_color: &RGBColor, background_color: &RGBColor) -> String {
        format!(
            "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m{}\x1b[0m",
            foreground_color.r, foreground_color.g, foreground_color.b, // foreground color
            background_color.r, background_color.g, background_color.b, // background color
            text
        )
    }
}

impl Clone for RGBColor {
    fn clone(&self) -> Self {
        Self { r: self.r.clone(), g: self.g.clone(), b: self.b.clone() }
    }
}

impl Copy for RGBColor {
    
}

/// Represents a singular pixel implementing the [`MultiPixel`] trait.
pub struct ColorSinglePixel {
    pixels: [RGBColor; 1],
}

impl MultiPixel<ColorSinglePixel> for ColorSinglePixel {
    type U = RGBColor;

    const WIDTH: usize = 1;

    const HEIGHT: usize = 1;
    
    fn new(pixels: [Self::U; 1]) -> ColorSinglePixel {
        ColorSinglePixel {
            pixels
        }
    }
    
    impl_getters!(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]);
}

impl ToString for ColorSinglePixel {
    fn to_string(&self) -> String {
        RGBColor::color("█",&self.pixels[0], &self.pixels[0])
    }
}

pub struct ColorDualPixel {
    pixels: [RGBColor; 2]
}

impl MultiPixel<ColorDualPixel> for ColorDualPixel {
    type U = RGBColor;

    const WIDTH: usize = 1;

    const HEIGHT: usize = 2;
    
    fn new(pixels: [Self::U; 2]) -> ColorDualPixel {
        ColorDualPixel {
            pixels
        }
    }
    
    impl_getters!(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]);
}

impl ToString for ColorDualPixel {
    fn to_string(&self) -> String {
        RGBColor::color("▀",&self.pixels[0], &self.pixels[1])
    }
}

pub struct ColorQuadPixel {
    pixels: [RGBColor; 4]
}

impl MultiPixel<ColorQuadPixel> for ColorQuadPixel {
    type U = RGBColor;

    const WIDTH: usize = 2;

    const HEIGHT: usize = 2;
    
    fn new(pixels: [Self::U; 4]) -> ColorQuadPixel {
        ColorQuadPixel {
            pixels
        }
    }
    
    impl_getters!(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]);
}

impl ToString for ColorQuadPixel {
    fn to_string(&self) -> String {
        let colors = self.pixels;
        let grouping = RGBColor::group(&colors);
        let symb = QuadPixel::new(
            grouping
        ).to_string().chars().next().unwrap();

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
        let col1 = RGBColor::mix(&col1).unwrap_or(RGBColor {r: 0, g: 0, b: 0});
        let col2 = RGBColor::mix(&col2).unwrap_or(RGBColor {r: 0, g: 0, b: 0});

        RGBColor::color(symb.to_string().as_str(), &col1, &col2)
    }
}

pub struct ColorHexPixel {
    pixels: [RGBColor; 6]
}

impl MultiPixel<ColorHexPixel> for ColorHexPixel {
    type U = RGBColor;

    const WIDTH: usize = 2;

    const HEIGHT: usize = 3;
    
    fn new(pixels: [Self::U; 6]) -> ColorHexPixel {
        ColorHexPixel {
            pixels
        }
    }
    
    impl_getters!(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]);
}

impl ToString for ColorHexPixel {
    fn to_string(&self) -> String {
        let colors = self.pixels;
        let grouping = RGBColor::group(&colors);
        let symb = HexPixel::new(
            grouping
        ).to_string().chars().next().unwrap();

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
        let col1 = RGBColor::mix(&col1).unwrap_or(RGBColor {r: 0, g: 0, b: 0});
        let col2 = RGBColor::mix(&col2).unwrap_or(RGBColor {r: 0, g: 0, b: 0});

        RGBColor::color(symb.to_string().as_str(), &col1, &col2)
    }
}

pub struct ColorOctPixel {
    pixels: [RGBColor; 8]
}

impl MultiPixel<ColorOctPixel> for ColorOctPixel {
    type U = RGBColor;

    const WIDTH: usize = 2;

    const HEIGHT: usize = 4;
    
    fn new(pixels: [Self::U; 8]) -> ColorOctPixel {
        ColorOctPixel {
            pixels
        }
    }
    
    impl_getters!(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]);
}

impl ToString for ColorOctPixel {
    fn to_string(&self) -> String {
        let colors = self.pixels;
        let grouping = RGBColor::group(&colors);
        let symb = OctPixel::new(
            grouping
        ).to_string().chars().next().unwrap();

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
        let col1 = RGBColor::mix(&col1).unwrap_or(RGBColor {r: 0, g: 0, b: 0});
        let col2 = RGBColor::mix(&col2).unwrap_or(RGBColor {r: 0, g: 0, b: 0});

        RGBColor::color(symb.to_string().as_str(), &col1, &col2)
    }
}