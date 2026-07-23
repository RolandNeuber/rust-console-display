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
    ///     Blendable,
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
    /// assert_eq!(top, Blendable::blend(&top, &bottom));
    /// ```
    #[must_use]
    fn blend(color_top: &Self, color_bottom: &Self) -> Self;
}
