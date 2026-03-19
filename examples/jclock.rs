#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::marker::PhantomData;

use chrono::{
    Local,
    Timelike,
};
use console_display::{
    color::{
        RGBColor,
        TerminalColor,
    },
    display_driver::{
        DisplayDriver,
        UpdateStatus,
    },
    drawing::{
        DynamicCanvas,
        Filled,
        Line,
        Rectangle,
    },
    pixel::color_pixel::ColorOctPixel,
    pixel_display::StaticPixelDisplay,
};

const WIDTH: usize = 200;
const HEIGHT: usize = 128;
type TuiDisplay = StaticPixelDisplay<ColorOctPixel, WIDTH, HEIGHT>;

fn main() {
    let disp = TuiDisplay::new(RGBColor::GRAY.into());

    let mut display = DisplayDriver::new(disp);

    display.initialize().expect("Could not initialize display.");
    display.set_target_frame_rate(10.);
    display.set_on_update(move |disp, _| {
        let now = Local::now();
        #[allow(clippy::cast_possible_truncation)]
        let hour = now.hour() as u8;
        #[allow(clippy::cast_possible_truncation)]
        let minute = now.minute() as u8;

        let digits = [hour / 10, hour % 10, minute / 10, minute % 10];

        let padding = 2;
        let segment_width = WIDTH / 4 - padding * 2;
        let segment_height = HEIGHT - padding * 2;

        disp.draw(
            &Rectangle {
                x1: 0.,
                y1: 0.,
                x2: WIDTH as f32 - 1.,
                y2: HEIGHT as f32 - 1.,
                fill: PhantomData::<Filled>,
            },
            RGBColor::GRAY.into(),
        );

        for (i, digit) in digits.iter().enumerate() {
            Segments::from_digit(*digit).draw_segments(
                disp,
                (segment_width + 2 * padding) * i + padding,
                padding,
                segment_width,
                segment_height,
                RGBColor::BLACK.into(),
            );
        }

        UpdateStatus::Continue
    });

    display.update().expect("Could not update display.");
}

// +--- 0 ---+
// |         |
// 1         2
// |         |
// +--- 3 ----
// |         |
// 4         5
// |         |
// +----6-----
struct Segments {
    inner: u8,
}

impl Segments {
    fn new(segments: [bool; 7]) -> Self {
        Self {
            inner: u8::from(segments[0]) +
                (u8::from(segments[1]) << 1) +
                (u8::from(segments[2]) << 2) +
                (u8::from(segments[3]) << 3) +
                (u8::from(segments[4]) << 4) +
                (u8::from(segments[5]) << 5) +
                (u8::from(segments[6]) << 6),
        }
    }

    fn from_digit(digit: u8) -> Self {
        Self::new(match digit {
            0 => [true, true, true, false, true, true, true],
            1 => [false, false, true, false, false, true, false],
            2 => [true, false, true, true, true, false, true],
            3 => [true, false, true, true, false, true, true],
            4 => [false, true, true, true, false, true, false],
            5 => [true, true, false, true, false, true, true],
            6 => [true, true, false, true, true, true, true],
            7 => [true, false, true, false, false, true, false],
            8 => [true, true, true, true, true, true, true],
            9 => [true, true, true, true, false, true, true],
            _ => panic!("{digit} is not a digit"),
        })
    }

    const fn is_set(&self, index: u8) -> bool {
        self.inner & (1 << index) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    fn draw_segments(
        &self,
        display: &mut TuiDisplay,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: TerminalColor,
    ) {
        // Horizontal segments
        for i in 0..=2 {
            if self.is_set(i * 3) {
                display.draw(
                    &Line {
                        x1: x as f32,
                        y1: (y + height / 2 * i as usize) as f32,
                        x2: (x + width) as f32,
                        y2: (y + height / 2 * i as usize) as f32,
                    },
                    color,
                );
            }
        }

        // Vertical segments
        for i in 0..=1 {
            // left
            if self.is_set(1 + 3 * i) {
                display.draw(
                    &Line {
                        x1: x as f32,
                        y1: (y + height / 2 * i as usize) as f32,
                        x2: x as f32,
                        y2: (y + height / 2 * (i as usize + 1)) as f32,
                    },
                    color,
                );
            }

            // right
            if self.is_set(2 + 3 * i) {
                display.draw(
                    &Line {
                        x1: (x + width) as f32,
                        y1: (y + height / 2 * i as usize) as f32,
                        x2: (x + width) as f32,
                        y2: (y + height / 2 * (i as usize + 1)) as f32,
                    },
                    color,
                );
            }
        }
    }
}
