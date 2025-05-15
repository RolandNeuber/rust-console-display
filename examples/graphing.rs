#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::time::Duration;
use crossterm::event::{
    self, 
    Event, 
    KeyCode, 
    KeyModifiers
};
use display::{
    console_display::PixelDisplay, 
    display_driver::DisplayDriver, 
    pixel::color_pixel::{
        RGBColor, 
        ColorOctPixel
    }, 
    widget::single_widget::UvWidget
};


fn main() {
    let dimensions: (usize, usize) = (200, 200);
    type PixelType = ColorOctPixel;
    let function = |x: f32| { (x * x).sin() };

    let uv_x = (-10.0, 10.0);
    let uv_y = (2.0, -2.0);
    
    let mut display = 
    DisplayDriver::new(
        UvWidget::new(
            PixelDisplay::<PixelType>::build(
                dimensions.0 as usize, 
                dimensions.1 as usize,
                RGBColor { r: 0, g: 0, b: 0 }
            ).expect("Could not construct display.")
        )
    );
    display.set_uv_x_min(uv_x.0);
    display.set_uv_x_max(uv_x.1);
    display.set_uv_y_min(uv_y.0);
    display.set_uv_y_max(uv_y.1);

    display.initialize().expect("Could not initialize display.");
    loop {
        let mut xs = display.get_x_values().collect::<Vec<_>>().into_iter();
        let mut old_x = xs.next().unwrap();
        let mut old_y = function(old_x);
        for x in xs {
            let y = function(x);
            
            let _ = display.draw_line(old_x, old_y, x, y, RGBColor{ r: 255, g: 255, b: 255 });

            old_x = x;
            old_y = y;
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