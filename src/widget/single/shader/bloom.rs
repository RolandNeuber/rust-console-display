use crate::{
    impl_getters,
    impl_new,
    widget::{
        StringData,
        single::shader_trait::Shader,
    },
};

pub struct Bloom {
    strength: f32,
}

impl Bloom {
    impl_new!(pub const Bloom, strength: f32);

    impl_getters!(pub const strength: f32);
}

impl Shader for Bloom {
    fn apply(&self, _data: StringData) -> StringData {
        todo!()
    }
}
