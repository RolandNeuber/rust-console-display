use std::ops::{
    Deref,
    DerefMut,
};

use crate::{
    color::TerminalColor,
    impl_getters,
    impl_new,
    widget::{
        DataCell,
        DynamicWidget,
        StringData,
    },
};

pub struct CrtWidget<T: DynamicWidget> {
    child: T,
    fill_color: TerminalColor,
    bend_strength: f32,
}

impl<T: DynamicWidget> CrtWidget<T> {
    impl_new!(pub const CrtWidget<T>, child: T, fill_color: TerminalColor, bend_strength: f32);

    impl_getters!(pub const child: T, pub const bend_strength: f32);
}

impl<T: DynamicWidget> DynamicWidget for CrtWidget<T> {
    fn width_characters(&self) -> usize {
        self.child.width_characters()
    }

    fn height_characters(&self) -> usize {
        self.child.height_characters()
    }

    fn string_data(&self) -> StringData {
        let data = self.child.string_data().data;

        let height = data.len();

        let mut new_data = vec![
            vec![
                DataCell {
                    character: ' ',
                    foreground: TerminalColor::Default,
                    background: self.fill_color
                };
                data[0].len()
            ];
            data.len()
        ];

        for (y, line) in data.into_iter().enumerate() {
            let width = line.len();

            for (x, cell) in line.into_iter().enumerate() {
                let mut uv_x: f32 =
                    (x as f32 / width as f32).mul_add(2., -1.);
                let mut uv_y = (y as f32 / height as f32).mul_add(2., -1.);
                uv_x *= (self.bend_strength * uv_y).mul_add(-uv_y, 1.);
                uv_y *= (self.bend_strength * uv_x).mul_add(-uv_x, 1.);
                #[allow(clippy::cast_possible_truncation)]
                #[allow(clippy::cast_sign_loss)]
                let new_x = (f32::midpoint(uv_x, 1.) * width as f32)
                    .round() as usize;
                #[allow(clippy::cast_possible_truncation)]
                #[allow(clippy::cast_sign_loss)]
                let new_y = (f32::midpoint(uv_y, 1.) * height as f32)
                    .round() as usize;
                new_data[new_y][new_x] = cell;
            }
        }

        StringData { data: new_data }
    }
}

impl<T: DynamicWidget> const Deref for CrtWidget<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T: DynamicWidget> const DerefMut for CrtWidget<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.child
    }
}
