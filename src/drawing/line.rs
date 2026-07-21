use core::f32;

use num_traits::NumCast;

use crate::{
    drawing::{
        Drawable, DynamicCanvas, DynamicDrawable, SetPixel, Transformable
    },
    pixel::Pixel,
};

/// Defines a line primitive by two endpoints.
#[derive(PartialEq, Debug)]
pub struct Line {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

impl DynamicDrawable<2> for Line {
    fn draw<T: DynamicCanvas<S>, S: Pixel>(
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
                if let Some(x) = x &&
                    let Some(y) = y
                {
                    let _ = display.set_pixel(x, y, value);
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
        Self {
            x1: trans_p1.0,
            y1: trans_p1.1,
            x2: trans_p2.0,
            y2: trans_p2.1,
        }
    }
}

impl Drawable<2> for Line {
    fn draw<T: SetPixel<S>, S: Pixel>(
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
                if let Some(x) = x &&
                    let Some(y) = y
                {
                    let _ = display.set_pixel(x, y, value);
                }
            }
            x += x_inc;
            y += y_inc;
        }
    }
}

impl Transformable for Line {
    fn transform<F: Fn((f32, f32)) -> (f32, f32)>(
        &self,
        transform: F,
    ) -> Self {
        let trans_p1 = transform((self.x1, self.y1));
        let trans_p2 = transform((self.x2, self.y2));
        Self {
            x1: trans_p1.0,
            y1: trans_p1.1,
            x2: trans_p2.0,
            y2: trans_p2.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform() {
        let expected = Line {
            x1: 1.,
            y1: 1.,
            x2: 3.,
            y2: 3.,
        };
        let line = Line {
            x1: 0.,
            y1: 1.,
            x2: 2.,
            y2: 3.,
        };
        let transform = Transformable::transform(&line, |(x, y)| (x + 1., y));
        assert_eq!(expected, transform);
    }
}
