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

use console_display_macros::{
    DynamicWidget,
    SingleWidget,
    StaticWidget,
};

use crate::{
    console_display::ConsoleDisplay,
    impl_display_for_dynamic_widget,
    impl_getters,
    impl_new,
    impl_setters,
    pixel::{
        character_pixel::CharacterPixel,
        monochrome_pixel::MultiPixel,
    },
    pixel_display::StaticPixelDisplay,
    widget::{
        DataCell,
        DynamicWidget,
    },
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

    fn child(&self) -> Self::Borrowed<'_>;
    fn child_mut(&mut self) -> Self::BorrowedMut<'_>;
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
    pub fn pixel(&self, x: f32, y: f32) -> Result<S::U, String>
    where
        [(); S::WIDTH * S::HEIGHT]:,
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
        display.pixel(uv.0, uv.1)
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
        let display = self.child();
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
                display.width(),
            ),
            Self::uv_to_texture(
                y,
                self.uv_y_min,
                self.uv_y_max,
                display.width(),
            ),
        );
        self.child_mut().set_pixel(uv.0, uv.1, value)
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
        let display = self.child();
        let uv1 = (
            Self::uv_to_texture_f32(
                x1,
                self.uv_x_min,
                self.uv_x_max,
                display.width() as f32,
            ),
            Self::uv_to_texture_f32(
                y1,
                self.uv_y_min,
                self.uv_y_max,
                display.width() as f32,
            ),
        );
        let uv2 = (
            Self::uv_to_texture_f32(
                x2,
                self.uv_x_min,
                self.uv_x_max,
                display.width() as f32,
            ),
            Self::uv_to_texture_f32(
                y2,
                self.uv_y_min,
                self.uv_y_max,
                display.width() as f32,
            ),
        );
        self.child_mut()
            .draw_line_f32(uv1.0, uv1.1, uv2.0, uv2.1, value);
    }

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
    pub fn x_values(
        &self,
    ) -> impl Iterator<Item = f32> + use<'_, S, WIDTH, HEIGHT> {
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
    pub fn y_values(
        &self,
    ) -> impl Iterator<Item = f32> + use<'_, S, WIDTH, HEIGHT> {
        let height = self.child().height();
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

    fn child(&self) -> &T {
        &self.child
    }

    fn child_mut(&mut self) -> &mut T {
        &mut self.child
    }
}

impl<T: ConsoleDisplay<S>, S: MultiPixel> Display for UvWidget<T, S> {
    impl_display_for_dynamic_widget!();
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
        let pixels = child.data().to_vec().into_boxed_slice();
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
            self.child.borrow_mut().data_mut(),
            &mut self.backbuffer.borrow_mut(),
        );
    }
}

impl<T: ConsoleDisplay<S>, S: MultiPixel> DynamicWidget
    for DoubleBufferWidget<T, S>
{
    fn width_characters(&self) -> usize {
        self.child.borrow().width_characters()
    }

    fn height_characters(&self) -> usize {
        self.child.borrow().height_characters()
    }

    fn string_data(&self) -> Vec<Vec<DataCell>> {
        if self.is_write.get() {
            self.swap_buffers();
            self.is_write.set(false);
        }
        self.child.borrow().string_data()
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

    fn child(&self) -> Ref<'_, T> {
        self.child.borrow()
    }

    fn child_mut(&mut self) -> RefMut<'_, T> {
        self.child.borrow_mut()
    }
}

impl<T: ConsoleDisplay<S>, S: MultiPixel> Display
    for DoubleBufferWidget<T, S>
{
    impl_display_for_dynamic_widget!();
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
    fn width_characters(&self) -> usize {
        self.child.width_characters() +
            self.padding_left +
            self.padding_right
    }

    fn height_characters(&self) -> usize {
        self.child.height_characters() +
            self.padding_top +
            self.padding_bottom
    }

    fn string_data(&self) -> Vec<Vec<DataCell>> {
        let mut data = self.child.string_data();
        let padding_top = vec![
            vec![
                CharacterPixel::default().into();
                self.width_characters()
            ];
            self.padding_top
        ];
        let padding_bottom = vec![
            vec![
                CharacterPixel::default().into();
                self.width_characters()
            ];
            self.padding_bottom
        ];
        data = data
            .into_iter()
            .map(|line| {
                [
                    vec![
                        CharacterPixel::default().into();
                        self.padding_left
                    ],
                    line,
                    vec![
                        CharacterPixel::default().into();
                        self.padding_right
                    ],
                ]
                .concat()
            })
            .collect();
        [padding_top, data, padding_bottom].concat()
    }
}

impl<T: DynamicWidget> Display for PaddingWidget<T> {
    impl_display_for_dynamic_widget!();
}

impl<T: DynamicWidget> Deref for PaddingWidget<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.child()
    }
}

impl<T: DynamicWidget> DerefMut for PaddingWidget<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.child_mut()
    }
}

pub trait Border {
    fn border_at(
        &self,
        width: usize,
        height: usize,
    ) -> impl Fn(usize, usize) -> CharacterPixel;
    fn width_top(&self) -> usize;
    fn width_left(&self) -> usize;
    fn width_bottom(&self) -> usize;
    fn width_right(&self) -> usize;
}

pub struct BorderDefault {
    top: CharacterPixel,
    top_left: CharacterPixel,
    left: CharacterPixel,
    bottom_left: CharacterPixel,
    bottom: CharacterPixel,
    bottom_right: CharacterPixel,
    right: CharacterPixel,
    top_right: CharacterPixel,
}

impl BorderDefault {
    impl_new!(
        #[must_use] pub BorderDefault,
        top: CharacterPixel,
        top_left: CharacterPixel,
        left: CharacterPixel,
        bottom_left: CharacterPixel,
        bottom: CharacterPixel,
        bottom_right: CharacterPixel,
        right: CharacterPixel,
        top_right: CharacterPixel
    );

    impl_getters!(
        #[must_use] pub top: CharacterPixel,
        #[must_use] pub top_left: CharacterPixel,
        #[must_use] pub left: CharacterPixel,
        #[must_use] pub bottom_left: CharacterPixel,
        #[must_use] pub bottom: CharacterPixel,
        #[must_use] pub bottom_right: CharacterPixel,
        #[must_use] pub right: CharacterPixel,
        #[must_use] pub top_right: CharacterPixel
    );
}

impl Border for BorderDefault {
    fn border_at(
        &self,
        width: usize,
        height: usize,
    ) -> impl Fn(usize, usize) -> CharacterPixel {
        move |x: usize, y: usize| match (x, y) {
            (0, 0) => self.top_left,
            (0, y) if y == height - 1 => self.bottom_left,
            (x, y) if x == width - 1 && y == height - 1 => {
                self.bottom_right
            }
            (x, 0) if x == width - 1 => self.top_right,
            (_, 0) => self.top,
            (0, _) => self.left,
            (_, y) if y == height - 1 => self.bottom,
            (x, _) if x == width - 1 => self.right,
            (_, _) => CharacterPixel::default(),
        }
    }

    fn width_top(&self) -> usize {
        1
    }

    fn width_left(&self) -> usize {
        1
    }

    fn width_bottom(&self) -> usize {
        1
    }

    fn width_right(&self) -> usize {
        1
    }
}

pub struct BorderWidget<T: DynamicWidget, S: Border> {
    child: T,
    border: S,
}

impl<T: DynamicWidget, S: Border> BorderWidget<T, S> {
    impl_new!(pub BorderWidget, <, T, S, >, child: T, border: S);
}

impl<T: DynamicWidget, S: Border> DynamicWidget for BorderWidget<T, S> {
    fn width_characters(&self) -> usize {
        self.child.width_characters() +
            self.border.width_left() +
            self.border.width_right()
    }

    fn height_characters(&self) -> usize {
        self.child.height_characters() +
            self.border.width_top() +
            self.border.width_bottom()
    }

    fn string_data(&self) -> Vec<Vec<DataCell>> {
        let border_at = self
            .border
            .border_at(self.width_characters(), self.height_characters());
        let mut data = self.child.string_data();
        let border_top = (0..self.border.width_top())
            .map(|y| {
                (0..self.width_characters())
                    .map(|x| border_at(x, y).into())
                    .collect()
            })
            .collect();
        let border_bottom = (self.height_characters() -
            self.border.width_bottom()..
            self.height_characters())
            .map(|y| {
                (0..self.width_characters())
                    .map(|x| border_at(x, y).into())
                    .collect()
            })
            .collect();
        data = data
            .into_iter()
            .enumerate()
            .map(|(y, line)| {
                [
                    (0..self.border.width_left())
                        .map(|x| {
                            border_at(x, y + self.border.width_top())
                                .into()
                        })
                        .collect(),
                    line,
                    (0..self.border.width_right())
                        .map(|x| {
                            border_at(x, y + self.border.width_top())
                                .into()
                        })
                        .collect(),
                ]
                .concat()
            })
            .collect();
        [border_top, data, border_bottom].concat()
    }
}

impl<T: DynamicWidget, S: Border> Display for BorderWidget<T, S> {
    impl_display_for_dynamic_widget!();
}

impl<T: DynamicWidget, S: Border> Deref for BorderWidget<T, S> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T: DynamicWidget, S: Border> DerefMut for BorderWidget<T, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.child
    }
}

pub struct InsetWidget<T: DynamicWidget> {
    child: T,
    inset_left: usize,
    inset_right: usize,
    inset_top: usize,
    inset_bottom: usize,
}

impl<T: DynamicWidget> InsetWidget<T> {
    impl_new!(pub InsetWidget, <, T, >, child: T, inset_left: usize, inset_right: usize, inset_top: usize, inset_bottom: usize);

    impl_getters!(pub child: T);
}

impl<T: DynamicWidget> DynamicWidget for InsetWidget<T> {
    fn width_characters(&self) -> usize {
        self.child
            .width_characters()
            .saturating_sub(self.inset_left)
            .saturating_sub(self.inset_right)
    }

    fn height_characters(&self) -> usize {
        self.child
            .height_characters()
            .saturating_sub(self.inset_top)
            .saturating_sub(self.inset_bottom)
    }

    fn string_data(&self) -> Vec<Vec<DataCell>> {
        let mut data = self.child.string_data();

        data = data
            .into_iter()
            .skip(self.inset_top)
            .take(self.height_characters())
            .collect::<Vec<_>>();

        data.into_iter()
            .map(|line| {
                line[self.inset_left..
                    self.width_characters() + self.inset_left]
                    .to_vec()
            })
            .collect()
    }
}

impl<T: DynamicWidget> Display for InsetWidget<T> {
    impl_display_for_dynamic_widget!();
}

impl<T: DynamicWidget> Deref for InsetWidget<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T: DynamicWidget> DerefMut for InsetWidget<T> {
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
            );
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
            assert_eq!(widget.width_characters(), 31);
            assert_eq!(widget.height_characters(), 71);
        }
    }
}
