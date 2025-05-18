#[macro_export]
macro_rules! impl_getters {
    ($($field:ident: $type:ty),*) => {
        $(paste::paste!{
            fn [<get_ $field>](&self) -> &$type {
                &self.$field
            }

            fn [<get_ $field _mut>](&mut self) -> &mut $type {
                &mut self.$field
            }
        })*
    };
}

#[macro_export]
macro_rules! impl_new {
    ($struct:ident, $($arg:ident: $type:ty), *) => {
        fn new($($arg: $type)*) -> $struct {
            $struct {
                $($arg), *
            }
        }
    };
}
