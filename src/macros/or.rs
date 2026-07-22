/// Inserts a eagerly evaluated `or` into a `constraint`.
/// See [constraint] for examples and usage.
#[macro_export]
macro_rules! or {
    ($( $item:expr ),*) => {
        ( 1 $( | $item as u8 )* ) == 1
    }
}
