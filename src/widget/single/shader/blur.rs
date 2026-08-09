use crate::{
    impl_getters,
    impl_new,
    widget::{
        StringData,
        single::shader_trait::Shader,
    },
};

pub struct Blur {
    strength: f32,
}

impl Blur {
    impl_new!(pub const Blur, strength: f32);

    impl_getters!(pub const strength: f32);
}

impl Shader for Blur {
    fn apply(&self, _data: StringData) -> StringData {
        todo!()
    }
}
