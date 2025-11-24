#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![allow(clippy::unwrap_used)]

use console_display::{
    character_display::StaticCharacterDisplay,
    color::TerminalColor,
    console_display::DynamicConsoleDisplay,
    display_driver::DisplayDriver,
    drawing::DynamicCanvas,
    pixel::character_pixel::CharacterPixel,
};

fn main() {
    let mut char_disp =
        StaticCharacterDisplay::<CharacterPixel, 40, 20>::new(
            CharacterPixel::new::<'あ'>(
                TerminalColor::Default,
                TerminalColor::Default,
            ),
        );

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
        let pixel = CharacterPixel::build(
            i,
            TerminalColor::Default,
            TerminalColor::Default,
        )
        .unwrap();
        char_disp.set_pixel(x, y, pixel.into()).unwrap();
        if x + pixel.width() > char_disp.width() {
            y += 1;
            x = 0;
        }
        else {
            x += pixel.width();
        }
    }

    let mut display = DisplayDriver::new(char_disp);

    display.initialize().expect("Could not initialize display.");
    display.update().expect("Could not update display.");
}
