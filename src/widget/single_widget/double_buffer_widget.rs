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

use console_display_macros::StaticWidget;

use crate::{
    console_display::{
        DynamicConsoleDisplay,
        GetData,
        GetDataMut,
    },
    pixel::Pixel,
    widget::{
        DynamicCharacterHeight,
        DynamicCharacterWidth,
        DynamicWidget,
        StaticCharacterHeight,
        StaticCharacterWidth,
        StringData,
        ToStringData,
        single_widget::{
            SingleWidget,
            SingleWidgetOld,
        },
    },
};

use crate::widget::StaticWidget;

#[derive(StaticWidget, Debug, Clone, PartialEq, Eq)]
#[deprecated]
pub struct DoubleBufferWidgetOld<T: DynamicConsoleDisplay<S>, S: Pixel> {
    pixel_type: PhantomData<S>,
    child: RefCell<T>,
    backbuffer: RefCell<Box<[S]>>,
    is_write: Cell<bool>,
}

impl<T: DynamicConsoleDisplay<S>, S: Pixel> DoubleBufferWidgetOld<T, S> {
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
    for DoubleBufferWidgetOld<T, S>
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

impl<T: DynamicConsoleDisplay<S>, S: Pixel> const SingleWidgetOld<T>
    for DoubleBufferWidgetOld<T, S>
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
    for DoubleBufferWidgetOld<T, S>
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
    for DoubleBufferWidgetOld<T, S>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        if !self.is_write.get() {
            self.swap_buffers();
            self.is_write.set(true);
        }
        self.child.get_mut()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoubleBufferWidget<T, S: Pixel> {
    pixel_type: PhantomData<S>,
    child: RefCell<T>,
    backbuffer: RefCell<Box<[S]>>,
    is_write: Cell<bool>,
}

impl<T: GetData<S> + GetDataMut<S>, S: Pixel> DoubleBufferWidget<T, S> {
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

impl<T: StaticCharacterWidth, S: Pixel> StaticCharacterWidth
    for DoubleBufferWidget<T, S>
{
    const WIDTH_CHARACTERS: usize = T::WIDTH_CHARACTERS;
}

impl<T: StaticCharacterHeight, S: Pixel> StaticCharacterHeight
    for DoubleBufferWidget<T, S>
{
    const HEIGHT_CHARACTERS: usize = T::HEIGHT_CHARACTERS;
}

impl<T: DynamicCharacterWidth, S: Pixel> DynamicCharacterWidth
    for DoubleBufferWidget<T, S>
{
    fn width_characters(&self) -> usize {
        self.child.borrow().width_characters()
    }
}

impl<T: DynamicCharacterHeight, S: Pixel> DynamicCharacterHeight
    for DoubleBufferWidget<T, S>
{
    fn height_characters(&self) -> usize {
        self.child.borrow().height_characters()
    }
}

impl<T: ToStringData + GetData<S> + GetDataMut<S>, S: Pixel> ToStringData
    for DoubleBufferWidget<T, S>
{
    fn string_data(&self) -> StringData {
        if self.is_write.get() {
            self.swap_buffers();
            self.is_write.set(false);
        }
        self.child.borrow().string_data()
    }
}

impl<T, S: Pixel> const SingleWidget<T> for DoubleBufferWidget<T, S> {
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

impl<T: GetData<S> + GetDataMut<S>, S: Pixel> Deref
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

impl<T: GetData<S> + GetDataMut<S>, S: Pixel> DerefMut
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

#[cfg(test)]
mod tests {
    use crate::{
        pixel::monochrome_pixel::SinglePixel,
        pixel_display::StaticPixelDisplay,
    };

    use crate::console_display::StaticConsoleDisplay;

    use super::*;

    #[test]
    fn buffer_swap() {
        let mut widget = DoubleBufferWidgetOld::new(StaticPixelDisplay::<
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
