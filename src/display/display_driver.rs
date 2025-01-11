use std::{io::{self, Write}, ops::{Deref, DerefMut}};

use crossterm::{cursor, terminal};

use crate::widget::Widget;

/// Represents a display driver responsible for handling the interaction between the displays and the terminal.
pub struct DisplayDriver<T: Widget> {
    original_width: u16,
    original_height: u16,
    display: T
}

impl<T: Widget> DisplayDriver<T> {

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

impl<T: Widget> Deref for DisplayDriver<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get_widget()
    }
}

impl<T: Widget> DerefMut for DisplayDriver<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_widget_mut()
    }
}

impl<T: Widget> Drop for DisplayDriver<T> {
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
