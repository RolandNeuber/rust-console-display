pub mod ellipse;
pub mod line;
pub mod rectangle;
pub mod traits;

pub use ellipse::*;
pub use line::*;
pub use rectangle::*;
pub use traits::*;

/// Defines a fill for a drawable.
pub const trait FillType {}

/// Defines no fill on a drawable, e.g. only outline.
#[derive(PartialEq, Eq, Debug)]
pub struct NoFill;
impl const FillType for NoFill {}

/// Defines flat fill on a drawable.
#[derive(PartialEq, Eq, Debug)]
pub struct Filled;
impl const FillType for Filled {}
