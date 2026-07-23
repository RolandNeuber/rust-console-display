/// Implements mutable getters for properties on a struct.
///
/// # Examples
/// ```
/// use console_display::impl_getters_mut;
/// struct Example {
///     flag: bool,
///     data: [String; 8],
/// }
///
/// impl Example {
///     impl_getters_mut!(
///         data: [String; 8]
///     );
/// }
/// ```
#[macro_export]
macro_rules! impl_getters_mut {
    ($($visibility:vis $field:ident: $type:ty),*) => {
        $(paste::paste!{
            $visibility fn [<$field _mut>](&mut self) -> &mut $type {
                &mut self.$field
            }
        })*
    };
    ($($visibility:vis const $field:ident: $type:ty),*) => {
        $(paste::paste!{
            $visibility const fn [<$field _mut>](&mut self) -> &mut $type {
                &mut self.$field
            }
        })*
    };
}
