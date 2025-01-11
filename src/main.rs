use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use display::{console_display::CharacterDisplay, display_driver::DisplayDriver, pixel::{self, character_pixel::CharacterPixel, color_pixel::{Color, RGBColor}}, widget::single_widget::NoneWidget};

fn main() {
    let mut char_disp = CharacterDisplay::build(
        40, 
        20, 
        CharacterPixel::build(
            ' ',
            Color::Default,
            Color::Default,
        ).unwrap()
    ).unwrap();

    let mut x = 0;
    for i in "Hallo World".chars() {
        let pixel = CharacterPixel::build( 
            i,
            Color::Default,
            Color::Default,
        ).unwrap();
        char_disp.set_pixel (
            x, 
            0, 
            pixel.clone()
        );
        x += pixel.get_width();
    }

    let display = DisplayDriver::new(
        NoneWidget::new(
            char_disp
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