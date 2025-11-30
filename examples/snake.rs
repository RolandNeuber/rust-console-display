#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![allow(clippy::unwrap_used)]

use console_display::{
    character_display::StaticCharacterDisplay,
    color::{
        RGBColor,
        TerminalColor,
    },
    console_display::{
        DynamicConsoleDisplay,
        StaticConsoleDisplay,
    },
    display_driver::{
        DisplayDriver,
        UpdateStatus,
    },
    drawing::DynamicCanvas,
    pixel::{
        character_pixel::CharacterPixel,
        color_pixel::ColorDualPixel,
    },
    pixel_display::StaticPixelDisplay,
    widget::{
        single_widget::{
            BorderDefault,
            BorderWidget,
        },
        two_widget::{
            AlternativeWidget,
            VerticalTilingWidget,
        },
    },
};
use crossterm::event::KeyCode;
use rand::{
    Rng,
    rng,
    rngs::ThreadRng,
};

#[allow(clippy::too_many_lines)]
fn main() {
    let mut rng = rng();
    let background_color: TerminalColor = RGBColor::BLACK.into();
    let snake_color: TerminalColor = RGBColor::GREEN.into();
    let apple_color: TerminalColor = RGBColor::RED.into();

    let mut disp = construct_display();

    disp.0.set_pixel_static::<99, 0>(
        CharacterPixel::new::<'1'>(
            RGBColor::BLACK.into(),
            RGBColor::WHITE.into(),
        )
        .into(),
    );

    initialize_end_screen(&mut disp.bottom_mut().1);

    let mut fps = 10.;
    let mut score = 1;
    let mut direction = (0, 0);
    let mut snake: Vec<(usize, usize)>;
    let mut apple;

    (snake, apple) =
        initialize_map(&mut rng, &mut disp.1.0, snake_color, apple_color);

    let mut lost = false;

    disp.set_target_frame_rate(fps);
    // TODO: Extract closure into separate function
    disp.set_on_update(move |disp, latest_event| {
        if let Some(key_event) = latest_event {
            let KeyCode::Char(key) = key_event.code
            else {
                return UpdateStatus::Continue;
            };

            let old_direction = direction;
            match key {
                'a' => direction = (-1, 0),
                'd' => direction = (1, 0),
                'w' => direction = (0, -1),
                's' => direction = (0, 1),
                _ => (),
            }
            if old_direction.0 + direction.0 == 0 &&
                old_direction.1 + direction.1 == 0
            {
                direction = old_direction;
            }
        }

        if lost {
            disp.bottom_mut().set_child1_on_top(false);
            return UpdateStatus::Continue;
        }

        let map_display = &mut disp.bottom_mut().0;
        // place new segment in front (direction) of snake head
        snake.insert(
            0,
            #[allow(clippy::cast_possible_truncation)]
            #[allow(clippy::cast_possible_wrap)]
            (
                (snake[0].0 as i32 + direction.0)
                    .rem_euclid(map_display.width() as i32)
                    as usize,
                (snake[0].1 as i32 + direction.1)
                    .rem_euclid(map_display.height() as i32)
                    as usize,
            ),
        );

        if snake[0] == apple {
            score += 1;
            for (i, digit) in score.to_string().chars().rev().enumerate() {
                let _ = disp.top_mut().set_pixel(
                    99 - i,
                    0,
                    CharacterPixel::build(
                        digit,
                        RGBColor::BLACK.into(),
                        RGBColor::WHITE.into(),
                    )
                    .unwrap()
                    .into(),
                );
            }

            let map_display = &mut disp.bottom_mut().0;
            if score == map_display.width() * map_display.height() {
                return UpdateStatus::Break;
            }
            // place new apple
            while snake.contains(&apple) {
                apple = (
                    rng.random_range(0..map_display.width()),
                    rng.random_range(0..map_display.height()),
                );
            }
            map_display
                .set_pixel(apple.0, apple.1, apple_color)
                .expect("Could not set pixel.");

            fps += 1.;
            disp.set_target_frame_rate(fps);
        }
        else {
            // remove pixel at last segment of snake
            map_display
                .set_pixel(
                    snake.last().unwrap().0,
                    snake.last().unwrap().1,
                    background_color,
                )
                .expect("Could not set pixel.");
            // remove the last segment of snake if it hasn't eaten
            snake.pop();
        }

        // place pixel at snake head
        disp.bottom_mut()
            .0
            .set_pixel(snake[0].0, snake[0].1, snake_color)
            .expect("Could not set pixel.");

        for i in 1..snake.len() {
            if snake[0] == snake[i] {
                lost = true;
            }
        }
        UpdateStatus::Continue
    });

    disp.initialize().expect("Could not initialize display.");
    disp.update().expect("Could not update display.");
}

type Display = DisplayDriver<
    BorderWidget<
        VerticalTilingWidget<
            StaticCharacterDisplay<CharacterPixel, 100, 1>,
            AlternativeWidget<
                StaticPixelDisplay<ColorDualPixel, 100, 42>,
                StaticCharacterDisplay<CharacterPixel, 100, 21>,
            >,
        >,
        BorderDefault,
    >,
>;

fn construct_display() -> Display {
    DisplayDriver::new(BorderWidget::new(
        VerticalTilingWidget::new(
            StaticCharacterDisplay::<_, 100, 1>::new(
                CharacterPixel::new::<' '>(
                    RGBColor::BLACK.into(),
                    RGBColor::WHITE.into(),
                ),
            ),
            AlternativeWidget::new(
                StaticPixelDisplay::<ColorDualPixel, 100, 42>::new(
                    RGBColor::BLACK.into(),
                ),
                StaticCharacterDisplay::<CharacterPixel, 100, 21>::new(
                    CharacterPixel::new::<' '>(
                        TerminalColor::Default,
                        TerminalColor::Default,
                    ),
                ),
                true,
            ),
        ),
        BorderDefault::double_stroke(
            TerminalColor::Default,
            RGBColor::BLACK.into(),
        ),
    ))
}

fn initialize_end_screen<const WIDTH: usize, const HEIGHT: usize>(
    endscreen: &mut StaticCharacterDisplay<CharacterPixel, WIDTH, HEIGHT>,
) {
    for (i, sym) in "You lost".chars().enumerate() {
        endscreen
            .set_pixel(
                46 + i,
                10,
                CharacterPixel::build(
                    sym,
                    TerminalColor::Default,
                    TerminalColor::Default,
                )
                .expect("Could not construct character pixel.")
                .into(),
            )
            .expect("Could not set character pixel.");
    }
}

fn initialize_map<const WIDTH: usize, const HEIGHT: usize>(
    rng: &mut ThreadRng,
    map_display: &mut StaticPixelDisplay<ColorDualPixel, WIDTH, HEIGHT>,
    snake_color: TerminalColor,
    apple_color: TerminalColor,
) -> (Vec<(usize, usize)>, (usize, usize)) {
    let snake = vec![(map_display.width() / 2, map_display.height() / 2)];

    map_display
        .set_pixel(snake[0].0, snake[0].1, snake_color)
        .expect("Could not set pixel.");

    let mut apple = (
        rng.random_range(0..map_display.width()),
        rng.random_range(0..map_display.height()),
    );

    while snake.contains(&apple) {
        apple = (
            rng.random_range(0..map_display.width()),
            rng.random_range(0..map_display.height()),
        );
    }

    map_display
        .set_pixel(apple.0, apple.1, apple_color)
        .expect("Could not set pixel.");

    (snake, apple)
}
