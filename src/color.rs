//! Provides abstractions over colors that are used in terminal context.

/// Defines a color used to color text.
pub const trait Color
where
    Self: Sized,
{
    /// Blends the top with the bottom color.
    /// In the case of opaque colors this simply returns the top color.
    /// If the top color is completely transparent it returns the bottom color.
    /// For a translucent top color this returns a mix of both colors weighted by their opacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use console_display::color::{
    ///     Color,
    ///     RGBColor,
    /// };
    /// use rand::{
    ///     Rng,
    ///     thread_rng,
    /// };
    ///
    /// let top = RGBColor {
    ///     r: thread_rng().gen_range(0..=255),
    ///     g: thread_rng().gen_range(0..=255),
    ///     b: thread_rng().gen_range(0..=255),
    /// };
    /// let bottom = RGBColor {
    ///     r: thread_rng().gen_range(0..=255),
    ///     g: thread_rng().gen_range(0..=255),
    ///     b: thread_rng().gen_range(0..=255),
    /// };
    ///
    /// // Opaque top color is returned.
    /// assert_eq!(top, RGBColor::blend(&top, &bottom));
    /// ```
    #[must_use]
    fn blend(color_top: &Self, color_bottom: &Self) -> Self;

    /// Colors a text with a foreground and background color.
    ///
    /// # Examples
    ///
    /// ```
    /// use console_display::color::{
    ///     Color,
    ///     RGBColor,
    /// };
    ///
    /// let msg =
    ///     RGBColor::color("test", &RGBColor::RED, &RGBColor::BLACK);
    ///
    /// // msg contains escape sequences containing color information.
    /// assert!(msg.chars().count() > 4);
    /// ```
    #[must_use]
    fn color(
        text: &str,
        foreground_color: &Self,
        background_color: &Self,
    ) -> String;

    /// Calculates the relative distance between two colors.
    /// For smaller distances the returned value is smaller.
    /// There are no guarantees for the exact return value, so this has only meaning in comparisons.
    ///
    /// # Examples
    ///
    /// ```
    /// use console_display::color::{
    ///     Color,
    ///     RGBColor,
    ///     TerminalColor,
    /// };
    ///
    /// let dist_prio = TerminalColor::distance(
    ///     &RGBColor::BLACK.into(),
    ///     &TerminalColor::Default,
    /// );
    /// let dist_min = TerminalColor::distance(
    ///     &RGBColor::BLACK.into(),
    ///     &RGBColor::BLACK.into(),
    /// );
    /// let dist = TerminalColor::distance(
    ///     &RGBColor::BLACK.into(),
    ///     &RGBColor::RED.into(),
    /// );
    /// let dist_max = TerminalColor::distance(
    ///     &RGBColor::BLACK.into(),
    ///     &RGBColor::WHITE.into(),
    /// );
    ///
    /// assert!(dist_prio <= dist_min);
    /// assert!(dist_min < dist);
    /// assert!(dist < dist_max);
    /// ```
    #[must_use]
    fn distance(color1: &Self, color2: &Self) -> f32;

    /// Mixes a list of colors into one.
    ///
    /// # Examples
    ///
    /// ```
    /// use console_display::color::{
    ///     Color,
    ///     RGBColor,
    ///     TerminalColor,
    /// };
    ///
    /// let colors = [
    ///     RGBColor::BLACK.into(),
    ///     TerminalColor::Default,
    ///     RGBColor::WHITE.into(),
    /// ];
    ///
    /// let mix = TerminalColor::mix(&colors);
    ///
    /// assert_eq!(
    ///     mix,
    ///     // gray, Default is ignored in calculation
    ///     RGBColor {
    ///         r: 255 / 2,
    ///         g: 255 / 2,
    ///         b: 255 / 2
    ///     }
    ///     .into()
    /// );
    /// ```
    #[must_use]
    fn mix(colors: &[Self]) -> Self;

    /// Groups a list of colors into two groups.
    /// The groups are formed by finding the most distant colors.
    /// All other colors are then put into a group with one of them depending on which distance is smaller.
    /// Returns an array of the size of the input. True and false are used to map each color into one of two groups.
    ///
    /// # Examples
    ///
    /// ```
    /// use console_display::color::{
    ///     Color,
    ///     RGBColor,
    /// };
    ///
    /// let colors =
    ///     [RGBColor::BLACK, RGBColor::WHITE, RGBColor::BLACK];
    ///
    /// let grouping = RGBColor::group(&colors);
    ///
    /// assert_ne!(grouping[0], grouping[1]);
    /// assert_ne!(grouping[1], grouping[2]);
    /// assert_eq!(grouping[0], grouping[2]);
    /// ```
    #[must_use]
    fn group<const N: usize>(colors: &[Self; N]) -> [bool; N] {
        let mut max = 0f32;
        let mut col1 = 0;
        let mut col2 = 0;
        konst::for_range! { i in 0..N =>
            konst::for_range! { j in (i + 1)..N =>
                let dist = Self::distance(&colors[i], &colors[j]);
                if dist > max {
                    max = dist;
                    col1 = i;
                    col2 = j;
                }
            }
        }
        let mut groups = [false; N];
        konst::for_range! { i in 0..N =>
            if Self::distance(&colors[col1], &colors[i]) >
                Self::distance(&colors[col2], &colors[i])
            {
                groups[i] = true;
            }
        }
        groups
    }
}

/// Defines a color used for foreground and background coloring of text.
///
/// `Default` - Uses the default color provided by the terminal for foreground or background respectively.\
/// `ARGBColor` - Displays a color made of RGB components and an alpha/opacity channel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[derive_const(Default)]
pub enum TerminalColor {
    #[default]
    Default,
    ARGBColor(ARGBColor),
}

// TODO: Check if this impl can be const
impl Color for TerminalColor {
    fn blend(color_top: &Self, color_bottom: &Self) -> Self {
        if let Self::ARGBColor(color_top) = color_top &&
            let Self::ARGBColor(color_bottom) = color_bottom
        {
            return Self::ARGBColor(ARGBColor::blend(
                color_top,
                color_bottom,
            ));
        }
        if let Self::ARGBColor(color) = color_top &&
            color.opacity > 0
        {
            *color_top
        }
        else {
            *color_bottom
        }
    }

    fn color(
        text: &str,
        foreground_color: &Self,
        background_color: &Self,
    ) -> String {
        let mut codes = Vec::new();

        if let Self::ARGBColor(top_color) = foreground_color {
            if let Self::ARGBColor(bottom_color) = *background_color {
                let top_color = ARGBColor::blend(top_color, &bottom_color);
                codes.push(format!(
                    "\x1b[38;2;{};{};{}m",
                    top_color.color.r,
                    top_color.color.g,
                    top_color.color.b, // foreground color
                ));
            }
            codes.push(format!(
                "\x1b[38;2;{};{};{}m",
                top_color.color.r,
                top_color.color.g,
                top_color.color.b, // foreground color
            ));
        }
        if let Self::ARGBColor(background_color) = background_color {
            codes.push(format!(
                "\x1b[48;2;{};{};{}m",
                background_color.color.r,
                background_color.color.g,
                background_color.color.b, // background color
            ));
        }
        if codes.is_empty() {
            return text.to_owned();
        }
        format!("{}{text}{}", codes.join(""), "\x1b[0m")
    }

    fn distance(color1: &Self, color2: &Self) -> f32 {
        if let Self::ARGBColor(col1) = color1 &&
            let Self::ARGBColor(col2) = color2
        {
            return ARGBColor::distance(col1, col2);
        }
        0.
    }

    fn mix(colors: &[Self]) -> Self {
        let mut argb_colors = Vec::with_capacity(colors.len());
        for color in colors {
            argb_colors.push(match color {
                Self::Default => continue,
                Self::ARGBColor(argbcolor) => *argbcolor,
            });
        }
        if argb_colors.is_empty() {
            return Self::Default;
        }

        Self::ARGBColor(ARGBColor::mix(argb_colors.as_slice()))
    }
}

// TODO: Check if this impl can be const
impl From<RGBColor> for TerminalColor {
    fn from(value: RGBColor) -> Self {
        Self::ARGBColor(value.into())
    }
}

impl const From<ARGBColor> for TerminalColor {
    fn from(value: ARGBColor) -> Self {
        Self::ARGBColor(value)
    }
}

/// Defines an RGB color used for foreground and background coloring of text.
/// `r`, `g`, `b` are the red, green and blue components of the color respectively.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// TODO: Check if this impl can be const
impl Color for RGBColor {
    fn blend(color_top: &Self, _color_bottom: &Self) -> Self {
        *color_top
    }

    fn color(
        text: &str,
        foreground_color: &Self,
        background_color: &Self,
    ) -> String {
        format!(
            "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m{}\x1b[0m",
            foreground_color.r,
            foreground_color.g,
            foreground_color.b, // foreground color
            background_color.r,
            background_color.g,
            background_color.b, // background color
            text
        )
    }

    #[rustfmt::skip]
    #[allow(clippy::suboptimal_flops)]
    fn distance(color1: &Self, color2: &Self) -> f32 {
        (
            ((f32::from(color1.r) - f32::from(color2.r)) / 255.).powi(2) +
            ((f32::from(color1.g) - f32::from(color2.g)) / 255.).powi(2) +
            ((f32::from(color1.b) - f32::from(color2.b)) / 255.).powi(2)
        )
        .sqrt()
    }

    fn mix(colors: &[Self]) -> Self {
        let mut sum = (0, 0, 0);
        for color in colors {
            sum.0 += usize::from(color.r);
            sum.1 += usize::from(color.g);
            sum.2 += usize::from(color.b);
        }
        let colors_len = colors.len();

        #[allow(clippy::cast_possible_truncation)]
        Self {
            r: (sum.0 / colors_len).clamp(0, 255) as u8,
            g: (sum.1 / colors_len).clamp(0, 255) as u8,
            b: (sum.2 / colors_len).clamp(0, 255) as u8,
        }
    }
}

impl RGBColor {
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
    pub const DARK_GRAY: Self = Self {
        r: 63,
        g: 63,
        b: 63,
    };
    pub const GRAY: Self = Self {
        r: 127,
        g: 127,
        b: 127,
    };
    pub const LIGHT_GRAY: Self = Self {
        r: 191,
        g: 191,
        b: 191,
    };
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
    };
    pub const RED: Self = Self { r: 255, g: 0, b: 0 };
    pub const GREEN: Self = Self { r: 0, g: 255, b: 0 };
    pub const BLUE: Self = Self { r: 0, g: 0, b: 255 };
    pub const YELLOW: Self = Self {
        r: 255,
        g: 255,
        b: 0,
    };
    pub const CYAN: Self = Self {
        r: 0,
        g: 255,
        b: 255,
    };
    pub const MAGENTA: Self = Self {
        r: 255,
        g: 0,
        b: 255,
    };
}

/// Defines an ARGB color used for foreground and background coloring of text.
/// `color` are the opaque RGB components of the color with an additional `opacity`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ARGBColor {
    pub opacity: u8,
    pub color: RGBColor,
}

// TODO: Check if this impl can be const
impl Color for ARGBColor {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn blend(color_top: &Self, color_bottom: &Self) -> Self {
        let opacity_top = f32::from(color_top.opacity) / 255.;
        let opacity_bottom = f32::from(color_bottom.opacity) / 255.;
        let opacity_res =
            opacity_bottom.mul_add(1. - opacity_top, opacity_top);
        let red = f32::from(color_top.color.r).mul_add(
            opacity_top,
            f32::from(color_bottom.color.r) *
                (1. - opacity_top) *
                opacity_bottom,
        ) / opacity_res;
        let green = f32::from(color_top.color.g).mul_add(
            opacity_top,
            f32::from(color_bottom.color.g) *
                (1. - opacity_top) *
                opacity_bottom,
        ) / opacity_res;
        let blue = f32::from(color_top.color.b).mul_add(
            opacity_top,
            f32::from(color_bottom.color.b) *
                (1. - opacity_top) *
                opacity_bottom,
        ) / opacity_res;
        Self {
            opacity: (opacity_res * 255.).clamp(0., 255.) as u8,
            color: RGBColor {
                r: red.clamp(0., 255.) as u8,
                g: green.clamp(0., 255.) as u8,
                b: blue.clamp(0., 255.) as u8,
            },
        }
    }

    fn color(
        text: &str,
        foreground_color: &Self,
        background_color: &Self,
    ) -> String {
        format!(
            "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m{}\x1b[0m",
            foreground_color.color.r,
            foreground_color.color.g,
            foreground_color.color.b, // foreground color
            background_color.color.r,
            background_color.color.g,
            background_color.color.b, // background color
            text
        )
    }

    fn distance(color1: &Self, color2: &Self) -> f32 {
        // Equivalent to d = sqrt(r²+g²+b²+a²)
        RGBColor::distance(&color1.color, &color2.color).hypot(
            (f32::from(color1.opacity) - f32::from(color2.opacity)) / 255.,
        )
    }

    fn mix(colors: &[Self]) -> Self {
        let mut sum_opacity = 0;
        for color in colors {
            sum_opacity += usize::from(color.opacity);
        }

        let colors_len = colors.len();

        #[allow(clippy::cast_possible_truncation)]
        Self {
            opacity: (sum_opacity / colors_len).clamp(0, 255) as u8,
            color: RGBColor::mix(
                &colors.iter().map(|x| x.color).collect::<Vec<_>>(),
            ),
        }
    }
}

impl ARGBColor {
    pub const TRANSPARENT: Self = Self {
        opacity: 0,
        color: RGBColor::BLACK,
    };
}

impl const From<RGBColor> for ARGBColor {
    fn from(value: RGBColor) -> Self {
        Self {
            opacity: u8::MAX,
            color: value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod rgb_color {
        use super::*;
        use rand::{
            Rng,
            thread_rng,
        };

        #[test]
        fn blend() {
            let top = RGBColor {
                r: thread_rng().gen_range(0..=255),
                g: thread_rng().gen_range(0..=255),
                b: thread_rng().gen_range(0..=255),
            };
            let bottom = RGBColor {
                r: thread_rng().gen_range(0..=255),
                g: thread_rng().gen_range(0..=255),
                b: thread_rng().gen_range(0..=255),
            };
            assert_eq!(top, RGBColor::blend(&top, &bottom));
        }

        #[test]
        fn color() {
            let msg =
                RGBColor::color("test", &RGBColor::RED, &RGBColor::BLACK);
            assert!(msg.chars().count() > 4);
        }

        #[test]
        fn group() {
            let colors =
                [RGBColor::BLACK, RGBColor::WHITE, RGBColor::BLACK];
            let grouping = RGBColor::group(&colors);
            assert_ne!(grouping[0], grouping[1]);
            assert_ne!(grouping[1], grouping[2]);
            assert_eq!(grouping[0], grouping[2]);
        }

        #[test]
        fn group_equal() {
            let colors =
                [RGBColor::BLACK, RGBColor::BLACK, RGBColor::BLACK];
            let grouping = RGBColor::group(&colors);
            assert_eq!(grouping[0], grouping[1]);
            assert_eq!(grouping[1], grouping[2]);
            assert_eq!(grouping[0], grouping[2]);
        }
    }

    mod terminal_color {
        use super::*;

        #[test]
        fn color_none() {
            let msg = TerminalColor::color(
                "test",
                &TerminalColor::Default,
                &TerminalColor::Default,
            );
            assert_eq!(msg, "test");
        }

        #[test]
        fn color_default() {
            let msg = TerminalColor::color(
                "test",
                &RGBColor::RED.into(),
                &TerminalColor::Default,
            );
            assert!(msg.chars().count() > 4);
        }

        #[test]
        fn distance() {
            let dist_prio = TerminalColor::distance(
                &RGBColor::BLACK.into(),
                &TerminalColor::Default,
            );
            let dist_min = TerminalColor::distance(
                &RGBColor::BLACK.into(),
                &RGBColor::BLACK.into(),
            );
            let dist = TerminalColor::distance(
                &RGBColor::BLACK.into(),
                &RGBColor::RED.into(),
            );
            let dist_max = TerminalColor::distance(
                &RGBColor::BLACK.into(),
                &RGBColor::WHITE.into(),
            );

            assert!(dist_prio <= dist_min);
            assert!(dist_min < dist);
            assert!(dist < dist_max);
        }

        #[test]
        fn mix() {
            let colors = [RGBColor::BLACK, RGBColor::RED, RGBColor::GREEN]
                .map(Into::into);
            let mix = TerminalColor::mix(&colors);
            assert_eq!(
                mix,
                RGBColor {
                    r: 255 / 3,
                    g: 255 / 3,
                    b: 0
                }
                .into()
            );
        }

        #[test]
        fn mix_empty() {
            let colors = [];
            let mix = TerminalColor::mix(&colors);
            assert_eq!(mix, TerminalColor::Default);
        }

        #[test]
        fn mix_only_default() {
            let colors = [TerminalColor::Default, TerminalColor::Default];
            let mix = TerminalColor::mix(&colors);
            assert_eq!(mix, TerminalColor::Default);
        }

        #[test]
        fn mix_default() {
            let colors = [
                RGBColor::BLACK.into(),
                TerminalColor::Default,
                RGBColor::WHITE.into(),
            ];
            let mix = TerminalColor::mix(&colors);
            assert_eq!(mix, RGBColor::GRAY.into());
        }

        #[test]
        fn blend_translucent_default() {
            let translucent_red = ARGBColor::mix(&[
                RGBColor::RED.into(),
                ARGBColor::TRANSPARENT,
            ])
            .into();
            let color = TerminalColor::blend(
                &translucent_red,
                &TerminalColor::Default,
            );
            assert_eq!(color, translucent_red);
        }

        #[test]
        fn blend_transparent_default() {
            let color = TerminalColor::blend(
                &ARGBColor::TRANSPARENT.into(),
                &TerminalColor::Default,
            );
            assert_eq!(color, TerminalColor::Default);
        }

        #[test]
        fn from_argb() {
            let color: TerminalColor = ARGBColor::TRANSPARENT.into();
            assert_eq!(
                color,
                TerminalColor::ARGBColor(ARGBColor::TRANSPARENT)
            );
        }
    }

    mod argb_color {
        use super::*;

        #[test]
        fn color() {
            let msg = ARGBColor::color(
                "test",
                &RGBColor::RED.into(),
                &RGBColor::BLACK.into(),
            );
            assert!(msg.chars().count() > 4);
        }
    }
}
