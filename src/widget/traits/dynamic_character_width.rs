pub const trait DynamicCharacterWidth {
    /// Returns the width of the display in characters.
    #[must_use]
    fn width_characters(&self) -> usize;
}
