/// Implements setters for properties on a struct.
/// May take a visibility for each setter.
///
/// # Examples
/// ```
/// use console_display::impl_setters;
/// struct Example {
///     flag: bool,
/// }
///
/// impl Example {
///     impl_setters!(
///         pub flag: bool
///     );
/// }
/// ```
#[macro_export]
macro_rules! impl_setters {
    ($($visibility:vis $field:ident: $type:ty),*) => {
        $(paste::paste!{
            $visibility fn [<set_ $field>](&mut self, val: $type) {
                self.$field = val;
            }
        })*
    };
    ($($visibility:vis const $field:ident: $type:ty),*) => {
        $(paste::paste!{
            $visibility const fn [<set_ $field>](&mut self, val: $type) {
                self.$field = val;
            }
        })*
    };
}
