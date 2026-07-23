/// Inserts a eagerly evaluated `and` into a `constraint`.
/// See [`crate::constraint`] for examples and usage.
#[macro_export]
macro_rules! and {
    ($( $item:expr ),*) => {
        ( 1 $( & $item as u8 )* ) == 1
    }
}
