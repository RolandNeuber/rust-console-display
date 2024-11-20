use display::{Display, HexPixel, QuadPixel, SinglePixel};
use std::{thread, time};

fn main() {
    let mut disp: Display<QuadPixel> = Display::build_from_bools(
        8, 
        6, 
        vec![
            true, false, true, false, true, false, true, false,
            false, true, false, true, false, true, false, true,
            true, false, true, false, true, false, true, false,
            false, true, false, true, false, true, false, true,
            true, false, true, false, true, false, true, false,
            false, true, false, true, false, true, false, true,
        ]
    ).unwrap();

    let duration = time::Duration::from_millis(100);
    for x in 0..8usize {
        for y in 0..6usize {
            thread::sleep(duration);
            disp.set_pixel(x, y, !disp.get_pixel(x, y).unwrap()).unwrap();
            println!("{}", disp.to_string());
            print!("\x1b[3A");
        }
    }
    print!("\x1b[3B");
}
