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
}

#[macro_export]
macro_rules! impl_getters_mut {
    ($($visibility:vis $field:ident: $type:ty),*) => {
        $(paste::paste!{
            $visibility fn [<$field _mut>](&mut self) -> &mut $type {
                &mut self.$field
            }
        })*
    };
}

#[macro_export]
macro_rules! impl_setters {
    ($($visibility:vis $field:ident: $type:ty),*) => {
        $(paste::paste!{
            $visibility fn [<set_ $field>](&mut self, val: $type) {
                self.$field = val;
            }
        })*
    };
}

#[macro_export]
macro_rules! impl_new {
    ($(#[$attr:meta])* $visibility:vis $struct:ident, $($arg:ident: $type:ty), *) => {
        //TODO: Refactor arguments
        #[allow(clippy::too_many_arguments)]
        $(#[$attr])*
        $visibility fn new($($arg: $type),*) -> $struct {
            $struct {
                $($arg), *
            }
        }
    };
    ($visibility:vis $struct:ident, <, $($generic:ty), *, >, $($arg:ident: $type:ty), *) => {
        $visibility fn new($($arg: $type),*) -> $struct<$($generic,)*> {
            $struct {
                $($arg), *
            }
        }
    };
}

#[macro_export]
macro_rules! constraint {
    {$x:expr} => {
        [(); 0 - !$x as usize]
    };
}

#[macro_export]
macro_rules! or {
    ($x:expr, $y:expr) => {
        ($x as u8 | $y as u8) == 1
    };
}

#[macro_export]
macro_rules! and {
    ($x:expr, $y:expr) => {
        ($x as u8 & $y as u8) == 1
    };
}
