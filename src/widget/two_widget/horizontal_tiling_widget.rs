use std::ops::{
    Deref,
    DerefMut,
};

use crate::{
    constraint,
    error::WidgetError,
    widget::{
        DynamicCharacterHeight,
        DynamicCharacterWidth,
        StaticCharacterHeight,
        StaticCharacterWidth,
        StringData,
        ToStringData,
        two_widget::TwoWidget,
    },
};

#[derive(TwoWidget, Debug, Clone, PartialEq, Eq)]
pub struct HorizontalTilingWidget<S, T> {
    children: (S, T),
}

impl<S: DynamicCharacterHeight, T: DynamicCharacterHeight>
    HorizontalTilingWidget<S, T>
{
    /// Builds horizontal tiling widget with two children.
    /// `child1` will be displayed on the left, `child2` on the right.
    ///
    /// # Errors
    ///
    /// Returns an error if the height of both children does not match.
    pub fn build(child1: S, child2: T) -> Result<Self, WidgetError> {
        if child1.height_characters() != child2.height_characters() {
            return Err(WidgetError::HeightMismatch(
                child1.height_characters(),
                child2.height_characters(),
            ));
        }
        Ok(Self {
            children: (child1, child2),
        })
    }

    pub const fn left(&self) -> &S {
        &self.0
    }

    pub const fn left_mut(&mut self) -> &mut S {
        &mut self.0
    }

    pub const fn right(&self) -> &T {
        &self.1
    }

    pub const fn right_mut(&mut self) -> &mut T {
        &mut self.1
    }
}

impl<S: StaticCharacterHeight, T: StaticCharacterHeight>
    HorizontalTilingWidget<S, T>
{
    pub const fn new(child1: S, child2: T) -> Self
    where
        constraint!(S::HEIGHT_CHARACTERS == T::HEIGHT_CHARACTERS):,
    {
        Self {
            children: (child1, child2),
        }
    }
}

impl<S: StaticCharacterWidth, T: StaticCharacterWidth> const
    StaticCharacterWidth for HorizontalTilingWidget<S, T>
{
    const WIDTH_CHARACTERS: usize =
        S::WIDTH_CHARACTERS + T::WIDTH_CHARACTERS;
}

impl<S: StaticCharacterHeight, T> const StaticCharacterHeight
    for HorizontalTilingWidget<S, T>
{
    const HEIGHT_CHARACTERS: usize = S::HEIGHT_CHARACTERS;
}

impl<S: DynamicCharacterWidth, T: DynamicCharacterWidth>
    DynamicCharacterWidth for HorizontalTilingWidget<S, T>
{
    fn width_characters(&self) -> usize {
        self.children.0.width_characters() +
            self.children.1.width_characters()
    }
}

impl<S: DynamicCharacterHeight, T> DynamicCharacterHeight
    for HorizontalTilingWidget<S, T>
{
    fn height_characters(&self) -> usize {
        self.children.0.height_characters()
    }
}

impl<S: DynamicCharacterHeight + ToStringData, T: ToStringData>
    ToStringData for HorizontalTilingWidget<S, T>
{
    fn string_data(&self) -> StringData {
        StringData {
            data: self
                .0
                .string_data()
                .data
                .into_iter()
                .zip(self.children.1.string_data().data)
                .map(|lines| [lines.0, lines.1].concat())
                .collect(),
        }
    }
}

impl<S, T> const Deref for HorizontalTilingWidget<S, T> {
    type Target = (S, T);

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl<S, T> const DerefMut for HorizontalTilingWidget<S, T> {
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
        let horizontal_tiling = HorizontalTilingWidget::build(
            StaticPixelDisplay::<SinglePixel, 3, 10>::new(false),
            StaticPixelDisplay::<SinglePixel, 2, 10>::new(true),
        );
        assert!(horizontal_tiling.is_ok());
    }

    #[test]
    fn build_failure() {
        let horizontal_tiling = HorizontalTilingWidget::build(
            StaticPixelDisplay::<SinglePixel, 10, 3>::new(false),
            StaticPixelDisplay::<SinglePixel, 10, 2>::new(true),
        );
        assert!(horizontal_tiling.is_err());
    }

    #[test]
    fn dimensions() {
        let horizontal_tiling = HorizontalTilingWidget::new(
            StaticPixelDisplay::<SinglePixel, 37, 20>::new(false),
            StaticPixelDisplay::<SinglePixel, 63, 20>::new(true),
        );
        assert_eq!(horizontal_tiling.width_characters(), 100);
        assert_eq!(horizontal_tiling.height_characters(), 20);
    }

    #[test]
    fn deref() {
        let mut horizontal_tiling = HorizontalTilingWidget::new(
            StaticPixelDisplay::<SinglePixel, 3, 10>::new(false),
            StaticPixelDisplay::<SinglePixel, 2, 10>::new(true),
        );

        assert_eq!(*horizontal_tiling.left(), horizontal_tiling.deref().0);
        assert_eq!(
            *horizontal_tiling.clone().left_mut(),
            horizontal_tiling.deref_mut().0
        );

        assert_eq!(
            *horizontal_tiling.right(),
            horizontal_tiling.deref().1
        );
        assert_eq!(
            *horizontal_tiling.clone().right_mut(),
            horizontal_tiling.deref_mut().1
        );
    }
}
