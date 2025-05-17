use std::fmt::Display;

use crate::{impl_getters, impl_new};

/// Specifies a block of pixels with specified dimensions.
pub trait MultiPixel: ToString where Self: Sized {
    type U: Copy;

    /// The width of the block of pixels.
    const WIDTH: usize;
    /// The height of the block of pixels.
    const HEIGHT: usize;

    fn new(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]) -> Self;

    fn get_pixels(&self) -> &[Self::U; Self::WIDTH * Self::HEIGHT];

    fn get_pixels_mut(&mut self) -> &mut [Self::U; Self::WIDTH * Self::HEIGHT];

    /// Builds a block of pixels from a slice of pixels.
    /// Returns an error, if the number of pixels does not match the dimensions of the block.
    fn build(args: &[Self::U]) -> Result<Self, String> where [(); Self::WIDTH * Self::HEIGHT]: {
        <[Self::U; Self::WIDTH * Self::HEIGHT]>::try_from(args).map_or_else(
            |_| Err(format!("Invalid number of arguments. Expected {}, got {}", Self::WIDTH * Self::HEIGHT, args.len())), 
            |pixels| Ok(Self::new(pixels))
        )
    }

    /// Returns the value of the block at the specified coordinates.
    /// Returns an error, if the coordinates are out-of-bounds.
    fn get_subpixel(&self, x: usize, y: usize) -> Result<Self::U, String> where [(); Self::WIDTH * Self::HEIGHT]: {
        self.get_pixels().get(x + y * Self::WIDTH).map_or_else(
            || Err("Coordinates out of range.".to_string()), 
            |subpixel| Ok(*subpixel)
        )
    }

    fn set_subpixel(&mut self, x: usize, y: usize, value: Self::U) -> Result<(), String> where [(); Self::WIDTH * Self::HEIGHT]: {
        let index = x + y * Self::WIDTH;
        if index < self.get_pixels().len() {
            self.get_pixels_mut()[index] = value;
            Ok(())
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
    /// #![allow(incomplete_features)]
    /// #![feature(generic_const_exprs)]
    /// 
    /// use display::pixel::monochrome_pixel::{
    ///     MultiPixel, 
    ///     SinglePixel
    /// };
    /// 
    /// let pixel = SinglePixel::new ([
    ///     true,
    /// ]);
    /// 
    /// let symbol = pixel.to_string();
    /// 
    /// assert_eq!(symbol, "█");
    /// 
    /// let pixel = SinglePixel::new ([
    ///     false,
    /// ]);
    /// 
    /// let symbol = pixel.to_string();
    /// 
    /// assert_eq!(symbol, " ");
    /// 
    /// ```
    const fn get_char(&self) -> char {
        if self.pixels[0] {'█'} else {' '}
    }
}

impl MultiPixel for SinglePixel {
    type U = bool;

    const WIDTH: usize = 1;

    const HEIGHT: usize = 1;
    
    impl_new!(SinglePixel, pixels: [bool; 1]);

    impl_getters!(pixels: [bool; 1]);
}

impl Display for SinglePixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_char())
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

    const fn index(&self) -> usize {
        (self.pixels[0] as usize) | 
        (self.pixels[1] as usize) << 1
    }

    /// See [`MultiPixel::get_char`] for details.
    /// 
    /// # Examples
    /// 
    /// ```
    /// #![allow(incomplete_features)]
    /// #![feature(generic_const_exprs)]
    /// 
    /// use display::pixel::monochrome_pixel::{
    ///     MultiPixel, 
    ///     DualPixel
    /// };
    /// let pixel = DualPixel::new ([
    ///     true,  // #
    ///     false, // _
    /// ]);
    /// 
    /// let symbol = pixel.to_string();
    /// 
    /// assert_eq!(symbol, "▀");
    /// 
    /// let pixel = DualPixel::new ([
    ///     false, // _
    ///     false, // _
    /// ]);
    /// 
    /// let symbol = pixel.to_string();
    /// 
    /// assert_eq!(symbol, " ");
    /// 
    /// ```
    const fn get_char(&self) -> char {
        Self::CHARS[self.index()]
    }
}

impl MultiPixel for DualPixel {
    type U = bool;

    const WIDTH: usize = 1;

    const HEIGHT: usize = 2;
    
    impl_new!(DualPixel, pixels: [bool; 2]);
    
    impl_getters!(pixels: [bool; 2]);
}

impl Display for DualPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_char())
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

    const fn index(&self) -> usize {
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
    /// #![allow(incomplete_features)]
    /// #![feature(generic_const_exprs)]
    /// 
    /// use display::pixel::monochrome_pixel::{
    ///     MultiPixel, 
    ///     QuadPixel
    /// };
    /// 
    /// let pixel = QuadPixel::new ([
    ///     true, false, // #_
    ///     false, true, // _#
    /// ]);
    /// 
    /// let symbol = pixel.to_string();
    /// 
    /// assert_eq!(symbol, "▚")
    /// ```
    const fn get_char(&self) -> char {
        Self::CHARS[self.index()]
    }
}

impl Display for QuadPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_char())
    }
}

impl MultiPixel for QuadPixel {
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

    const fn index(&self) -> usize {
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
    /// #![allow(incomplete_features)]
    /// #![feature(generic_const_exprs)]
    /// 
    /// use display::pixel::monochrome_pixel::{
    ///     MultiPixel, 
    ///     HexPixel
    /// };
    /// 
    /// let pixel = HexPixel::new ([
    ///     true, false, // #_
    ///     false, true, // _#
    ///     true, true,  // ##
    /// ]);
    /// 
    /// let symbol = pixel.to_string();
    /// 
    /// assert_eq!(symbol, "🬶")
    /// ```
    const fn get_char(&self) -> char {
        Self::CHARS[self.index()]
    }
}

impl MultiPixel for HexPixel {
    type U = bool;

    const WIDTH: usize = 2;

    const HEIGHT: usize = 3;
    
    impl_new!(HexPixel, pixels: [bool; 6]);
    
    impl_getters!(pixels: [bool; 6]);
}

impl Display for HexPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_char())
    }
}

/// Specifies a block of pixels with dimensions 2 (width) by 4 (height).
pub struct OctPixel {
    pixels: [bool; 8]
}

impl OctPixel {
    const CHARS: [char; 256] = [
        ' ', '𜺨', '𜺫', '🮂', '𜴀', '▘', '𜴁', '𜴂', '𜴃', '𜴄', '▝', '𜴅', '𜴆', '𜴇', '𜴈', '▀',
        '𜴉', '𜴊', '𜴋', '𜴌', '🯦', '𜴍', '𜴎', '𜴏', '𜴐', '𜴑', '𜴒', '𜴓', '𜴔', '𜴕', '𜴖', '𜴗',
        '𜴘', '𜴙', '𜴚', '𜴛', '𜴜', '𜴝', '𜴞', '𜴟', '🯧', '𜴠', '𜴡', '𜴢', '𜴣', '𜴤', '𜴥', '𜴦',
        '𜴧', '𜴨', '𜴩', '𜴪', '𜴫', '𜴬', '𜴭', '𜴮', '𜴯', '𜴰', '𜴱', '𜴲', '𜴳', '𜴴', '𜴵', '🮅',
        '𜺣', '𜴶', '𜴷', '𜴸', '𜴹', '𜴺', '𜴻', '𜴼', '𜴽', '𜴾', '𜴿', '𜵀', '𜵁', '𜵂', '𜵃', '𜵄',
        '▖', '𜵅', '𜵆', '𜵇', '𜵈', '▌', '𜵉', '𜵊', '𜵋', '𜵌', '▞', '𜵍', '𜵎', '𜵏', '𜵐', '▛',
        '𜵑', '𜵒', '𜵓', '𜵔', '𜵕', '𜵖', '𜵗', '𜵘', '𜵙', '𜵚', '𜵛', '𜵜', '𜵝', '𜵞', '𜵟', '𜵠',
        '𜵡', '𜵢', '𜵣', '𜵤', '𜵥', '𜵦', '𜵧', '𜵨', '𜵩', '𜵪', '𜵫', '𜵬', '𜵭', '𜵮', '𜵯', '𜵰',
        '𜺠', '𜵱', '𜵲', '𜵳', '𜵴', '𜵵', '𜵶', '𜵷', '𜵸', '𜵹', '𜵺', '𜵻', '𜵼', '𜵽', '𜵾', '𜵿',
        '𜶀', '𜶁', '𜶂', '𜶃', '𜶄', '𜶅', '𜶆', '𜶇', '𜶈', '𜶉', '𜶊', '𜶋', '𜶌', '𜶍', '𜶎', '𜶏',
        '▗', '𜶐', '𜶑', '𜶒', '𜶓', '▚', '𜶔', '𜶕', '𜶖', '𜶗', '▐', '𜶘', '𜶙', '𜶚', '𜶛', '▜',
        '𜶜', '𜶝', '𜶞', '𜶟', '𜶠', '𜶡', '𜶢', '𜶣', '𜶤', '𜶥', '𜶦', '𜶧', '𜶨', '𜶩', '𜶪', '𜶫',
        '▂', '𜶬', '𜶭', '𜶮', '𜶯', '𜶰', '𜶱', '𜶲', '𜶳', '𜶴', '𜶵', '𜶶', '𜶷', '𜶸', '𜶹', '𜶺',
        '𜶻', '𜶼', '𜶽', '𜶾', '𜶿', '𜷀', '𜷁', '𜷂', '𜷃', '𜷄', '𜷅', '𜷆', '𜷇', '𜷈', '𜷉', '𜷊',
        '𜷋', '𜷌', '𜷍', '𜷎', '𜷏', '𜷐', '𜷑', '𜷒', '𜷓', '𜷔', '𜷕', '𜷖', '𜷗', '𜷘', '𜷙', '𜷚',
        '▄', '𜷛', '𜷜', '𜷝', '𜷞', '▙', '𜷟', '𜷠', '𜷡', '𜷢', '▟', '𜷣', '▆', '𜷤', '𜷥', '█',
    ];

    const fn index(&self) -> usize {
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
    /// #![allow(incomplete_features)]
    /// #![feature(generic_const_exprs)]
    /// 
    /// use display::pixel::monochrome_pixel::{
    ///     MultiPixel, 
    ///     OctPixel
    /// };
    /// let pixel = OctPixel::new ([
    ///     true, false, // #_
    ///     false, true, // _#
    ///     true, true,  // ##
    ///     false, false // __
    /// ]);
    /// 
    /// let symbol = pixel.to_string();
    /// 
    /// assert_eq!(symbol, "𜴰")
    /// ```
    const fn get_char(&self) -> char {
        Self::CHARS[self.index()]
    }
}

impl MultiPixel for OctPixel {
    type U = bool;
    
    const WIDTH: usize = 2;

    const HEIGHT: usize = 4;

    impl_new!(OctPixel, pixels: [bool; 8]);
    
    impl_getters!(pixels: [bool; 8]);
}

impl Display for OctPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_char())
    }
}

/// Specifies a block of pixels with dimensions 2 (width) by 4 (height) with braille points.
pub struct BrailleOctPixel {
    pixels: [bool; 8]
}

impl BrailleOctPixel {
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

    const fn index(&self) -> usize {
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
    /// #![allow(incomplete_features)]
    /// #![feature(generic_const_exprs)]
    /// 
    /// use display::pixel::monochrome_pixel::{
    ///     MultiPixel, 
    ///     BrailleOctPixel
    /// };
    /// 
    /// let pixel = BrailleOctPixel::new ([
    ///     true, false, // #_
    ///     false, true, // _#
    ///     true, true,  // ##
    ///     false, false // __
    /// ]);
    /// 
    /// let symbol = pixel.to_string();
    /// 
    /// assert_eq!(symbol, "⠵")
    /// ```
    const fn get_char(&self) -> char {
        Self::CHARS[self.index()]
    }
}

impl MultiPixel for BrailleOctPixel {
    type U = bool;
    
    const WIDTH: usize = 2;

    const HEIGHT: usize = 4;

    impl_new!(BrailleOctPixel, pixels: [bool; 8]);
    
    impl_getters!(pixels: [bool; 8]);
}

impl Display for BrailleOctPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_char())
    }
}