use crate::pixel::MultiPixel;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    fn distance(color1: Color, color2: Color) -> f32 {
        (
            (color1.r - color2.r).pow(2) as f32 +
            (color1.g - color2.g).pow(2) as f32 +
            (color1.b - color2.b).pow(2) as f32
        ).sqrt()
    }

    fn mix(colors: &[Color]) -> Color {
        let mut sum = (0, 0, 0);
        for color in colors {
            sum.0 += color.r as u32;
            sum.1 += color.g as u32;
            sum.2 += color.b as u32;
        }
        Color {
            r: (sum.0 / colors.len() as u32) as u8,
            g: (sum.0 / colors.len() as u32) as u8,
            b: (sum.0 / colors.len() as u32) as u8,
        }
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
            _ => return Err(format!("Invalid number of arguments. Expected 1, got {}", args.len())), 
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
        format!(
            "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m█\x1b[0m",
            self.pixel.r, self.pixel.g, self.pixel.b, // foreground color
            self.pixel.r, self.pixel.g, self.pixel.b  // background color
        )
    }
}