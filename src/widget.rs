pub mod single_widget;
pub mod two_widget;

pub trait Widget: ToString {
    fn get_width_characters(&self) -> usize;
    fn get_height_characters(&self) -> usize;
}