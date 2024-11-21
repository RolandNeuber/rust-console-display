use display::{Display, HexPixel, OctPixel, QuadPixel, SinglePixel};
use std::{thread, time};

fn main() {
    let mut disp: Display<QuadPixel> = Display::build_from_bools(
        256, 
        64, 
/*         vec![
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

    let duration = time::Duration::from_millis(0);
    for x in 0..256usize {
        for y in 0..64usize {
            thread::sleep(duration);
            disp.set_pixel(x, y, !disp.get_pixel(x, y).unwrap()).unwrap();
            disp.print_display(0);
        }
    }
    disp.finalize_display();
}
