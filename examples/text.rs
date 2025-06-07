#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use console_display::{
    console_display::{
        CharacterDisplay,
        ConsoleDisplay,
    },
    display_driver::DisplayDriver,
    pixel::{
        character_pixel::CharacterPixel,
        color_pixel::Color,
    },
};

fn main() {
    let mut char_disp: CharacterDisplay<CharacterPixel, 40, 20> =
        CharacterDisplay::build(
            CharacterPixel::build(' ', Color::Default, Color::Default)
                .unwrap(),
        )
        .unwrap();

    let mut x = 0;
    let mut y = 0;
    for i in "Lorem ipsum dolor sit amet,\n\
        consetetur sadipscing elitr,\n\
        sed diam nonumy eirmod tempor invidunt\n\
        ut labore et dolore magna aliquyam erat,\n\
        sed diam voluptua."
        .chars()
    {
        if i == '\n' {
            y += 1;
            x = 0;
            continue;
        }
        let pixel =
            CharacterPixel::build(i, Color::Default, Color::Default)
                .unwrap();
        let _ = char_disp.set_pixel(x, y, &pixel);
        if x + pixel.get_width() > char_disp.get_width() {
            y += 1;
            x = 0;
        }
        else {
            x += pixel.get_width();
        }
    }

    let mut display = DisplayDriver::new(char_disp);

    display.initialize().expect("Could not initialize display.");
    display.update();
}
