#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use console_display::{
    character_display::StaticCharacterDisplay,
    console_display::{
        DynamicConsoleDisplay,
        StaticConsoleDisplay,
    },
    display_driver::{
        DisplayDriver,
        UpdateStatus,
    },
    pixel::{
        Pixel,
        character_pixel::CharacterPixel,
        color_pixel::{
            ARGBColor,
            ColorOctPixel,
            RGBColor,
        },
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

    let uv_x = (-10.0, 10.0);
    let uv_y = (2.0, -2.0);

    let transparent = ARGBColor {
        opacity: 63,
        color: RGBColor::BLACK,
    };

    let mut axis: StaticCharacterDisplay<
        CharacterPixel,
        { DIMENSIONS.0 / PixelType::WIDTH },
        { DIMENSIONS.1 / PixelType::HEIGHT },
    > = StaticCharacterDisplay::new(CharacterPixel::new::<' '>(
        transparent.into(),
        transparent.into(),
    ));
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_possible_wrap)]
    axis.draw_line(
        0,
        (DIMENSIONS.1 / PixelType::HEIGHT) as i32 / 2,
        (DIMENSIONS.0 / PixelType::WIDTH) as i32,
        (DIMENSIONS.1 / PixelType::HEIGHT) as i32 / 2,
        CharacterPixel::new::<'─'>(
            RGBColor::WHITE.into(),
            transparent.into(),
        )
        .into(),
    );
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_possible_wrap)]
    axis.draw_line(
        (DIMENSIONS.0 / PixelType::WIDTH) as i32 / 2,
        0,
        (DIMENSIONS.0 / PixelType::WIDTH) as i32 / 2,
        (DIMENSIONS.1 / PixelType::HEIGHT) as i32,
        CharacterPixel::new::<'│'>(
            RGBColor::WHITE.into(),
            transparent.into(),
        )
        .into(),
    );
    axis.set_pixel_static::<
        {DIMENSIONS.0 / PixelType::WIDTH / 2},
        {DIMENSIONS.1 / PixelType::HEIGHT / 2}>(
        CharacterPixel::new::<'┼'>(
            RGBColor::WHITE.into(),
            transparent.into(),
        )
        .into(),
    );

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

            this.1.draw_line(old_x, old_y, x, y, RGBColor::WHITE.into());

            old_x = x;
            old_y = y;
        }
        UpdateStatus::Continue
    });

    display.initialize().expect("Could not initialize display.");
    display.update();
}
