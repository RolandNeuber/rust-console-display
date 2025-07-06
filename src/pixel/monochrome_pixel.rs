use std::fmt::Display;

use crate::pixel::Pixel;

use crate::{
    constraint,
    impl_getters,
    impl_getters_mut,
    impl_new,
};

/// Specifies a block of pixels with specified dimensions.
pub trait MultiPixel: Pixel
where
    Self: Sized,
{
    fn new(pixels: [Self::U; Self::WIDTH * Self::HEIGHT]) -> Self;

    /// Builds a block of pixels from a slice of pixels.
    ///
    /// # Errors
    ///
    /// Returns an error, if the number of pixels does not match the dimensions of the block.
    fn build(args: &[Self::U]) -> Result<Self, String>
    where
        [(); Self::WIDTH * Self::HEIGHT]:,
    {
        <[Self::U; Self::WIDTH * Self::HEIGHT]>::try_from(args)
            .map_or_else(
                |_| {
                    Err(format!(
                        "Invalid number of arguments. Expected {}, got {}",
                        Self::WIDTH * Self::HEIGHT,
                        args.len()
                    ))
                },
                |pixels| Ok(Self::new(pixels)),
            )
    }

    /// Returns the value of the block at the specified coordinates.
    ///
    /// # Errors
    ///
    /// Returns an error, if the coordinates are out of bounds.
    fn subpixel(&self, x: usize, y: usize) -> Result<Self::U, String>
    where
        [(); Self::WIDTH * Self::HEIGHT]:,
    {
        self.pixels().get(x + y * Self::WIDTH).map_or_else(
            || Err("Coordinates out of range.".to_string()),
            |subpixel| Ok(*subpixel),
        )
    }

    fn subpixel_static<const X: usize, const Y: usize>(&self) -> Self::U
    where
        [(); Self::WIDTH * Self::HEIGHT]:,
        constraint!(X < Self::WIDTH):,
        constraint!(Y < Self::HEIGHT):,
    {
        self.pixels()[X + Y * Self::WIDTH]
    }

    /// Returns the value of the block at the specified coordinates.
    ///
    /// # Errors
    ///
    /// Returns an error, if the coordinates are out of bounds.
    fn set_subpixel(
        &mut self,
        x: usize,
        y: usize,
        value: Self::U,
    ) -> Result<(), String>
    where
        [(); Self::WIDTH * Self::HEIGHT]:,
    {
        let index = x + y * Self::WIDTH;
        if index < self.pixels().len() {
            self.pixels_mut()[index] = value;
            Ok(())
        }
        else {
            Err("Coordinates out of range.".to_string())
        }
    }

    fn set_subpixel_static<const X: usize, const Y: usize>(
        &mut self,
        value: Self::U,
    ) where
        [(); Self::WIDTH * Self::HEIGHT]:,
        constraint!(X < Self::WIDTH):,
        constraint!(Y < Self::HEIGHT):,
    {
        self.pixels_mut()[X + Y * Self::WIDTH] = value;
    }
}

/// Represents a singular pixel implementing the [`MultiPixel`] trait.
#[derive(Clone, Copy)]
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
    /// use console_display::pixel::monochrome_pixel::{
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
    const fn get_char(self) -> char {
        if self.pixels[0] { '█' } else { ' ' }
    }
}

impl Pixel for SinglePixel {
    type U = bool;

    const WIDTH: usize = 1;

    const HEIGHT: usize = 1;

    impl_getters!(pixels: [bool; 1]);

    impl_getters_mut!(pixels: [bool; 1]);
}

impl MultiPixel for SinglePixel {
    impl_new!(SinglePixel, pixels: [bool; 1]);
}

impl Display for SinglePixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_char())
    }
}

/// Specifies a block of pixels with dimensions 1 (width) by 2 (height).
#[derive(Clone, Copy)]
pub struct DualPixel {
    pixels: [bool; 2],
}

// Needed because rustfmt panics when skipping elements with some
// special characters directly.
// https://github.com/rust-lang/rustfmt/issues/6523
#[rustfmt::skip]
impl DualPixel {
    #[rustfmt::skip]
    const CHARS: [char; 4] = [
        ' ', '▀',  
        '▄', '█',
    ];

    #[rustfmt::skip]
    const fn index(self) -> usize {
        (self.pixels[0] as usize)      |
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
    /// use console_display::pixel::monochrome_pixel::{
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
    const fn get_char(self) -> char {
        Self::CHARS[self.index()]
    }
}

impl Pixel for DualPixel {
    type U = bool;

    const WIDTH: usize = 1;

    const HEIGHT: usize = 2;

    impl_getters!(pixels: [bool; 2]);

    impl_getters_mut!(pixels: [bool; 2]);
}

impl MultiPixel for DualPixel {
    impl_new!(DualPixel, pixels: [bool; 2]);
}

impl Display for DualPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_char())
    }
}

/// Specifies a block of pixels with dimensions 2 (width) by 2 (height).
#[derive(Clone, Copy)]
pub struct QuadPixel {
    pixels: [bool; 4],
}

// Needed because rustfmt panics when skipping elements with some
// special characters directly.
// https://github.com/rust-lang/rustfmt/issues/6523
#[rustfmt::skip]
impl QuadPixel {
    #[rustfmt::skip]
    const CHARS: [char; 16] = [
        ' ', '▘', '▝', '▀', 
        '▖', '▌', '▞', '▛', 
        '▗', '▚', '▐', '▜', 
        '▄', '▙', '▟', '█',
    ];

    #[rustfmt::skip]
    const fn index(self) -> usize {
        (self.pixels[0] as usize)      |
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
    /// use console_display::pixel::monochrome_pixel::{
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
    const fn get_char(self) -> char {
        Self::CHARS[self.index()]
    }
}

impl Display for QuadPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_char())
    }
}

impl Pixel for QuadPixel {
    type U = bool;

    const WIDTH: usize = 2;

    const HEIGHT: usize = 2;

    impl_getters!(pixels: [bool; 4]);

    impl_getters_mut!(pixels: [bool; 4]);
}

impl MultiPixel for QuadPixel {
    impl_new!(QuadPixel, pixels: [bool; 4]);
}

/// Specifies a block of pixels with dimensions 2 (width) by 3 (height).
#[derive(Clone, Copy)]
pub struct HexPixel {
    pixels: [bool; 6],
}

// Needed because rustfmt panics when skipping elements with some
// special characters directly.
// https://github.com/rust-lang/rustfmt/issues/6523
#[rustfmt::skip]
impl HexPixel {
    #[rustfmt::skip]
    const CHARS: [char; 64] = [
        ' ', '🬀', '🬁', '🬂', '🬃', '🬄', '🬅', '🬆', '🬇', '🬈', '🬉', '🬊', '🬋', '🬌', '🬍', '🬎', 
        '🬏', '🬐', '🬑', '🬒', '🬓', '▌', '🬔', '🬕', '🬖', '🬗', '🬘', '🬙', '🬚', '🬛', '🬜', '🬝', 
        '🬞', '🬟', '🬠', '🬡', '🬢', '🬣', '🬤', '🬥', '🬦', '🬧', '▐', '🬨', '🬩', '🬪', '🬫', '🬬', 
        '🬭', '🬮', '🬯', '🬰', '🬱', '🬲', '🬳', '🬴', '🬵', '🬶', '🬷', '🬸', '🬹', '🬺', '🬻', '█'
    ];

    #[rustfmt::skip]
    const fn index(self) -> usize {
        (self.pixels[0] as usize)      |
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
    /// use console_display::pixel::monochrome_pixel::{
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
    const fn get_char(self) -> char {
        Self::CHARS[self.index()]
    }
}

impl Pixel for HexPixel {
    type U = bool;

    const WIDTH: usize = 2;

    const HEIGHT: usize = 3;

    impl_getters!(pixels: [bool; 6]);

    impl_getters_mut!(pixels: [bool; 6]);
}

impl MultiPixel for HexPixel {
    impl_new!(HexPixel, pixels: [bool; 6]);
}

impl Display for HexPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_char())
    }
}

/// Specifies a block of pixels with dimensions 2 (width) by 4 (height).
#[derive(Clone, Copy)]
pub struct OctPixel {
    pixels: [bool; 8],
}

// Needed because rustfmt panics when skipping elements with some
// special characters directly.
// https://github.com/rust-lang/rustfmt/issues/6523
#[rustfmt::skip]
impl OctPixel {
    #[rustfmt::skip]
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

    #[rustfmt::skip]
    const fn index(self) -> usize {
        (self.pixels[0] as usize)      |
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
    /// use console_display::pixel::monochrome_pixel::{
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
    const fn get_char(self) -> char {
        Self::CHARS[self.index()]
    }
}

impl Pixel for OctPixel {
    type U = bool;

    const WIDTH: usize = 2;

    const HEIGHT: usize = 4;

    impl_getters!(pixels: [bool; 8]);

    impl_getters_mut!(pixels: [bool; 8]);
}

impl MultiPixel for OctPixel {
    impl_new!(OctPixel, pixels: [bool; 8]);
}

impl Display for OctPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_char())
    }
}

/// Specifies a block of pixels with dimensions 2 (width) by 4 (height) with braille points.
#[derive(Clone, Copy)]
pub struct BrailleOctPixel {
    pixels: [bool; 8],
}

impl BrailleOctPixel {
    #[rustfmt::skip]
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

    #[rustfmt::skip]
    const fn index(self) -> usize {
        (self.pixels[0] as usize)      |
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
    /// use console_display::pixel::monochrome_pixel::{
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
    const fn get_char(self) -> char {
        Self::CHARS[self.index()]
    }
}

impl Pixel for BrailleOctPixel {
    type U = bool;

    const WIDTH: usize = 2;

    const HEIGHT: usize = 4;

    impl_getters!(pixels: [bool; 8]);

    impl_getters_mut!(pixels: [bool; 8]);
}

impl MultiPixel for BrailleOctPixel {
    impl_new!(BrailleOctPixel, pixels: [bool; 8]);
}

impl Display for BrailleOctPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_char())
    }
}
