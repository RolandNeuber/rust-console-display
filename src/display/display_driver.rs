use std::{
    io::{
        self, 
        Write
    }, 
    ops::{
        Deref, 
        DerefMut
    }, 
    time::Duration
};

use crossterm::{
    cursor, 
    event::{
        self, 
        Event, 
        KeyCode, 
        KeyEvent, 
        KeyModifiers
    }, 
    terminal
};

use crate::widget::Widget;

pub enum UpdateStatus {
    Break,
    Continue,
}

type UpdateFunction<T> = dyn FnMut(&mut DisplayDriver<T>, Option<KeyEvent>) -> UpdateStatus;

/// Represents a display driver responsible for handling the interaction between the displays and the terminal.
pub struct DisplayDriver<T: Widget> {
    original_width: u16,
    original_height: u16,
    display: T,
    on_update: Option<Box<UpdateFunction<T>>>
}

impl<T: Widget> DisplayDriver<T> {

    /// Convenience method to build a blank display struct with specified dimensions
    pub fn new(widget: T) -> Self {
        let (original_width, original_height) = match crossterm::terminal::size(){
            Ok((w, h)) => (w, h),
            Err(_) => (0, 0)
        }; 
        
        Self {
            original_width,
            original_height,
            display: widget,
            on_update: None
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

    const fn get_original_width(&self) -> &u16 {
        &self.original_width
    }

    const fn get_orignal_height(&self) -> &u16 {
        &self.original_height
    }

    const fn get_widget(&self) -> &T {
        &self.display
    }

    const fn get_widget_mut(&mut self) -> &mut T {
        &mut self.display
    }

    pub fn set_on_update<F>(&mut self, on_update: F)
        where F: FnMut(&mut Self, Option<KeyEvent>) -> UpdateStatus + 'static
    {
        self.on_update = Some(Box::new(on_update));
    }

    pub fn update(&mut self) {
        loop {
            self.print_display().expect("Could not print display.");
            
            let mut latest_event = None;
            while event::poll(Duration::from_millis(0)).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    latest_event = Some(key_event);
                }
            }

            if let Some(key_event) = latest_event {
                if 
                    key_event.code == KeyCode::Char('c') && 
                    key_event.modifiers.contains(KeyModifiers::CONTROL)
                {
                    break; // Exit on Ctrl-C
                }
            }

            let mut update_status = UpdateStatus::Continue;
            if let Some(mut callback) = self.on_update.take() {
                update_status = callback(self, latest_event);
                self.on_update = Some(callback);
            }
            match update_status {
                UpdateStatus::Break => break,
                UpdateStatus::Continue => { },
            }
        }
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
                *self.get_original_width(), 
                *self.get_orignal_height()
            ));
        }

        // disable terminal raw mode
        let _ = terminal::disable_raw_mode();
    }
}
