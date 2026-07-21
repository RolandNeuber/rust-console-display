use core::f32;
use std::marker::PhantomData;

use num_traits::NumCast;

use crate::{
    drawing::{
        DynamicCanvas,
        DynamicDrawable,
        FillType,
        NoFill,
        line::Line,
    },
    pixel::Pixel,
};

// TODO: Check correctness of implementation
/// Defines a Circle by its midpoint and two vectors from the midpoint to the circumference.
#[derive(PartialEq, Debug)]
pub struct Ellipse<FILL: FillType> {
    pub midpoint_x: f32,
    pub midpoint_y: f32,
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub num_points: u32,
    pub fill: PhantomData<FILL>,
}

impl DynamicDrawable<3> for Ellipse<NoFill> {
    fn draw<T: DynamicCanvas<S>, S: Pixel>(
        &self,
        display: &mut T,
        value: S::U,
    ) where
        [(); S::WIDTH * S::HEIGHT]:,
    {
        let first_point;
        let mut last_point;
        let x = self.midpoint_x + self.x1;
        let y = self.midpoint_y + self.y1;
        let x = NumCast::from(x.round());
        let y = NumCast::from(y.round());
        if let Some(x) = x &&
            let Some(y) = y
        {
            first_point = (x, y);
            last_point = (x, y);
        }
        else {
            return;
        }

        for i in 1..self.num_points {
            let angle =
                f32::consts::TAU / self.num_points as f32 * i as f32;
            let x = self.x2.mul_add(
                f32::sin(angle),
                self.x1.mul_add(f32::cos(angle), self.midpoint_x),
            );
            let y = self.y2.mul_add(
                f32::sin(angle),
                self.y1.mul_add(f32::cos(angle), self.midpoint_y),
            );
            let x = NumCast::from(x.round());
            let y = NumCast::from(y.round());
            if let Some(x) = x &&
                let Some(y) = y
            {
                display.draw(
                    &Line {
                        x1: last_point.0,
                        y1: last_point.1,
                        x2: x,
                        y2: y,
                    },
                    value,
                );
                last_point = (x, y);
            }
            else {
                return;
            }
        }

        display.draw(
            &Line {
                x1: last_point.0,
                y1: last_point.1,
                x2: first_point.0,
                y2: first_point.1,
            },
            value,
        );
    }

    fn transform<F: Fn((f32, f32)) -> (f32, f32)>(
        &self,
        transform: F,
    ) -> Self {
        let trans_mid = transform((self.midpoint_x, self.midpoint_y));
        let trans_p1 = transform((self.x1, self.y1));
        let trans_p2 = transform((self.x2, self.y2));
        Self {
            midpoint_x: trans_mid.0,
            midpoint_y: trans_mid.1,
            x1: trans_p1.0,
            y1: trans_p1.1,
            x2: trans_p2.0,
            y2: trans_p2.1,
            num_points: self.num_points,
            fill: PhantomData::<NoFill>,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use super::*;
    use crate::drawing::{
        DynamicDrawable,
        NoFill,
    };

    #[test]
    fn transform() {
        let expected = Ellipse {
            x1: -8.,
            y1: -20.,
            x2: 12.,
            y2: 0.,
            midpoint_x: 1.,
            midpoint_y: -9.,
            num_points: 10,
            fill: PhantomData::<NoFill>,
        };
        let ellipse = Ellipse {
            x1: -10.,
            y1: -10.,
            x2: 10.,
            y2: 10.,
            midpoint_x: -1.,
            midpoint_y: 1.,
            num_points: 10,
            fill: PhantomData::<NoFill>,
        };
        let transform = ellipse.transform(|(x, y)| (x + 2., y - 10.));
        assert_eq!(expected, transform);
    }
}
