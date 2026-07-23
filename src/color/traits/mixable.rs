pub const trait Mixable: Sized {
    /// Mixes a list of colors into one.
    ///
    /// # Examples
    ///
    /// ```
    /// use console_display::color::{
    ///     Mixable,
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
    /// let mix = Mixable::mix(&colors);
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
