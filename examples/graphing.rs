#![feature(generic_const_exprs)]

use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use display::color_pixel::{Color, ColorOctPixel};
use display::widget::{NoneWidget, SingleWidget};
use display::{DisplayDriver, PixelDisplay};

fn main() {
    let dimensions: (u32, u32) = (200, 200);
    type PixelType = ColorOctPixel;
    let function = |x: f32| { x };

    let uv_x = (-1.0, 1.0);
    let uv_y = (1.0, -1.0);
    
    let mut display = 
    DisplayDriver::new(
        NoneWidget::new(
            PixelDisplay::<PixelType>::build(
                dimensions.0 as usize, 
                dimensions.1 as usize,
                Color { r: 0, g: 0, b: 0 }
            ).expect("Could not construct display.")
        )
    );

    display.initialize().expect("Could not initialize display.");
    loop {
        for x_text in 0..dimensions.0 {
            let x = texture_to_uv(
                x_text, 
                dimensions.0, 
                uv_x.0, 
                uv_x.1
            );

            let y = function(x);

            let y_text = uv_to_texture(
                y, 
                uv_y.0, 
                uv_y.1, 
                dimensions.1
            );

            display.get_child_mut().set_pixel(x_text as usize, y_text as usize, Color{ r: 255, g: 255, b: 255 });
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

fn texture_to_uv(texture_coordinate: u32, texture_coordinate_max: u32, uv_min: f32, uv_max: f32) -> f32 {
    texture_coordinate as f32 / (texture_coordinate_max as f32 / (uv_max - uv_min)) + uv_min
}

fn uv_to_texture(uv: f32, uv_min: f32, uv_max: f32, texture_coordinate_max: u32) -> u32 {
    ((uv - uv_min) * (texture_coordinate_max as f32 / (uv_max - uv_min))).round() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_to_uv() {
        let uv = texture_to_uv(
            500, 
            1000, 
            -0.5, 
            0.5
        );
        assert_eq!(uv, 0.0);
    }

    #[test]
    fn test_uv_to_texture() {
        let texture_coordinate = uv_to_texture(
            0.5, 
            -1.0, 
            1.0, 
            2000
        );
        assert_eq!(texture_coordinate, 1500); 
    }

}