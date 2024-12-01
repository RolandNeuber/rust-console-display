use crate::pixel::MultiPixel;

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
        let ul_ur = Color::distance(self.u_l, self.u_r);
        let ul_ll = Color::distance(self.u_l, self.l_l);
        let ul_lr = Color::distance(self.u_l, self.l_r);
        let ur_ll = Color::distance(self.u_r, self.l_l);
        let ur_lr = Color::distance(self.u_r, self.l_r);
        let ll_lr = Color::distance(self.l_l, self.l_r);
        let mut max = 0f32;

        for dist in [ul_ur, ul_ll, ul_lr, ur_ll, ur_lr, ll_lr] {
            if dist > max {
                max = dist;
            }
        }
        
        let col1;
        let col2;
        let symb;
        if ul_ur == max {
            col1 = self.u_l; // #_
            col2 = self.u_r; // ??
            symb = match (ul_ll < ur_ll, ul_lr < ur_lr) {
                (true,  true ) => '▙',
                (true,  false) => '▌',
                (false, true ) => '▚',
                (false, false) => '▘',
            }
        }
        else if ul_ll == max {
            col1 = self.u_l; // #?
            col2 = self.l_l; // _?
            symb = match (ul_ur < ur_ll, ul_lr < ll_lr) {
                (true,  true ) => '▜',
                (true,  false) => '▀',
                (false, true ) => '▚',
                (false, false) => '▘',
            }
        }
        else if ul_lr == max {
            col1 = self.u_l; // #?
            col2 = self.l_r; // ?_
            symb = match (ul_ur < ur_lr, ul_ll < ll_lr) {
                (true,  true ) => '▛',
                (true,  false) => '▀',
                (false, true ) => '▌',
                (false, false) => '▘',
            }
        }
        else if ur_ll == max {
            col1 = self.u_r; // ?#
            col2 = self.l_l; // _?
            symb = match (ul_ur < ul_ll, ur_lr < ll_lr) {
                (true,  true ) => '▜',
                (true,  false) => '▀',
                (false, true ) => '▐',
                (false, false) => '▝',
            }
        }
        else if ur_lr == max {
            col1 = self.u_r; // ?#
            col2 = self.l_r; // ?_
            symb = match (ul_ur < ul_lr, ur_ll < ll_lr) {
                (true,  true ) => '▛',
                (true,  false) => '▀',
                (false, true ) => '▞',
                (false, false) => '▝',
            }
        }
        else /* if ll_lr == max */ {
            col1 = self.l_l; // ??
            col2 = self.l_r; // #_
            symb = match (ul_ll < ul_lr, ur_ll < ur_lr) {
                (true,  true ) => '▛',
                (true,  false) => '▌',
                (false, true ) => '▞',
                (false, false) => '▖',
            }
        }
        
        Color::color(symb.to_string().as_str(), &col1, &col2)
    }
}