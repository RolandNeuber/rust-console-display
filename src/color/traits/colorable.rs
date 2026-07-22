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
