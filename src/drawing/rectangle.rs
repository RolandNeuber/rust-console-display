use core::f32;
use std::marker::PhantomData;

use crate::{
    drawing::{
        Drawable, DynamicCanvas, DynamicDrawable, FillType, Filled, NoFill, SetPixel, Transformable, line::Line
    },
    pixel::Pixel,
};

/// Defines a Rectangle by two corners.
#[derive(PartialEq, Debug)]
pub struct Rectangle<FILL: FillType> {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub fill: PhantomData<FILL>,
}

impl DynamicDrawable<2> for Rectangle<NoFill> {
    fn draw<T: DynamicCanvas<S>, S: Pixel>(
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
            DynamicDrawable::draw(&line, display, value);
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
            fill: PhantomData::<NoFill>,
        }
    }
}

impl Drawable<2> for Rectangle<NoFill> {
    fn draw<T: SetPixel<S>, S: Pixel>(
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
            Drawable::draw(&line, display, value);
        }
    }
}

impl Transformable for Rectangle<NoFill> {
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
            fill: PhantomData::<NoFill>,
        }
    }
}

impl DynamicDrawable<2> for Rectangle<Filled> {
    fn draw<T: DynamicCanvas<S>, S: Pixel>(
        &self,
        display: &mut T,
        value: S::U,
    ) where
        [(); S::WIDTH * S::HEIGHT]:,
    {
        #[allow(clippy::cast_possible_truncation)]
        for x in self.x1.round() as i32..=self.x2.round() as i32 {
            let line = Line {
                x1: x as f32,
                y1: self.y1,
                x2: x as f32,
                y2: self.y2,
            };
            DynamicDrawable::draw(&line, display, value);
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
            fill: PhantomData::<Filled>,
        }
    }
}

impl Drawable<2> for Rectangle<Filled> {
    fn draw<T: SetPixel<S>, S: Pixel>(
        &self,
        display: &mut T,
        value: S::U,
    ) where
        [(); S::WIDTH * S::HEIGHT]:,
    {
        #[allow(clippy::cast_possible_truncation)]
        for x in self.x1.round() as i32..=self.x2.round() as i32 {
            let line = Line {
                x1: x as f32,
                y1: self.y1,
                x2: x as f32,
                y2: self.y2,
            };
            Drawable::draw(&line, display, value);
        }
    }
}

impl Transformable for Rectangle<Filled> {
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
            fill: PhantomData::<Filled>,
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
        let expected = Rectangle {
            x1: -10.,
            y1: -10.,
            x2: 10.,
            y2: 10.,
            fill: PhantomData::<NoFill>,
        };
        let rect = Rectangle {
            x1: 0.,
            y1: 0.,
            x2: 10.,
            y2: 20.,
            fill: PhantomData::<NoFill>,
        };
        let transform = Transformable::transform(&rect, |(x, y)| (if x == 0. { -10. } else { x }, y - 10.));
        assert_eq!(expected, transform);
    }
}
