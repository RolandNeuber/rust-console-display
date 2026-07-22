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
    {$( $x:expr ),*} => {
        [(); 0 - $( !$x as usize )*]
    };
}
