#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![allow(clippy::unwrap_used)]

use std::{
    env::args,
    io::{
        self,
        Write,
    },
};

use console_display::{
    color::{
        RGBColor,
        TerminalColor,
    },
    display_driver::DisplayDriverOld,
    drawing::DynamicCanvas,
    pixel::{
        Pixel,
        color_pixel::ColorQuadPixel,
    },
    pixel_display::DynamicPixelDisplay,
    widget::single_widget::UvWidget,
};
use image::{
    GenericImageView,
    ImageReader,
    imageops::FilterType,
};

fn main() {
    type PixelType = ColorQuadPixel;
    #[allow(clippy::cast_possible_truncation)]
    const WIDTH: u32 = PixelType::WIDTH as u32;
    #[allow(clippy::cast_possible_truncation)]
    const HEIGHT: u32 = PixelType::HEIGHT as u32;

    #[allow(clippy::cast_possible_truncation)]
    let max_dimensions: (u32, u32) =
        (100 * PixelType::WIDTH as u32, 20 * PixelType::HEIGHT as u32);

    let path_in = args().nth(1).unwrap_or_else(|| {
        let mut temp = String::new();
        println!("Input absolute image path:");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");
        temp
    });
    let path_in = path_in.trim();

    println!("Loading image...");
    let mut img = ImageReader::open(path_in)
        .expect("File could not be read.")
        .with_guessed_format()
        .expect("Could not guess format.")
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
    let rgb = img.into_rgb8();
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

    let mut display = DisplayDriverOld::new(
        // CrtWidget::new(
        //     ScanLineWidget::new(
        UvWidget::new_with_aspect_ratio(
            DynamicPixelDisplay::<PixelType>::new(
                padded_dimensions.0 as usize,
                padded_dimensions.1 as usize,
                TerminalColor::Default,
            ),
        ),
        //         2,
        //         3,
        //         false,
        //         0.1
        //     ),
        //     TerminalColor::ARGBColor(RGBColor::BLACK.into()),
        //     0.1,
        // ),
    );

    for x in 0..padded_dimensions.0 as usize {
        for y in 0..padded_dimensions.1 as usize {
            if let Some(value) =
                data.get(x + y * padded_dimensions.0 as usize)
            {
                let _ = display.set_pixel(x as f32, y as f32, *value);
            }
        }
    }

    // dbg!(dimensions.0 as usize, dimensions.1 as usize, data.len());

    display.initialize().expect("Could not initialize display.");
    display.update().expect("Could not update display.");
}
