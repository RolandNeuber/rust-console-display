use std::ops::{
    Deref,
    DerefMut,
};

use console_display_macros::StaticWidget;

use crate::{
    constraint,
    error::WidgetError,
    impl_getters,
    impl_setters,
    widget::{
        DynamicWidget,
        StringData,
    },
};

use super::{
    StaticWidget,
    TwoWidget,
};

#[derive(StaticWidget, TwoWidget, Debug, Clone, PartialEq, Eq)]
pub struct AlternativeWidget<S: DynamicWidget, T: DynamicWidget> {
    child1_on_top: bool,
    children: (S, T),
}

impl<S: StaticWidget, T: StaticWidget> AlternativeWidget<S, T> {
    pub const fn new(child1: S, child2: T, child1_on_top: bool) -> Self
    where
        constraint!(S::WIDTH_CHARACTERS == T::WIDTH_CHARACTERS):,
        constraint!(S::HEIGHT_CHARACTERS == T::HEIGHT_CHARACTERS):,
    {
        Self {
            child1_on_top,
            children: (child1, child2),
        }
    }
}

impl<S: DynamicWidget, T: DynamicWidget> AlternativeWidget<S, T> {
    /// Builds an alternative widget with two children.
    /// The `child1_on_top` parameter determines whether the first child should be
    /// displayed instead of the second child and vice versa.
    ///
    /// # Errors
    ///
    /// Returns an error if the dimensions of both children don't match.
    pub fn build(
        child1: S,
        child2: T,
        child1_on_top: bool,
    ) -> Result<Self, WidgetError> {
        if child1.width_characters() != child2.width_characters() ||
            child1.height_characters() != child2.height_characters()
        {
            return Err(WidgetError::WidthAndOrHeightMismatch(
                child1.width_characters(),
                child2.width_characters(),
                child1.height_characters(),
                child2.height_characters(),
            ));
        }
        Ok(Self {
            child1_on_top,
            children: (child1, child2),
        })
    }

    impl_getters!(pub const child1_on_top: bool);

    impl_setters!(pub const child1_on_top: bool);
}

impl<S: [const] DynamicWidget, T: [const] DynamicWidget> const
    DynamicWidget for AlternativeWidget<S, T>
{
    fn width_characters(&self) -> usize {
        self.children.0.width_characters()
    }

    fn height_characters(&self) -> usize {
        self.children.0.height_characters()
    }

    fn string_data(&self) -> StringData {
        if self.child1_on_top {
            self.children.0.string_data()
        }
        else {
            self.children.1.string_data()
        }
    }
}

impl<S: DynamicWidget, T: DynamicWidget> const Deref
    for AlternativeWidget<S, T>
{
    type Target = (S, T);

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl<S: DynamicWidget, T: DynamicWidget> const DerefMut
    for AlternativeWidget<S, T>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.children
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        pixel::monochrome_pixel::{
            DualPixel,
            SinglePixel,
        },
        pixel_display::StaticPixelDisplay,
    };

    use super::*;

    #[test]
    fn build_success() {
        let alternative = AlternativeWidget::build(
            StaticPixelDisplay::<SinglePixel, 1, 1>::new(false),
            StaticPixelDisplay::<SinglePixel, 1, 1>::new(true),
            true,
        );
        assert!(alternative.is_ok());
    }

    #[test]
    fn build_failure() {
        let alternative = AlternativeWidget::build(
            StaticPixelDisplay::<SinglePixel, 1, 1>::new(false),
            StaticPixelDisplay::<SinglePixel, 1, 2>::new(true),
            true,
        );
        assert!(alternative.is_err());
    }

    #[test]
    fn dimensions() {
        let alternative = AlternativeWidget::new(
            StaticPixelDisplay::<SinglePixel, 37, 63>::new(false),
            StaticPixelDisplay::<SinglePixel, 37, 63>::new(true),
            true,
        );
        assert_eq!(alternative.width_characters(), 37);
        assert_eq!(alternative.height_characters(), 63);
    }

    #[test]
    fn deref() {
        let mut alternative = AlternativeWidget::new(
            StaticPixelDisplay::<SinglePixel, 1, 1>::new(false),
            StaticPixelDisplay::<DualPixel, 1, 2>::new(true),
            true,
        );
        assert_eq!(
            alternative.clone().deref().0,
            alternative.deref_mut().0
        );
        assert_eq!(
            alternative.clone().deref().1,
            alternative.deref_mut().1
        );
    }
}
