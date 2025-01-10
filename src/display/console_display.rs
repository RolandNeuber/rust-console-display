use crate::pixel::{character_pixel::CharacterPixel, monochrome_pixel::MultiPixel};

pub trait ConsoleDisplay: ToString {
    /// Returns the width of the display in a display specific unit (e.g. pixels).
    fn get_width(&self) -> usize;
    /// Returns the height of the display in a display specific unit (e.g. pixels).
    fn get_height(&self) -> usize;
    /// Returns the width of the display in characters.
    fn get_width_characters(&self) -> usize;
    /// Returns the height of the display in characters.
    fn get_height_characters(&self) -> usize;
}

/// Represents a console display with a width and height in pixels.
pub struct PixelDisplay<T: MultiPixel<T>> {
    data: Vec<T>,
    width: usize,
    height: usize,
    block_count_x: usize,
    block_count_y: usize,
}

impl<T: MultiPixel<T>> ConsoleDisplay for PixelDisplay<T> {
    fn get_width_characters(&self) -> usize {
        self.block_count_x
    }
    
    fn get_height_characters(&self) -> usize {
        self.block_count_y
    }
    
    fn get_width(&self) -> usize {
        self.width
    }
    
    fn get_height(&self) -> usize {
        self.height
    }
}

impl<T: MultiPixel<T>> PixelDisplay<T> {
    /// Convenience method to build a blank display struct with specified dimensions
    pub fn build(width: usize, height: usize, fill: T::U) -> Result<PixelDisplay<T>, String> where [(); T::WIDTH * T::HEIGHT]: {
        let data: Vec<T::U> = vec![fill; width * height];
        Self::build_from_data(width, height, data)
    }

    /// Builds a display struct with the specified dimensions from the given data.
    pub fn build_from_data(width: usize, height: usize, data: Vec<T::U>) -> Result<PixelDisplay<T>, String> where [(); T::WIDTH * T::HEIGHT]: {
        if width % T::WIDTH != 0 || height % T::HEIGHT != 0 {
            return Err(
                format!(
                    "Width and height must be multiples of multipixel dimensions. Got width = {}, height = {}.", 
                    width, 
                    height
                )
            );
        }
        if data.len() / width / height != 1 {
            return Err(
                format!(
                    "Data does not match specified dimensions. Expected length of {}, got {}.", 
                    width * height, 
                    data.len()
                )
            );
        }

        let block_count_x = width / T::WIDTH;
        let block_count_y = height / T::HEIGHT;

        let mut multi_pixels = Vec::with_capacity(block_count_x * block_count_y);

        for row in 0..block_count_y {
            for col in 0..block_count_x {
                let block_x: usize = col * T::WIDTH;
                let block_y: usize = row * T::HEIGHT;

                let mut args: Vec<T::U> = Vec::with_capacity(T::WIDTH * T::HEIGHT);

                for y in 0..T::HEIGHT {
                    for x in 0..T::WIDTH {
                        args.push(data[block_x + x + (block_y + y) * width]);
                    }
                }

                multi_pixels.push(T::build(&args)?);
            }
        }

        Ok(PixelDisplay {
            width, 
            height, 
            block_count_x,
            block_count_y,
            data: multi_pixels,
        })
    }


    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    fn get_data_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    pub fn get_block_count_x(&self) -> &usize {
        &self.block_count_x
    }

    pub fn get_block_count_y(&self) -> &usize {
        &self.block_count_y
    }

    /// Returns a bool representing the state of the pixel at the specified coordinate.
    /// Returns an error if the coordinates are out of bounds.
    /// 
    /// # Examples
    /// 
    /// ```
    /// #![feature(generic_const_exprs)]
    /// 
    /// use display::{DisplayDriver, pixel::SinglePixel};
    /// 
    /// let disp: DisplayDriver<SinglePixel> = DisplayDriver::build_from_bools(
    ///     6, 
    ///     6, 
    ///     vec![
    ///         true, true, true, true,  true, true, // 0
    ///         true, true, true, true,  true, true, // 1
    ///         true, true, true, false, true, true, //-2-
    ///         true, true, true, true,  true, true, // 3
    ///         true, true, true, true,  true, true, // 4
    ///         true, true, true, true,  true, true, // 5
    ///     ] //  0     1     2   --3--    4     5
    /// ).expect("Dimensions of data should match the passed witdh and height");
    /// // Replace with actual error handling
    /// 
    /// let pixel = disp.get_pixel(3, 2);
    /// 
    /// assert_eq!(pixel, Ok(false));
    /// 
    /// let pixel = disp.get_pixel(5, 6);
    /// 
    /// assert!(matches!(pixel, Err(_)));
    /// ```
    pub fn get_pixel(&self, x: usize, y: usize) -> Result<T::U, String> where [(); T::WIDTH * T::HEIGHT]: {
        if x >= self.get_width() || y >= self.get_height() {
            return Err(format!("Pixel coordinates out of bounds. Got x = {}, y = {}.", x, y))
        }

        let block_x: usize = x / T::WIDTH;
        let block_y: usize = y / T::HEIGHT;
        let offset_x: usize = x % T::WIDTH;
        let offset_y: usize = y % T::HEIGHT;

        let pixel = &self.get_data()[block_x + block_y * self.get_block_count_x()];
        match pixel.get_subpixel(offset_x, offset_y) {
            Ok(val) => Ok(val),
            Err(_) => Err("Offset should be 0 or 1.".to_string()),
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: T::U) -> Result<(), String> where [(); T::WIDTH * T::HEIGHT]: {
        if x >= self.get_width() || y >= self.get_height() {
            return Err(format!("Pixel coordinates out of bounds. Got x = {}, y = {}.", x, y))
        }

        let block_x: usize = x / T::WIDTH;
        let block_y: usize = y / T::HEIGHT;
        let offset_x: usize = x % T::WIDTH;
        let offset_y: usize = y % T::HEIGHT;

        let block_count_x = *self.get_block_count_x();

        let pixel = &mut self.get_data_mut()[block_x + block_y * block_count_x];
        match pixel.set_subpixel(offset_x, offset_y, value) {
            Ok(val) => Ok(val),
            Err(_) => Err("Offset should be 0 or 1.".to_string()),
        }
    }
}

impl<T: MultiPixel<T>> ToString for PixelDisplay<T> {
    fn to_string(&self) -> String {
        let mut string_repr = String::new();
        for y in 0..*self.get_block_count_y() {
            for x in 0..*self.get_block_count_x() {
                string_repr.push_str(
                    self.get_data()[x + y * self.get_block_count_x()]
                    .to_string()
                    .as_str()
                );
            }
            string_repr.push_str("\r\n");
        }
        string_repr.pop();
        string_repr
    }
}

struct CharacterDisplay<CharacterPixel> {
    data: Vec<CharacterPixel>,
    width: usize,
    height: usize,
}

impl CharacterDisplay<CharacterPixel> {
    fn get_data(&self) -> &Vec<CharacterPixel> {
        &self.data
    }
}

impl ConsoleDisplay for CharacterDisplay<CharacterPixel> {
    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn get_width_characters(&self) -> usize {
        self.width
    }

    fn get_height_characters(&self) -> usize {
        self.height
    }
}

impl ToString for CharacterDisplay<CharacterPixel> {
    fn to_string(&self) -> String {
        let mut string_repr = String::new();
        for y in 0..self.get_height_characters() {
            for x in 0..self.get_width_characters() {
                string_repr.push_str(
                    self.get_data()[x + y * self.get_width_characters()]
                    .to_string()
                    .as_str()
                );
            }
            string_repr.push_str("\r\n");
        }
        string_repr.pop();
        string_repr
    }
}