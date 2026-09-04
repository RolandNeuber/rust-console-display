use crate::{
    color::Shadable,
    impl_getters,
    impl_new,
    widget::{
        StringData,
        single::shader_trait::Shader,
    },
};

pub struct ScanLine {
    shade_strength: f32,
}

impl ScanLine {
    impl_new!(pub const ScanLine, shade_strength: f32);

    impl_getters!(pub const shade_strength: f32);
}

impl Shader for ScanLine {
    fn apply(&self, mut data: StringData) -> StringData {
        for even_line in data.iter_mut().step_by(2) {
            for cell in even_line.iter_mut() {
                cell.foreground = cell
                    .foreground
                    .adjust_lightness(self.shade_strength / 2.);
                cell.background = cell
                    .background
                    .adjust_lightness(self.shade_strength / 2.);
            }
        }

        for even_line in data.iter_mut().skip(1).step_by(2) {
            for cell in even_line.iter_mut() {
                cell.foreground = cell
                    .foreground
                    .adjust_lightness(-self.shade_strength / 2.);
                cell.background = cell
                    .background
                    .adjust_lightness(-self.shade_strength / 2.);
            }
        }

        data
    }
}
