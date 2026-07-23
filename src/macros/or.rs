/// Inserts a eagerly evaluated `or` into a `constraint`.
/// See [`crate::constraint`] for examples and usage.
#[macro_export]
macro_rules! or {
    ($( $item:expr ),*) => {
        ( 0 $( | $item as u8 )* ) != 0
    }
}
