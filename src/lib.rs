/// Represents a console display with a width and height in pixels.
pub struct Display<T: MultiPixel<T>> {
    width: usize,
    height: usize,
    block_count_x: usize,
    block_count_y: usize,
    data: Vec<T>,
}

impl<T: MultiPixel<T>> Display<T> {
    /// Builds a display struct with the specified dimensions from the given data.
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

        let block_count_x = width / T::WIDTH;
        let block_count_y = height / T::HEIGHT;

        let mut multi_pixels = Vec::with_capacity(block_count_x * block_count_y);

        for row in 0..block_count_y {
            for col in 0..block_count_x {
                let block_x: usize = col * T::WIDTH;
                let block_y: usize = row * T::HEIGHT;

                let mut args = Vec::with_capacity(T::WIDTH * T::HEIGHT);

                for y in 0..T::HEIGHT {
                    for x in 0..T::WIDTH {
                        args.push(data[block_x + x + (block_y + y) * width]);
                    }
                }

                multi_pixels.push(T::build(&args)?);
            }
        }
        Ok(Display {
            width, 
            height, 
            block_count_x,
            block_count_y,
            data: multi_pixels
        })
    }

    /// Returns a bool representing the state of the pixel at the specified coordinate.
    /// Returns an error if the coordinates are out of bounds.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use display::{Display, SinglePixel};
    /// let disp: Display<SinglePixel> = Display::build_from_bools(
    ///     6, 
    ///     6, 
    ///     vec![
    ///         true, true, true, true,  true, true, // 0
    ///         true, true, true, true,  true, true, // 1
    ///         true, true, true, false, true, true, //-2-
    ///         true, true, true, true,  true, true, // 3
    ///         true, true, true, true,  true, true, // 4
    ///         true, true, true, true,  true, true, // 5
    ///     ] //  0     1     2   --3--    4     5
    /// ).expect("Dimensions of data should match the passed witdh and height");
    /// // Replace with actual error handling
    /// 
    /// let pixel = disp.get_pixel(3, 2);
    /// 
    /// assert_eq!(pixel, Ok(false));
    /// 
    /// let pixel = disp.get_pixel(5, 6);
    /// 
    /// assert!(matches!(pixel, Err(_)));
    /// ```
    pub fn get_pixel(&self, x: usize, y: usize) -> Result<bool, String> {
        if x >= self.width || y >= self.height {
            return Err(format!("Pixel coordinates out of bounds. Got x = {}, y = {}.", x, y))
        }

        let block_x: usize = x / T::WIDTH;
        let block_y: usize = y / T::HEIGHT;
        let offset_x: usize = x % T::WIDTH;
        let offset_y: usize = y % T::HEIGHT;

        let pixel = &self.data[block_x + block_y * self.block_count_x];
        match pixel.get_subpixel(offset_x, offset_y) {
            Ok(val) => Ok(val),
            Err(_) => Err("Offset should be 0 or 1.".to_string()),
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) -> Result<(), String> {
        if x >= self.width || y >= self.height {
            return Err(format!("Pixel coordinates out of bounds. Got x = {}, y = {}.", x, y))
        }

        let block_x: usize = x / T::WIDTH;
        let block_y: usize = y / T::HEIGHT;
        let offset_x: usize = x % T::WIDTH;
        let offset_y: usize = y % T::HEIGHT;

        let pixel = &mut self.data[block_x + block_y * self.block_count_x];
        match pixel.set_subpixel(offset_x, offset_y, value) {
            Ok(val) => Ok(val),
            Err(_) => Err("Offset should be 0 or 1.".to_string()),
        }
    }

    pub fn print_display(&self) {
        print!("\x1B[H");
        print!("{}", self.to_string());
    }

    pub fn initialize(&self) {
        print!("\x1B[?1049h");    // use new screen

        print!(
            "\x1B[8;{};{}t", 
            self.block_count_y, 
            self.block_count_x
        );                        // set dimensions of screen
        print!("\x1B[2J");        // clear screen
    }
}

impl<T: MultiPixel<T>> ToString for Display<T> {
    fn to_string(&self) -> String {
        let mut string_repr = String::new();
        for y in 0..self.block_count_y {
            for x in 0..self.block_count_x {
                string_repr.push(self.data[x + y * self.block_count_x].get_char());
            }
            string_repr.push('\n');
        }
        string_repr.pop();
        string_repr
    }
}

impl<T: MultiPixel<T>> Drop for Display<T> {
    fn drop(&mut self) {
        print!("\x1B[?1049l"); // return to previous screen
    }
}

/// Specifies a block of pixels with specified dimensions.
pub trait MultiPixel<T> {
    /// The width of the block of pixels.
    const WIDTH: usize;
    /// The height of the block of pixels.
    const HEIGHT: usize;

    /// Builds a block of pixels from a slice of bool.
    /// Returns an error, if the number of booleans does not match the dimensions of the block.
    fn build(args: &[bool]) -> Result<T, String>;
    /// Returns the char representing the data of the block visually.
    fn get_char(&self) -> char;
    /// Returns the value of the block at the specified coordinates.
    /// Returns an error, if the coordinates are out-of-bounds.
    fn get_subpixel(&self, x: usize, y: usize) -> Result<bool, String>;

    fn set_subpixel(&mut self, x: usize, y: usize, value: bool) -> Result<(), String>;
}

/// Represents a singular pixel implementing the [`MultiPixel`] trait.
pub struct SinglePixel {
    pixel: bool,
}

impl SinglePixel {
    pub fn new(pixel: bool) -> SinglePixel {
        SinglePixel {
            pixel
        }
    }
}

impl MultiPixel<SinglePixel> for SinglePixel {
    const WIDTH: usize = 1;

    const HEIGHT: usize = 1;

    fn build(args: &[bool]) -> Result<SinglePixel, String> {
        let pixel = match args {
            [pixel] => *pixel,
            _ => return Err(format!("Invalid number of arguments. Expected 1, got {}", args.len())), 
        };
        Ok(SinglePixel::new(pixel))
    }

    /// See [`MultiPixel::get_char`] for details.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use display::{MultiPixel, SinglePixel};
    /// let pixel = SinglePixel::new (
    ///     true,
    /// );
    /// 
    /// let symbol = pixel.get_char();
    /// 
    /// assert_eq!(symbol, '█');
    /// 
    /// let pixel = SinglePixel::new (
    ///     false,
    /// )
    /// 
    /// let symbol = pixel.get_char();
    /// 
    /// assert_eq!(symbol, ' ');
    /// 
    /// ```
    fn get_char(&self) -> char {
        if self.pixel {'█'} else {' '}
    }

    fn get_subpixel(&self, x: usize, y: usize) -> Result<bool, String> {
        match (x, y) {
            (0, 0) => Ok(self.pixel),
            _ => Err("Coordinates out of range.".to_string())
        }
    }
    
    fn set_subpixel(&mut self, x: usize, y: usize, value: bool) -> Result<(), String> {
        match (x, y) {
            (0, 0) => Ok(self.pixel = value),
            _ => Err("Coordinates out of range.".to_string()),
        }
    }
}

/// Specifies a block of pixels with dimensions 2 (width) by 2 (height).
#[derive(Debug)]
pub struct QuadPixel {
    u_l: bool, u_r: bool,
    l_l: bool, l_r: bool,
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
            u_l, u_r,
            l_l, l_r,
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

    /// See [`MultiPixel::get_char`] for details.
    /// 
    /// # Examples
    /// 
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

    fn set_subpixel(&mut self, x: usize, y: usize, value: bool) -> Result<(), String> {
        match (x, y) {
            (0, 0) => self.u_l = value,
            (1, 0) => self.u_r = value,
            (0, 1) => self.l_l = value,
            (1, 1) => self.l_r = value,
            _ => return Err("Coordinates out of range.".to_string())
        };
        Ok(())
    }
}

/// Specifies a block of pixels with dimensions 2 (width) by 3 (height).
pub struct HexPixel {
    u_l: bool, u_r: bool,
    m_l: bool, m_r: bool,
    l_l: bool, l_r: bool,
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
            u_l, u_r,
            m_l, m_r,
            l_l, l_r,
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
    /// See [`MultiPixel::get_char`] for details.
    /// 
    /// # Examples
    /// 
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

    fn set_subpixel(&mut self, x: usize, y: usize, value: bool) -> Result<(), String> {
        match (x, y) {
            (0, 0) => self.u_l = value,
            (1, 0) => self.u_r = value,
            (0, 1) => self.m_l = value,
            (1, 1) => self.m_r = value,
            (0, 2) => self.l_l = value,
            (1, 2) => self.l_r = value,
            _ => return Err("Coordinates out of range.".to_string())
        };
        Ok(())
    }
}

/// Specifies a block of pixels with dimensions 2 (width) by 4 (height) with braille points.
pub struct OctPixel {
    uu_l: bool, uu_r: bool,
    um_l: bool, um_r: bool,
    lm_l: bool, lm_r: bool,
    ll_l: bool, ll_r: bool,
}

impl OctPixel {
    const CHARS: [char; 256] = [
        '⠀', '⠁', '⠈', '⠉', '⠂', '⠃', '⠊', '⠋', '⠐', '⠑', '⠘', '⠙', '⠒', '⠓', '⠚', '⠛',
        '⠄', '⠅', '⠌', '⠍', '⠆', '⠇', '⠎', '⠏', '⠔', '⠕', '⠜', '⠝', '⠖', '⠗', '⠞', '⠟',
        '⠠', '⠡', '⠨', '⠩', '⠢', '⠣', '⠪', '⠫', '⠰', '⠱', '⠸', '⠹', '⠲', '⠳', '⠺', '⠻',
        '⠤', '⠥', '⠬', '⠭', '⠦', '⠧', '⠮', '⠯', '⠴', '⠵', '⠼', '⠽', '⠶', '⠷', '⠾', '⠿',
        '⡀', '⡁', '⡈', '⡉', '⡂', '⡃', '⡊', '⡋', '⡐', '⡑', '⡘', '⡙', '⡒', '⡓', '⡚', '⡛',
        '⡄', '⡅', '⡌', '⡍', '⡆', '⡇', '⡎', '⡏', '⡔', '⡕', '⡜', '⡝', '⡖', '⡗', '⡞', '⡟',
        '⡠', '⡡', '⡨', '⡩', '⡢', '⡣', '⡪', '⡫', '⡰', '⡱', '⡸', '⡹', '⡲', '⡳', '⡺', '⡻',
        '⡤', '⡥', '⡬', '⡭', '⡦', '⡧', '⡮', '⡯', '⡴', '⡵', '⡼', '⡽', '⡶', '⡷', '⡾', '⡿',
        '⢀', '⢁', '⢈', '⢉', '⢂', '⢃', '⢊', '⢋', '⢐', '⢑', '⢘', '⢙', '⢒', '⢓', '⢚', '⢛',
        '⢄', '⢅', '⢌', '⢍', '⢆', '⢇', '⢎', '⢏', '⢔', '⢕', '⢜', '⢝', '⢖', '⢗', '⢞', '⢟',
        '⢠', '⢡', '⢨', '⢩', '⢢', '⢣', '⢪', '⢫', '⢰', '⢱', '⢸', '⢹', '⢲', '⢳', '⢺', '⢻',
        '⢤', '⢥', '⢬', '⢭', '⢦', '⢧', '⢮', '⢯', '⢴', '⢵', '⢼', '⢽', '⢶', '⢷', '⢾', '⢿',
        '⣀', '⣁', '⣈', '⣉', '⣂', '⣃', '⣊', '⣋', '⣐', '⣑', '⣘', '⣙', '⣒', '⣓', '⣚', '⣛',
        '⣄', '⣅', '⣌', '⣍', '⣆', '⣇', '⣎', '⣏', '⣔', '⣕', '⣜', '⣝', '⣖', '⣗', '⣞', '⣟',
        '⣠', '⣡', '⣨', '⣩', '⣢', '⣣', '⣪', '⣫', '⣰', '⣱', '⣸', '⣹', '⣲', '⣳', '⣺', '⣻',
        '⣤', '⣥', '⣬', '⣭', '⣦', '⣧', '⣮', '⣯', '⣴', '⣵', '⣼', '⣽', '⣶', '⣷', '⣾', '⣿',
    ];

    pub fn new(uu_l: bool, uu_r: bool, um_l: bool, um_r: bool, lm_l: bool, lm_r: bool, ll_l: bool, ll_r: bool)
     -> OctPixel {
        OctPixel {
            uu_l, uu_r,
            um_l, um_r,
            lm_l, lm_r,
            ll_l, ll_r,
        }
    }

    fn index(&self) -> usize {
        (self.uu_l as usize) | 
        (self.uu_r as usize) << 1 | 
        (self.um_l as usize) << 2 | 
        (self.um_r as usize) << 3 | 
        (self.lm_l as usize) << 4 | 
        (self.lm_r as usize) << 5 |
        (self.ll_l as usize) << 6 |
        (self.ll_r as usize) << 7
    }
}

impl MultiPixel<OctPixel> for OctPixel {
    const WIDTH: usize = 2;
    const HEIGHT: usize = 4;

    fn build(args: &[bool]) -> Result<OctPixel, String> {
        let (uu_l, uu_r, um_l, um_r, lm_l, lm_r, ll_l, ll_r) = match args {
            [uu_l, uu_r, um_l, um_r, lm_l, lm_r, ll_l, ll_r] 
            => (*uu_l, *uu_r, *um_l, *um_r, *lm_l, *lm_r, *ll_l, *ll_r),
            _ => return Err(format!("Invalid number of arguments. Expected 4, got {}", args.len())), 
        };
        Ok(OctPixel::new(uu_l, uu_r, um_l, um_r, lm_l, lm_r, ll_l, ll_r))
    }
    /// See [`MultiPixel::get_char`] for details.
    /// 
    /// # Examples
    /// 
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
            (0, 0) => Ok(self.uu_l),
            (1, 0) => Ok(self.uu_r),
            (0, 1) => Ok(self.um_l),
            (1, 1) => Ok(self.um_r),
            (0, 2) => Ok(self.lm_l),
            (1, 2) => Ok(self.lm_r),
            (0, 3) => Ok(self.ll_l),
            (1, 3) => Ok(self.ll_r),
            _ => Err("Coordinates out of range.".to_string())
        }
    }

    fn set_subpixel(&mut self, x: usize, y: usize, value: bool) -> Result<(), String> {
        match (x, y) {
            (0, 0) => self.uu_l = value,
            (1, 0) => self.uu_r = value,
            (0, 1) => self.um_l = value,
            (1, 1) => self.um_r = value,
            (0, 2) => self.lm_l = value,
            (1, 2) => self.lm_r = value,
            (0, 3) => self.ll_l = value,
            (1, 3) => self.ll_r = value,
            _ => return Err("Coordinates out of range.".to_string())
        };
        Ok(())
    }
}