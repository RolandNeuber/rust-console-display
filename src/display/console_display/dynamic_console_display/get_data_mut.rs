use crate::pixel::Pixel;

pub const trait GetDataMut<T: Pixel> {
    fn data_mut(&mut self) -> &mut Box<[T]>;
}
