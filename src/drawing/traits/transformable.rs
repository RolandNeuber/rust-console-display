pub const trait Transformable {
    /// Transforms the drawable by applying a function to all coordinate pairs that define it.
    /// Returns a new transformed drawable.
    #[must_use]
    fn transform<F: [const] Fn((f32, f32)) -> (f32, f32)>(
        &self,
        transform: F,
    ) -> Self;
}
