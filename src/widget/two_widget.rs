use std::ops::{
    Deref,
    DerefMut,
};

use console_display_macros::{
    StaticWidget,
    TwoWidget,
};

use crate::{
    color::TerminalColor,
    constraint,
    impl_getters,
    impl_setters,
    widget::{
        DynamicWidget,
        StringData,
    },
};

use super::StaticWidget;

pub trait TwoWidget<S: DynamicWidget, T: DynamicWidget>:
    DynamicWidget + Deref + DerefMut
{
    fn children(&self) -> (&S, &T);
    fn children_mut(&mut self) -> (&mut S, &mut T);
}

#[derive(StaticWidget, TwoWidget)]
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
    ) -> Result<Self, String> {
        if child1.width_characters() != child2.width_characters() ||
            child1.height_characters() != child2.height_characters()
        {
            return Err(format!(
                "Height and/or width in characters of arguments does not match. Height {} and {}. Width: {} and {}",
                child1.height_characters(),
                child2.height_characters(),
                child1.width_characters(),
                child2.width_characters(),
            ));
        }
        Ok(Self {
            child1_on_top,
            children: (child1, child2),
        })
    }

    impl_getters!(pub child1_on_top: bool);

    impl_setters!(pub child1_on_top: bool);
}

impl<S: DynamicWidget, T: DynamicWidget> DynamicWidget
    for AlternativeWidget<S, T>
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

impl<S: DynamicWidget, T: DynamicWidget> Deref
    for AlternativeWidget<S, T>
{
    type Target = (S, T);

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl<S: DynamicWidget, T: DynamicWidget> DerefMut
    for AlternativeWidget<S, T>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.children
    }
}

#[derive(TwoWidget)]
pub struct HorizontalTilingWidget<S: DynamicWidget, T: DynamicWidget> {
    children: (S, T),
}

impl<S: DynamicWidget, T: DynamicWidget> HorizontalTilingWidget<S, T> {
    /// Builds horizontal tiling widget with two children.
    /// `child1` will be displayed on the left, `child2` on the right.
    ///
    /// # Errors
    ///
    /// Returns an error if the height of both children does not match.
    pub fn build(child1: S, child2: T) -> Result<Self, String> {
        if child1.height_characters() != child2.height_characters() {
            return Err(format!(
                "Height in characters of arguments does not match. {} and {}.",
                child1.height_characters(),
                child2.height_characters()
            ));
        }
        Ok(Self {
            children: (child1, child2),
        })
    }
}

impl<S: StaticWidget, T: StaticWidget> HorizontalTilingWidget<S, T> {
    pub const fn new(child1: S, child2: T) -> Self
    where
        constraint!(S::HEIGHT_CHARACTERS == T::HEIGHT_CHARACTERS):,
    {
        Self {
            children: (child1, child2),
        }
    }
}

impl<S: StaticWidget, T: StaticWidget> StaticWidget
    for HorizontalTilingWidget<S, T>
{
    const WIDTH_CHARACTERS: usize =
        S::WIDTH_CHARACTERS + T::WIDTH_CHARACTERS;

    const HEIGHT_CHARACTERS: usize = S::HEIGHT_CHARACTERS;
}

impl<S: DynamicWidget, T: DynamicWidget> DynamicWidget
    for HorizontalTilingWidget<S, T>
{
    fn width_characters(&self) -> usize {
        self.children.0.width_characters() +
            self.children.1.width_characters()
    }

    fn height_characters(&self) -> usize {
        self.children.0.height_characters()
    }

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
impl<S: DynamicWidget, T: DynamicWidget> Deref
    for HorizontalTilingWidget<S, T>
{
    type Target = (S, T);

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl<S: DynamicWidget, T: DynamicWidget> DerefMut
    for HorizontalTilingWidget<S, T>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.children
    }
}

#[derive(TwoWidget)]
pub struct VerticalTilingWidget<S: DynamicWidget, T: DynamicWidget> {
    children: (S, T),
}

impl<S: DynamicWidget, T: DynamicWidget> VerticalTilingWidget<S, T> {
    /// Builds vertical tiling widget with two children.
    /// `child1` will be displayed on the top, `child2` on the bottom.
    ///
    /// # Errors
    ///
    /// Returns an error if the width of both children does not match.
    pub fn build(child1: S, child2: T) -> Result<Self, String> {
        if child1.width_characters() != child2.width_characters() {
            return Err(format!(
                "Width in characters of arguments does not match. {} and {}.",
                child1.width_characters(),
                child2.width_characters()
            ));
        }
        Ok(Self {
            children: (child1, child2),
        })
    }
}

impl<S: StaticWidget, T: StaticWidget> VerticalTilingWidget<S, T> {
    pub const fn new(child1: S, child2: T) -> Self
    where
        constraint!(S::WIDTH_CHARACTERS == T::WIDTH_CHARACTERS):,
    {
        Self {
            children: (child1, child2),
        }
    }
}

impl<S: StaticWidget, T: StaticWidget> StaticWidget
    for VerticalTilingWidget<S, T>
{
    const WIDTH_CHARACTERS: usize = S::WIDTH_CHARACTERS;

    const HEIGHT_CHARACTERS: usize =
        S::HEIGHT_CHARACTERS + T::HEIGHT_CHARACTERS;
}

impl<S: DynamicWidget, T: DynamicWidget> DynamicWidget
    for VerticalTilingWidget<S, T>
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

impl<S: DynamicWidget, T: DynamicWidget> Deref
    for VerticalTilingWidget<S, T>
{
    type Target = (S, T);

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl<S: DynamicWidget, T: DynamicWidget> DerefMut
    for VerticalTilingWidget<S, T>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.children
    }
}

#[derive(StaticWidget)]
pub struct OverlayWidget<S: DynamicWidget, T: DynamicWidget> {
    children: (S, T),
}

impl<S: StaticWidget, T: StaticWidget> OverlayWidget<S, T> {
    pub const fn new(overlay: S, base: T) -> Self
    where
        constraint!(S::WIDTH_CHARACTERS == T::WIDTH_CHARACTERS):,
        constraint!(S::HEIGHT_CHARACTERS == T::HEIGHT_CHARACTERS):,
    {
        Self {
            children: (overlay, base),
        }
    }
}

impl<S: DynamicWidget, T: DynamicWidget> OverlayWidget<S, T> {
    /// Builds an overlay widget with two children.
    ///
    /// # Errors
    ///
    /// Returns an error if the dimensions of both children don't match.
    pub fn build(overlay: S, base: T) -> Result<Self, String> {
        if overlay.width_characters() != base.width_characters() ||
            overlay.height_characters() != base.height_characters()
        {
            return Err(format!(
                "Height and/or width in characters of arguments does not match. Height {} and {}. Width: {} and {}",
                overlay.height_characters(),
                base.height_characters(),
                overlay.width_characters(),
                base.width_characters(),
            ));
        }
        Ok(Self {
            children: (overlay, base),
        })
    }
}

impl<S: DynamicWidget, T: DynamicWidget> DynamicWidget
    for OverlayWidget<S, T>
{
    fn width_characters(&self) -> usize {
        self.children.0.width_characters()
    }

    fn height_characters(&self) -> usize {
        self.children.0.height_characters()
    }

    fn string_data(&self) -> StringData {
        let overlay = self.0.string_data().data;
        let display = self.1.string_data().data;
        StringData {
            data: overlay
                .into_iter()
                .zip(display)
                .map(|(overlay_row, display_row)| {
                    overlay_row
                        .into_iter()
                        .zip(display_row)
                        .map(|(cell_top, cell_bottom)| {
                            let mut cell = cell_top;
                            if let TerminalColor::ARGBColor(foreground) =
                                cell.foreground &&
                                let TerminalColor::ARGBColor(background) =
                                    cell.background &&
                                background.opacity < u8::MAX / 2 &&
                                foreground.opacity < u8::MAX / 2
                            {
                                cell.character = cell_bottom.character;
                            }

                            cell.background = TerminalColor::blend(
                                &cell.background,
                                &cell_bottom.background,
                            );
                            cell.foreground = TerminalColor::blend(
                                &cell.foreground,
                                &cell_bottom.foreground,
                            );
                            cell
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

impl<S: DynamicWidget, T: DynamicWidget> Deref for OverlayWidget<S, T> {
    type Target = (S, T);

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl<S: DynamicWidget, T: DynamicWidget> DerefMut for OverlayWidget<S, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.children
    }
}

// TODO: Add more tests for functionality rather than initialization
#[cfg(test)]
mod tests {
    use crate::{
        pixel::{
            color_pixel::ColorSinglePixel,
            monochrome_pixel::SinglePixel,
        },
        pixel_display::StaticPixelDisplay,
    };

    use super::*;

    mod alternative_widget {
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
    }

    mod horizontal_tiling {
        use super::*;

        #[test]
        fn build_success() {
            let overlay = HorizontalTilingWidget::build(
                StaticPixelDisplay::<SinglePixel, 3, 10>::new(false),
                StaticPixelDisplay::<SinglePixel, 2, 10>::new(true),
            );
            assert!(overlay.is_ok());
        }

        #[test]
        fn build_failure() {
            let overlay = HorizontalTilingWidget::build(
                StaticPixelDisplay::<SinglePixel, 10, 3>::new(false),
                StaticPixelDisplay::<SinglePixel, 10, 2>::new(true),
            );
            assert!(overlay.is_err());
        }

        #[test]
        fn dimensions() {
            let overlay = HorizontalTilingWidget::new(
                StaticPixelDisplay::<SinglePixel, 37, 20>::new(false),
                StaticPixelDisplay::<SinglePixel, 63, 20>::new(true),
            );
            assert_eq!(overlay.width_characters(), 100);
            assert_eq!(overlay.height_characters(), 20);
        }
    }

    mod vertical_tiling {
        use super::*;

        #[test]
        fn build_success() {
            let overlay = VerticalTilingWidget::build(
                StaticPixelDisplay::<SinglePixel, 20, 5>::new(false),
                StaticPixelDisplay::<SinglePixel, 20, 4>::new(true),
            );
            assert!(overlay.is_ok());
        }

        #[test]
        fn build_failure() {
            let overlay = VerticalTilingWidget::build(
                StaticPixelDisplay::<SinglePixel, 5, 20>::new(false),
                StaticPixelDisplay::<SinglePixel, 4, 20>::new(true),
            );
            assert!(overlay.is_err());
        }

        #[test]
        fn dimensions() {
            let overlay = VerticalTilingWidget::new(
                StaticPixelDisplay::<SinglePixel, 30, 45>::new(false),
                StaticPixelDisplay::<SinglePixel, 30, 54>::new(true),
            );
            assert_eq!(overlay.width_characters(), 30);
            assert_eq!(overlay.height_characters(), 99);
        }
    }

    mod overlay_widget {
        use crate::{
            color::{
                ARGBColor,
                RGBColor,
            },
            console_display::DynamicConsoleDisplay,
            pixel_display::DynamicPixelDisplay,
        };

        use super::*;

        #[test]
        fn build_success() {
            let overlay = OverlayWidget::build(
                StaticPixelDisplay::<SinglePixel, 1, 1>::new(false),
                StaticPixelDisplay::<SinglePixel, 1, 1>::new(true),
            );
            assert!(overlay.is_ok());
        }

        #[test]
        fn build_failure() {
            let overlay = OverlayWidget::build(
                StaticPixelDisplay::<SinglePixel, 1, 1>::new(false),
                StaticPixelDisplay::<SinglePixel, 1, 2>::new(true),
            );
            assert!(overlay.is_err());
        }

        #[test]
        fn dimensions() {
            let overlay = OverlayWidget::new(
                StaticPixelDisplay::<SinglePixel, 37, 63>::new(false),
                StaticPixelDisplay::<SinglePixel, 37, 63>::new(true),
            );
            assert_eq!(overlay.width_characters(), 37);
            assert_eq!(overlay.height_characters(), 63);
        }

        #[test]
        fn transparency() {
            let top = DynamicPixelDisplay::<ColorSinglePixel>::new(
                37,
                63,
                TerminalColor::ARGBColor(ARGBColor {
                    opacity: 0,
                    color: RGBColor::BLACK,
                }),
            );

            let bottom =
                StaticPixelDisplay::<ColorSinglePixel, 37, 63>::new(
                    RGBColor::WHITE.into(),
                );

            let mut overlay = OverlayWidget::build(top, bottom).unwrap();

            assert_eq!(
                overlay.string_data().to_string(),
                overlay.1.string_data().to_string()
            );

            overlay.0.set_pixel(10, 10, RGBColor::BLACK.into()).unwrap();

            assert_ne!(
                overlay.string_data().to_string(),
                overlay.1.string_data().to_string()
            );
        }
    }
}
