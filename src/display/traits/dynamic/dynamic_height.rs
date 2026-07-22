pub const trait DynamicHeight {
    /// Returns the height of the display in a display specific, individually addressable unit (e.g. pixels, characters).
    #[must_use]
    fn height(&self) -> usize;
}
