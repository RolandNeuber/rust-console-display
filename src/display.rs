use std::{io::{self, Write}, ops::{Deref, DerefMut}};

use crossterm::{cursor, terminal};

use crate::pixel::MultiPixel;

/// Represents a display driver responsible for handling the interaction between the displays and the terminal.
pub struct DisplayDriver<T: ConsoleDisplay> {
    original_width: u16,
    original_height: u16,
    display: T
}

impl<T: ConsoleDisplay> DisplayDriver<T> {

    /// Convenience method to build a blank display struct with specified dimensions
    pub fn new(widget: T) -> DisplayDriver<T> {
        let (original_width, original_height) = match crossterm::terminal::size(){
            Ok((w, h)) => (w, h),
            Err(_) => (0, 0)
        }; 
        
        DisplayDriver {
            original_width,
            original_height,
            display: widget
        }
    }

    pub fn print_display(&self) -> Result<(), String> {
        let mut stdout = io::stdout();
        
        if let Err(e) = write!(stdout, "\x1B[H") {
            return Err(e.to_string());
        };
        if let Err(e) = write!(stdout, "{}", self.get_widget().to_string()) {
            return Err(e.to_string());
        };

        Ok(())
    }

    pub fn initialize(&self) -> Result<(), String> {
        let mut stdout = io::stdout();

        // enables terminal raw mode
        if let Err(e) = terminal::enable_raw_mode() {
            return Err(e.to_string());
        }

        // use alternate screen
        if let Err(e) = crossterm::execute!(stdout, terminal::EnterAlternateScreen) {
            return Err(e.to_string());
        };

        // set dimensions of screen
        if let Err(e) = crossterm::execute!(stdout, terminal::SetSize(
            self.get_widget().get_width_characters() as u16, 
            self.get_widget().get_height_characters() as u16
        )) {
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

    fn get_original_width(&self) -> &u16 {
        &self.original_width
    }

    fn get_orignal_height(&self) -> &u16 {
        &self.original_height
    }

    fn get_widget(&self) -> &T {
        &self.display
    }

    fn get_widget_mut(&mut self) -> &mut T {
        &mut self.display
    }
}

impl<T: ConsoleDisplay> Deref for DisplayDriver<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get_widget()
    }
}

impl<T: ConsoleDisplay> DerefMut for DisplayDriver<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_widget_mut()
    }
}

impl<T: ConsoleDisplay> Drop for DisplayDriver<T> {
    fn drop(&mut self) {
        let mut stdout = io::stdout();

        // return to previous screen
        let _ = crossterm::execute!(stdout, terminal::LeaveAlternateScreen);

        // show cursor blinking
        let _ = crossterm::execute!(stdout, cursor::Show);
        
        // reset dimensions of screen
        if *self.get_original_width() != 0 && *self.get_orignal_height() != 0 {
            let _ = crossterm::execute!(stdout, terminal::SetSize(
                self.get_original_width().clone() as u16, 
                self.get_orignal_height().clone() as u16
            ));
        }

        // disable terminal raw mode
        let _ = terminal::disable_raw_mode();
    }
}

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