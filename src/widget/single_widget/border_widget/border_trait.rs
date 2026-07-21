use crate::pixel::character_pixel::CharacterPixel;

pub const trait Border {
    fn border_at(
        &self,
        width: usize,
        height: usize,
    ) -> impl Fn(usize, usize) -> CharacterPixel;
    fn width_top(&self) -> usize;
    fn width_left(&self) -> usize;
    fn width_bottom(&self) -> usize;
    fn width_right(&self) -> usize;
}
