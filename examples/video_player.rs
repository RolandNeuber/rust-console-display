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
    console_display::DynamicConsoleDisplay,
    display_driver::{
        DisplayDriverOld,
        UpdateStatus,
    },
    drawing::DynamicCanvas,
    pixel::{
        Pixel,
        color_pixel::ColorOctPixel,
    },
    pixel_display::DynamicPixelDisplay,
    widget::single_widget::{
        CrtWidget,
        UvWidgetOld,
    },
};
use crossterm::event::{
    Event,
    KeyCode,
};
use video_rs::{
    Location,
    ffmpeg::{
        frame::Video,
        software::scaling::Flags,
    },
};

#[allow(clippy::too_many_lines)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
fn main() {
    type PixelType = ColorOctPixel;
    #[allow(clippy::cast_possible_truncation)]
    const WIDTH: u32 = PixelType::WIDTH as u32;
    #[allow(clippy::cast_possible_truncation)]
    const HEIGHT: u32 = PixelType::HEIGHT as u32;

    let max_dimensions: (u32, u32) =
        (50 * PixelType::WIDTH as u32, 30 * PixelType::HEIGHT as u32);

    let path_in = args().nth(1).unwrap_or_else(|| {
        let mut temp = String::new();
        println!("Input absolute video path:");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");
        temp
    });
    let path_in = path_in.trim();

    video_rs::init().unwrap();
    let mut decoder =
        video_rs::decode::Decoder::new(Location::File(path_in.into()))
            .unwrap();

    let size = decoder.size();
    let dimensions =
        if size.1 / size.0 >= max_dimensions.1 / max_dimensions.0 {
            (
                (size.0 as f32 * max_dimensions.1 as f32 / size.1 as f32 *
                    19.0 /
                    9.0 *
                    PixelType::WIDTH as f32 /
                    PixelType::HEIGHT as f32) as u32,
                max_dimensions.1,
            )
        }
        else {
            (
                max_dimensions.0,
                (size.1 as f32 * max_dimensions.0 as f32 /
                    size.0 as f32 /
                    19.0 *
                    9.0 /
                    PixelType::WIDTH as f32 *
                    PixelType::HEIGHT as f32) as u32,
            )
        };

    #[allow(clippy::modulo_one)]
    let padded_dimensions = (
        dimensions.0 + (WIDTH - dimensions.0 % WIDTH) % WIDTH,
        dimensions.1 + (HEIGHT - dimensions.1 % HEIGHT) % HEIGHT,
    );

    let mut display = DisplayDriverOld::new(CrtWidget::new(
        UvWidgetOld::new_with_aspect_ratio(
            DynamicPixelDisplay::<PixelType>::new(
                padded_dimensions.0 as usize,
                padded_dimensions.1 as usize,
                TerminalColor::Default,
            ),
        ),
        TerminalColor::Default,
        0.3,
    ));

    let mut video = decoder.decode_raw().unwrap();
    let mut video_out = Video::empty();
    let mut scaler = video
        .scaler(dimensions.0, dimensions.1, Flags::BILINEAR)
        .unwrap();

    let width = dimensions.0 as usize;
    let height = dimensions.1 as usize;

    let row_bytes = width * 3;

    let mut packed = Vec::with_capacity(row_bytes * height);
    let mut color_data: Vec<TerminalColor> =
        Vec::with_capacity(width * height);

    display.set_target_frame_rate(decoder.frame_rate());

    let mut paused = false;
    display.set_on_update(move |disp, e| {
        if let Some(Event::Key(k)) = e {
            if k.code == KeyCode::Char(' ') {
                paused ^= true;
            }
            if k.code == KeyCode::Right {
                video = decoder.decode_raw().unwrap();
                video = decoder.decode_raw().unwrap();
                video = decoder.decode_raw().unwrap();
            }
        }
        if paused {
            return UpdateStatus::Continue;
        }

        scaler.run(&video, &mut video_out).unwrap();

        let stride = video_out.stride(0);
        let data = video_out.data(0);

        packed.clear();
        for y in 0..height {
            let start = y * stride;
            let end = start + row_bytes;

            packed.extend_from_slice(&data[start..end]);
        }

        color_data.clear();
        let mut pixel_index = 0;
        for pixel in packed.chunks(3) {
            color_data.push(
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
                    color_data.push(RGBColor::BLACK.into());
                }
                pixel_index = 0;
            }
        }
        for _ in 0..padded_dimensions.1 - dimensions.1 {
            for _ in 0..padded_dimensions.0 {
                color_data.push(RGBColor::BLACK.into());
            }
        }

        let width = disp.width();
        #[allow(clippy::needless_collect)]
        for (index_x, x) in disp.x_values().enumerate().collect::<Vec<_>>()
        {
            #[allow(clippy::needless_collect)]
            for (index_y, y) in
                disp.y_values().enumerate().collect::<Vec<_>>()
            {
                disp.set_pixel(
                    x,
                    y,
                    color_data[index_x + index_y * width],
                )
                .unwrap();
            }
        }
        disp.print_display().unwrap();
        video = decoder.decode_raw().unwrap();
        UpdateStatus::Continue
    });

    display.initialize().expect("Could not initialize display.");
    display.update().expect("Could not update display.");
}
