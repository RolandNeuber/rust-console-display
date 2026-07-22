/// Implements new for a struct.
/// May take a visibility and generics.
///
/// # Examples
/// ```
/// use console_display::impl_new;
/// struct Example {
///     flag: bool,
///     data: [String; 8],
/// }
///
/// impl Example {
///     impl_new!(
///         pub Example,
///         flag: bool,
///         data: [String; 8]
///     );
/// }
///
/// struct GenericExample<T> {
///     flag: T,
///     data: [String; 8],
/// }
///
/// impl<T> GenericExample<T> {
///     impl_new!(
///         pub GenericExample<T>,
///         flag: T,
///         data: [String; 8]
///     );
/// }
/// ```
#[macro_export]
macro_rules! impl_new {
    ($(#[$attr:meta])* $visibility:vis $struct:ident$(< $($generic:ty),* >)?, $($arg:ident: $type:ty), *) => {
        $(#[$attr])*
        $visibility fn new($($arg: $type),*) -> $struct$(< $($generic),* >)?  {
            $struct {
                $($arg), *
            }
        }
    };
    ($(#[$attr:meta])* $visibility:vis const $struct:ident$(< $($generic:ty),* >)?, $($arg:ident: $type:ty), *) => {
        $(#[$attr])*
        $visibility const fn new($($arg: $type),*) -> $struct$(< $($generic),* >)?  {
            $struct {
                $($arg), *
            }
        }
    };
}
