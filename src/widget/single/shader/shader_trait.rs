use crate::widget::StringData;

pub trait Shader {
    fn apply(&self, data: StringData) -> StringData;
}
