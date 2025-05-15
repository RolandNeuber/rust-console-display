use crate::{console_display::{ConsoleDisplay, PixelDisplay}, pixel::monochrome_pixel::MultiPixel};

use super::Widget;

pub trait SingleWidget<T>: Widget {
    fn get_child(&self) -> &T;
    fn get_child_mut(&mut self) -> &mut T;
}

pub struct UvWidget<T: ConsoleDisplay> {
    child: T,
    uv_x_min: f32,
    uv_x_max: f32,
    uv_y_min: f32,
    uv_y_max: f32,
}

impl<T: ConsoleDisplay> UvWidget<T> {
    pub fn new(child: T) -> Self {
        let (width, height) = (child.get_width(), child.get_height());
        UvWidget {
            child,
            uv_x_min: 0.0,
            uv_x_max: width as f32,
            uv_y_min: 0.0,
            uv_y_max: height as f32,
        }
    }
}

impl<S: MultiPixel<S>> UvWidget<PixelDisplay<S>> {
    pub fn set_uv_x_min(&mut self, x: f32) {
       self.uv_x_min = x; 
    }

    pub fn set_uv_x_max(&mut self, x: f32) {
       self.uv_x_max = x; 
    }
    
    pub fn set_uv_y_min(&mut self, y: f32) {
       self.uv_y_min = y; 
    }
    
    pub fn set_uv_y_max(&mut self, y: f32) {
       self.uv_y_max = y; 
    }
    
    pub fn get_pixel(&self, x: f32, y: f32) -> Result<S::U, String> where [(); S::WIDTH * S::HEIGHT]: {
        let uv = (
            Self::uv_to_texture(
                x, 
                self.uv_x_min, 
                self.uv_x_max, 
                self.get_child().get_width()
            ),
            Self::uv_to_texture(
                y, 
                self.uv_y_min, 
                self.uv_y_max, 
                self.get_child().get_width()
            )
        );
        self.get_child().get_pixel(uv.0, uv.1)
    }

    pub fn set_pixel(&mut self, x: f32, y: f32, value: S::U) -> Result<(), String> where [(); S::WIDTH * S::HEIGHT]: {        
        let uv = (
            Self::uv_to_texture(
                x, 
                self.uv_x_min, 
                self.uv_x_max, 
                self.get_child().get_width()
            ),
            Self::uv_to_texture(
                y, 
                self.uv_y_min, 
                self.uv_y_max, 
                self.get_child().get_width()
            )
        );
        self.get_child_mut().set_pixel(uv.0, uv.1, value)
    }

    pub fn uv_x_to_texture(&self, x: f32) -> usize {
        Self::uv_to_texture(x, self.uv_x_min, self.uv_x_max, self.get_child().get_width())
    }

    pub fn uv_y_to_texture(&self, y: f32) -> usize {
        Self::uv_to_texture(y, self.uv_y_min, self.uv_y_max, self.get_child().get_height())
    }

    pub fn texture_to_uv_x(&self, x: usize) -> f32 {
        Self::texture_to_uv(x, self.get_child().get_width(), self.uv_x_min, self.uv_x_max)
    }

    pub fn texture_to_uv_y(&self, y: usize) -> f32 {
        Self::texture_to_uv(y, self.get_child().get_height(), self.uv_y_min, self.uv_y_max)
    }
    
    fn uv_to_texture(uv: f32, uv_min: f32, uv_max: f32, texture_coordinate_max: usize) -> usize {
        ((uv - uv_min) * (texture_coordinate_max as f32 / (uv_max - uv_min))).round() as usize
    }

    fn texture_to_uv(texture_coordinate: usize, texture_coordinate_max: usize, uv_min: f32, uv_max: f32) -> f32 {
        texture_coordinate as f32 / (texture_coordinate_max as f32 / (uv_max - uv_min)) + uv_min
    }
}

impl<T: ConsoleDisplay> Widget for UvWidget<T> {
    fn get_width_characters(&self) -> usize {
        self.get_child().get_width_characters()
    }

    fn get_height_characters(&self) -> usize {
        self.get_child().get_height_characters()
    }
}

impl<T: ConsoleDisplay> SingleWidget<T> for UvWidget<T> {
    fn get_child(&self) -> &T {
        &self.child
    }
    
    fn get_child_mut(&mut self) -> &mut T {
        &mut self.child 
    }
}

impl<T: ConsoleDisplay> ToString for UvWidget<T> {
    fn to_string(&self) -> String {
        self.get_child().to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::pixel::monochrome_pixel::SinglePixel;

    use super::*;

    #[test]
    fn test_texture_to_uv() {
        let uv = UvWidget::<PixelDisplay<SinglePixel>>::texture_to_uv(
            500, 
            1000, 
            -0.5, 
            0.5
        );
        assert_eq!(uv, 0.0);
    }

    #[test]
    fn test_uv_to_texture() {
        let texture_coordinate = UvWidget::<PixelDisplay<SinglePixel>>::uv_to_texture(
            0.5, 
            -1.0, 
            1.0, 
            2000
        );
        assert_eq!(texture_coordinate, 1500); 
    }

}