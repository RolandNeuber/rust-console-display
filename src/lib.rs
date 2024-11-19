#[derive(Debug)]
pub struct Display {
    width: usize,
    height: usize,
    data: Vec<QuadPixel>,
}

impl Display {
    pub fn build(width: usize, height: usize, data: Vec<QuadPixel>) -> Result<Display, String> {
        if width % 2 == 1 || height % 2 == 1 {
            return Err(
                format!(
                    "Width and height must be even. Got width = {}, height = {}.", 
                    width, 
                    height
                )
            );
        }
        if data.len() * 4 / width / height != 1 {
            return Err(
                format!(
                    "Data does not match specified dimensions. Expected length of {}, got {}.", 
                    width * height, 
                    data.len()
                )
            );
        }
        Ok(Display {
            width, 
            height, 
            data
        })
    }

    pub fn build_from_bools(width: usize, height: usize, data: Vec<bool>) -> Result<Display, String> {
        if width % 2 == 1 || height % 2 == 1 {
            return Err(
                format!(
                    "Width and height must be even. Got width = {}, height = {}.", 
                    width, 
                    height
                )
            );
        }
        if data.len() / width / height != 1 {
            return Err(
                format!(
                    "Data does not match specified dimensions. Expected length of {}, got {}.", 
                    width * height, 
                    data.len()
                )
            );
        }

        let mut quad_pixels = Vec::with_capacity(width * height / 4);

        for row in 0..(height / 2) {
            for col in 0..(width / 2) {
                let block_x: usize = row * 2;
                let block_y: usize = col * 2;

                quad_pixels.push(QuadPixel {
                    u_l: data[block_x     +  block_y      * width],
                    u_r: data[block_x + 1 +  block_y      * width],
                    l_l: data[block_x     + (block_y + 1) * width],
                    l_r: data[block_x + 1 + (block_y + 1) * width]
                })
            }
        }
        Display::build(width, height, quad_pixels)
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Result<bool, String> {
        if x >= self.width || y >= self.height {
            return Err(format!("Pixel coordinates out of bounds. Got x = {}, y = {}.", x, y))
        }

        let block_x: usize = x / 2;
        let block_y: usize = y / 2;
        let offset_x: usize = x % 2;
        let offset_y: usize = y % 2;

        let pixel = &self.data[block_x + block_y * self.width / 2];
        match pixel.get_subpixel(offset_x, offset_y) {
            Ok(val) => Ok(val),
            Err(_) => Err("Offset should be 0 or 1.".to_string()),
        }
    }
}

pub trait MultiPixel {
    fn get_char(&self) -> char;
    fn get_subpixel(&self, x: usize, y: usize) -> Result<bool, String>;
}

pub enum Pixel {
    QuadPixel(QuadPixel),
    HexPixel(HexPixel),
}

impl Pixel {
    pub fn quad_pixel(u_l: bool, u_r: bool, l_l: bool, l_r: bool) -> Self {
        Pixel::QuadPixel(
            QuadPixel {
                u_l, u_r, 
                l_l, l_r
            }
        )
    }

    pub fn hex_pixel(u_l: bool, u_r: bool, m_l: bool, m_r: bool, l_l: bool, l_r: bool) -> Self {
        Pixel::HexPixel(
            HexPixel {
                u_l, u_r, 
                m_l, m_r, 
                l_l, l_r
            }
        )
    }

    pub fn get_char(&self) -> char {
        match self {
            Self::QuadPixel(pix) => pix.get_char(),
            Self::HexPixel(pix) => pix.get_char(),
        }
    }
}

#[derive(Debug)]
pub struct QuadPixel {
    u_l: bool,
    u_r: bool,
    l_l: bool,
    l_r: bool,
}

impl QuadPixel {
    const CHARS: [char; 16] = [
        ' ', '▘', '▝', '▀', 
        '▖', '▌', '▞', '▛', 
        '▗', '▚', '▐', '▜', 
        '▄', '▙', '▟', '█',
    ];

    fn index(&self) -> usize {
        (self.u_l as usize) | 
        (self.u_r as usize) << 1 | 
        (self.l_l as usize) << 2 | 
        (self.l_r as usize) << 3
    }
}

impl MultiPixel for QuadPixel {
    /// ```
    /// use display::Pixel;
    /// let pixel = Pixel::quad_pixel (
    ///     true, false, // #_
    ///     false, true, // _#
    /// );
    /// 
    /// let symbol = pixel.get_char();
    /// 
    /// assert_eq!(symbol, '▚')
    /// ```
    fn get_char(&self) -> char {
        Self::CHARS[self.index()]
    }

    fn get_subpixel(&self, x: usize, y: usize) -> Result<bool, String> {
        match (x, y) {
            (0, 0) => Ok(self.u_l),
            (1, 0) => Ok(self.u_r),
            (0, 1) => Ok(self.l_l),
            (1, 1) => Ok(self.l_r),
            _ => Err("Coordinates out of range.".to_string())
        }
    }
}

pub struct HexPixel {
    u_l: bool,
    u_r: bool,
    m_l: bool,
    m_r: bool,
    l_l: bool,
    l_r: bool,
}

impl HexPixel {
    const CHARS: [char; 64] = [
        ' ', '🬀', '🬁', '🬂', '🬃', '🬄', '🬅', '🬆', '🬇', '🬈', '🬉', '🬊', '🬋', '🬌', '🬍', '🬎', 
        '🬏', '🬐', '🬑', '🬒', '🬓', '▌', '🬔', '🬕', '🬖', '🬗', '🬘', '🬙', '🬚', '🬛', '🬜', '🬝', 
        '🬞', '🬟', '🬠', '🬡', '🬢', '🬣', '🬤', '🬥', '🬦', '🬧', '▐', '🬨', '🬩', '🬪', '🬫', '🬬', 
        '🬭', '🬮', '🬯', '🬰', '🬱', '🬲', '🬳', '🬴', '🬵', '🬶', '🬷', '🬸', '🬹', '🬺', '🬻', '█'
    ];

    fn index(&self) -> usize {
        (self.u_l as usize) | 
        (self.u_r as usize) << 1 | 
        (self.m_l as usize) << 2 | 
        (self.m_r as usize) << 3 | 
        (self.l_l as usize) << 4 | 
        (self.l_r as usize) << 5
    }
}

impl MultiPixel for HexPixel {
    /// ```
    /// use display::Pixel;
    /// let pixel = Pixel::hex_pixel (
    ///     true, false, // #_
    ///     false, true, // _#
    ///     true, true,  // ##
    /// );
    /// 
    /// let symbol = pixel.get_char();
    /// 
    /// assert_eq!(symbol, '🬶')
    /// ```
    fn get_char(&self) -> char {
        Self::CHARS[self.index()]
    }
    
    fn get_subpixel(&self, x: usize, y: usize) -> Result<bool, String> {
        match (x, y) {
            (0, 0) => Ok(self.u_l),
            (1, 0) => Ok(self.u_r),
            (0, 1) => Ok(self.m_l),
            (1, 1) => Ok(self.m_r),
            (0, 2) => Ok(self.l_l),
            (1, 2) => Ok(self.l_r),
            _ => Err("Coordinates out of range.".to_string())
        }
    }
}