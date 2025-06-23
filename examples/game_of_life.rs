#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::array;

use console_display::{
    console_display::ConsoleDisplay,
    display_driver::{
        DisplayDriver,
        UpdateStatus,
    },
    pixel::monochrome_pixel::OctPixel,
    pixel_display::StaticPixelDisplay,
    widget::single_widget::DoubleBufferWidget,
};
use rand::{
    Rng,
    thread_rng,
};

fn main() {
    let disp =
        DoubleBufferWidget::new(
            StaticPixelDisplay::<OctPixel, 200, 100>::new_from_data(
                &array::from_fn::<_, 20_000, _>(|_| {
                    let rng = thread_rng().gen_range(0..=1);
                    rng != 0
                }),
            ),
        );

    let mut display = DisplayDriver::new(disp);

    let offsets = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    display.set_target_frame_rate(30.);
    display.set_on_update(move |disp, _| {
        let width = disp.get_width() as i32;
        let height = disp.get_height() as i32;
        for x in 0..width {
            for y in 0..height {
                let mut neighbors = 0;
                for i in offsets {
                    let x_i = (x + i.0).rem_euclid(width) as usize;
                    let y_i = (y + i.1).rem_euclid(height) as usize;
                    if disp
                        .get_pixel(x_i, y_i)
                        .expect("Could not get pixel.")
                    {
                        neighbors += 1;
                    }
                }
                let pixel = disp
                    .get_pixel(x as usize, y as usize)
                    .expect("Could not get pixel.");

                let _ = disp.set_pixel(
                    x as usize,
                    y as usize,
                    neighbors == 3 || neighbors == 2 && pixel,
                );
            }
        }
        disp.swap_buffers();

        UpdateStatus::Continue
    });

    display.initialize().expect("Could not initialize display.");
    display.update();
}
