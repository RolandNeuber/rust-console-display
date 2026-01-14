#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![allow(clippy::unwrap_used)]

use std::{
    array,
    vec,
};

use console_display::{
    color::RGBColor,
    console_display::DynamicConsoleDisplay,
    display_driver::{
        DisplayDriver,
        UpdateStatus,
    },
    drawing::DynamicCanvas,
    pixel::color_pixel::ColorDualPixel,
    pixel_display::StaticPixelDisplay,
    widget::single_widget::DoubleBufferWidget,
};
use rand::{
    Rng,
    rng,
};

struct BubbleSortState {
    x: usize,
    y: usize,
    reads: Vec<usize>,
    writes: Vec<usize>,
}

impl BubbleSortState {
    pub const fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            reads: vec![],
            writes: vec![],
        }
    }
}

fn bubble_sort<const N: usize>(
    list: &mut [i32; N],
    state: &mut BubbleSortState,
) {
    state.reads.clear();
    state.writes.clear();
    let y = state.y;
    if y == list.len() {
        return;
    }
    let x = state.x;
    state.reads.push(x);
    state.reads.push(x + 1);
    if list[x] > list[x + 1] {
        list.swap(x, x + 1);
        state.writes.push(x);
        state.writes.push(x + 1);
    }
    if x + 2 == list.len() {
        state.y += 1;
        state.x = 0;
    }
    else {
        state.x += 1;
    }
}

fn main() {
    const WIDTH: usize = 40;
    let mut trng = rng();
    let mut list: [i32; WIDTH] = array::from_fn(move |_| {
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_possible_wrap)]
        trng.random_range(0..WIDTH as i32)
    });
    let disp = DoubleBufferWidget::new(StaticPixelDisplay::<
        ColorDualPixel,
        WIDTH,
        WIDTH,
    >::new(RGBColor::BLACK.into()));

    let mut display = DisplayDriver::new(disp);

    display.set_target_frame_rate(60.);
    let mut state = BubbleSortState::new();
    display.set_on_update(move |disp, _| {
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_possible_wrap)]
        let height = disp.height();
        for (x, list_item) in list.iter().enumerate() {
            for y in 0..height {
                #[allow(clippy::cast_sign_loss)]
                let _pixel =
                    disp.pixel(x, y).expect("Could not get pixel.");

                #[allow(clippy::cast_sign_loss)]
                let _ = disp.set_pixel(
                    x,
                    y,
                    if (*list_item as usize) < height - y {
                        RGBColor::BLACK.into()
                    }
                    else if state.writes.contains(&x) {
                        RGBColor::RED.into()
                    }
                    else if state.reads.contains(&x) {
                        RGBColor::GREEN.into()
                    }
                    else {
                        RGBColor::WHITE.into()
                    },
                );
            }
        }
        disp.swap_buffers();

        bubble_sort(&mut list, &mut state);
        UpdateStatus::Continue
    });

    display.initialize().expect("Could not initialize display.");
    display.update().expect("Could not update display.");
}
