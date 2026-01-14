#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![allow(clippy::unwrap_used)]

use std::array::from_fn;

use console_display::{
    color::RGBColor,
    display_driver::{
        DisplayDriver,
        UpdateStatus,
    },
    drawing::{
        DynamicCanvas,
        Line,
    },
    pixel::{
        Pixel,
        color_pixel::{
            ColorDualPixel,
            ColorSinglePixel,
        },
    },
    pixel_display::StaticPixelDisplay,
    widget::two_widget::HorizontalTilingWidget,
};
use crossterm::event::{
    Event,
    MouseEventKind,
};

// TODO: Fix mouse events, when terminal dimensions don't match application dimensions
type PixelType = ColorDualPixel;
const WIDTH: usize = 80;
const HEIGHT: usize = 80;
fn main() {
    let pixel_disp = StaticPixelDisplay::<PixelType, WIDTH, HEIGHT>::new(
        RGBColor::BLACK.into(),
    );

    let palette = StaticPixelDisplay::<
        ColorSinglePixel,
        3,
        { HEIGHT / PixelType::HEIGHT },
    >::new_from_data(&from_fn(|i| {
        match i * 4 * PixelType::HEIGHT / 3 / HEIGHT {
            0 => RGBColor::WHITE,
            1 => RGBColor::RED,
            2 => RGBColor::GREEN,
            3 => RGBColor::BLUE,
            _ => unreachable!(),
        }
        .into()
    }));

    let mut display = DisplayDriver::new(HorizontalTilingWidget::new(
        palette, pixel_disp,
    ));

    let mut last_pos: Option<(usize, usize)> = None;
    let mut color = RGBColor::WHITE.into();
    display.set_on_update(move |disp, event| {
        if let Some(Event::Mouse(mouse_event)) = event {
            let current_pos = (
                ((mouse_event.column as usize * PixelType::WIDTH) *
                    (WIDTH + 1) /
                    (WIDTH - PixelType::WIDTH + 1))
                    .saturating_sub(3),
                (mouse_event.row as usize * PixelType::HEIGHT) *
                    (HEIGHT + 1) /
                    (HEIGHT - PixelType::HEIGHT + 1),
            );
            if let MouseEventKind::Down(_) = mouse_event.kind &&
                mouse_event.column < 3
            {
                color = match mouse_event.row as usize *
                    4 *
                    PixelType::HEIGHT /
                    HEIGHT
                {
                    0 => RGBColor::WHITE,
                    1 => RGBColor::RED,
                    2 => RGBColor::GREEN,
                    3 => RGBColor::BLUE,
                    _ => unreachable!(),
                }
                .into();
            }
            if let MouseEventKind::Drag(_) = mouse_event.kind {
                let _ =
                    disp.1.set_pixel(current_pos.0, current_pos.1, color);

                if let Some(last_pos) = last_pos {
                    let line = Line {
                        x1: current_pos.0 as f32,
                        y1: current_pos.1 as f32,
                        x2: last_pos.0 as f32,
                        y2: last_pos.1 as f32,
                    };
                    disp.1.draw(&line, color);
                }
            }
            last_pos = Some(current_pos);
        }

        UpdateStatus::Continue
    });

    display.initialize().expect("Could not initialize display.");
    display.update().expect("Could not update display.");
}
