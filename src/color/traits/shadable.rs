pub const trait Shadable {
    #[must_use]
    fn adjust_lightness(&self, amount: f32) -> Self;
}
