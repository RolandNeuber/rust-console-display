use std::{
    io::{
        self,
        Write,
    },
    ops::{
        Deref,
        DerefMut,
    },
    thread,
    time::{
        Duration,
        Instant,
    },
};

use crossterm::{
    cursor,
    event::{
        self,
        Event,
        KeyCode,
        KeyEvent,
        KeyModifiers,
    },
    terminal,
};

use crate::widget::DynamicWidget;

pub enum UpdateStatus {
    Break,
    Continue,
}

type UpdateFunction<T> =
    dyn FnMut(&mut DisplayDriver<T>, Option<KeyEvent>) -> UpdateStatus;

/// Represents a display driver responsible for handling the interaction between the displays and the terminal.
pub struct DisplayDriver<T: DynamicWidget> {
    original_width: u16,
    original_height: u16,
    display: T,
    on_update: Option<Box<UpdateFunction<T>>>,
    target_frame_time: Duration,
}

impl<T: DynamicWidget> DisplayDriver<T> {
    /// Convenience method to build a blank display struct with specified dimensions
    pub fn new(widget: T) -> Self {
        let (original_width, original_height) =
            match crossterm::terminal::size() {
                Ok((w, h)) => (w, h),
                Err(_) => (0, 0),
            };

        Self {
            original_width,
            original_height,
            display: widget,
            target_frame_time: Duration::ZERO,
            on_update: None,
        }
    }

    /// Prints the display to the terminal.
    ///
    /// # Errors
    ///
    /// May return an error if write! is unsuccessful.
    pub fn print_display(&self) -> Result<(), io::Error> {
        let mut stdout = io::stdout();

        write!(stdout, "\x1B[H")?;
        write!(stdout, "{}", self.get_widget())?;

        Ok(())
    }

    /// Initializes the display driver.
    /// This function enables terminal raw mode and
    /// sets the dimensions of the screen to match the widget's dimensions.
    /// It enters alternate screen mode,
    /// hides the cursor and disables line wrapping.
    ///
    /// # Error
    ///
    /// Returns an error when any on the actions above fail.
    /// Note that resizing the terminal does not fail, if the terminal does not support it.
    pub fn initialize(&self) -> Result<(), io::Error> {
        let mut stdout = io::stdout();

        // enables terminal raw mode
        terminal::enable_raw_mode()?;

        crossterm::execute!(
            stdout,
            terminal::EnterAlternateScreen, // use alternate screen
            terminal::SetSize(
                self.get_widget().get_width_characters() as u16,
                self.get_widget().get_height_characters() as u16
            ), // set dimensions of screen
            terminal::DisableLineWrap,      // disable line wrapping
            terminal::Clear(terminal::ClearType::All), // clear screen
            cursor::Hide,                   // hide cursor blinking
        )?;

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
    where
        F: FnMut(&mut Self, Option<KeyEvent>) -> UpdateStatus + 'static,
    {
        self.on_update = Some(Box::new(on_update));
    }

    pub const fn set_target_frame_time(&mut self, frame_time: Duration) {
        self.target_frame_time = frame_time;
    }

    pub fn set_target_frame_rate(&mut self, frame_rate: f32) {
        self.target_frame_time = Duration::from_secs_f32(1. / frame_rate);
    }

    /// This function encapsulates the update loop of the display.
    /// As such it may or may not return depending on the update callback set with
    /// `set_on_update`.
    /// This function prints the display.
    /// Queries user input and exits on Ctrl-C.
    /// Forwards keystrokes to the provided callback and invokes it.
    /// Sleeps so the target frame rate is not exceeded.
    ///
    /// # Panics
    ///
    /// This function panics when the display could not be printed.
    pub fn update(&mut self) {
        loop {
            let start = Instant::now();
            self.print_display().expect("Could not print display.");

            let mut latest_event = None;
            while event::poll(Duration::from_millis(0)).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    latest_event = Some(key_event);
                }
            }

            if let Some(key_event) = latest_event &&
                key_event.code == KeyCode::Char('c') &&
                key_event.modifiers.contains(KeyModifiers::CONTROL)
            {
                break; // Exit on Ctrl-C
            }

            let mut update_status = UpdateStatus::Continue;
            if let Some(mut callback) = self.on_update.take() {
                update_status = callback(self, latest_event);
                self.on_update = Some(callback);
            }
            match update_status {
                UpdateStatus::Break => break,
                UpdateStatus::Continue => {}
            }

            thread::sleep(
                self.target_frame_time.saturating_sub(start.elapsed()),
            );
        }
    }
}

impl<T: DynamicWidget> Deref for DisplayDriver<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get_widget()
    }
}

impl<T: DynamicWidget> DerefMut for DisplayDriver<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_widget_mut()
    }
}

impl<T: DynamicWidget> Drop for DisplayDriver<T> {
    fn drop(&mut self) {
        let mut stdout = io::stdout();

        let _ = crossterm::execute!(
            stdout,
            terminal::EnableLineWrap, // disable line wrapping
            terminal::LeaveAlternateScreen, // return to previous screen
            cursor::Show,             // show cursor blinking
        );

        // reset dimensions of screen
        if *self.get_original_width() != 0 &&
            *self.get_orignal_height() != 0
        {
            let _ = crossterm::execute!(
                stdout,
                terminal::SetSize(
                    *self.get_original_width(),
                    *self.get_orignal_height()
                )
            );
        }

        // disable terminal raw mode
        let _ = terminal::disable_raw_mode();
    }
}
