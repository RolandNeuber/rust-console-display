use crate::widget::StringData;

pub const trait ToStringData {
    /// Returns a string representation.
    /// The first vector contains rows.
    /// The vectors inside/rows contain individual characters.
    #[must_use]
    fn string_data(&self) -> StringData;
}
