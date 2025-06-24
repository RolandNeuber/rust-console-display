#[macro_export]
macro_rules! impl_getters {
    ($($visibility:vis $field:ident: $type:ty),*) => {
        $(
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
    ($visibility:vis $struct:ident, $($arg:ident: $type:ty), *) => {
        $visibility fn new($($arg: $type),*) -> $struct {
            $struct {
                $($arg), *
            }
        }
    };
    ($visibility:vis $struct:ident, <, $($generic:ty), *, >, $($arg:ident: $type:ty), *) => {
        $visibility fn new($($arg: $type),*) -> $struct<$($generic)*> {
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
    }
}
