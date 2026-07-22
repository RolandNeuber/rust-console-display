use std::ops::{
    Deref,
    DerefMut,
};

use console_display_macros::TwoWidgetOld;

use super::{
    StaticWidget,
    TwoWidgetOld,
};
use crate::{
    constraint,
    error::WidgetError,
    widget::{
        DynamicCharacterHeight,
        DynamicCharacterWidth,
        DynamicWidget,
        StaticCharacterHeight,
        StaticCharacterWidth,
        StringData,
        ToStringData,
        two_widget::TwoWidget,
    },
};

#[derive(TwoWidgetOld, Debug, Clone, PartialEq, Eq)]
#[deprecated]
pub struct VerticalTilingWidgetOld<S: DynamicWidget, T: DynamicWidget> {
    children: (S, T),
}

impl<S: DynamicWidget, T: DynamicWidget> VerticalTilingWidgetOld<S, T> {
    /// Builds vertical tiling widget with two children.
    /// `child1` will be displayed on the top, `child2` on the bottom.
    ///
    /// # Errors
    ///
    /// Returns an error if the width of both children does not match.
    pub fn build(child1: S, child2: T) -> Result<Self, WidgetError> {
        if child1.width_characters() != child2.width_characters() {
            return Err(WidgetError::WidthMismatch(
                child1.width_characters(),
                child2.width_characters(),
            ));
        }
        Ok(Self {
            children: (child1, child2),
        })
    }

    pub const fn top(&self) -> &S {
        &self.0
    }

    pub const fn top_mut(&mut self) -> &mut S {
        &mut self.0
    }

    pub const fn bottom(&self) -> &T {
        &self.1
    }

    pub const fn bottom_mut(&mut self) -> &mut T {
        &mut self.1
    }
}

impl<S: StaticWidget, T: StaticWidget> VerticalTilingWidgetOld<S, T> {
    pub const fn new(child1: S, child2: T) -> Self
    where
        constraint!(S::WIDTH_CHARACTERS == T::WIDTH_CHARACTERS):,
    {
        Self {
            children: (child1, child2),
        }
    }
}

impl<S: StaticWidget, T: StaticWidget> const StaticWidget
    for VerticalTilingWidgetOld<S, T>
{
    const WIDTH_CHARACTERS: usize = S::WIDTH_CHARACTERS;

    const HEIGHT_CHARACTERS: usize =
        S::HEIGHT_CHARACTERS + T::HEIGHT_CHARACTERS;
}

impl<S: DynamicWidget, T: DynamicWidget> DynamicWidget
    for VerticalTilingWidgetOld<S, T>
{
    fn width_characters(&self) -> usize {
        self.children.0.width_characters()
    }

    fn height_characters(&self) -> usize {
        self.children.0.height_characters() +
            self.children.1.height_characters()
    }

    fn string_data(&self) -> StringData {
        StringData {
            data: [self.0.string_data().data, self.1.string_data().data]
                .concat(),
        }
    }
}

impl<S: DynamicWidget, T: DynamicWidget> const Deref
    for VerticalTilingWidgetOld<S, T>
{
    type Target = (S, T);

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl<S: DynamicWidget, T: DynamicWidget> const DerefMut
    for VerticalTilingWidgetOld<S, T>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.children
    }
}

#[derive(TwoWidget, Debug, Clone, PartialEq, Eq)]
pub struct VerticalTilingWidget<S, T> {
    children: (S, T),
}

impl<S: DynamicCharacterWidth, T: DynamicCharacterWidth>
    VerticalTilingWidget<S, T>
{
    /// Builds vertical tiling widget with two children.
    /// `child1` will be displayed on the top, `child2` on the bottom.
    ///
    /// # Errors
    ///
    /// Returns an error if the width of both children does not match.
    pub fn build(child1: S, child2: T) -> Result<Self, WidgetError> {
        if child1.width_characters() != child2.width_characters() {
            return Err(WidgetError::WidthMismatch(
                child1.width_characters(),
                child2.width_characters(),
            ));
        }
        Ok(Self {
            children: (child1, child2),
        })
    }

    pub const fn top(&self) -> &S {
        &self.0
    }

    pub const fn top_mut(&mut self) -> &mut S {
        &mut self.0
    }

    pub const fn bottom(&self) -> &T {
        &self.1
    }

    pub const fn bottom_mut(&mut self) -> &mut T {
        &mut self.1
    }
}

impl<S: StaticCharacterWidth, T: StaticCharacterWidth>
    VerticalTilingWidget<S, T>
{
    pub const fn new(child1: S, child2: T) -> Self
    where
        constraint!(S::WIDTH_CHARACTERS == T::WIDTH_CHARACTERS):,
    {
        Self {
            children: (child1, child2),
        }
    }
}

impl<S: StaticCharacterWidth, T> StaticCharacterWidth
    for VerticalTilingWidget<S, T>
{
    const WIDTH_CHARACTERS: usize = S::WIDTH_CHARACTERS;
}

impl<S: StaticCharacterHeight, T: StaticCharacterHeight>
    StaticCharacterHeight for VerticalTilingWidget<S, T>
{
    const HEIGHT_CHARACTERS: usize =
        S::HEIGHT_CHARACTERS + T::HEIGHT_CHARACTERS;
}

impl<S: DynamicCharacterWidth, T> DynamicCharacterWidth
    for VerticalTilingWidget<S, T>
{
    fn width_characters(&self) -> usize {
        self.children.0.width_characters()
    }
}

impl<S: DynamicCharacterHeight, T: DynamicCharacterHeight>
    DynamicCharacterHeight for VerticalTilingWidget<S, T>
{
    fn height_characters(&self) -> usize {
        self.children.0.height_characters() +
            self.children.1.height_characters()
    }
}

impl<S: ToStringData, T: ToStringData> ToStringData
    for VerticalTilingWidget<S, T>
{
    fn string_data(&self) -> StringData {
        StringData {
            data: [self.0.string_data().data, self.1.string_data().data]
                .concat(),
        }
    }
}

impl<S, T> const Deref for VerticalTilingWidget<S, T> {
    type Target = (S, T);

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl<S, T> const DerefMut for VerticalTilingWidget<S, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.children
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
    fn build_success() {
        let vertical_tiling = VerticalTilingWidgetOld::build(
            StaticPixelDisplay::<SinglePixel, 20, 5>::new(false),
            StaticPixelDisplay::<SinglePixel, 20, 4>::new(true),
        );
        assert!(vertical_tiling.is_ok());
    }

    #[test]
    fn build_failure() {
        let vertical_tiling = VerticalTilingWidgetOld::build(
            StaticPixelDisplay::<SinglePixel, 5, 20>::new(false),
            StaticPixelDisplay::<SinglePixel, 4, 20>::new(true),
        );
        assert!(vertical_tiling.is_err());
    }

    #[test]
    fn dimensions() {
        let vertical_tiling = VerticalTilingWidgetOld::new(
            StaticPixelDisplay::<SinglePixel, 30, 45>::new(false),
            StaticPixelDisplay::<SinglePixel, 30, 54>::new(true),
        );
        assert_eq!(vertical_tiling.width_characters(), 30);
        assert_eq!(vertical_tiling.height_characters(), 99);
    }

    #[test]
    fn deref() {
        let mut vertical_tiling = VerticalTilingWidgetOld::new(
            StaticPixelDisplay::<SinglePixel, 20, 5>::new(false),
            StaticPixelDisplay::<SinglePixel, 20, 4>::new(true),
        );

        assert_eq!(*vertical_tiling.top(), vertical_tiling.deref().0);
        assert_eq!(
            *vertical_tiling.clone().top_mut(),
            vertical_tiling.deref_mut().0
        );

        assert_eq!(*vertical_tiling.bottom(), vertical_tiling.deref().1);
        assert_eq!(
            *vertical_tiling.clone().bottom_mut(),
            vertical_tiling.deref_mut().1
        );
    }
}
