use crate::{
    color::TerminalColor,
    constraint,
    error::WidgetError,
    impl_getters,
    impl_new,
    or,
    pixel::character_pixel::CharacterPixel,
    widget::single_widget::border_trait::Border,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BorderDefault {
    top: CharacterPixel,
    top_left: CharacterPixel,
    left: CharacterPixel,
    bottom_left: CharacterPixel,
    bottom: CharacterPixel,
    bottom_right: CharacterPixel,
    right: CharacterPixel,
    top_right: CharacterPixel,
}

impl BorderDefault {
    impl_new!(
        #[allow(clippy::too_many_arguments)]
        #[must_use] pub const BorderDefault,
        top: CharacterPixel,
        top_left: CharacterPixel,
        left: CharacterPixel,
        bottom_left: CharacterPixel,
        bottom: CharacterPixel,
        bottom_right: CharacterPixel,
        right: CharacterPixel,
        top_right: CharacterPixel
    );

    impl_getters!(
        #[must_use] pub const top: CharacterPixel,
        #[must_use] pub const top_left: CharacterPixel,
        #[must_use] pub const left: CharacterPixel,
        #[must_use] pub const bottom_left: CharacterPixel,
        #[must_use] pub const bottom: CharacterPixel,
        #[must_use] pub const bottom_right: CharacterPixel,
        #[must_use] pub const right: CharacterPixel,
        #[must_use] pub const top_right: CharacterPixel
    );

    /// # Errors
    ///
    /// Returns an error if the passed chars are control characters.
    pub fn symmetric(
        horizontal: char,
        vertical: char,
        corner: char,
        foreground: TerminalColor,
        background: TerminalColor,
    ) -> Result<Self, WidgetError> {
        let horizontal =
            CharacterPixel::build(horizontal, foreground, background)?;
        let vertical =
            CharacterPixel::build(vertical, foreground, background)?;
        let corner =
            CharacterPixel::build(corner, foreground, background)?;

        Ok(Self::new(
            horizontal, corner, vertical, corner, horizontal, corner,
            vertical, corner,
        ))
    }

    #[must_use]
    pub fn symmetric_static<
        const HORIZONTAL: char,
        const VERTICAL: char,
        const CORNER: char,
    >(
        foreground: TerminalColor,
        background: TerminalColor,
    ) -> Self
    where
        constraint!(HORIZONTAL >= '\u{20}'):, // Exclude C0 control chars
        constraint!(or!(HORIZONTAL < '\u{7F}', HORIZONTAL >= '\u{A0}')):, // Exclude C1 control chars
        constraint!(VERTICAL >= '\u{20}'):, // Exclude C0 control chars
        constraint!(or!(VERTICAL < '\u{7F}', VERTICAL >= '\u{A0}')):, // Exclude C1 control chars
        constraint!(CORNER >= '\u{20}'):, // Exclude C0 control chars
        constraint!(or!(CORNER < '\u{7F}', CORNER >= '\u{A0}')):, // Exclude C1 control chars
    {
        let horizontal =
            CharacterPixel::new::<HORIZONTAL>(foreground, background);
        let vertical =
            CharacterPixel::new::<VERTICAL>(foreground, background);
        let corner = CharacterPixel::new::<CORNER>(foreground, background);

        Self::new(
            horizontal, corner, vertical, corner, horizontal, corner,
            vertical, corner,
        )
    }

    #[must_use]
    pub fn ascii(
        foreground: TerminalColor,
        background: TerminalColor,
    ) -> Self {
        Self::symmetric_static::<'-', '|', '+'>(foreground, background)
    }

    #[must_use]
    pub fn overdrawn(
        foreground: TerminalColor,
        background: TerminalColor,
    ) -> Self {
        Self::symmetric_static::<'─', '│', '┼'>(
            foreground, background,
        )
    }

    /// # Errors
    ///
    /// Returns an error if the passed chars are control characters.
    #[allow(clippy::too_many_arguments)]
    pub fn same_color(
        top: char,
        top_left: char,
        left: char,
        bottom_left: char,
        bottom: char,
        bottom_right: char,
        right: char,
        top_right: char,
        foreground: TerminalColor,
        background: TerminalColor,
    ) -> Result<Self, WidgetError> {
        let top = CharacterPixel::build(top, foreground, background)?;
        let top_left =
            CharacterPixel::build(top_left, foreground, background)?;
        let left = CharacterPixel::build(left, foreground, background)?;
        let bottom_left =
            CharacterPixel::build(bottom_left, foreground, background)?;
        let bottom =
            CharacterPixel::build(bottom, foreground, background)?;
        let bottom_right =
            CharacterPixel::build(bottom_right, foreground, background)?;
        let right = CharacterPixel::build(right, foreground, background)?;
        let top_right =
            CharacterPixel::build(top_right, foreground, background)?;

        Ok(Self::new(
            top,
            top_left,
            left,
            bottom_left,
            bottom,
            bottom_right,
            right,
            top_right,
        ))
    }

    #[must_use]
    pub fn same_color_static<
        const TOP: char,
        const TOP_LEFT: char,
        const LEFT: char,
        const BOTTOM_LEFT: char,
        const BOTTOM: char,
        const BOTTOM_RIGHT: char,
        const RIGHT: char,
        const TOP_RIGHT: char,
    >(
        foreground: TerminalColor,
        background: TerminalColor,
    ) -> Self
    where
        // TODO: This can not be verified to be equivalent to the constraints below, so it fails
        // constraint!(for_all!(for v in [TOP, TOP_LEFT, LEFT, BOTTOM_LEFT, BOTTOM, BOTTOM_RIGHT, RIGHT, TOP_RIGHT] => v >= '\u{20}')):,
        constraint!(TOP >= '\u{20}'):, // Exclude C0 control chars
        constraint!(or!(TOP < '\u{7F}', TOP >= '\u{A0}')):, // Exclude C1 control chars
        constraint!(TOP_LEFT >= '\u{20}'):, // Exclude C0 control chars
        constraint!(or!(TOP_LEFT < '\u{7F}', TOP_LEFT >= '\u{A0}')):, // Exclude C1 control chars
        constraint!(LEFT >= '\u{20}'):, // Exclude C0 control chars
        constraint!(or!(LEFT < '\u{7F}', LEFT >= '\u{A0}')):, // Exclude C1 control chars
        constraint!(BOTTOM_LEFT >= '\u{20}'):, // Exclude C0 control chars
        constraint!(or!(BOTTOM_LEFT < '\u{7F}', BOTTOM_LEFT >= '\u{A0}')):, // Exclude C1 control chars
        constraint!(BOTTOM >= '\u{20}'):, // Exclude C0 control chars
        constraint!(or!(BOTTOM < '\u{7F}', BOTTOM >= '\u{A0}')):, // Exclude C1 control chars
        constraint!(BOTTOM_RIGHT >= '\u{20}'):, // Exclude C0 control chars
        constraint!(or!(
            BOTTOM_RIGHT < '\u{7F}',
            BOTTOM_RIGHT >= '\u{A0}'
        )):, // Exclude C1 control chars
        constraint!(RIGHT >= '\u{20}'):,        // Exclude C0 control chars
        constraint!(or!(RIGHT < '\u{7F}', RIGHT >= '\u{A0}')):, // Exclude C1 control chars
        constraint!(TOP_RIGHT >= '\u{20}'):, // Exclude C0 control chars
        constraint!(or!(TOP_RIGHT < '\u{7F}', TOP_RIGHT >= '\u{A0}')):, // Exclude C1 control chars
    {
        let top = CharacterPixel::new::<TOP>(foreground, background);
        let top_left =
            CharacterPixel::new::<TOP_LEFT>(foreground, background);
        let left = CharacterPixel::new::<LEFT>(foreground, background);
        let bottom_left =
            CharacterPixel::new::<BOTTOM_LEFT>(foreground, background);
        let bottom = CharacterPixel::new::<BOTTOM>(foreground, background);
        let bottom_right =
            CharacterPixel::new::<BOTTOM_RIGHT>(foreground, background);
        let right = CharacterPixel::new::<RIGHT>(foreground, background);
        let top_right =
            CharacterPixel::new::<TOP_RIGHT>(foreground, background);

        Self::new(
            top,
            top_left,
            left,
            bottom_left,
            bottom,
            bottom_right,
            right,
            top_right,
        )
    }

    #[must_use]
    pub fn single_stroke(
        foreground: TerminalColor,
        background: TerminalColor,
    ) -> Self {
        Self::same_color_static::<'─', '┌', '│', '└', '─', '┘', '│', '┐'>(
            foreground, background,
        )
    }

    #[must_use]
    pub fn rounded_corners(
        foreground: TerminalColor,
        background: TerminalColor,
    ) -> Self {
        Self::same_color_static::<'─', '╭', '│', '╰', '─', '╯', '│', '╮'>(
            foreground, background,
        )
    }

    #[must_use]
    pub fn double_stroke(
        foreground: TerminalColor,
        background: TerminalColor,
    ) -> Self {
        Self::same_color_static::<'═', '╔', '║', '╚', '═', '╝', '║', '╗'>(
            foreground, background,
        )
    }

    #[must_use]
    pub fn bold_stroke(
        foreground: TerminalColor,
        background: TerminalColor,
    ) -> Self {
        Self::same_color_static::<'━', '┏', '┃', '┗', '━', '┛', '┃', '┓'>(
            foreground, background,
        )
    }
}

impl Border for BorderDefault {
    fn border_at(
        &self,
        width: usize,
        height: usize,
    ) -> impl Fn(usize, usize) -> CharacterPixel {
        move |x: usize, y: usize| match (x, y) {
            (0, 0) => self.top_left,
            (0, y) if y == height - 1 => self.bottom_left,
            (x, y) if x == width - 1 && y == height - 1 => {
                self.bottom_right
            }
            (x, 0) if x == width - 1 => self.top_right,
            (_, 0) => self.top,
            (0, _) => self.left,
            (_, y) if y == height - 1 => self.bottom,
            (x, _) if x == width - 1 => self.right,
            (_, _) => CharacterPixel::default(),
        }
    }

    fn width_top(&self) -> usize {
        1
    }

    fn width_left(&self) -> usize {
        1
    }

    fn width_bottom(&self) -> usize {
        1
    }

    fn width_right(&self) -> usize {
        1
    }
}
