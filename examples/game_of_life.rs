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
};
use rand::{
    Rng,
    thread_rng,
};

fn main() {
    let disp = StaticPixelDisplay::<OctPixel, 200, 100>::new_from_data(
        &array::from_fn::<_, 20_000, _>(|_| {
            let rng = thread_rng().gen_range(0..=1);
            rng != 0
        }),
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
        let mut data = disp.get_pixels();
        for x in 0..disp.get_width() {
            for y in 0..disp.get_height() {
                let mut neighbors = 0;
                for i in offsets {
                    let x_i = (x as isize + i.0)
                        .rem_euclid(disp.get_width() as isize)
                        as usize;
                    let y_i = (y as isize + i.1)
                        .rem_euclid(disp.get_height() as isize)
                        as usize;
                    if disp
                        .get_pixel(x_i, y_i)
                        .expect("Could not get pixel.")
                    {
                        neighbors += 1;
                    }
                }
                data[x + y * disp.get_width()] = neighbors == 3 ||
                    neighbors == 2 &&
                        disp.get_pixel(x, y)
                            .expect("Could not get pixel.");
            }
        }
        disp.set_pixels(&data);
        UpdateStatus::Continue
    });

    display.initialize().expect("Could not initialize display.");
    display.update();
}
