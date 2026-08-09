use crate::{
    impl_getters,
    impl_new,
    widget::{
        StringData,
        single::shader_trait::Shader,
    },
};

pub struct Composite {
    shaders: Vec<Box<dyn Shader>>,
}

impl Composite {
    impl_new!(pub const Composite, shaders: Vec<Box<dyn Shader>>);

    impl_getters!(pub const shaders: Vec<Box<dyn Shader>>);
}

impl Shader for Composite {
    fn apply(&self, mut data: StringData) -> StringData {
        for shader in &self.shaders {
            data = shader.apply(data);
        }
        data
    }
}
