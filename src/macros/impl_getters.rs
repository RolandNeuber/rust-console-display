/// Implements getters for properties on a struct.
/// May take attributes and visibility.
///
/// # Examples
/// ```
/// use console_display::impl_getters;
/// struct Example {
///     flag: bool,
///     data: [String; 8],
/// }
///
/// impl Example {
///     impl_getters!(
///         #[must_use] pub flag: bool,
///         data: [String; 8]
///     );
/// }
/// ```
#[macro_export]
macro_rules! impl_getters {
    ($($(#[$attr:meta])* $visibility:vis $field:ident: $type:ty),*) => {
        $(
            $(#[$attr])*
            $visibility fn $field(&self) -> &$type {
                &self.$field
            }
        )*
    };
    ($($(#[$attr:meta])* $visibility:vis const $field:ident: $type:ty),*) => {
        $(
            $(#[$attr])*
            $visibility const fn $field(&self) -> &$type {
                &self.$field
            }
        )*
    };
}
