#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::{io::{self, Write}, time::Duration};

use crossterm::event::{
    self, 
    Event, 
    KeyCode, 
    KeyModifiers
};
use display::{
    console_display::PixelDisplay, 
    display_driver::DisplayDriver, 
    pixel::{
        color_pixel::{
            RGBColor, 
            ColorOctPixel
        }, 
        monochrome_pixel::MultiPixel
    }
};
use image::{
    imageops::FilterType, 
    GenericImageView, 
    ImageReader
};

fn main() {
    let max_dimensions: (u32, u32) = (200, 160);
    type PixelType = ColorOctPixel;

    let mut path_in = String::new();
    
    println!("Input absolute image path:");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut path_in)
        .expect("Failed to read line");
    let path_in = path_in.trim();

    println!("Loading image...");
    let mut img = 
        ImageReader::open(path_in).expect("File could not be read.")
        .decode().expect("Image could not be decoded.");
    img = img.resize(max_dimensions.0, max_dimensions.1, FilterType::Gaussian);
    
    let dimensions = img.dimensions();
    let padded_dimensions = (
        dimensions.0 + (PixelType::WIDTH as u32 - dimensions.0 % PixelType::WIDTH as u32) % PixelType::WIDTH as u32, 
        dimensions.1 + (PixelType::HEIGHT as u32 - dimensions.1 % PixelType::HEIGHT as u32) % PixelType::HEIGHT as u32
    );
    let rgb = img.as_rgb8().expect("Could not extract rgb data.");
    let mut data = Vec::with_capacity((dimensions.0 * dimensions.1) as usize);
    let mut pixel_index = 0;
    for pixel in rgb.pixels() {
        data.push(RGBColor {
            r: pixel[0],
            g: pixel[1],
            b: pixel[2]
        });
        pixel_index += 1;
        if pixel_index == dimensions.0 && padded_dimensions.0 > dimensions.0 {
            for _ in 0..padded_dimensions.0 - dimensions.0 {
                data.push(RGBColor {
                    r: 0,
                    g: 0,
                    b: 0
                });
            }
            pixel_index = 0;
        }
    }
    for _ in 0..padded_dimensions.1-dimensions.1 {
        for _ in 0..padded_dimensions.0 {
            data.push(RGBColor {
                r: 0,
                g: 0,
                b: 0
            })
        }
    }
    
    let display = 
    DisplayDriver::new(
        PixelDisplay::<PixelType>::build_from_data(
            padded_dimensions.0 as usize, 
            padded_dimensions.1 as usize, 
            data
        ).expect("Could not construct display.")
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