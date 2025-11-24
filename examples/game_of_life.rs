#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![allow(clippy::unwrap_used)]

use std::array;

use console_display::{
    console_display::DynamicConsoleDisplay,
    display_driver::{
        DisplayDriver,
        UpdateStatus,
    },
    drawing::DynamicCanvas,
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
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_possible_wrap)]
        let width = disp.width() as i32;
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_possible_wrap)]
        let height = disp.height() as i32;
        for x in 0..width {
            for y in 0..height {
                let mut neighbors = 0;
                for i in offsets {
                    let x_i = (x + i.0).rem_euclid(width) as usize;
                    let y_i = (y + i.1).rem_euclid(height) as usize;
                    if disp.pixel(x_i, y_i).expect("Could not get pixel.")
                    {
                        neighbors += 1;
                    }
                }
                #[allow(clippy::cast_sign_loss)]
                let pixel = disp
                    .pixel(x as usize, y as usize)
                    .expect("Could not get pixel.");

                #[allow(clippy::cast_sign_loss)]
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
    display.update().expect("Could not update display.");
}
