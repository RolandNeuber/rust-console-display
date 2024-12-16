use crate::{impl_getters, impl_new};

/// Specifies a block of pixels with specified dimensions.
pub trait MultiPixel<T: ToString> {
    type U: Copy;

    /// The width of the block of pixels.
    const WIDTH: usize;
    /// The height of the block of pixels.
    const HEIGHT: usize;

    fn new(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]) -> T;

    fn get_pixels(&self) -> &[Self::U; Self::WIDTH * Self::HEIGHT];

    fn get_pixels_mut(&mut self) -> &mut [Self::U; Self::WIDTH * Self::HEIGHT];

    /// Builds a block of pixels from a slice of pixels.
    /// Returns an error, if the number of pixels does not match the dimensions of the block.
    fn build(args: &[Self::U]) -> Result<T, String> where [(); Self::WIDTH * Self::HEIGHT]: {
        if let Ok(pixels) = <[Self::U; Self::WIDTH * Self::HEIGHT]>::try_from(args) {
            Ok(Self::new(pixels))
        }
        else {
            Err(format!("Invalid number of arguments. Expected {}, got {}", Self::WIDTH * Self::HEIGHT, args.len()))
        }        
    }

    /// Returns the value of the block at the specified coordinates.
    /// Returns an error, if the coordinates are out-of-bounds.
    fn get_subpixel(&self, x: usize, y: usize) -> Result<Self::U, String> where [(); Self::WIDTH * Self::HEIGHT]: {
        if let Some(subpixel) = self.get_pixels().get(x + y * Self::WIDTH) {
            Ok(*subpixel)
        }
        else {
            Err("Coordinates out of range.".to_string())
        }
    }

    fn set_subpixel(&mut self, x: usize, y: usize, value: Self::U) -> Result<(), String> where [(); Self::WIDTH * Self::HEIGHT]: {
        let index = x + y * Self::WIDTH;
        if index < self.get_pixels().len() {
            Ok(self.get_pixels_mut()[index] = value)
        }
        else {
            Err("Coordinates out of range.".to_string())
        }
    }
}

/// Represents a singular pixel implementing the [`MultiPixel`] trait.
pub struct SinglePixel {
    pixels: [bool; 1],
}

impl SinglePixel {

    /// See [`MultiPixel::get_char`] for details.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use display::pixel::{MultiPixel, SinglePixel};
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
    /// );
    /// 
    /// let symbol = pixel.get_char();
    /// 
    /// assert_eq!(symbol, ' ');
    /// 
    /// ```
    fn get_char(&self) -> char {
        if self.pixels[0] {'█'} else {' '}
    }
}

impl MultiPixel<SinglePixel> for SinglePixel {
    type U = bool;

    const WIDTH: usize = 1;

    const HEIGHT: usize = 1;
    
    impl_new!(SinglePixel, pixels: [bool; 1]);

    impl_getters!(pixels: [bool; 1]);
}

impl ToString for SinglePixel {
    fn to_string(&self) -> String {
        self.get_char().to_string()
    }
}

/// Specifies a block of pixels with dimensions 1 (width) by 2 (height).
pub struct DualPixel {
    pixels: [bool; 2]
}

impl DualPixel {
    const CHARS: [char; 4] = [
        ' ', '▀',  
        '▄', '█',
    ];

    fn index(&self) -> usize {
        (self.pixels[0] as usize) | 
        (self.pixels[1] as usize) << 1
    }

    /// See [`MultiPixel::get_char`] for details.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use display::pixel::{MultiPixel, DualPixel};
    /// let pixel = DualPixel::new (
    ///     true,  // #
    ///     false, // _
    /// );
    /// 
    /// let symbol = pixel.get_char();
    /// 
    /// assert_eq!(symbol, '▀');
    /// 
    /// let pixel = DualPixel::new (
    ///     false, // _
    ///     false, // _
    /// );
    /// 
    /// let symbol = pixel.get_char();
    /// 
    /// assert_eq!(symbol, ' ');
    /// 
    /// ```
    fn get_char(&self) -> char {
        Self::CHARS[self.index()]
    }
}

impl MultiPixel<DualPixel> for DualPixel {
    type U = bool;

    const WIDTH: usize = 1;

    const HEIGHT: usize = 2;
    
    impl_new!(DualPixel, pixels: [bool; 2]);
    
    impl_getters!(pixels: [bool; 2]);
}

impl ToString for DualPixel {
    fn to_string(&self) -> String {
        self.get_char().to_string()
    }
}

/// Specifies a block of pixels with dimensions 2 (width) by 2 (height).
#[derive(Debug)]
pub struct QuadPixel {
    pixels: [bool; 4]
}

impl QuadPixel {
    const CHARS: [char; 16] = [
        ' ', '▘', '▝', '▀', 
        '▖', '▌', '▞', '▛', 
        '▗', '▚', '▐', '▜', 
        '▄', '▙', '▟', '█',
    ];

    fn index(&self) -> usize {
        (self.pixels[0] as usize) | 
        (self.pixels[1] as usize) << 1 | 
        (self.pixels[2] as usize) << 2 | 
        (self.pixels[3] as usize) << 3
    }

    /// See [`MultiPixel::get_char`] for details.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use display::pixel::{MultiPixel, QuadPixel};
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
}

impl ToString for QuadPixel {
    fn to_string(&self) -> String {
        self.get_char().to_string()
    }
}

impl MultiPixel<QuadPixel> for QuadPixel {
    type U = bool;

    const WIDTH: usize = 2;

    const HEIGHT: usize = 2;
    
    impl_new!(QuadPixel, pixels: [bool; 4]);
    
    impl_getters!(pixels: [bool; 4]);
}

/// Specifies a block of pixels with dimensions 2 (width) by 3 (height).
pub struct HexPixel {
    pixels: [bool; 6]
}

impl HexPixel {
    const CHARS: [char; 64] = [
        ' ', '🬀', '🬁', '🬂', '🬃', '🬄', '🬅', '🬆', '🬇', '🬈', '🬉', '🬊', '🬋', '🬌', '🬍', '🬎', 
        '🬏', '🬐', '🬑', '🬒', '🬓', '▌', '🬔', '🬕', '🬖', '🬗', '🬘', '🬙', '🬚', '🬛', '🬜', '🬝', 
        '🬞', '🬟', '🬠', '🬡', '🬢', '🬣', '🬤', '🬥', '🬦', '🬧', '▐', '🬨', '🬩', '🬪', '🬫', '🬬', 
        '🬭', '🬮', '🬯', '🬰', '🬱', '🬲', '🬳', '🬴', '🬵', '🬶', '🬷', '🬸', '🬹', '🬺', '🬻', '█'
    ];

    fn index(&self) -> usize {
        (self.pixels[0] as usize) | 
        (self.pixels[1] as usize) << 1 | 
        (self.pixels[2] as usize) << 2 | 
        (self.pixels[3] as usize) << 3 | 
        (self.pixels[4] as usize) << 4 | 
        (self.pixels[5] as usize) << 5
    }
    
    /// See [`MultiPixel::get_char`] for details.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use display::pixel::{MultiPixel, HexPixel};
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
}

impl MultiPixel<HexPixel> for HexPixel {
    type U = bool;

    const WIDTH: usize = 2;

    const HEIGHT: usize = 3;
    
    impl_new!(HexPixel, pixels: [bool; 6]);
    
    impl_getters!(pixels: [bool; 6]);
}

impl ToString for HexPixel {
    fn to_string(&self) -> String {
        self.get_char().to_string()
    }
}

/// Specifies a block of pixels with dimensions 2 (width) by 4 (height) with braille points.
pub struct OctPixel {
    pixels: [bool; 8]
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

    fn index(&self) -> usize {
        (self.pixels[0] as usize) | 
        (self.pixels[1] as usize) << 1 | 
        (self.pixels[2] as usize) << 2 | 
        (self.pixels[3] as usize) << 3 | 
        (self.pixels[4] as usize) << 4 | 
        (self.pixels[5] as usize) << 5 |
        (self.pixels[6] as usize) << 6 |
        (self.pixels[7] as usize) << 7
    }

    /// See [`MultiPixel::get_char`] for details.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use display::pixel::{MultiPixel, HexPixel};
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
}

impl MultiPixel<OctPixel> for OctPixel {
    type U = bool;
    
    const WIDTH: usize = 2;

    const HEIGHT: usize = 4;

    impl_new!(OctPixel, pixels: [bool; 8]);
    
    impl_getters!(pixels: [bool; 8]);
}

impl ToString for OctPixel {
    fn to_string(&self) -> String {
        self.get_char().to_string()
    }
}