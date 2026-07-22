use std::{
    marker::PhantomData,
    ops::{
        Deref,
        DerefMut,
    },
};

use console_display_macros::{
    DynamicWidget,
    StaticWidget,
};
use crossterm::terminal::window_size;
use num_traits::NumCast;

use crate::{
    console_display::DynamicConsoleDisplay,
    drawing::DynamicCanvas,
    error::{
        COULD_NOT_CAST_X_COORD,
        COULD_NOT_CAST_Y_COORD,
        DrawingError,
        WidgetError,
    },
    impl_setters,
    pixel::Pixel,
    widget::single_widget::SingleWidgetOld,
};

use crate::widget::{
    DynamicWidget,
    StaticWidget,
};

#[derive(StaticWidget, DynamicWidget, Debug, Clone, PartialEq)]
pub struct UvWidget<T: DynamicConsoleDisplay<S>, S: Pixel> {
    pixel_type: PhantomData<S>,
    child: T,
    uv_x_min: f32,
    uv_x_max: f32,
    uv_y_min: f32,
    uv_y_max: f32,
}

impl<T: DynamicConsoleDisplay<S>, S: Pixel> DynamicCanvas<S>
    for UvWidget<T, S>
{
    type A = f32;

    /// Gets the pixel at the _uv_ coordinate (x, y).
    /// Using coordinates outside the uv mapping is considered
    /// undefined behaviour at the moment and is subject to change.
    ///
    /// # Errors
    ///
    /// Returns an error if the pixel coordinates calculated by the UV mapping are out of bounds.
    fn pixel(
        &self,
        x: Self::A,
        y: Self::A,
    ) -> Result<<S as Pixel>::U, DrawingError>
    where
        [(); <S as Pixel>::WIDTH * <S as Pixel>::HEIGHT]:,
    {
        let display = self.child();
        let uv = (
            Self::uv_to_texture(
                x,
                self.uv_x_min,
                self.uv_x_max,
                display.width(),
            ),
            Self::uv_to_texture(
                y,
                self.uv_y_min,
                self.uv_y_max,
                display.width(),
            ),
        );
        display.pixel(
            NumCast::from(uv.0).expect(COULD_NOT_CAST_X_COORD),
            NumCast::from(uv.1).expect(COULD_NOT_CAST_Y_COORD),
        )
    }

    /// Sets the pixel at the _uv_ coordinate (x, y).
    ///
    /// # Errors
    ///
    /// Returns an error if the coordinates are outside the uv mapping.
    fn set_pixel(
        &mut self,
        x: Self::A,
        y: Self::A,
        value: <S as Pixel>::U,
    ) -> Result<(), DrawingError>
    where
        [(); <S as Pixel>::WIDTH * <S as Pixel>::HEIGHT]:,
    {
        let display = self.child();
        // Note: Checks need to consider that uv_max < uv_min.
        // While unintuitive, this is used to flip the uv mapping. (Especially with the y coordinate.)
        if x < self.uv_x_min.min(self.uv_x_max) ||
            x > self.uv_x_max.max(self.uv_x_min)
        {
            return Err(WidgetError::UvCoordinateOutOfBounds('x'))?;
        }
        if y < self.uv_y_min.min(self.uv_y_max) ||
            y > self.uv_y_max.max(self.uv_y_min)
        {
            return Err(WidgetError::UvCoordinateOutOfBounds('y'))?;
        }
        let uv = (
            Self::uv_to_texture(
                x,
                self.uv_x_min,
                self.uv_x_max,
                display.width(),
            ),
            Self::uv_to_texture(
                y,
                self.uv_y_min,
                self.uv_y_max,
                display.height(),
            ),
        );
        self.child_mut().set_pixel(
            NumCast::from(uv.0).expect(COULD_NOT_CAST_X_COORD),
            NumCast::from(uv.1).expect(COULD_NOT_CAST_Y_COORD),
            value,
        )
    }

    fn draw<D: crate::drawing::DynamicDrawable<N>, const N: usize>(
        &mut self,
        drawable: &D,
        value: <S as Pixel>::U,
    ) where
        Self: Sized,
        [(); <S as Pixel>::WIDTH * <S as Pixel>::HEIGHT]:,
    {
        let drawable = drawable.transform(|(x, y)| {
            (
                Self::uv_to_texture_f32(
                    x,
                    self.uv_x_min,
                    self.uv_x_max,
                    self.width() as f32,
                ),
                Self::uv_to_texture_f32(
                    y,
                    self.uv_y_min,
                    self.uv_y_max,
                    self.height() as f32,
                ),
            )
        });
        drawable.draw(self.child_mut(), value);
    }
}

impl<T: DynamicConsoleDisplay<S>, S: Pixel> UvWidget<T, S> {
    pub fn new(child: T) -> Self {
        let (width, height) = (child.width(), child.height());
        Self {
            pixel_type: PhantomData::<S>,
            child,
            uv_x_min: 0.0,
            uv_x_max: width as f32,
            uv_y_min: 0.0,
            uv_y_max: height as f32,
        }
    }

    pub fn new_with_aspect_ratio(child: T) -> Self {
        let (width, height) = (child.width(), child.height());

        let (fragment_width, fragment_height) = if let Ok(size) =
            window_size() &&
            size.width != 0 &&
            size.height != 0
        {
            (
                <f32 as std::convert::From<u16>>::from(size.width) /
                    <f32 as std::convert::From<u16>>::from(size.columns),
                <f32 as std::convert::From<u16>>::from(size.height) /
                    <f32 as std::convert::From<u16>>::from(size.rows),
            )
        }
        else {
            (9., 19.)
        };

        let pixel_aspect = fragment_height / fragment_width;

        Self {
            pixel_type: PhantomData::<S>,
            child,
            uv_x_min: 0.0,
            uv_x_max: width as f32,
            uv_y_min: 0.0,
            uv_y_max: height as f32 * pixel_aspect,
        }
    }
}

impl<S: Pixel, T: DynamicConsoleDisplay<S>> UvWidget<T, S> {
    impl_setters!(pub const uv_x_min: f32, pub const uv_x_max: f32, pub const uv_y_min: f32, pub const uv_y_max: f32);

    #[must_use]
    pub fn uv_x_to_texture(&self, x: f32) -> usize {
        Self::uv_to_texture(
            x,
            self.uv_x_min,
            self.uv_x_max,
            self.child().width(),
        )
    }

    #[must_use]
    pub fn uv_y_to_texture(&self, y: f32) -> usize {
        Self::uv_to_texture(
            y,
            self.uv_y_min,
            self.uv_y_max,
            self.child().height(),
        )
    }

    #[must_use]
    pub fn texture_to_uv_x(&self, x: usize) -> f32 {
        Self::texture_to_uv(
            x,
            self.child().width(),
            self.uv_x_min,
            self.uv_x_max,
        )
    }

    #[must_use]
    pub fn texture_to_uv_y(&self, y: usize) -> f32 {
        Self::texture_to_uv(
            y,
            self.child().height(),
            self.uv_y_min,
            self.uv_y_max,
        )
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn uv_to_texture(
        uv: f32,
        uv_min: f32,
        uv_max: f32,
        texture_coordinate_max: usize,
    ) -> usize {
        ((uv - uv_min) / (uv_max - uv_min))
            .mul_add(texture_coordinate_max as f32, -0.5)
            .round() as usize
    }

    fn uv_to_texture_f32(
        uv: f32,
        uv_min: f32,
        uv_max: f32,
        texture_coordinate_max: f32,
    ) -> f32 {
        ((uv - uv_min) / (uv_max - uv_min))
            .mul_add(texture_coordinate_max, -0.5)
            .round()
    }

    fn texture_to_uv(
        texture_coordinate: usize,
        texture_coordinate_max: usize,
        uv_min: f32,
        uv_max: f32,
    ) -> f32 {
        ((texture_coordinate as f32 + 0.5) / texture_coordinate_max as f32)
            .mul_add(uv_max - uv_min, uv_min)
    }

    /// Returns an iterator that contains the uv x coordinates of the underlying display in ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// #![allow(incomplete_features)]
    /// #![feature(generic_const_exprs)]
    ///
    /// use console_display::{
    ///     widget::single_widget::UvWidget,
    ///     pixel_display::StaticPixelDisplay,
    ///     pixel::monochrome_pixel::SinglePixel,
    /// };
    ///
    /// let mut widget = UvWidget::new(
    ///     StaticPixelDisplay::<SinglePixel, 5, 1>::new(
    ///         false
    ///     )
    /// );
    ///
    /// widget.set_uv_x_min(-1.);
    /// widget.set_uv_x_max(2.);
    ///
    /// assert_eq!(vec![-0.7, -0.1, 0.5, 1.1, 1.7], widget.x_values().map(|x| (x * 100.).round() / 100.).collect::<Vec<_>>())
    /// ```
    pub fn x_values(&self) -> impl Iterator<Item = f32> {
        let width = self.child().width();
        (0..width).map(|x| self.texture_to_uv_x(x))
    }

    /// Returns an iterator that contains the uv y coordinates of the underlying display in ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// #![allow(incomplete_features)]
    /// #![feature(generic_const_exprs)]
    ///
    /// use console_display::{
    ///     widget::single_widget::UvWidget,
    ///     pixel_display::StaticPixelDisplay,
    ///     pixel::monochrome_pixel::SinglePixel,
    /// };
    ///
    /// let mut widget = UvWidget::new(
    ///     StaticPixelDisplay::<SinglePixel, 1, 5>::new(
    ///         false
    ///     )
    /// );
    ///
    /// widget.set_uv_y_min(-1.);
    /// widget.set_uv_y_max(1.);
    ///
    /// assert_eq!(vec![-0.8, -0.4, 0.0, 0.4, 0.8], widget.y_values().map(|y| (y * 100.).round() / 100.).collect::<Vec<_>>())
    /// ```
    pub fn y_values(&self) -> impl Iterator<Item = f32> {
        let height = self.child().height();
        (0..height).map(|x| self.texture_to_uv_y(x))
    }
}

impl<T: DynamicConsoleDisplay<S>, S: Pixel> const SingleWidgetOld<T>
    for UvWidget<T, S>
{
    type Borrowed<'a>
        = &'a T
    where
        T: 'a,
        S: 'a;

    type BorrowedMut<'a>
        = &'a mut T
    where
        T: 'a,
        Self: 'a;

    fn child(&self) -> &T {
        &self.child
    }

    fn child_mut(&mut self) -> &mut T {
        &mut self.child
    }
}

impl<T: DynamicConsoleDisplay<S>, S: Pixel> const Deref
    for UvWidget<T, S>
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T: DynamicConsoleDisplay<S>, S: Pixel> const DerefMut
    for UvWidget<T, S>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.child
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        pixel::monochrome_pixel::SinglePixel,
        pixel_display::StaticPixelDisplay,
    };

    use super::*;

    #[test]
    fn texture_to_uv() {
        let expected = 0.0005;
        let actual = UvWidget::<
            StaticPixelDisplay<SinglePixel, 1, 1>,
            SinglePixel,
        >::texture_to_uv(500, 1000, -0.5, 0.5);
        let error = expected * 0.0001;
        assert!((actual - 0.0005).abs() < error);
    }

    #[test]
    fn uv_to_texture() {
        let expected = 1500;
        let actual = UvWidget::<
            StaticPixelDisplay<SinglePixel, 1, 1>,
            SinglePixel,
        >::uv_to_texture(0.5, -1.0, 1.0, 2000);
        assert_eq!(actual, expected);
    }
}
