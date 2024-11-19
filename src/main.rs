use display::Display;

fn main() {
    let disp = Display::build_from_bools(
        8, 
        8, 
        vec![
            true, false, true, false, true, false, true, false,
            false, true, false, true, false, true, false, true,
            true, false, true, false, true, false, true, false,
            false, true, false, true, false, true, false, true,
            true, false, true, false, true, false, true, false,
            false, true, false, true, false, true, false, true,
            true, false, true, false, true, false, true, false,
            false, true, false, true, false, true, false, true,
        ]
    ).unwrap();

    println!("{}", disp.get_pixel(3, 5).unwrap());

    println!("{:#?}", disp);
}
