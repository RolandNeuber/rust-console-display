use crate::{
    color::TerminalColor,
    impl_getters,
    impl_new,
    widget::{
        DataCell,
        StringData,
        single::shader_trait::Shader,
    },
};

pub struct Bend {
    fill_color: TerminalColor,
    strength: f32,
}

impl Bend {
    impl_new!(pub const Bend, fill_color: TerminalColor, strength: f32);

    impl_getters!(pub const strength: f32);
}

impl Shader for Bend {
    fn apply(&self, data: StringData) -> StringData {
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

        for (y, line) in data.iter().enumerate() {
            let width = line.len();

            for (x, cell) in line.iter().enumerate() {
                let mut uv_x: f32 =
                    (x as f32 / width as f32).mul_add(2., -1.);
                let mut uv_y = (y as f32 / height as f32).mul_add(2., -1.);
                uv_x *= (self.strength * uv_y).mul_add(-uv_y, 1.);
                uv_y *= (self.strength * uv_x).mul_add(-uv_x, 1.);
                #[allow(clippy::cast_possible_truncation)]
                #[allow(clippy::cast_sign_loss)]
                let new_x = (f32::midpoint(uv_x, 1.) * width as f32)
                    .round() as usize;
                #[allow(clippy::cast_possible_truncation)]
                #[allow(clippy::cast_sign_loss)]
                let new_y = (f32::midpoint(uv_y, 1.) * height as f32)
                    .round() as usize;
                new_data[new_y][new_x] = *cell;
            }
        }

        StringData { data: new_data }
    }
}
