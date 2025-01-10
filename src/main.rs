use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use display::{console_display::CharacterDisplay, display_driver::DisplayDriver, pixel::{character_pixel::CharacterPixel, color_pixel::Color}, widget::single_widget::NoneWidget};

fn main() {
    let mut char_disp = CharacterDisplay::build(
        40, 
        20, 
        CharacterPixel::build(
            'A',
            Color {
                r: 255,
                g: 255,
                b: 255
            },
            Color {
                r: 0,
                g: 0,
                b: 0
            }
        ).unwrap()
    ).unwrap();

    char_disp.set_pixel(
        20, 
        10, 
        CharacterPixel::build( 
            'あ',
            Color {
                r: 255,
                g: 255,
                b: 255
            },
            Color {
                r: 0,
                g: 0,
                b: 0
            },
        ).unwrap()
    );


    char_disp.set_pixel(
        23, 
        10, 
        CharacterPixel::build( 
            'あ',
            Color {
                r: 255,
                g: 255,
                b: 255
            },
            Color {
                r: 0,
                g: 0,
                b: 0
            },
        ).unwrap()
    );

    // println!("{:#?}", char_disp.get_data());
    // panic!();

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