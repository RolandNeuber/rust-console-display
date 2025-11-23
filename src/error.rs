use thiserror::Error;

pub(crate) const INVARIANT_VIOLATED: &str = "Invariant violated.";
pub(crate) const FOUND_CONTROL_CHAR: &str = konst::string::str_concat!(&[INVARIANT_VIOLATED, " Found control character."]);
pub(crate) const DATA_DOES_NOT_MATCH_DIMENSIONS: &str = konst::string::str_concat!(&[INVARIANT_VIOLATED, " Data does not match specified dimensions."]);
pub(crate) const PIXEL_INDEX_OUT_OF_RANGE: &str = konst::string::str_concat!(&[INVARIANT_VIOLATED, " Pixel index out of range."]);

pub(crate) const COLORS_CONTAINS_TOO_MANY_ELEMENTS: &str = "Colors contains too many elements.";
pub(crate) const OFFSET_SHOULD_BE_0_OR_1: &str = "Offset should be 0 or 1.";
pub(crate) const OFFSET_SHOULD_BE_0: &str = "Offset should be 0.";
pub(crate) const COULD_NOT_CAST_X_COORD: &str = "Could not cast x coordinate.";
pub(crate) const COULD_NOT_CAST_Y_COORD: &str = "Could not cast y coordinate.";

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PixelError {
    #[error("Control characters are not allowed. Found {0}")]
    ControlCharacter(char),
    #[error("Invalid number of arguments. Expected {0}, got {1}")]
    InvalidNumberOfArguments(usize, usize),
    #[error(
        "Coordinates out of bounds. x = {0} (expected < {1}), y = {2} (expected < {3})"
    )]
    CoordinatesOutOfBounds(usize, usize, usize, usize),
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum DisplayError {
    #[error("Pixel error: {0}")]
    PixelError(#[from] PixelError),
    #[error("Data is malformed, character spans multiple rows. At '{0}'")]
    MalformedCharacterData(char),
    #[error(
        "Data does not match specified dimensions. Expected length of {0}, got {1}."
    )]
    MismatchedDimensions(usize, usize),
    #[error(
        "Pixel coordinates out of bounds. Got x = {0} (expected < {1}), y = {2} (expected < {3})"
    )]
    CoordinatesOutOfBounds(usize, usize, usize, usize),
    #[error("Coordinates could not be converted to usize.")]
    CoordinatesToUsizeConversionFailed,
    #[error(
        "Width and height must be multiples of multipixel dimensions. Got width = {0} (expected multiple of {1}), height = {2} (expected multiple of {3})"
    )]
    DisplayDimensionsNotMultipleOfPixelDimensions(
        usize,
        usize,
        usize,
        usize,
    ),
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum WidgetError {
    #[error("Pixel error: {0}")]
    PixelError(#[from] PixelError),
    #[error("Display error: {0}")]
    DisplayError(#[from] DisplayError),
    #[error("{0} is outside the uv bounds.")]
    UvCoordinateOutOfBounds(char),
    #[error(
        "Height and/or width in characters of arguments does not match. Got width: {0} and {1}, and height {2} and {3}"
    )]
    WidthAndOrHeightMismatch(usize, usize, usize, usize),
    #[error(
        "Width in characters of arguments does not match. Got {0} and {1}"
    )]
    WidthMismatch(usize, usize),
    #[error(
        "Height in characters of arguments does not match. Got {0} and {1}"
    )]
    HeightMismatch(usize, usize),
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum DrawingError {
    #[error("Display error: {0}")]
    DisplayError(#[from] DisplayError),
    #[error("Widget error: {0}")]
    WidgetError(#[from] WidgetError),
}
