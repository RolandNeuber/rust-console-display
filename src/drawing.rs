use std::{
    array,
    marker::PhantomData,
};

use num_traits::{Num, NumCast};

use crate::{
    pixel::Pixel,
    widget::{
        DynamicWidget,
        StaticWidget,
    },
};

pub trait FillType {}

pub struct NoFill;
impl FillType for NoFill {}
pub struct Filled;
impl FillType for Filled {}

/// TODO: Improve docs
/// Implement on all displays and widgets you can draw on and query pixels from.
pub trait DynamicCanvas<S: Pixel, A: NumCast>: DynamicWidget {
    fn pixel(&self, x: A, y: A) -> Result<S::U, String>
    where
        [(); S::WIDTH * S::HEIGHT]:;

    fn set_pixel(&mut self, x: A, y: A, value: S::U) -> Result<(), String>
    where
        [(); S::WIDTH * S::HEIGHT]:;

    fn draw<D: DynamicDrawable<N>, const N: usize>(
        &mut self,
        drawable: &D,
        value: S::U,
    ) where
        Self: Sized,
        [(); S::WIDTH * S::HEIGHT]:,
    {
        drawable.draw(self, value);
    }
}

pub trait DynamicDrawable<const N: usize> {
    fn draw<T: DynamicCanvas<S, A>, S: Pixel, A: NumCast>(
        &self,
        display: &mut T,
        value: S::U,
    ) where
        [(); S::WIDTH * S::HEIGHT]:;
    fn transform<F: Fn((f32, f32)) -> (f32, f32)>(
        &self,
        transform: F,
    ) -> Self;
}

pub struct Line {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

impl DynamicDrawable<2> for Line {
    fn draw<T: DynamicCanvas<S, A>, S: Pixel, A: NumCast>(
        &self,
        display: &mut T,
        value: S::U,
    ) where
        [(); S::WIDTH * S::HEIGHT]:,
    {
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
                let x = NumCast::from(x.round());
                let y = NumCast::from(y.round());
                if let Some(x) = x && let Some(y) = y {
                    let _ = display.set_pixel(
                        x,
                        y,
                        value,
                    );
                }
            }
            x += x_inc;
            y += y_inc;
        }
    }

    fn transform<F: Fn((f32, f32)) -> (f32, f32)>(
        &self,
        transform: F,
    ) -> Self {
        let trans_p1 = transform((self.x1, self.y1));
        let trans_p2 = transform((self.x2, self.y2));
        Line {
            x1: trans_p1.0,
            y1: trans_p1.1,
            x2: trans_p2.0,
            y2: trans_p2.1,
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

impl DynamicDrawable<2> for Rectangle<NoFill> {
    fn draw<T: DynamicCanvas<S, A>, S: Pixel, A: NumCast>(
        &self,
        display: &mut T,
        value: S::U,
    ) where
        [(); S::WIDTH * S::HEIGHT]:,
    {
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

    fn transform<F: Fn((f32, f32)) -> (f32, f32)>(
        &self,
        transform: F,
    ) -> Self {
        let trans_p1 = transform((self.x1, self.y1));
        let trans_p2 = transform((self.x2, self.y2));
        Rectangle {
            x1: trans_p1.0,
            y1: trans_p1.1,
            x2: trans_p2.0,
            y2: trans_p2.1,
            fill: PhantomData::<NoFill>,
        }
    }
}

impl DynamicDrawable<2> for Rectangle<Filled> {
    fn draw<T: DynamicCanvas<S, A>, S: Pixel, A: NumCast>(
        &self,
        display: &mut T,
        value: S::U,
    ) where
        [(); S::WIDTH * S::HEIGHT]:,
    {
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

    fn transform<F: Fn((f32, f32)) -> (f32, f32)>(
        &self,
        transform: F,
    ) -> Self {
        let trans_p1 = transform((self.x1, self.y1));
        let trans_p2 = transform((self.x2, self.y2));
        Rectangle {
            x1: trans_p1.0,
            y1: trans_p1.1,
            x2: trans_p2.0,
            y2: trans_p2.1,
            fill: PhantomData::<Filled>,
        }
    }
}
