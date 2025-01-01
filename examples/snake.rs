#![feature(generic_const_exprs)]

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use display::{color_pixel::{Color, ColorDualPixel, ColorHexPixel}, widget::{NoneWidget, SingleWidget}, ConsoleDisplay, DisplayDriver, PixelDisplay};
use rand::Rng;
use std::{thread, time::Duration};

fn main() {
    let background_color = Color {r: 0, g: 0, b: 0}; 
    let snake_color = Color {r: 0, g: 255, b: 0};
    let apple_color = Color {r: 255, g: 0, b: 0};

    let mut disp = 
    DisplayDriver::new(
        NoneWidget::new(
            PixelDisplay::<ColorDualPixel>::build(
                100, 
                42,
                Color {r: 0, b: 0, g: 0}
            ).unwrap()
        )
    );
    
    disp.initialize().expect("Could not initialize display."); 
    let duration = Duration::from_millis(75);
    
    let mut snake: Vec<(usize, usize)> = vec![(disp.get_child().get_width() / 2, disp.get_child().get_height() / 2)];
    disp.get_child_mut().set_pixel(snake[0].0, snake[0].1, snake_color).expect("Could not set pixel.");

    let mut score = 1;

    let mut apple = (
        rand::thread_rng().gen_range(0..disp.get_child().get_width().clone()), 
        rand::thread_rng().gen_range(0..disp.get_child().get_height().clone())
    );
    while snake.contains(&apple) {
        apple = (
            rand::thread_rng().gen_range(0..disp.get_child().get_width().clone()), 
            rand::thread_rng().gen_range(0..disp.get_child().get_height().clone())
        );
    }

    let mut direction = (0, 0);

    disp.get_child_mut().set_pixel(apple.0, apple.1, apple_color).expect("Could not set pixel.");
    disp.print_display().expect("Could not print display.");

    let mut lost = false;

    loop {
        for i in 1..snake.len() {
            if snake[0] == snake[i] {
                lost = true;
            }
        }

        thread::sleep(duration);
        disp.print_display().expect("Could not print display.");

        let mut latest_event = None;
        // Wait for events (non-blocking, adjust duration as needed)
        while event::poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                latest_event = Some(key_event);
            }
        }

        if let Some(key_event) = latest_event {
            if 
                key_event.code == KeyCode::Char('c') && 
                key_event.modifiers.contains(KeyModifiers::CONTROL)
            {
                break; // Exit on Ctrl-C
            }
            let key = match key_event.code {
                KeyCode::Char(x) => x,
                _ => continue,
            };

            let old_direction = direction;
            match key {
                'a' => direction = (-1,  0),
                'd' => direction = ( 1,  0),
                'w' => direction = ( 0, -1),
                's' => direction = ( 0,  1),
                _ => (),
            };
            if 
                old_direction.0 + direction.0 == 0 &&
                old_direction.1 + direction.1 == 0 
            {
                direction = old_direction;
            }
        }

        if lost {
            continue;
        }
        
        // place new segment in front (direction) of snake head
        snake.insert(0, (
            (snake[0].0 as i32 + direction.0).rem_euclid(disp.get_child().get_width().clone()  as i32) as usize,
            (snake[0].1 as i32 + direction.1).rem_euclid(disp.get_child().get_height().clone() as i32) as usize
        ));

        if snake[0] == apple {
            score += 1;
            if score == disp.get_child().get_width().clone() * disp.get_child().get_height().clone() {
                break;
            }
            // place new apple
            while snake.contains(&apple) {
                apple = (
                    rand::thread_rng().gen_range(0..disp.get_child().get_width().clone()), 
                    rand::thread_rng().gen_range(0..disp.get_child().get_height().clone())
                );
            }
            disp.get_child_mut().set_pixel(apple.0, apple.1, apple_color).expect("Could not set pixel.");
        }
        else {
            // remove pixel at last segment of snake
            disp.get_child_mut().set_pixel(snake.last().unwrap().0, snake.last().unwrap().1, background_color).expect("Could not set pixel.");
            // remove the last segment of snake if it hasn't eaten
            snake.pop();
        }

        // place pixel at snake head
        disp.get_child_mut().set_pixel(snake[0].0, snake[0].1, snake_color).expect("Could not set pixel.");
    }
}
