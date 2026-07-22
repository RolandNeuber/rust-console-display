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
