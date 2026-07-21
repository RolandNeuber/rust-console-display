//! Provides abstractions over colors that are used in terminal context.

pub mod argb_color;
pub mod rgb_color;
pub mod terminal_color;

pub use argb_color::*;
pub use rgb_color::*;
pub use terminal_color::*;

/// Defines a color used to color text.
#[deprecated]
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
    ///     RngExt,
    ///     rng,
    /// };
    ///
    /// let top = RGBColor {
    ///     r: rng().random_range(0..=255),
    ///     g: rng().random_range(0..=255),
    ///     b: rng().random_range(0..=255),
    /// };
    /// let bottom = RGBColor {
    ///     r: rng().random_range(0..=255),
    ///     g: rng().random_range(0..=255),
    ///     b: rng().random_range(0..=255),
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

pub const trait Blendable {
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
    ///     RngExt,
    ///     rng,
    /// };
    ///
    /// let top = RGBColor {
    ///     r: rng().random_range(0..=255),
    ///     g: rng().random_range(0..=255),
    ///     b: rng().random_range(0..=255),
    /// };
    /// let bottom = RGBColor {
    ///     r: rng().random_range(0..=255),
    ///     g: rng().random_range(0..=255),
    ///     b: rng().random_range(0..=255),
    /// };
    ///
    /// // Opaque top color is returned.
    /// assert_eq!(top, RGBColor::blend(&top, &bottom));
    /// ```
    #[must_use]
    fn blend(color_top: &Self, color_bottom: &Self) -> Self;
}

pub const trait Colorable {
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
}

pub const trait MetricSpace {
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
}

pub const trait Mixable: Sized {
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
}

pub const trait Groupable: const MetricSpace + Sized {
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

pub const trait Shadable {
    #[must_use]
    fn adjust_lightness(&self, amount: f32) -> Self;
}
