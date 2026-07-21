use std::ops::{
    Deref,
    DerefMut,
};

use crate::{
    impl_new,
    widget::{
        DynamicWidget,
        StringData,
        single_widget::border_trait::Border,
    },
};

pub mod border_default;
pub mod border_trait;

pub use border_default::*;

#[derive(Debug, Clone, PartialEq, Eq)]
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
