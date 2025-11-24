use core::f32;
use std::marker::PhantomData;

use num_traits::NumCast;

use crate::{
    error::DrawingError,
    pixel::Pixel,
    widget::DynamicWidget,
};

/// Defines a fill for a drawable.
pub const trait FillType {}

/// Defines no fill on a drawable, e.g. only outline.
#[derive(PartialEq, Eq, Debug)]
pub struct NoFill;
impl const FillType for NoFill {}

/// Defines flat fill on a drawable.
#[derive(PartialEq, Eq, Debug)]
pub struct Filled;
impl const FillType for Filled {}

/// Defines an object that you can draw on and query pixels from.
pub const trait DynamicCanvas<S: Pixel>: DynamicWidget {
    type A: NumCast;
    /// Returns a bool representing the state of the pixel at the specified coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// #![allow(incomplete_features)]
    /// #![feature(generic_const_exprs)]
    ///
    /// use console_display::{
    ///     console_display::DynamicConsoleDisplay,
    ///     display_driver::DisplayDriver,
    ///     pixel::monochrome_pixel::SinglePixel,
    ///     pixel_display::DynamicPixelDisplay,
    ///     drawing::DynamicCanvas
    /// };
    ///
    /// let disp: DisplayDriver<DynamicPixelDisplay<SinglePixel>> = DisplayDriver::new(
    ///     DynamicPixelDisplay::<SinglePixel>::build_from_data(
    ///         6,
    ///         6,
    ///         &vec![
    ///             true, true, true, true,  true, true, // 0
    ///             true, true, true, true,  true, true, // 1
    ///             true, true, true, false, true, true, //-2-
    ///             true, true, true, true,  true, true, // 3
    ///             true, true, true, true,  true, true, // 4
    ///             true, true, true, true,  true, true, // 5
    ///         ] //  0     1     2   --3--    4     5
    ///     ).expect("Could not construct display.")
    /// );
    /// // Replace with actual error handling
    ///
    /// let pixel = disp.pixel(3, 2);
    ///
    /// assert_eq!(pixel, Ok(false));
    ///
    /// let pixel = disp.pixel(5, 6);
    ///
    /// assert!(matches!(pixel, Err(_)));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the pixel coordinates are out of bounds.
    ///
    /// # Panics
    ///
    /// If the index of a subpixel is out of bounds.
    /// This should not happen and is subject to change in the future.
    fn pixel(&self, x: Self::A, y: Self::A) -> Result<S::U, DrawingError>
    where
        [(); S::WIDTH * S::HEIGHT]:;

    /// Set a pixel at the specified coordinate with a given value.
    ///
    /// # Errors
    ///
    /// Returns an error if the pixel coordinates are out of bounds.
    ///
    /// # Panics
    ///
    /// If the index of a subpixel is out of bounds.
    /// This should not happen and is subject to change in the future.
    fn set_pixel(
        &mut self,
        x: Self::A,
        y: Self::A,
        value: S::U,
    ) -> Result<(), DrawingError>
    where
        [(); S::WIDTH * S::HEIGHT]:;

    /// Draw a drawable/shape onto a canvas with the specified pixel type/brush.
    /// Convenience method for inversing `DynamicDrawable::draw` by using double dispatch.
    fn draw<D: [const] DynamicDrawable<N>, const N: usize>(
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

/// Defines an object that can be drawn onto a canvas.
pub const trait DynamicDrawable<const N: usize> {
    /// Draws the drawable onto a canvas with the specified pixel type/brush.
    fn draw<T: DynamicCanvas<S>, S: Pixel>(
        &self,
        display: &mut T,
        value: S::U,
    ) where
        [(); S::WIDTH * S::HEIGHT]:;

    /// Transforms the drawable by applying a function to all coordinate pairs that define it.
    /// Returns a new transformed drawable.
    #[must_use]
    fn transform<F: [const] Fn((f32, f32)) -> (f32, f32)>(
        &self,
        transform: F,
    ) -> Self;
}

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
            line.draw(display, value);
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
            line.draw(display, value);
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
    mod line {
        use crate::drawing::{
            DynamicDrawable,
            Line,
        };

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
            let transform = line.transform(|(x, y)| (x + 1., y));
            assert_eq!(expected, transform);
        }
    }

    mod rectagle {
        use std::marker::PhantomData;

        use crate::drawing::{
            DynamicDrawable,
            NoFill,
            Rectangle,
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
            let transform = rect.transform(|(x, y)| {
                (if x == 0. { -10. } else { x }, y - 10.)
            });
            assert_eq!(expected, transform);
        }
    }
}
