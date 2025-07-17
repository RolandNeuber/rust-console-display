// TODO: Deprecate monochrome pixel in favor of color pixel.
use std::fmt::Display;

use crate::pixel::{
    Pixel,
    color_pixel::TerminalColor,
};

use crate::{
    impl_getters,
    impl_getters_mut,
    impl_new,
    widget::DataCell,
};

/// Represents a singular pixel implementing the [`MultiPixel`] trait.
#[derive(Clone, Copy)]
pub struct SinglePixel {
    pixels: [bool; 1],
}

impl SinglePixel {
    /// # Examples
    ///
    /// ```
    /// #![allow(incomplete_features)]
    /// #![feature(generic_const_exprs)]
    ///
    /// use console_display::pixel::{
    ///     Pixel,
    ///     monochrome_pixel::SinglePixel
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
    #[must_use]
    pub const fn character(self) -> char {
        if self.pixels[0] { '█' } else { ' ' }
    }
}

impl Pixel for SinglePixel {
    type U = bool;

    const WIDTH: usize = 1;

    const HEIGHT: usize = 1;

    impl_getters!(pixels: [bool; 1]);

    impl_getters_mut!(pixels: [bool; 1]);

    impl_new!(SinglePixel, pixels: [bool; 1]);
}

impl Display for SinglePixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.character())
    }
}

impl From<SinglePixel> for DataCell {
    fn from(val: SinglePixel) -> Self {
        Self {
            character: val.character(),
            foreground: TerminalColor::Default,
            background: TerminalColor::Default,
        }
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

    /// # Examples
    ///
    /// ```
    /// #![allow(incomplete_features)]
    /// #![feature(generic_const_exprs)]
    ///
    /// use console_display::pixel::{
    ///     Pixel,
    ///     monochrome_pixel::DualPixel
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
    #[must_use]
    pub const fn character(self) -> char {
        Self::CHARS[self.index()]
    }
}

impl Pixel for DualPixel {
    type U = bool;

    const WIDTH: usize = 1;

    const HEIGHT: usize = 2;

    impl_getters!(pixels: [bool; 2]);

    impl_getters_mut!(pixels: [bool; 2]);

    impl_new!(DualPixel, pixels: [bool; 2]);
}

impl Display for DualPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.character())
    }
}

impl From<DualPixel> for DataCell {
    fn from(val: DualPixel) -> Self {
        Self {
            character: val.character(),
            foreground: TerminalColor::Default,
            background: TerminalColor::Default,
        }
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

    /// # Examples
    ///
    /// ```
    /// #![allow(incomplete_features)]
    /// #![feature(generic_const_exprs)]
    ///
    /// use console_display::pixel::{
    ///     Pixel,
    ///     monochrome_pixel::QuadPixel,
    /// };
    ///
    /// let pixel = QuadPixel::new([
    ///     true, false, // #_
    ///     false, true, // _#
    /// ]);
    ///
    /// let symbol = pixel.to_string();
    ///
    /// assert_eq!(symbol, "▚")
    /// ```
    #[must_use]
    pub const fn character(self) -> char {
        Self::CHARS[self.index()]
    }
}

impl Display for QuadPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.character())
    }
}

impl Pixel for QuadPixel {
    type U = bool;

    const WIDTH: usize = 2;

    const HEIGHT: usize = 2;

    impl_getters!(pixels: [bool; 4]);

    impl_getters_mut!(pixels: [bool; 4]);

    impl_new!(QuadPixel, pixels: [bool; 4]);
}

impl From<QuadPixel> for DataCell {
    fn from(val: QuadPixel) -> Self {
        Self {
            character: val.character(),
            foreground: TerminalColor::Default,
            background: TerminalColor::Default,
        }
    }
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

    /// # Examples
    ///
    /// ```
    /// #![allow(incomplete_features)]
    /// #![feature(generic_const_exprs)]
    ///
    /// use console_display::pixel::{
    ///     Pixel,
    ///     monochrome_pixel::HexPixel
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
    #[must_use]
    pub const fn character(self) -> char {
        Self::CHARS[self.index()]
    }
}

impl Pixel for HexPixel {
    type U = bool;

    const WIDTH: usize = 2;

    const HEIGHT: usize = 3;

    impl_getters!(pixels: [bool; 6]);

    impl_getters_mut!(pixels: [bool; 6]);

    impl_new!(HexPixel, pixels: [bool; 6]);
}

impl Display for HexPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.character())
    }
}

impl From<HexPixel> for DataCell {
    fn from(val: HexPixel) -> Self {
        Self {
            character: val.character(),
            foreground: TerminalColor::Default,
            background: TerminalColor::Default,
        }
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

    /// # Examples
    ///
    /// ```
    /// #![allow(incomplete_features)]
    /// #![feature(generic_const_exprs)]
    ///
    /// use console_display::pixel::{
    ///     Pixel,
    ///     monochrome_pixel::OctPixel
    /// };
    /// 
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
    #[must_use]
    pub const fn character(self) -> char {
        Self::CHARS[self.index()]
    }
}

impl Pixel for OctPixel {
    type U = bool;

    const WIDTH: usize = 2;

    const HEIGHT: usize = 4;

    impl_getters!(pixels: [bool; 8]);

    impl_getters_mut!(pixels: [bool; 8]);

    impl_new!(OctPixel, pixels: [bool; 8]);
}

impl Display for OctPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.character())
    }
}

impl From<OctPixel> for DataCell {
    fn from(val: OctPixel) -> Self {
        Self {
            character: val.character(),
            foreground: TerminalColor::Default,
            background: TerminalColor::Default,
        }
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

    /// # Examples
    ///
    /// ```
    /// #![allow(incomplete_features)]
    /// #![feature(generic_const_exprs)]
    ///
    /// use console_display::pixel::{
    ///     Pixel,
    ///     monochrome_pixel::BrailleOctPixel
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
    #[must_use]
    pub const fn character(self) -> char {
        Self::CHARS[self.index()]
    }
}

impl Pixel for BrailleOctPixel {
    type U = bool;

    const WIDTH: usize = 2;

    const HEIGHT: usize = 4;

    impl_getters!(pixels: [bool; 8]);

    impl_getters_mut!(pixels: [bool; 8]);

    impl_new!(BrailleOctPixel, pixels: [bool; 8]);
}

impl Display for BrailleOctPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.character())
    }
}

impl From<BrailleOctPixel> for DataCell {
    fn from(val: BrailleOctPixel) -> Self {
        Self {
            character: val.character(),
            foreground: TerminalColor::Default,
            background: TerminalColor::Default,
        }
    }
}
