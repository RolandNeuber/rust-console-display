pub const trait DynamicCharacterHeight {
    /// Returns the height of the display in characters.
    #[must_use]
    fn height_characters(&self) -> usize;
}
