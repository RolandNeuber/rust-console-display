#[macro_export]
macro_rules! for_all {
    ( for $v:ident in [ $( $vals:ident ),* ] => $expr:expr ) => {
        and!($( console_display_macros::replace!($v => $vals; $expr ) ),*)
    };
}
