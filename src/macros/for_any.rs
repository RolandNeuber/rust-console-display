#[macro_export]
macro_rules! for_any {
    ( for $v:ident in [ $( $vals:ident ),* ] => $expr:expr ) => {
        or!($( console_display_macros::replace!($v => $vals; $expr ) ),*)
    };
}
