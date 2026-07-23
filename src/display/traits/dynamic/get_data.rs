use crate::pixel::Pixel;

pub const trait GetData<T: Pixel> {
    #[must_use]
    fn data(&self) -> &[T];
}
