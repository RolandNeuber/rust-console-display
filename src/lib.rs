#[derive(Debug)]
pub struct Display<T: MultiPixel<T>> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: MultiPixel<T>> Display<T> {
    pub fn build_from_bools(width: usize, height: usize, data: Vec<bool>) -> Result<Display<T>, String> {
        if width % T::WIDTH != 0 || height % T::HEIGHT != 0 {
            return Err(
                format!(
                    "Width and height must be multiples of multipixel dimensions. Got width = {}, height = {}.", 
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

        let mut multi_pixels = Vec::with_capacity(width * height / T::WIDTH / T::HEIGHT);

        for row in 0..(height / T::HEIGHT) {
            for col in 0..(width / T::WIDTH) {
                let block_x: usize = row * T::WIDTH;
                let block_y: usize = col * T::HEIGHT;

                let mut args = Vec::with_capacity(T::WIDTH * T::HEIGHT);

                for x in 0..T::WIDTH {
                    for y in 0..T::HEIGHT {
                        args.push(data[block_x + x + (block_y + y) * width]);
                    }
                }

                multi_pixels.push(T::build(&args)?);
            }
        }
        Ok(Display {
            width, 
            height, 
            data: multi_pixels
        })
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

impl<T: MultiPixel<T>> ToString for Display<T> {
    fn to_string(&self) -> String {
        let mut string_repr = String::new();
        for y in 0..(self.height / 2) {
            for x in 0..(self.width / 2) {
                string_repr.push(self.data[x + y * self.width / 2].get_char());
            }
            string_repr.push('\n');
        }
        string_repr.pop();
        string_repr
    }
}

pub trait MultiPixel<T> {
    const WIDTH: usize;
    const HEIGHT: usize;

    fn build(args: &[bool]) -> Result<T, String>;
    fn get_char(&self) -> char;
    fn get_subpixel(&self, x: usize, y: usize) -> Result<bool, String>;
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

    pub fn new(u_l: bool, u_r: bool, l_l: bool, l_r: bool) -> QuadPixel {
        QuadPixel {
            u_l,
            u_r,
            l_l,
            l_r,
        }
    }

    fn index(&self) -> usize {
        (self.u_l as usize) | 
        (self.u_r as usize) << 1 | 
        (self.l_l as usize) << 2 | 
        (self.l_r as usize) << 3
    }
}

impl ToString for QuadPixel {
    fn to_string(&self) -> String {
        self.get_char().to_string()
    }
}

impl MultiPixel<QuadPixel> for QuadPixel {
    const WIDTH: usize = 2;
    const HEIGHT: usize = 2;

    fn build(args: &[bool]) -> Result<QuadPixel, String> {
        let (u_l, u_r, l_l, l_r) = match args {
            [u_l, u_r, l_l, l_r] => (*u_l, *u_r, *l_l, *l_r),
            _ => return Err(format!("Invalid number of arguments. Expected 4, got {}", args.len())), 
        };
        Ok(QuadPixel::new(u_l, u_r, l_l, l_r))
    }

    /// ```
    /// use display::{MultiPixel, QuadPixel};
    /// let pixel = QuadPixel::new (
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

    pub fn new(u_l: bool, u_r: bool, m_l: bool, m_r: bool, l_l: bool, l_r: bool) -> HexPixel {
        HexPixel {
            u_l,
            u_r,
            m_l,
            m_r,
            l_l,
            l_r,
        }
    }

    fn index(&self) -> usize {
        (self.u_l as usize) | 
        (self.u_r as usize) << 1 | 
        (self.m_l as usize) << 2 | 
        (self.m_r as usize) << 3 | 
        (self.l_l as usize) << 4 | 
        (self.l_r as usize) << 5
    }
}

impl MultiPixel<HexPixel> for HexPixel {
    const WIDTH: usize = 2;
    const HEIGHT: usize = 3;

    fn build(args: &[bool]) -> Result<HexPixel, String> {
        let (u_l, u_r, m_l, m_r, l_l, l_r) = match args {
            [u_l, u_r, m_l, m_r, l_l, l_r] => (*u_l, *u_r, *m_l, *m_r, *l_l, *l_r),
            _ => return Err(format!("Invalid number of arguments. Expected 4, got {}", args.len())), 
        };
        Ok(HexPixel::new(u_l, u_r, m_l, m_r, l_l, l_r))
    }
    
    /// ```
    /// use display::{MultiPixel, HexPixel};
    /// let pixel = HexPixel::new (
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