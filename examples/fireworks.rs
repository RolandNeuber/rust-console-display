#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use core::f32;
use std::{
    array,
    marker::PhantomData,
};

use console_display::{
    color::{
        ARGBColor,
        RGBColor,
        TerminalColor,
    },
    console_display::DynamicConsoleDisplay,
    display_driver::{
        DisplayDriver,
        UpdateStatus,
    },
    drawing::{
        DynamicCanvas,
        Ellipse,
        Line,
        NoFill,
    },
    pixel::color_pixel::ColorOctPixel,
    pixel_display::StaticPixelDisplay,
    widget::two_widget::OverlayWidget,
};
use rand::{
    Rng,
    rngs::ThreadRng,
    thread_rng,
};

type Display = StaticPixelDisplay<ColorOctPixel, 200, 100>;

#[derive(Clone, Copy)]
struct Firework {
    start: f32,
    height: f32,
    drift: f32,
    speed: f32,
    color: RGBColor,
    running: bool,
    explosion: Explosion,
}

impl Firework {
    fn new_random(disp: &Display, rng: &mut ThreadRng) -> Self {
        let disp_width = disp.width();
        let start = rng.gen_range(0..disp_width) as f32;
        let height = 0.;
        let drift = rng.gen_range(0..=200) as f32 / 100. - 1.;
        let speed = rng.gen_range(30..=100) as f32 / 100.;
        let color = RGBColor {
            r: rng.gen_range(0..=255),
            g: rng.gen_range(0..=255),
            b: rng.gen_range(0..=255),
        };
        Self {
            start,
            height,
            drift,
            speed,
            color,
            running: true,
            explosion: match rng.gen_range(0..5) {
                0 => Explosion::Star(
                    rng.gen_range(300..=800) as f32 / 100.,
                    rng.gen_range(5..=11),
                ),
                1 => Explosion::Round(
                    rng.gen_range(300..=800) as f32 / 100.,
                ),
                x if x > 1 => Explosion::None,
                _ => unreachable!(),
            },
        }
    }

    fn draw(&mut self, disp: &mut OverlayWidget<Display, Display>) {
        let old_y = self.start + self.drift;
        let base = disp.base_mut();
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        let _ = base.set_pixel(
            old_y as usize,
            (base.height() as f32 - self.height) as usize,
            RGBColor::BLACK.into(),
        );
        self.drift *= (1.05f32).powf(self.speed);
        let new_y = self.start + self.drift;
        if base.height() as f32 <= self.height ||
            new_y < 0. ||
            new_y > base.width() as f32 ||
            self.speed < 0.1
        {
            self.running = false;
            let y = base.height() as f32 - self.height;
            self.explosion.draw(
                disp.overlay_mut(),
                old_y,
                y,
                self.color.into(),
            );
            return;
        }
        self.height += self.speed;
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        let _ = base.set_pixel(
            new_y as usize,
            (base.height() as f32 - self.height) as usize,
            self.color.into(),
        );
        self.speed *= 0.99;
    }
}

#[derive(Clone, Copy)]
enum Explosion {
    None,
    Star(f32, u8),
    Round(f32),
}

impl Explosion {
    fn draw(
        self,
        disp: &mut Display,
        x: f32,
        y: f32,
        value: TerminalColor,
    ) {
        match self {
            Self::None => (),
            Self::Star(radius, arms) => {
                for i in 0..arms {
                    let (y2, x2) = f32::sin_cos(
                        f32::consts::TAU / f32::from(arms) * f32::from(i),
                    );
                    disp.draw(
                        &Line {
                            x1: x,
                            y1: y,
                            x2: x2.mul_add(radius, x),
                            y2: y2.mul_add(radius, y),
                        },
                        value,
                    );
                }
            }
            Self::Round(radius) => {
                disp.draw(
                    &Ellipse {
                        midpoint_x: x,
                        midpoint_y: y,
                        x1: radius,
                        y1: 0.,
                        x2: 0.,
                        y2: radius,
                        num_points: 25,
                        fill: PhantomData::<NoFill>,
                    },
                    value,
                );
            }
        }
    }
}

fn main() {
    const NUM_FIREWORKS: usize = 10;

    let disp = Display::new(RGBColor::BLACK.into());
    let expl = Display::new(ARGBColor::TRANSPARENT.into());

    let mut display = DisplayDriver::new(OverlayWidget::new(expl, disp));

    display.initialize().expect("Could not initialize display.");

    display.set_target_frame_rate(60.);

    let mut rng = thread_rng();

    let mut fireworks: [Firework; NUM_FIREWORKS] =
        array::from_fn(|_| Firework::new_random(display.base(), &mut rng));

    display.set_on_update(move |display, _| {
        for firework in fireworks.iter_mut().take(NUM_FIREWORKS) {
            firework.draw(display);
            if !firework.running {
                *firework = Firework::new_random(display.base(), &mut rng);
            }
        }
        let mut pixels = display.overlay_mut().pixels();
        let _ = display.overlay_mut().set_pixels(
            &pixels
                .iter_mut()
                .map(|color| {
                    if let TerminalColor::ARGBColor(color) = color {
                        color.opacity = color.opacity.saturating_sub(1);
                    }
                    *color
                })
                .collect::<Vec<_>>(),
        );
        UpdateStatus::Continue
    });

    display.update();
}
