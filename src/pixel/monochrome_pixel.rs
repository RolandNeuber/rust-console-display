use crate::{
    color::TerminalColor,
    impl_from_mono_chrome_pixel_for_datacell,
    pixel::Pixel,
};

use crate::{
    impl_getters,
    impl_getters_mut,
    impl_new,
    widget::DataCell,
};

/// Represents a singular pixel implementing the [`Pixel`] trait.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
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
    /// let symbol = pixel.character();
    ///
    /// assert_eq!(symbol, '█');
    ///
    /// let pixel = SinglePixel::new ([
    ///     false,
    /// ]);
    ///
    /// let symbol = pixel.character();
    ///
    /// assert_eq!(symbol, ' ');
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

impl_from_mono_chrome_pixel_for_datacell!(SinglePixel);

/// Specifies a block of pixels with dimensions 1 (width) by 2 (height).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
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
    /// let symbol = pixel.character();
    ///
    /// assert_eq!(symbol, '▀');
    ///
    /// let pixel = DualPixel::new ([
    ///     false, // _
    ///     false, // _
    /// ]);
    ///
    /// let symbol = pixel.character();
    ///
    /// assert_eq!(symbol, ' ');
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

impl_from_mono_chrome_pixel_for_datacell!(DualPixel);

/// Specifies a block of pixels with dimensions 2 (width) by 2 (height).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
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
    /// let symbol = pixel.character();
    ///
    /// assert_eq!(symbol, '▚')
    /// ```
    #[must_use]
    pub const fn character(self) -> char {
        Self::CHARS[self.index()]
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

impl_from_mono_chrome_pixel_for_datacell!(QuadPixel);

/// Specifies a block of pixels with dimensions 2 (width) by 3 (height).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
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
    /// let symbol = pixel.character();
    ///
    /// assert_eq!(symbol, '🬶')
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

impl_from_mono_chrome_pixel_for_datacell!(HexPixel);

/// Specifies a block of pixels with dimensions 2 (width) by 4 (height).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
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
    /// let symbol = pixel.character();
    ///
    /// assert_eq!(symbol, '𜴰')
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

impl_from_mono_chrome_pixel_for_datacell!(OctPixel);

/// Specifies a block of pixels with dimensions 2 (width) by 4 (height) with braille points.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
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
    /// let symbol = pixel.character();
    ///
    /// assert_eq!(symbol, '⠵')
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

impl_from_mono_chrome_pixel_for_datacell!(BrailleOctPixel);

#[cfg(test)]
mod tests {
    mod single_pixel {
        use crate::pixel::{
            Pixel,
            monochrome_pixel::SinglePixel,
        };

        #[test]
        fn character() {
            assert_eq!('█', SinglePixel::new([true]).character());
        }
    }

    mod dual_pixel {
        use crate::pixel::{
            Pixel,
            monochrome_pixel::DualPixel,
        };

        #[test]
        fn character() {
            assert_eq!('▀', DualPixel::new([true, false]).character());
        }
    }

    mod quad_pixel {
        use crate::pixel::{
            Pixel,
            monochrome_pixel::QuadPixel,
        };

        #[test]
        fn character() {
            assert_eq!(
                '▌',
                QuadPixel::new([true, false, true, false]).character()
            );
        }
    }

    mod hex_pixel {
        use crate::pixel::{
            Pixel,
            monochrome_pixel::HexPixel,
        };

        #[test]
        fn character() {
            assert_eq!(
                '🬕',
                HexPixel::new([true, true, true, false, true, false])
                    .character()
            );
        }
    }

    mod oct_pixel {
        use crate::pixel::{
            Pixel,
            monochrome_pixel::OctPixel,
        };

        #[test]
        fn character() {
            assert_eq!(
                '▚',
                OctPixel::new([
                    true, false, true, false, false, true, false, true
                ])
                .character()
            );
        }
    }

    mod braille_oct_pixel {
        use crate::pixel::{
            Pixel,
            monochrome_pixel::BrailleOctPixel,
        };

        #[test]
        fn character() {
            assert_eq!(
                '⢑',
                BrailleOctPixel::new([
                    true, false, false, true, false, false, false, true
                ])
                .character()
            );
        }
    }
}
