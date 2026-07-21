use std::ops::{
    Deref,
    DerefMut,
};

use crate::{
    impl_getters,
    impl_new,
    widget::{
        DynamicWidget,
        StringData,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
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
