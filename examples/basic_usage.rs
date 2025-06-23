#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use console_display::{
    display_driver::DisplayDriver,
    pixel::color_pixel::{
        self,
        ColorOctPixel,
    },
    pixel_display::StaticPixelDisplay,
};

fn main() {
    // Construct the display with the ColorOctPixel Type
    // (8 pixels per console character, 4 high, 2 wide).
    // Set dimensions to 100 by 100 pixels (50 by 25 characters)
    // with a red fill.
    let disp = StaticPixelDisplay::<ColorOctPixel, 100, 100>::new(
        color_pixel::RGBColor { r: 255, g: 0, b: 0 },
    );

    // Wrap the display in a driver to manage interactions with the terminal
    // like resizing, enabling raw mode, providing an update loop.
    let mut display = DisplayDriver::new(disp);

    // Initialize an alternate terminal screen and resize.
    display.initialize().expect("Could not initialize display.");
    // Run the default update loop.
    display.update();
}
