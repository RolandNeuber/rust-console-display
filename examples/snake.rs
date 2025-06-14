#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use console_display::{
    console_display::{
        CharacterDisplay,
        ConsoleDisplay,
        PixelDisplay,
    },
    display_driver::{
        DisplayDriver,
        UpdateStatus,
    },
    pixel::{
        character_pixel::CharacterPixel,
        color_pixel::{
            Color,
            ColorDualPixel,
            ColorSinglePixel,
            RGBColor,
        },
    },
    widget::two_widget::{
        OverlayWidget,
        TwoWidget,
        VerticalTilingWidget,
    },
};
use crossterm::event::KeyCode;
use rand::Rng;
use std::{
    thread,
    time::Duration,
};

fn main() {
    let background_color = RGBColor { r: 0, g: 0, b: 0 };
    let snake_color = RGBColor { r: 0, g: 255, b: 0 };
    let apple_color = RGBColor { r: 255, g: 0, b: 0 };

    let mut disp = DisplayDriver::new(
        VerticalTilingWidget::build(
            PixelDisplay::<ColorSinglePixel>::build(
                100,
                1,
                RGBColor {
                    r: 255,
                    b: 255,
                    g: 255,
                },
            )
            .unwrap(),
            OverlayWidget::build(
                PixelDisplay::<ColorDualPixel>::build(
                    100,
                    42,
                    RGBColor { r: 0, b: 0, g: 0 },
                )
                .unwrap(),
                CharacterDisplay::<CharacterPixel>::build(
                    100,
                    21,
                    CharacterPixel::build(
                        ' ',
                        Color::Default,
                        Color::Default,
                    )
                    .unwrap(),
                )
                .unwrap(),
                true,
            )
            .unwrap(),
        )
        .unwrap(),
    );

    let endscreen = disp.get_children_mut().1.get_children_mut().1;

    for (i, sym) in "You lost".chars().enumerate() {
        endscreen
            .set_pixel(
                46 + i,
                10,
                CharacterPixel::build(sym, Color::Default, Color::Default)
                    .expect("Could not construct character pixel."),
            )
            .expect("Could not set character pixel.");
    }

    let duration = Duration::from_millis(75);
    let mut score = 1;
    let mut direction = (0, 0);
    let mut snake: Vec<(usize, usize)>;
    let mut apple;

    {
        let map_display = disp.get_children_mut().1;

        snake = vec![(
            map_display.get_children().0.get_width() / 2,
            map_display.get_children().0.get_height() / 2,
        )];

        map_display
            .get_children_mut()
            .0
            .set_pixel(snake[0].0, snake[0].1, snake_color)
            .expect("Could not set pixel.");

        apple = (
            rand::thread_rng()
                .gen_range(0..map_display.get_children().0.get_width()),
            rand::thread_rng()
                .gen_range(0..map_display.get_children().0.get_height()),
        );

        while snake.contains(&apple) {
            apple = (
                rand::thread_rng().gen_range(
                    0..map_display.get_children().0.get_width(),
                ),
                rand::thread_rng().gen_range(
                    0..map_display.get_children().0.get_height(),
                ),
            );
        }

        map_display
            .get_children_mut()
            .0
            .set_pixel(apple.0, apple.1, apple_color)
            .expect("Could not set pixel.");
    }

    let mut lost = false;

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
            disp.get_children_mut().1.set_child1_on_top(false);
            return UpdateStatus::Continue;
        }

        let map_display = disp.get_children_mut().1;
        // place new segment in front (direction) of snake head
        snake.insert(
            0,
            (
                (snake[0].0 as i32 + direction.0).rem_euclid(
                    map_display.get_children().0.get_width() as i32,
                ) as usize,
                (snake[0].1 as i32 + direction.1).rem_euclid(
                    map_display.get_children().0.get_height() as i32,
                ) as usize,
            ),
        );

        if snake[0] == apple {
            score += 1;
            if score ==
                map_display.get_children().0.get_width() *
                    map_display.get_children().0.get_height()
            {
                return UpdateStatus::Break;
            }
            // place new apple
            while snake.contains(&apple) {
                apple = (
                    rand::thread_rng().gen_range(
                        0..map_display.get_children().0.get_width(),
                    ),
                    rand::thread_rng().gen_range(
                        0..map_display.get_children().0.get_height(),
                    ),
                );
            }
            map_display
                .get_children_mut()
                .0
                .set_pixel(apple.0, apple.1, apple_color)
                .expect("Could not set pixel.");
        }
        else {
            // remove pixel at last segment of snake
            map_display
                .get_children_mut()
                .0
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
        map_display
            .get_children_mut()
            .0
            .set_pixel(snake[0].0, snake[0].1, snake_color)
            .expect("Could not set pixel.");

        for i in 1..snake.len() {
            if snake[0] == snake[i] {
                lost = true;
            }
        }
        thread::sleep(duration);
        UpdateStatus::Continue
    });

    disp.initialize().expect("Could not initialize display.");
    disp.update();
}
