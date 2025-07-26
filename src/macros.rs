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

/// Constrains a constant generic parameter with the given constraint.
/// Should be used in conjunction with the `or` and `and` macros.
///
/// # Examples
/// ```
/// #![allow(incomplete_features)]
/// #![feature(generic_const_exprs)]
///
/// use console_display::constraint;
/// use console_display::and;
///
/// fn subpixel_static<const X: usize, const Y: usize>() -> u8
/// where
///     constraint!(and!(X < 10, Y < 20)):,
/// {
///     generate_vec()[X + Y * 10]
/// }
///
/// fn generate_vec() -> Vec<u8> {
///     let mut vec = Vec::with_capacity(200);
///     for i in 0..200 {
///         vec.push(i);
///     }
///     vec
/// }
/// ```
#[macro_export]
macro_rules! constraint {
    {$x:expr} => {
        [(); 0 - !$x as usize]
    };
}

/// Inserts a eagerly evaluated `or` into a `constraint`.
/// See [constraint] for examples and usage.
#[macro_export]
macro_rules! or {
    ($x:expr, $y:expr) => {
        ($x as u8 | $y as u8) == 1
    };
}

/// Inserts a eagerly evaluated `and` into a `constraint`.
/// See [constraint] for examples and usage.
#[macro_export]
macro_rules! and {
    ($x:expr, $y:expr) => {
        ($x as u8 & $y as u8) == 1
    };
}

#[macro_export]
macro_rules! impl_from_mono_chrome_pixel_for_datacell {
    ($type:ty) => {
        impl From<$type> for DataCell {
            fn from(val: $type) -> Self {
                Self {
                    character: val.character(),
                    foreground: TerminalColor::Default,
                    background: TerminalColor::Default,
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_from_color_pixel_for_datacell {
    ($type:ty, $base:ty) => {
        impl From<$type> for DataCell {
            fn from(val: $type) -> Self {
                let colors = val.pixels();
                let grouping = TerminalColor::group(&colors);
                let symb = <$base>::new(grouping).character();

                let mut col1 = vec![];
                let mut col2 = vec![];
                for i in 0..grouping.len() {
                    if grouping[i] {
                        col1.push(colors[i]);
                    }
                    else {
                        col2.push(colors[i]);
                    }
                }
                let col1 = TerminalColor::mix(&col1);
                let col2 = TerminalColor::mix(&col2);

                Self {
                    character: symb,
                    foreground: col1,
                    background: col2,
                }
            }
        }
    };
}
