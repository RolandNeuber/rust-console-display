use std::ops::{
    Deref,
    DerefMut,
};

use console_display_macros::SingleWidgetOld;

use crate::{
    impl_new,
    impl_setters,
    pixel::character_pixel::CharacterPixel,
    widget::{
        DynamicCharacterHeight,
        DynamicCharacterWidth,
        DynamicWidget,
        StringData,
        ToStringData,
        single_widget::{
            SingleWidget,
            SingleWidgetOld,
        },
    },
};

#[derive(SingleWidgetOld, Debug, Clone, PartialEq, Eq)]
pub struct PaddingWidgetOld<T: DynamicWidget> {
    child: T,
    padding_left: usize,
    padding_right: usize,
    padding_top: usize,
    padding_bottom: usize,
}

impl<T: DynamicWidget> PaddingWidgetOld<T> {
    impl_new!(pub const PaddingWidgetOld<T>, child: T, padding_left: usize, padding_right: usize, padding_top: usize, padding_bottom: usize);

    impl_setters!(pub const padding_left: usize, pub const padding_right: usize, pub const padding_top: usize, pub const padding_bottom: usize);
}

impl<T: DynamicWidget> DynamicWidget for PaddingWidgetOld<T> {
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

impl<T: DynamicWidget> const Deref for PaddingWidgetOld<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T: DynamicWidget> const DerefMut for PaddingWidgetOld<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.child
    }
}

#[derive(SingleWidget, Debug, Clone, PartialEq, Eq)]
pub struct PaddingWidget<T> {
    child: T,
    padding_left: usize,
    padding_right: usize,
    padding_top: usize,
    padding_bottom: usize,
}

impl<T> PaddingWidget<T> {
    impl_new!(pub const PaddingWidget<T>, child: T, padding_left: usize, padding_right: usize, padding_top: usize, padding_bottom: usize);

    impl_setters!(pub const padding_left: usize, pub const padding_right: usize, pub const padding_top: usize, pub const padding_bottom: usize);
}

impl<T: DynamicCharacterWidth> DynamicCharacterWidth for PaddingWidget<T> {
    fn width_characters(&self) -> usize {
        self.child.width_characters() +
            self.padding_left +
            self.padding_right
    }
}

impl<T: DynamicCharacterHeight> DynamicCharacterHeight
    for PaddingWidget<T>
{
    fn height_characters(&self) -> usize {
        self.child.height_characters() +
            self.padding_top +
            self.padding_bottom
    }
}

impl<T> ToStringData for PaddingWidget<T>
where
    T: ToStringData,
    Self: DynamicCharacterWidth + DynamicCharacterHeight,
{
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

impl<T> const Deref for PaddingWidget<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T> const DerefMut for PaddingWidget<T> {
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
    fn dimensions() {
        let widget = PaddingWidgetOld::new(
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
