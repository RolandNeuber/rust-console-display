#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::io::{
    self,
    Write,
};

use console_display::{
    color::RGBColor,
    display_driver::DisplayDriver,
    pixel::{
        Pixel,
        color_pixel::ColorOctPixel,
    },
    pixel_display::DynamicPixelDisplay,
};
use image::{
    GenericImageView,
    ImageReader,
    imageops::FilterType,
};

fn main() {
    type PixelType = ColorOctPixel;
    #[allow(clippy::cast_possible_truncation)]
    const WIDTH: u32 = PixelType::WIDTH as u32;
    #[allow(clippy::cast_possible_truncation)]
    const HEIGHT: u32 = PixelType::HEIGHT as u32;

    let max_dimensions: (u32, u32) = (200, 160);

    let mut path_in = String::new();

    println!("Input absolute image path:");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut path_in)
        .expect("Failed to read line");
    let path_in = path_in.trim();

    println!("Loading image...");
    let mut img = ImageReader::open(path_in)
        .expect("File could not be read.")
        .decode()
        .expect("Image could not be decoded.");
    img = img.resize(
        max_dimensions.0,
        max_dimensions.1,
        FilterType::Gaussian,
    );

    let dimensions = img.dimensions();
    let padded_dimensions = (
        dimensions.0 + (WIDTH - dimensions.0 % WIDTH) % WIDTH,
        dimensions.1 + (HEIGHT - dimensions.1 % HEIGHT) % HEIGHT,
    );
    let rgb = img.as_rgb8().expect("Could not extract rgb data.");
    let mut data =
        Vec::with_capacity((dimensions.0 * dimensions.1) as usize);
    let mut pixel_index = 0;
    for pixel in rgb.pixels() {
        data.push(
            RGBColor {
                r: pixel[0],
                g: pixel[1],
                b: pixel[2],
            }
            .into(),
        );
        pixel_index += 1;
        if pixel_index == dimensions.0 &&
            padded_dimensions.0 > dimensions.0
        {
            for _ in 0..padded_dimensions.0 - dimensions.0 {
                data.push(RGBColor::BLACK.into());
            }
            pixel_index = 0;
        }
    }
    for _ in 0..padded_dimensions.1 - dimensions.1 {
        for _ in 0..padded_dimensions.0 {
            data.push(RGBColor::BLACK.into());
        }
    }

    let mut display = DisplayDriver::new(
        DynamicPixelDisplay::<PixelType>::build_from_data(
            padded_dimensions.0 as usize,
            padded_dimensions.1 as usize,
            &data,
        )
        .expect("Could not construct display."),
    );

    display.initialize().expect("Could not initialize display.");
    display.update();
}
