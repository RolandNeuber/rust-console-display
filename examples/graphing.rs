#![feature(generic_const_exprs)]

use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use display::color_pixel::{Color, ColorOctPixel};
use display::widget::UvWidget;
use display::{DisplayDriver, PixelDisplay};

fn main() {
    let dimensions: (usize, usize) = (200, 200);
    type PixelType = ColorOctPixel;
    let function = |x: f32| { x };

    let uv_x = (-1.0, 1.0);
    let uv_y = (1.0, -1.0);
    
    let mut display = 
    DisplayDriver::new(
        UvWidget::new(
            PixelDisplay::<PixelType>::build(
                dimensions.0 as usize, 
                dimensions.1 as usize,
                Color { r: 0, g: 0, b: 0 }
            ).expect("Could not construct display.")
        )
    );
    display.set_uv_x_min(uv_x.0);
    display.set_uv_x_max(uv_x.1);
    display.set_uv_y_min(uv_y.0);
    display.set_uv_y_max(uv_y.1);

    display.initialize().expect("Could not initialize display.");
    loop {
        for x_text in 0..dimensions.0 {
            let x = display.texture_to_uv_x(x_text);

            let y = function(x);

            display.set_pixel(x, y, Color{ r: 255, g: 255, b: 255 });
        }

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