# Console Display

A tui library that focuses primarily on providing high resolution display functionality in the console.

![Continuous Integration](https://github.com/RolandNeuber/rust-console-display/actions/workflows/ci.yaml/badge.svg)

`console-display` provides a simple API for rendering pixels (among other primitives) inside the terminal. For that matter it uses various Unicode characters with differing levels of support by fonts as well as terminal emulators.

These Unicode blocks include: \
[Block Elements](https://en.wikipedia.org/wiki/Block_Elements) \
[Symbols for Legacy Computing](https://en.wikipedia.org/wiki/Symbols_for_Legacy_Computing) \
[Symbols for Legacy Computing Supplement](https://en.wikipedia.org/wiki/Symbols_for_Legacy_Computing_Supplement) \
[Braille Patterns](https://en.wikipedia.org/wiki/Braille_Patterns)

For optimal compatibility I recommend using: \
Gnome Terminal \
Cascadia Code NF


# Disclaimer

This crate uses rust's nightly toolchain to provide bleeding-edge functionality.
Although I try to minimize errors, I cannot guarantee stability and correctness.
In order to use the API you need the following attributes at the top of your files:
```rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
```
This enables the crate to perform certain compile-time checks.
This is a hard/required dependency, without it you may get cryptic compiler errors.


# Using this crate

To add this crate as dependency, add this line to your `cargo.toml` under `[dependencies]`.
```toml
console-display = "0.1.0"
```


# Examples

Here is a minimal example that constructs a 100 by 100 display colored red:

```rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use console_display::{
    display_driver::{
        DisplayDriver,
        UpdateStatus,
    },
    pixel::color_pixel::{
        self,
        ColorOctPixel,
    },
    pixel_display::StaticPixelDisplay,
};

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
// Set the update function (in this case instantly terminate the update loop)
display.set_on_update(
    move |_ /* display instance */, _ /* key events */| {
        UpdateStatus::Break
    },
);
// Run the default update loop.
display.update();
```

More in depth examples can be found in the [examples](examples/) folder.


# Contributing

Contributions are welcome! Please see the steps below:

To contribute, please install rust's nightly toolchain. \
I'd also recommend to install the [`act`](https://github.com/nektos/act) command to run the [CI pipeline](.github/workflows/ci.yaml) locally. \
Alternatively, you can also run the pipeline steps manually. \
Make sure you have run formatting, building, linting and testing.


# License

Licensed under MIT or Apache-2.0.