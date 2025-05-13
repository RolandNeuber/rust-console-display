#![feature(generic_const_exprs)]

use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use display::{console_display::PixelDisplay, display_driver::DisplayDriver, pixel::color_pixel::{self, ColorOctPixel}, widget::single_widget::NoneWidget};

fn main() {
    let disp: PixelDisplay<ColorOctPixel> = PixelDisplay::<ColorOctPixel>::build(100, 100, color_pixel::RGBColor{ r: 255, g: 0, b: 0 }).unwrap();

    let display = DisplayDriver::new(
        NoneWidget::new(
            disp
        )
    );

    display.initialize().expect("Could not initialize display.");

    loop {
        display.print_display().expect("Could not print display.");
        
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
    }
}