use crossterm::{event::{self, Event, KeyCode, KeyModifiers}, terminal};
use display::{Display, HexPixel, OctPixel, QuadPixel, SinglePixel};
use std::{thread, time};

fn main() {
    let mut disp: Display<QuadPixel> = Display::build_from_bools(
        256, 
        64, 
        /* vec![
            true, false, true, false, true, false, true, false,
            false, true, false, true, false, true, false, true,
            true, false, true, false, true, false, true, false,
            false, true, false, true, false, true, false, true,
            true, false, true, false, true, false, true, false,
            false, true, false, true, false, true, false, true,
            true, false, true, false, true, false, true, false,
            false, true, false, true, false, true, false, true,
        ] */
       vec![true; 16384]
    ).unwrap();

    let _ = terminal::enable_raw_mode();
    let _ = disp.initialize(); 
    let duration = time::Duration::from_millis(0);
    'outer: for x in 0..256usize {
        for y in 0..64usize {
            thread::sleep(duration);
            disp.set_pixel(x, y, !disp.get_pixel(x, y).unwrap()).unwrap();
            let _ = disp.print_display();

            // Wait for events (non-blocking, adjust duration as needed)
            if event::poll(std::time::Duration::from_millis(0)).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    if 
                        key_event.code == KeyCode::Char('c') && 
                        key_event.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        break 'outer; // Exit on Ctrl-C
                    }
                }
            }
        }
    }
    let _ = terminal::disable_raw_mode();
}
