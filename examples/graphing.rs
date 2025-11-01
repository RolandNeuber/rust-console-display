#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use console_display::{
    character_display::StaticCharacterDisplay,
    color::{
        ARGBColor,
        RGBColor,
    },
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
        character_pixel::CharacterPixel,
        color_pixel::ColorOctPixel,
    },
    pixel_display::StaticPixelDisplay,
    widget::{
        single_widget::UvWidget,
        two_widget::OverlayWidget,
    },
};

fn main() {
    type PixelType = ColorOctPixel;
    const DIMENSIONS: (usize, usize) = (200, 200);
    const DIMENSIONS_CHARS: (usize, usize) = (
        DIMENSIONS.0 / PixelType::WIDTH,
        DIMENSIONS.1 / PixelType::HEIGHT,
    );

    let uv_x = (-10.0, 10.0);
    let uv_y = (2.0, -2.0);

    let foreground = RGBColor::WHITE.into();
    let transparent = ARGBColor {
        opacity: 63,
        color: RGBColor::BLACK,
    }
    .into();

    let mut axis = UvWidget::new(StaticCharacterDisplay::<
        CharacterPixel,
        { DIMENSIONS_CHARS.0 },
        { DIMENSIONS_CHARS.1 },
    >::new(CharacterPixel::new::<' '>(
        transparent,
        transparent,
    )));

    axis.set_uv_x_min(0.);
    axis.set_uv_x_max(1.);
    axis.set_uv_y_min(0.);
    axis.set_uv_y_max(1.);

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_possible_wrap)]
    axis.draw(
        &Line {
            x1: 0.,
            y1: 0.5,
            x2: 1.,
            y2: 0.5,
        },
        CharacterPixel::new::<'─'>(foreground, transparent).into(),
    );
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_possible_wrap)]
    axis.draw(
        &Line {
            x1: 0.5,
            y1: 0.,
            x2: 0.5,
            y2: 1.,
        },
        CharacterPixel::new::<'│'>(foreground, transparent).into(),
    );
    axis.set_pixel(
        0.0,
        0.5,
        CharacterPixel::new::<'┼'>(foreground, transparent).into(),
    )
    .unwrap();

    let mut graph = UvWidget::new(StaticPixelDisplay::<
        PixelType,
        { DIMENSIONS.0 },
        { DIMENSIONS.1 },
    >::new(RGBColor::BLACK.into()));

    graph.set_uv_x_min(uv_x.0);
    graph.set_uv_x_max(uv_x.1);
    graph.set_uv_y_min(uv_y.0);
    graph.set_uv_y_max(uv_y.1);

    let mut display = DisplayDriver::new(OverlayWidget::new(axis, graph));

    display.set_on_update(|this: &mut DisplayDriver<_>, _| {
        let function = |x: f32| (x * x).sin();
        let mut xs = this.1.x_values().collect::<Vec<_>>().into_iter();
        let mut old_x = xs.next().unwrap();
        let mut old_y = function(old_x);
        for x in xs {
            let y = function(x);

            this.base_mut().draw(
                &Line {
                    x1: old_x,
                    y1: old_y,
                    x2: x,
                    y2: y,
                },
                RGBColor::WHITE.into(),
            );

            old_x = x;
            old_y = y;
        }
        UpdateStatus::Continue
    });

    display.initialize().expect("Could not initialize display.");
    display.update();
}
