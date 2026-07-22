use std::ops::{
    Deref,
    DerefMut,
};

use crate::{
    impl_new,
    widget::{
        DynamicCharacterHeight,
        DynamicCharacterWidth,
        StringData,
        ToStringData,
        single_widget::border_trait::Border,
    },
};

pub mod border_default;
pub mod border_trait;

pub use border_default::*;

pub struct BorderWidget<T, S: Border> {
    child: T,
    border: S,
}

impl<T, S: Border> BorderWidget<T, S> {
    impl_new!(pub const BorderWidget<T, S>, child: T, border: S);
}

impl<T: DynamicCharacterWidth, S: Border> DynamicCharacterWidth
    for BorderWidget<T, S>
{
    fn width_characters(&self) -> usize {
        self.child.width_characters() +
            self.border.width_left() +
            self.border.width_right()
    }
}

impl<T: DynamicCharacterHeight, S: Border> DynamicCharacterHeight
    for BorderWidget<T, S>
{
    fn height_characters(&self) -> usize {
        self.child.height_characters() +
            self.border.width_top() +
            self.border.width_bottom()
    }
}

impl<
    T: ToStringData + DynamicCharacterWidth + DynamicCharacterHeight,
    S: Border,
> ToStringData for BorderWidget<T, S>
{
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

impl<T, S: Border> const Deref for BorderWidget<T, S> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T, S: Border> const DerefMut for BorderWidget<T, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.child
    }
}
