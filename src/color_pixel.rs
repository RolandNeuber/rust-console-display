use crate::pixel::{MultiPixel, QuadPixel};

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    fn distance(color1: Color, color2: Color) -> f32 {
        (
            (color1.r as i32 - color2.r as i32).pow(2) as f32 +
            (color1.g as i32 - color2.g as i32).pow(2) as f32 +
            (color1.b as i32 - color2.b as i32).pow(2) as f32
        ).sqrt()
    }

    fn mix(colors: &[Color]) -> Result<Color, &str> {
        if colors.len() == 0 {
            return Err("colors must contain at least one element");
        }
        let mut sum = (0, 0, 0);
        for color in colors {
            sum.0 += color.r as u32;
            sum.1 += color.g as u32;
            sum.2 += color.b as u32;
        }
        Ok(Color {
            r: (sum.0 / colors.len() as u32) as u8,
            g: (sum.1 / colors.len() as u32) as u8,
            b: (sum.2 / colors.len() as u32) as u8,
        })
    }

    fn group<const N: usize>(colors: &[Color; N]) -> [bool; N] {
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

    fn color(text: &str, foreground_color: &Color, background_color: &Color) -> String {
        format!(
            "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m{}\x1b[0m",
            foreground_color.r, foreground_color.g, foreground_color.b, // foreground color
            background_color.r, background_color.g, background_color.b, // background color
            text
        )
    }
}

impl Clone for Color {
    fn clone(&self) -> Self {
        Self { r: self.r.clone(), g: self.g.clone(), b: self.b.clone() }
    }
}

impl Copy for Color {
    
}

/// Represents a singular pixel implementing the [`MultiPixel`] trait.
pub struct ColorSinglePixel {
    pixel: Color,
}

impl ColorSinglePixel {
    pub fn new(pixel: Color) -> ColorSinglePixel {
        ColorSinglePixel {
            pixel
        }
    }
}

impl MultiPixel<ColorSinglePixel> for ColorSinglePixel {
    type U = Color;

    const WIDTH: usize = 1;

    const HEIGHT: usize = 1;

    fn build(args: &[Color]) -> Result<ColorSinglePixel, String> {
        let pixel = match args {
            [pixel] => *pixel,
            _ => return Err(format!("Invalid number of arguments. Expected {}, got {}", Self::WIDTH * Self::HEIGHT, args.len())), 
        };
        Ok(ColorSinglePixel::new(pixel))
    }

    fn get_subpixel(&self, x: usize, y: usize) -> Result<Color, String> {
        match (x, y) {
            (0, 0) => Ok(self.pixel),
            _ => Err("Coordinates out of range.".to_string())
        }
    }
    
    fn set_subpixel(&mut self, x: usize, y: usize, value: Color) -> Result<(), String> {
        match (x, y) {
            (0, 0) => Ok(self.pixel = value),
            _ => Err("Coordinates out of range.".to_string()),
        }
    }
}

impl ToString for ColorSinglePixel {
    fn to_string(&self) -> String {
        Color::color("█",&self.pixel, &self.pixel)
    }
}

pub struct ColorDualPixel {
    upper: Color,
    lower: Color,
}

impl ColorDualPixel {
    pub fn new(upper: Color, lower: Color) -> ColorDualPixel {
        ColorDualPixel {
            upper,
            lower,
        }
    }
}

impl MultiPixel<ColorDualPixel> for ColorDualPixel {
    type U = Color;

    const WIDTH: usize = 1;

    const HEIGHT: usize = 2;

    fn build(args: &[Color]) -> Result<ColorDualPixel, String> {
        let (upper, lower) = match args {
            [upper, lower] => (*upper, *lower),
            _ => return Err(format!("Invalid number of arguments. Expected {}, got {}", Self::WIDTH * Self::HEIGHT, args.len())), 
        };
        Ok(ColorDualPixel::new(upper, lower))
    }

    fn get_subpixel(&self, x: usize, y: usize) -> Result<Color, String> {
        match (x, y) {
            (0, 0) => Ok(self.upper),
            (0, 1) => Ok(self.lower),
            _ => Err("Coordinates out of range.".to_string())
        }
    }
    
    fn set_subpixel(&mut self, x: usize, y: usize, value: Color) -> Result<(), String> {
        match (x, y) {
            (0, 0) => Ok(self.upper = value),
            (0, 1) => Ok(self.lower = value),
            _ => Err("Coordinates out of range.".to_string()),
        }
    }
}

impl ToString for ColorDualPixel {
    fn to_string(&self) -> String {
        Color::color("▀",&self.upper, &self.lower)
    }
}

pub struct ColorQuadPixel {
    u_l: Color, u_r: Color,
    l_l: Color, l_r: Color
}

impl ColorQuadPixel {
    pub fn new(u_l: Color, u_r: Color, l_l: Color, l_r: Color) -> ColorQuadPixel {
        ColorQuadPixel {
            u_l, u_r,
            l_l, l_r,
        }
    }
}

impl MultiPixel<ColorQuadPixel> for ColorQuadPixel {
    type U = Color;

    const WIDTH: usize = 2;

    const HEIGHT: usize = 2;

    fn build(args: &[Color]) -> Result<ColorQuadPixel, String> {
        let (u_l, u_r, l_l, l_r,) = match args {
            [u_l, u_r, l_l, l_r,] => (*u_l, *u_r, *l_l, *l_r),
            _ => return Err(format!("Invalid number of arguments. Expected {}, got {}", Self::WIDTH * Self::HEIGHT, args.len())), 
        };
        Ok(ColorQuadPixel::new(u_l, u_r, l_l, l_r))
    }

    fn get_subpixel(&self, x: usize, y: usize) -> Result<Color, String> {
        match (x, y) {
            (0, 0) => Ok(self.u_l),
            (1, 0) => Ok(self.u_r),
            (0, 1) => Ok(self.l_l),
            (1, 1) => Ok(self.l_r),
            _ => Err("Coordinates out of range.".to_string())
        }
    }
    
    fn set_subpixel(&mut self, x: usize, y: usize, value: Color) -> Result<(), String> {
        match (x, y) {
            (0, 0) => Ok(self.u_l = value),
            (1, 0) => Ok(self.u_r = value),
            (0, 1) => Ok(self.l_l = value),
            (1, 1) => Ok(self.l_r = value),
            _ => Err("Coordinates out of range.".to_string()),
        }
    }
}

impl ToString for ColorQuadPixel {
    fn to_string(&self) -> String {
        let colors = [self.u_l, self.u_r, self.l_l, self.l_r];
        let grouping = Color::group(&colors);
        let symb = QuadPixel::new(
            grouping[0], grouping[1], 
            grouping[2], grouping[3]
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
        let col1 = Color::mix(&col1).unwrap_or(Color {r: 0, g: 0, b: 0});
        let col2 = Color::mix(&col2).unwrap_or(Color {r: 0, g: 0, b: 0});

        Color::color(symb.to_string().as_str(), &col1, &col2)
    }
}