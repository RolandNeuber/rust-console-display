pub mod get_pixel_static;
pub mod height;
pub mod set_pixel_static;
pub mod width;

pub use get_pixel_static::*;
pub use height::*;
pub use set_pixel_static::*;
pub use width::*;

#[cfg(test)]
mod tests {
    use crate::{
        pixel::monochrome::SinglePixel,
        pixel_display::StaticPixelDisplay,
        traits::{
            GetPixelStatic,
            SetPixelStatic,
        },
    };

    #[test]
    fn pixel_static() {
        let mut display =
            StaticPixelDisplay::<SinglePixel, 10, 10>::new(true);
        display.set_pixel_static::<2, 4>(false);
        assert!(!display.pixel_static::<2, 4>());
        assert!(display.pixel_static::<1, 3>());
        assert!(display.pixel_static::<0, 0>());
    }
}
