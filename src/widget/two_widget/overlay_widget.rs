use std::ops::{
    Deref,
    DerefMut,
};

use crate::{
    color::{
        Blendable,
        TerminalColor,
    },
    constraint,
    error::WidgetError,
    widget::{
        DynamicCharacterHeight,
        DynamicCharacterWidth,
        StaticCharacterHeight,
        StaticCharacterWidth,
        StringData,
        ToStringData,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverlayWidget<S, T> {
    children: (S, T),
}

impl<
    S: StaticCharacterWidth + StaticCharacterHeight,
    T: StaticCharacterWidth + StaticCharacterHeight,
> OverlayWidget<S, T>
{
    pub const fn new(overlay: S, base: T) -> Self
    where
        constraint!(S::WIDTH_CHARACTERS == T::WIDTH_CHARACTERS):,
        constraint!(S::HEIGHT_CHARACTERS == T::HEIGHT_CHARACTERS):,
    {
        Self {
            children: (overlay, base),
        }
    }

    pub const fn overlay(&self) -> &S {
        &self.0
    }

    pub const fn overlay_mut(&mut self) -> &mut S {
        &mut self.0
    }

    pub const fn base(&self) -> &T {
        &self.1
    }

    pub const fn base_mut(&mut self) -> &mut T {
        &mut self.1
    }
}

impl<
    S: DynamicCharacterWidth + DynamicCharacterHeight,
    T: DynamicCharacterWidth + DynamicCharacterHeight,
> OverlayWidget<S, T>
{
    /// Builds an overlay widget with two children.
    ///
    /// # Errors
    ///
    /// Returns an error if the dimensions of both children don't match.
    pub fn build(overlay: S, base: T) -> Result<Self, WidgetError> {
        if overlay.width_characters() != base.width_characters() ||
            overlay.height_characters() != base.height_characters()
        {
            return Err(WidgetError::WidthAndOrHeightMismatch(
                overlay.width_characters(),
                base.width_characters(),
                overlay.height_characters(),
                base.height_characters(),
            ));
        }
        Ok(Self {
            children: (overlay, base),
        })
    }
}

impl<S: StaticCharacterWidth, T> StaticCharacterWidth
    for OverlayWidget<S, T>
{
    const WIDTH_CHARACTERS: usize = S::WIDTH_CHARACTERS;
}

impl<S: StaticCharacterHeight, T> StaticCharacterHeight
    for OverlayWidget<S, T>
{
    const HEIGHT_CHARACTERS: usize = S::HEIGHT_CHARACTERS;
}

impl<S: DynamicCharacterWidth, T> DynamicCharacterWidth
    for OverlayWidget<S, T>
{
    fn width_characters(&self) -> usize {
        self.children.0.width_characters()
    }
}

impl<S: DynamicCharacterHeight, T> DynamicCharacterHeight
    for OverlayWidget<S, T>
{
    fn height_characters(&self) -> usize {
        self.children.0.height_characters()
    }
}

impl<S: ToStringData, T: ToStringData> ToStringData
    for OverlayWidget<S, T>
{
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
                            // TODO: Rework this blending
                            if let TerminalColor::ARGBColor(foreground) =
                                cell.foreground &&
                                let TerminalColor::ARGBColor(background) =
                                    cell.background &&
                                background.opacity < u8::MAX / 2 &&
                                foreground.opacity < u8::MAX / 2
                            {
                                cell.character = cell_bottom.character;

                                cell.background = Blendable::blend(
                                    &cell.background,
                                    &cell_bottom.background,
                                );
                                cell.foreground = Blendable::blend(
                                    &cell.foreground,
                                    &cell_bottom.foreground,
                                );
                            }
                            else if cell.background ==
                                TerminalColor::Default ||
                                cell.foreground ==
                                    TerminalColor::Default
                            {
                                cell.character = cell_bottom.character;

                                cell.background = TerminalColor::blend(
                                    &cell.background,
                                    &cell_bottom.background,
                                );
                                cell.foreground = TerminalColor::blend(
                                    &cell.foreground,
                                    &cell_bottom.foreground,
                                );
                            }
                            cell
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

impl<S, T> const Deref for OverlayWidget<S, T> {
    type Target = (S, T);

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl<S, T> const DerefMut for OverlayWidget<S, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.children
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        color::{
            ARGBColor,
            RGBColor,
        },
        drawing::SetPixel,
        pixel::{
            color_pixel::ColorSinglePixel,
            monochrome_pixel::SinglePixel,
        },
        pixel_display::{
            DynamicPixelDisplay,
            StaticPixelDisplay,
        },
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
    fn deref() {
        let mut overlay = OverlayWidget::new(
            StaticPixelDisplay::<SinglePixel, 20, 10>::new(false),
            StaticPixelDisplay::<SinglePixel, 20, 10>::new(true),
        );

        assert_eq!(*overlay.overlay(), overlay.deref().0);
        assert_eq!(*overlay.clone().overlay_mut(), overlay.deref_mut().0);

        assert_eq!(*overlay.base(), overlay.deref().1);
        assert_eq!(*overlay.clone().base_mut(), overlay.deref_mut().1);
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

        let bottom = StaticPixelDisplay::<ColorSinglePixel, 37, 63>::new(
            RGBColor::WHITE.into(),
        );

        let mut overlay = OverlayWidget::build(top, bottom).unwrap();

        assert_eq!(
            overlay.string_data().to_string(),
            ToStringData::string_data(&overlay.1).to_string()
        );

        overlay.0.set_pixel(10, 10, RGBColor::BLACK.into()).unwrap();

        assert_ne!(
            overlay.string_data().to_string(),
            ToStringData::string_data(&overlay.1).to_string()
        );
    }
}
