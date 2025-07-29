use std::marker::PhantomData;

use crate::{
    console_display::{
        DynamicConsoleDisplay,
        StaticConsoleDisplay,
    },
    pixel::Pixel,
};

pub trait FillType {}

pub struct NoFill;
impl FillType for NoFill {}
pub struct Filled;
impl FillType for Filled {}

pub trait DynamicDrawable<T: DynamicConsoleDisplay<S>, S: Pixel> {
    fn draw(&self, display: &mut T, value: S::U);
}

pub trait StaticDrawable<T: StaticConsoleDisplay<S>, S: Pixel> {
    fn draw(&self, display: &mut T, value: S::U);
}

pub struct Line {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

impl<T: DynamicConsoleDisplay<S>, S: Pixel> DynamicDrawable<T, S> for Line
where
    [(); S::WIDTH * S::HEIGHT]:,
{
    fn draw(&self, display: &mut T, value: S::U) {
        let dx = self.x2 - self.x1;
        let dy = self.y2 - self.y1;

        let steps: f32 = dx.abs().max(dy.abs());
        let x_inc = dx / steps;
        let y_inc = dy / steps;

        let mut x = self.x1;
        let mut y = self.y1;

        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        for _ in 0..=steps.round() as usize {
            if x > -0.5 && y > -0.5 {
                let _ = display.set_pixel(
                    x.round() as usize,
                    y.round() as usize,
                    value,
                );
            }
            x += x_inc;
            y += y_inc;
        }
    }
}

pub struct Rectangle<FILL: FillType> {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    fill: PhantomData<FILL>,
}

impl<T: DynamicConsoleDisplay<S>, S: Pixel> DynamicDrawable<T, S>
    for Rectangle<NoFill>
where
    [(); S::WIDTH * S::HEIGHT]:,
{
    fn draw(&self, display: &mut T, value: S::U) {
        let lines = [
            Line {
                x1: self.x1,
                y1: self.y1,
                x2: self.x2,
                y2: self.y1,
            },
            Line {
                x1: self.x1,
                y1: self.y2,
                x2: self.x2,
                y2: self.y2,
            },
            Line {
                x1: self.x1,
                y1: self.y1,
                x2: self.x1,
                y2: self.y2,
            },
            Line {
                x1: self.x2,
                y1: self.y1,
                x2: self.x2,
                y2: self.y2,
            },
        ];

        for line in lines {
            line.draw(display, value);
        }
    }
}

impl<T: DynamicConsoleDisplay<S>, S: Pixel> DynamicDrawable<T, S>
    for Rectangle<Filled>
where
    [(); S::WIDTH * S::HEIGHT]:,
{
    fn draw(&self, display: &mut T, value: S::U) {
        for x in self.x1.round() as i32..=self.x2.round() as i32 {
            let line = Line {
                x1: x as f32,
                y1: self.y1,
                x2: x as f32,
                y2: self.y2,
            };
            line.draw(display, value);
        }
    }
}

pub struct Ellipse<FILL: FillType> {
    pub x: i32,
    pub y: i32,
    fill: PhantomData<FILL>,
}

impl<T: DynamicConsoleDisplay<S>, S: Pixel> DynamicDrawable<T, S>
    for Ellipse<NoFill>
where
    [(); S::WIDTH * S::HEIGHT]:,
{
    fn draw(&self, display: &mut T, value: S::U) {
        todo!();
    }
}

impl<T: DynamicConsoleDisplay<S>, S: Pixel> DynamicDrawable<T, S>
    for Ellipse<Filled>
where
    [(); S::WIDTH * S::HEIGHT]:,
{
    fn draw(&self, display: &mut T, value: S::U) {
        todo!();
    }
}
