#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use console_display::{
    console_display::PixelDisplay,
    display_driver::{
        DisplayDriver,
        UpdateStatus,
    },
    pixel::color_pixel::{
        ColorOctPixel,
        RGBColor,
    },
    widget::single_widget::UvWidget,
};

fn main() {
    type PixelType = ColorOctPixel;
    let dimensions: (usize, usize) = (200, 200);

    let uv_x = (-10.0, 10.0);
    let uv_y = (2.0, -2.0);

    let mut display = DisplayDriver::new(UvWidget::new(
        PixelDisplay::<PixelType>::build(
            dimensions.0,
            dimensions.1,
            RGBColor { r: 0, g: 0, b: 0 },
        )
        .expect("Could not construct display."),
    ));
    display.set_uv_x_min(uv_x.0);
    display.set_uv_x_max(uv_x.1);
    display.set_uv_y_min(uv_y.0);
    display.set_uv_y_max(uv_y.1);

    display.set_on_update(|this: &mut DisplayDriver<_>, _| {
        let function = |x: f32| (x * x).sin();
        let mut xs = this.get_x_values().collect::<Vec<_>>().into_iter();
        let mut old_x = xs.next().unwrap();
        let mut old_y = function(old_x);
        for x in xs {
            let y = function(x);

            this.draw_line(
                old_x,
                old_y,
                x,
                y,
                RGBColor {
                    r: 255,
                    g: 255,
                    b: 255,
                },
            );

            old_x = x;
            old_y = y;
        }
        UpdateStatus::Continue
    });

    display.initialize().expect("Could not initialize display.");
    display.update();
}
