use std::{
    cell::{
        Cell,
        Ref,
        RefCell,
        RefMut,
    },
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
use num_traits::NumCast;

use crate::{
    color::TerminalColor,
    console_display::DynamicConsoleDisplay,
    constraint,
    drawing::DynamicCanvas,
    impl_getters,
    impl_new,
    impl_setters,
    or,
    pixel::{
        Pixel,
        character_pixel::CharacterPixel,
    },
    widget::{
        DynamicWidget,
        StringData,
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
pub struct UvWidget<T: DynamicConsoleDisplay<S>, S: Pixel> {
    pixel_type: PhantomData<S>,
    child: T,
    uv_x_min: f32,
    uv_x_max: f32,
    uv_y_min: f32,
    uv_y_max: f32,
}

impl<T: DynamicConsoleDisplay<S> + StaticWidget, S: Pixel> DynamicCanvas<S>
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
    ) -> Result<<S as Pixel>::U, String>
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
            NumCast::from(uv.0).unwrap(),
            NumCast::from(uv.1).unwrap(),
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
    ) -> Result<(), String>
    where
        [(); <S as Pixel>::WIDTH * <S as Pixel>::HEIGHT]:,
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
                display.height(),
            ),
        );
        self.child_mut().set_pixel(
            NumCast::from(uv.0).unwrap(),
            NumCast::from(uv.1).unwrap(),
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
}

impl<S: Pixel, T: DynamicConsoleDisplay<S> + StaticWidget> UvWidget<T, S> {
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

impl<T: DynamicConsoleDisplay<S> + StaticWidget, S: Pixel> SingleWidget<T>
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

impl<T: DynamicConsoleDisplay<S>, S: Pixel> const Deref for UvWidget<T, S> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T: DynamicConsoleDisplay<S>, S: Pixel> const DerefMut for UvWidget<T, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.child
    }
}

#[derive(StaticWidget)]
pub struct DoubleBufferWidget<T: DynamicConsoleDisplay<S>, S: Pixel> {
    pixel_type: PhantomData<S>,
    child: RefCell<T>,
    backbuffer: RefCell<Box<[S]>>,
    is_write: Cell<bool>,
}

impl<T: DynamicConsoleDisplay<S>, S: Pixel> DoubleBufferWidget<T, S> {
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

impl<T: DynamicConsoleDisplay<S>, S: Pixel> DynamicWidget
    for DoubleBufferWidget<T, S>
{
    fn width_characters(&self) -> usize {
        self.child.borrow().width_characters()
    }

    fn height_characters(&self) -> usize {
        self.child.borrow().height_characters()
    }

    fn string_data(&self) -> StringData {
        if self.is_write.get() {
            self.swap_buffers();
            self.is_write.set(false);
        }
        self.child.borrow().string_data()
    }
}

impl<T: DynamicConsoleDisplay<S>, S: Pixel> SingleWidget<T>
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

impl<T: DynamicConsoleDisplay<S>, S: Pixel> Deref
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

impl<T: DynamicConsoleDisplay<S>, S: Pixel> DerefMut
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
    impl_new!(pub const PaddingWidget<T>, child: T, padding_left: usize, padding_right: usize, padding_top: usize, padding_bottom: usize);

    impl_setters!(pub const padding_left: usize, pub const padding_right: usize, pub const padding_top: usize, pub const padding_bottom: usize);
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

    fn string_data(&self) -> StringData {
        let mut data = self.child.string_data().data;
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
        StringData {
            data: [padding_top, data, padding_bottom].concat(),
        }
    }
}

impl<T: DynamicWidget> const Deref for PaddingWidget<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T: DynamicWidget> const DerefMut for PaddingWidget<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.child
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
        #[allow(clippy::too_many_arguments)]
        #[must_use] pub const BorderDefault,
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
        #[must_use] pub const top: CharacterPixel,
        #[must_use] pub const top_left: CharacterPixel,
        #[must_use] pub const left: CharacterPixel,
        #[must_use] pub const bottom_left: CharacterPixel,
        #[must_use] pub const bottom: CharacterPixel,
        #[must_use] pub const bottom_right: CharacterPixel,
        #[must_use] pub const right: CharacterPixel,
        #[must_use] pub const top_right: CharacterPixel
    );

    /// # Errors
    ///
    /// Returns an error if the passed chars are control characters.
    pub fn symmetric(
        horizontal: char,
        vertical: char,
        corner: char,
        foreground: TerminalColor,
        background: TerminalColor,
    ) -> Result<Self, String> {
        let horizontal =
            CharacterPixel::build(horizontal, foreground, background)?;
        let vertical =
            CharacterPixel::build(vertical, foreground, background)?;
        let corner =
            CharacterPixel::build(corner, foreground, background)?;

        Ok(Self::new(
            horizontal, corner, vertical, corner, horizontal, corner,
            vertical, corner,
        ))
    }

    #[must_use]
    pub fn symmetric_static<
        const HORIZONTAL: char,
        const VERTICAL: char,
        const CORNER: char,
    >(
        foreground: TerminalColor,
        background: TerminalColor,
    ) -> Self
    where
        constraint!(HORIZONTAL >= '\u{20}'):, // Exclude C0 control chars
        constraint!(or!(HORIZONTAL < '\u{7F}', HORIZONTAL >= '\u{A0}')):, // Exclude C1 control chars
        constraint!(VERTICAL >= '\u{20}'):, // Exclude C0 control chars
        constraint!(or!(VERTICAL < '\u{7F}', VERTICAL >= '\u{A0}')):, // Exclude C1 control chars
        constraint!(CORNER >= '\u{20}'):, // Exclude C0 control chars
        constraint!(or!(CORNER < '\u{7F}', CORNER >= '\u{A0}')):, // Exclude C1 control chars
    {
        let horizontal =
            CharacterPixel::new::<HORIZONTAL>(foreground, background);
        let vertical =
            CharacterPixel::new::<VERTICAL>(foreground, background);
        let corner = CharacterPixel::new::<CORNER>(foreground, background);

        Self::new(
            horizontal, corner, vertical, corner, horizontal, corner,
            vertical, corner,
        )
    }

    #[must_use]
    pub fn ascii(
        foreground: TerminalColor,
        background: TerminalColor,
    ) -> Self {
        Self::symmetric_static::<'-', '|', '+'>(foreground, background)
    }

    #[must_use]
    pub fn overdrawn(
        foreground: TerminalColor,
        background: TerminalColor,
    ) -> Self {
        Self::symmetric_static::<'─', '│', '┼'>(
            foreground, background,
        )
    }

    /// # Errors
    ///
    /// Returns an error if the passed chars are control characters.
    #[allow(clippy::too_many_arguments)]
    pub fn same_color(
        top: char,
        top_left: char,
        left: char,
        bottom_left: char,
        bottom: char,
        bottom_right: char,
        right: char,
        top_right: char,
        foreground: TerminalColor,
        background: TerminalColor,
    ) -> Result<Self, String> {
        let top = CharacterPixel::build(top, foreground, background)?;
        let top_left =
            CharacterPixel::build(top_left, foreground, background)?;
        let left = CharacterPixel::build(left, foreground, background)?;
        let bottom_left =
            CharacterPixel::build(bottom_left, foreground, background)?;
        let bottom =
            CharacterPixel::build(bottom, foreground, background)?;
        let bottom_right =
            CharacterPixel::build(bottom_right, foreground, background)?;
        let right = CharacterPixel::build(right, foreground, background)?;
        let top_right =
            CharacterPixel::build(top_right, foreground, background)?;

        Ok(Self::new(
            top,
            top_left,
            left,
            bottom_left,
            bottom,
            bottom_right,
            right,
            top_right,
        ))
    }

    #[must_use]
    pub fn same_color_static<
        const TOP: char,
        const TOP_LEFT: char,
        const LEFT: char,
        const BOTTOM_LEFT: char,
        const BOTTOM: char,
        const BOTTOM_RIGHT: char,
        const RIGHT: char,
        const TOP_RIGHT: char,
    >(
        foreground: TerminalColor,
        background: TerminalColor,
    ) -> Self
    where
        // TODO: Make this less ugly, with another macro maybe
        constraint!(TOP >= '\u{20}'):, // Exclude C0 control chars
        constraint!(or!(TOP < '\u{7F}', TOP >= '\u{A0}')):, // Exclude C1 control chars
        constraint!(TOP_LEFT >= '\u{20}'):, // Exclude C0 control chars
        constraint!(or!(TOP_LEFT < '\u{7F}', TOP_LEFT >= '\u{A0}')):, // Exclude C1 control chars
        constraint!(LEFT >= '\u{20}'):, // Exclude C0 control chars
        constraint!(or!(LEFT < '\u{7F}', LEFT >= '\u{A0}')):, // Exclude C1 control chars
        constraint!(BOTTOM_LEFT >= '\u{20}'):, // Exclude C0 control chars
        constraint!(or!(BOTTOM_LEFT < '\u{7F}', BOTTOM_LEFT >= '\u{A0}')):, // Exclude C1 control chars
        constraint!(BOTTOM >= '\u{20}'):, // Exclude C0 control chars
        constraint!(or!(BOTTOM < '\u{7F}', BOTTOM >= '\u{A0}')):, // Exclude C1 control chars
        constraint!(BOTTOM_RIGHT >= '\u{20}'):, // Exclude C0 control chars
        constraint!(or!(
            BOTTOM_RIGHT < '\u{7F}',
            BOTTOM_RIGHT >= '\u{A0}'
        )):, // Exclude C1 control chars
        constraint!(RIGHT >= '\u{20}'):,        // Exclude C0 control chars
        constraint!(or!(RIGHT < '\u{7F}', RIGHT >= '\u{A0}')):, // Exclude C1 control chars
        constraint!(TOP_RIGHT >= '\u{20}'):, // Exclude C0 control chars
        constraint!(or!(TOP_RIGHT < '\u{7F}', TOP_RIGHT >= '\u{A0}')):, // Exclude C1 control chars
    {
        let top = CharacterPixel::new::<TOP>(foreground, background);
        let top_left =
            CharacterPixel::new::<TOP_LEFT>(foreground, background);
        let left = CharacterPixel::new::<LEFT>(foreground, background);
        let bottom_left =
            CharacterPixel::new::<BOTTOM_LEFT>(foreground, background);
        let bottom = CharacterPixel::new::<BOTTOM>(foreground, background);
        let bottom_right =
            CharacterPixel::new::<BOTTOM_RIGHT>(foreground, background);
        let right = CharacterPixel::new::<RIGHT>(foreground, background);
        let top_right =
            CharacterPixel::new::<TOP_RIGHT>(foreground, background);

        Self::new(
            top,
            top_left,
            left,
            bottom_left,
            bottom,
            bottom_right,
            right,
            top_right,
        )
    }

    #[must_use]
    pub fn single_stroke(
        foreground: TerminalColor,
        background: TerminalColor,
    ) -> Self {
        Self::same_color_static::<'─', '┌', '│', '└', '─', '┘', '│', '┐'>(
            foreground, background,
        )
    }

    #[must_use]
    pub fn rounded_corners(
        foreground: TerminalColor,
        background: TerminalColor,
    ) -> Self {
        Self::same_color_static::<'─', '╭', '│', '╰', '─', '╯', '│', '╮'>(
            foreground, background,
        )
    }

    #[must_use]
    pub fn double_stroke(
        foreground: TerminalColor,
        background: TerminalColor,
    ) -> Self {
        Self::same_color_static::<'═', '╔', '║', '╚', '═', '╝', '║', '╗'>(
            foreground, background,
        )
    }

    #[must_use]
    pub fn bold_stroke(
        foreground: TerminalColor,
        background: TerminalColor,
    ) -> Self {
        Self::same_color_static::<'━', '┏', '┃', '┗', '━', '┛', '┃', '┓'>(
            foreground, background,
        )
    }
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
    impl_new!(pub const BorderWidget<T, S>, child: T, border: S);
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

    fn string_data(&self) -> StringData {
        let border_at = self
            .border
            .border_at(self.width_characters(), self.height_characters());
        let mut data = self.child.string_data().data;
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
        StringData {
            data: [border_top, data, border_bottom].concat(),
        }
    }
}

impl<T: DynamicWidget, S: Border> const Deref for BorderWidget<T, S> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T: DynamicWidget, S: Border> const DerefMut for BorderWidget<T, S> {
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
    impl_new!(pub const InsetWidget<T>, child: T, inset_left: usize, inset_right: usize, inset_top: usize, inset_bottom: usize);

    impl_getters!(pub const child: T);
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

    fn string_data(&self) -> StringData {
        let mut data = self.child.string_data().data;

        data = data
            .into_iter()
            .skip(self.inset_top)
            .take(self.height_characters())
            .collect::<Vec<_>>();

        StringData {
            data: data
                .into_iter()
                .map(|line| {
                    line[self.inset_left..
                        self.width_characters() + self.inset_left]
                        .to_vec()
                })
                .collect(),
        }
    }
}

impl<T: DynamicWidget> const Deref for InsetWidget<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T: DynamicWidget> const DerefMut for InsetWidget<T> {
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
        use crate::console_display::StaticConsoleDisplay;

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
                buffer1.borrow()[0].character().to_string(),
                buffer2.borrow()[0].character().to_string()
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
