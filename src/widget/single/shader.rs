use std::ops::{
    Deref,
    DerefMut,
};

use crate::{
    impl_getters,
    impl_new,
    widget::{
        DynamicCharacterHeight,
        DynamicCharacterWidth,
        StringData,
        ToStringData,
        single::shader_trait::Shader,
    },
};

pub mod shader_trait;

pub mod bend;
pub mod bloom;
pub mod blur;
pub mod composite;
pub mod scan_line;

pub struct ShaderWidget<T, S: Shader> {
    child: T,
    shader: S,
}

impl<T, S: Shader> ShaderWidget<T, S> {
    impl_new!(pub const ShaderWidget<T, S>, child: T, shader: S);

    impl_getters!(pub const child: T);
}

impl<T: DynamicCharacterWidth, S: Shader> DynamicCharacterWidth
    for ShaderWidget<T, S>
{
    fn width_characters(&self) -> usize {
        self.child.width_characters()
    }
}

impl<T: DynamicCharacterHeight, S: Shader> DynamicCharacterHeight
    for ShaderWidget<T, S>
{
    fn height_characters(&self) -> usize {
        self.child.height_characters()
    }
}

impl<T: ToStringData, S: Shader> ToStringData for ShaderWidget<T, S> {
    fn string_data(&self) -> StringData {
        let data = self.child().string_data();
        self.shader.apply(data)
    }
}

impl<T, S: Shader> const Deref for ShaderWidget<T, S> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl<T, S: Shader> const DerefMut for ShaderWidget<T, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.child
    }
}
