use std::{
    cell::{
        Cell,
        Ref,
        RefCell,
        RefMut,
    },
    fmt::Display,
    marker::PhantomData,
    mem,
    ops::{
        Deref,
        DerefMut,
    },
};

use rust_console_display_macros::{
    DynamicWidget,
    SingleWidget,
    StaticWidget,
};

use crate::{
    console_display::ConsoleDisplay,
    impl_new,
    impl_setters,
    pixel::monochrome_pixel::MultiPixel,
    pixel_display::StaticPixelDisplay,
    widget::DynamicWidget,
};

use super::StaticWidget;

pub trait SingleWidget<T: DynamicWidget>:
    DynamicWidget + Deref + DerefMut
{
    type Borrowed<'a>: Deref<Target = T>
    where
        T: 'a,
        Self: 'a;
    type BorrowedMut<'a>: DerefMut<Target = T>
    where
        T: 'a,
        Self: 'a;

    fn get_child(&self) -> Self::Borrowed<'_>;
    fn get_child_mut(&mut self) -> Self::BorrowedMut<'_>;
}

#[derive(StaticWidget, DynamicWidget)]
pub struct UvWidget<T: ConsoleDisplay<S>, S: MultiPixel> {
    pixel_type: PhantomData<S>,
    child: T,
    uv_x_min: f32,
    uv_x_max: f32,
    uv_y_min: f32,
    uv_y_max: f32,
}

impl<T: ConsoleDisplay<S>, S: MultiPixel> UvWidget<T, S> {
    pub fn new(child: T) -> Self {
        let (width, height) = (child.get_width(), child.get_height());
        Self {
            pixel_type: PhantomData::<S>,
            child,
            uv_x_min: 0.0,
            uv_x_max: width as f32,
            uv_y_min: 0.0,
            uv_y_max: height as f32,
        }
    }
}

impl<S: MultiPixel, const WIDTH: usize, const HEIGHT: usize>
    UvWidget<StaticPixelDisplay<S, WIDTH, HEIGHT>, S>
{
    impl_setters!(pub uv_x_min: f32, pub uv_x_max: f32, pub uv_y_min: f32, pub uv_y_max: f32);

    /// Gets the pixel at the _uv_ coordinate (x, y).
    /// Using coordinates outside the uv mapping is considered
    /// undefined behaviour at the moment and is subject to change.
    ///
    /// # Errors
    ///
    /// Returns an error if the pixel coordinates calculated by the UV mapping are out of bounds.
    pub fn get_pixel(&self, x: f32, y: f32) -> Result<S::U, String>
    where
        [(); S::WIDTH * S::HEIGHT]:,
    {
        let display = self.get_child();
        let uv = (
            Self::uv_to_texture(
                x,
                self.uv_x_min,
                self.uv_x_max,
                display.get_width(),
            ),
            Self::uv_to_texture(
                y,
                self.uv_y_min,
                self.uv_y_max,
                display.get_width(),
            ),
        );
        display.get_pixel(uv.0, uv.1)
    }

    /// Sets the pixel at the _uv_ coordinate (x, y).
    ///
    /// # Errors
    ///
    /// Returns an error if the coordinates are outside the uv mapping.
    pub fn set_pixel(
        &mut self,
        x: f32,
        y: f32,
        value: S::U,
    ) -> Result<(), String>
    where
        [(); S::WIDTH * S::HEIGHT]:,
    {
        let display = self.get_child();
        // Note: Checks need to consider that uv_max < uv_min.
        // While unintuitive, this is used to flip the uv mapping. (Especially with the y coordinate.)
        if x < self.uv_x_min.min(self.uv_x_max) ||
            x > self.uv_x_max.max(self.uv_x_min)
        {
            return Err("x is outside the uv bounds.".to_owned());
        }
        if y < self.uv_y_min.min(self.uv_y_max) ||
            y > self.uv_y_max.max(self.uv_y_min)
        {
            return Err("y is outside the uv bounds.".to_owned());
        }
        let uv = (
            Self::uv_to_texture(
                x,
                self.uv_x_min,
                self.uv_x_max,
                display.get_width(),
            ),
            Self::uv_to_texture(
                y,
                self.uv_y_min,
                self.uv_y_max,
                display.get_width(),
            ),
        );
        self.get_child_mut().set_pixel(uv.0, uv.1, value)
    }

    pub fn draw_line(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        value: S::U,
    ) where
        [(); S::WIDTH * S::HEIGHT]:,
    {
        let display = self.get_child();
        let uv1 = (
            Self::uv_to_texture_f32(
                x1,
                self.uv_x_min,
                self.uv_x_max,
                display.get_width() as f32,
            ),
            Self::uv_to_texture_f32(
                y1,
                self.uv_y_min,
                self.uv_y_max,
                display.get_width() as f32,
            ),
        );
        let uv2 = (
            Self::uv_to_texture_f32(
                x2,
                self.uv_x_min,
                self.uv_x_max,
                display.get_width() as f32,
            ),
            Self::uv_to_texture_f32(
                y2,
                self.uv_y_min,
                self.uv_y_max,
                display.get_width() as f32,
            ),
        );
        self.get_child_mut()
            .draw_line(uv1.0, uv1.1, uv2.0, uv2.1, value);
    }

    #[must_use]
    pub fn uv_x_to_texture(&self, x: f32) -> usize {
        Self::uv_to_texture(
            x,
            self.uv_x_min,
            self.uv_x_max,
            self.get_child().get_width(),
        )
    }

    #[must_use]
    pub fn uv_y_to_texture(&self, y: f32) -> usize {
        Self::uv_to_texture(
            y,
            self.uv_y_min,
            self.uv_y_max,
            self.get_child().get_height(),
        )
    }

    #[must_use]
    pub fn texture_to_uv_x(&self, x: usize) -> f32 {
        Self::texture_to_uv(
            x,
            self.get_child().get_width(),
            self.uv_x_min,
            self.uv_x_max,
        )
    }

    #[must_use]
    pub fn texture_to_uv_y(&self, y: usize) -> f32 {
        Self::texture_to_uv(
            y,
            self.get_child().get_height(),
            self.uv_y_min,
            self.uv_y_max,
        )
    }

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
    /// assert_eq!(vec![-0.7, -0.1, 0.5, 1.1, 1.7], widget.get_x_values().map(|x| (x * 100.).round() / 100.).collect::<Vec<_>>())
    /// ```
    pub fn get_x_values(
        &self,
    ) -> impl Iterator<Item = f32> + use<'_, S, WIDTH, HEIGHT> {
        let width = self.get_child().get_width();
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
    /// assert_eq!(vec![-0.8, -0.4, 0.0, 0.4, 0.8], widget.get_y_values().map(|y| (y * 100.).round() / 100.).collect::<Vec<_>>())
    /// ```
    pub fn get_y_values(
        &self,
    ) -> impl Iterator<Item = f32> + use<'_, S, WIDTH, HEIGHT> {
        let height = self.get_child().get_height();
        (0..height).map(|x| self.texture_to_uv_y(x))
    }
}

impl<T: ConsoleDisplay<S> + StaticWidget, S: MultiPixel> SingleWidget<T>
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

    fn get_child(&self) -> &T {
        &self.child
    }

    fn get_child_mut(&mut self) -> &mut T {
        &mut self.child
    }
}

impl<T: ConsoleDisplay<S> + StaticWidget, S: MultiPixel> Display
    for UvWidget<T, S>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_child())
    }
}

impl<T: ConsoleDisplay<S>, S: MultiPixel> Deref for UvWidget<T, S> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T: ConsoleDisplay<S>, S: MultiPixel> DerefMut for UvWidget<T, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.child
    }
}

#[derive(StaticWidget)]
pub struct DoubleBufferWidget<T: ConsoleDisplay<S>, S: MultiPixel> {
    pixel_type: PhantomData<S>,
    child: RefCell<T>,
    backbuffer: RefCell<Box<[S]>>,
    is_write: Cell<bool>,
}

impl<T: ConsoleDisplay<S>, S: MultiPixel> DoubleBufferWidget<T, S> {
    pub fn new(child: T) -> Self
    where
        [(); S::WIDTH * S::HEIGHT]:,
    {
        let pixels = child.get_data().to_vec().into_boxed_slice();
        Self {
            pixel_type: PhantomData::<S>,
            child: RefCell::new(child),
            backbuffer: RefCell::new(pixels),
            is_write: false.into(),
        }
    }

    #[allow(clippy::swap_with_temporary)]
    pub fn swap_buffers(&self) {
        mem::swap(
            self.child.borrow_mut().get_data_mut(),
            &mut self.backbuffer.borrow_mut(),
        );
    }
}

impl<T: ConsoleDisplay<S>, S: MultiPixel> DynamicWidget
    for DoubleBufferWidget<T, S>
{
    fn get_width_characters(&self) -> usize {
        self.child.borrow().get_width_characters()
    }

    fn get_height_characters(&self) -> usize {
        self.child.borrow().get_height_characters()
    }
}

impl<T: ConsoleDisplay<S>, S: MultiPixel> SingleWidget<T>
    for DoubleBufferWidget<T, S>
{
    type Borrowed<'a>
        = Ref<'a, T>
    where
        T: 'a,
        Self: 'a;

    type BorrowedMut<'a>
        = RefMut<'a, T>
    where
        T: 'a,
        Self: 'a;

    fn get_child(&self) -> Ref<'_, T> {
        self.child.borrow()
    }

    fn get_child_mut(&mut self) -> RefMut<'_, T> {
        self.child.borrow_mut()
    }
}

impl<T: ConsoleDisplay<S>, S: MultiPixel> Display
    for DoubleBufferWidget<T, S>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_write.get() {
            self.swap_buffers();
            self.is_write.set(false);
        }
        self.child.borrow().fmt(f)
    }
}

impl<T: ConsoleDisplay<S>, S: MultiPixel> Deref
    for DoubleBufferWidget<T, S>
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        if self.is_write.get() {
            self.swap_buffers();
            self.is_write.set(false);
        }
        // TODO: Make this implementation safe
        unsafe { &*self.child.as_ptr() }
    }
}

impl<T: ConsoleDisplay<S>, S: MultiPixel> DerefMut
    for DoubleBufferWidget<T, S>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        if !self.is_write.get() {
            self.swap_buffers();
            self.is_write.set(true);
        }
        self.child.get_mut()
    }
}

#[derive(SingleWidget)]
pub struct PaddingWidget<T: DynamicWidget> {
    child: T,
    padding_left: usize,
    padding_right: usize,
    padding_top: usize,
    padding_bottom: usize,
}

impl<T: DynamicWidget> PaddingWidget<T> {
    impl_new!(pub PaddingWidget, <, T, >, child: T, padding_left: usize, padding_right: usize, padding_top: usize, padding_bottom: usize);

    impl_setters!(pub padding_left: usize, pub padding_right: usize, pub padding_top: usize, pub padding_bottom: usize);
}

impl<T: DynamicWidget> DynamicWidget for PaddingWidget<T> {
    fn get_width_characters(&self) -> usize {
        self.child.get_width_characters() +
            self.padding_left +
            self.padding_right
    }

    fn get_height_characters(&self) -> usize {
        self.child.get_height_characters() +
            self.padding_top +
            self.padding_bottom
    }
}

impl<T: DynamicWidget> Display for PaddingWidget<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str_repr = String::new();
        for _ in 0..self.padding_top {
            str_repr.push_str(&" ".repeat(self.get_width_characters()));
            str_repr.push_str("\r\n");
        }
        let str_repr_child = self.child.to_string();
        for line_child in str_repr_child.lines() {
            str_repr.push_str(&" ".repeat(self.padding_left));
            str_repr.push_str(line_child);
            str_repr.push_str(&" ".repeat(self.padding_right));
            str_repr.push_str("\r\n");
        }
        for _ in 0..self.padding_bottom {
            str_repr.push_str(&" ".repeat(self.get_width_characters()));
            str_repr.push_str("\r\n");
        }
        str_repr = str_repr.trim_end_matches("\r\n").to_string();
        write!(f, "{str_repr}")
    }
}

impl<T: DynamicWidget> Deref for PaddingWidget<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get_child()
    }
}

impl<T: DynamicWidget> DerefMut for PaddingWidget<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_child_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        pixel::monochrome_pixel::SinglePixel,
        pixel_display::StaticPixelDisplay,
    };

    use super::*;

    mod uv_widget {
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

    mod double_buffer_widget {
        use super::*;

        #[test]
        fn buffer_swap() {
            let mut widget =
                DoubleBufferWidget::new(StaticPixelDisplay::<
                    SinglePixel,
                    1,
                    1,
                >::new(false));
            widget.set_pixel_static::<0, 0>(true);
            let buffer1 = widget.backbuffer.clone();
            widget.swap_buffers();
            let buffer2 = widget.backbuffer;
            assert_ne!(
                buffer1.borrow()[0].to_string(),
                buffer2.borrow()[0].to_string()
            )
        }
    }
    mod padding_widget {
        use super::*;

        #[test]
        fn dimensions() {
            let widget = PaddingWidget::new(
                StaticPixelDisplay::<SinglePixel, 1, 1>::new(false),
                10,
                20,
                30,
                40,
            );
            assert_eq!(widget.get_width_characters(), 31);
            assert_eq!(widget.get_height_characters(), 71)
        }
    }
}
