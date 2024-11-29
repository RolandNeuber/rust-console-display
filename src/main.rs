use crossterm::{event::{self, Event, KeyCode, KeyModifiers}, terminal};
use display::{Display, HexPixel, OctPixel, QuadPixel, SinglePixel};
use rand::Rng;
use std::{thread, time::Duration};

fn main() {
    let mut disp: Display<HexPixel> = Display::build_from_bools(
        256, 
        96, 
        vec![false; 16384 * 3 / 2]
    ).unwrap();
    
    let _ = terminal::enable_raw_mode();
    let _ = disp.initialize(); 
    let duration = Duration::from_millis(0);
    
    let mut snake: Vec<(usize, usize)> = vec![(disp.width / 2, disp.height / 2)];
    disp.set_pixel(snake[0].0, snake[0].1, true);

    let apple = (
        rand::thread_rng().gen_range(0..disp.width), 
        rand::thread_rng().gen_range(0..disp.height)
    );
    disp.set_pixel(apple.0, apple.1, true);
    disp.print_display();

    loop {
        thread::sleep(duration);
        let _ = disp.print_display();

        // Wait for events (non-blocking, adjust duration as needed)
        if event::poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
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

                disp.set_pixel(snake[0].0, snake[0].1, false);

                match key {
                    'w' => snake[0].1 = (snake[0].1 as i32 - 1).rem_euclid(disp.height as i32) as usize,
                    's' => snake[0].1 = (snake[0].1 as i32 + 1).rem_euclid(disp.height as i32) as usize,
                    'a' => snake[0].0 = (snake[0].0 as i32 - 1).rem_euclid(disp.width  as i32) as usize,
                    'd' => snake[0].0 = (snake[0].0 as i32 + 1).rem_euclid(disp.width  as i32) as usize,
                    _ => (),
                };

                disp.set_pixel(snake[0].0, snake[0].1, true);
            }

        }
    }
    let _ = terminal::disable_raw_mode();
}
