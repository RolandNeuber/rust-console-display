pub const trait DynamicWidth {
    /// Returns the width of the display in a display specific, individually addressable unit (e.g. pixels, characters).
    #[must_use]
    fn width(&self) -> usize;
}
