use std::io::{self, Write};

use crossterm::{cursor, terminal};

use crate::pixel::MultiPixel;

/// Represents a console display with a width and height in pixels.
pub struct Display<T: MultiPixel<T>> {
    pub width: usize,
    pub height: usize,
    block_count_x: usize,
    block_count_y: usize,
    data: Vec<T>,
}

impl<T: MultiPixel<T>> Display<T> {

    /// Convenience method to build a blank display struct with specified dimensions
    pub fn build(width: usize, height: usize) -> Result<Display<T>, String> {
        let data = vec![false; width * height];
        Self::build_from_bools(width, height, data)
    }

    /// Builds a display struct with the specified dimensions from the given data.
    pub fn build_from_bools(width: usize, height: usize, data: Vec<bool>) -> Result<Display<T>, String> {
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

                let mut args = Vec::with_capacity(T::WIDTH * T::HEIGHT);

                for y in 0..T::HEIGHT {
                    for x in 0..T::WIDTH {
                        args.push(data[block_x + x + (block_y + y) * width]);
                    }
                }

                multi_pixels.push(T::build(&args)?);
            }
        }
        Ok(Display {
            width, 
            height, 
            block_count_x,
            block_count_y,
            data: multi_pixels
        })
    }

    /// Returns a bool representing the state of the pixel at the specified coordinate.
    /// Returns an error if the coordinates are out of bounds.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use display::{Display, pixel::SinglePixel};
    /// let disp: Display<SinglePixel> = Display::build_from_bools(
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
    pub fn get_pixel(&self, x: usize, y: usize) -> Result<bool, String> {
        if x >= self.width || y >= self.height {
            return Err(format!("Pixel coordinates out of bounds. Got x = {}, y = {}.", x, y))
        }

        let block_x: usize = x / T::WIDTH;
        let block_y: usize = y / T::HEIGHT;
        let offset_x: usize = x % T::WIDTH;
        let offset_y: usize = y % T::HEIGHT;

        let pixel = &self.data[block_x + block_y * self.block_count_x];
        match pixel.get_subpixel(offset_x, offset_y) {
            Ok(val) => Ok(val),
            Err(_) => Err("Offset should be 0 or 1.".to_string()),
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) -> Result<(), String> {
        if x >= self.width || y >= self.height {
            return Err(format!("Pixel coordinates out of bounds. Got x = {}, y = {}.", x, y))
        }

        let block_x: usize = x / T::WIDTH;
        let block_y: usize = y / T::HEIGHT;
        let offset_x: usize = x % T::WIDTH;
        let offset_y: usize = y % T::HEIGHT;

        let pixel = &mut self.data[block_x + block_y * self.block_count_x];
        match pixel.set_subpixel(offset_x, offset_y, value) {
            Ok(val) => Ok(val),
            Err(_) => Err("Offset should be 0 or 1.".to_string()),
        }
    }

    pub fn print_display(&self) -> Result<(), String> {
        let mut stdout = io::stdout();
        
        if let Err(e) = write!(stdout, "\x1B[H") {
            return Err(e.to_string());
        };
        if let Err(e) = write!(stdout, "{}", self.to_string()) {
            return Err(e.to_string());
        };

        Ok(())
    }

    pub fn initialize(&self) -> Result<(), String> {
        let mut stdout = io::stdout();

        // use alternate screen
        if let Err(e) = crossterm::execute!(stdout, terminal::EnterAlternateScreen) {
            return Err(e.to_string());
        };

        // set dimensions of screen
        if let Err(e) = crossterm::execute!(stdout, terminal::SetSize(self.block_count_x as u16, self.block_count_y as u16)) {
            return Err(e.to_string());
        };
        
        // clear screen
        if let Err(e) = crossterm::execute!(stdout, terminal::Clear(terminal::ClearType::All)) {
            return Err(e.to_string());
        };

        // hide cursor blinking
        if let Err(e) = crossterm::execute!(stdout, cursor::Hide) {
            return Err(e.to_string());
        };

        Ok(())
    }
}

impl<T: MultiPixel<T>> ToString for Display<T> {
    fn to_string(&self) -> String {
        let mut string_repr = String::new();
        for y in 0..self.block_count_y {
            for x in 0..self.block_count_x {
                string_repr.push(self.data[x + y * self.block_count_x].get_char());
            }
            string_repr.push_str("\r\n");
        }
        string_repr.pop();
        string_repr
    }
}

impl<T: MultiPixel<T>> Drop for Display<T> {
    fn drop(&mut self) {
        let mut stdout = io::stdout();

        // return to previous screen
        let _ = crossterm::execute!(stdout, terminal::LeaveAlternateScreen);

        // show cursor blinking
        let _ = crossterm::execute!(stdout, cursor::Show);
    }
}