use super::metric_space::MetricSpace;

pub const trait Groupable: [const] MetricSpace + Sized {
    /// Groups a list of colors into two groups.
    /// The groups are formed by finding the most distant colors.
    /// All other colors are then put into a group with one of them depending on which distance is smaller.
    /// Returns an array of the size of the input. True and false are used to map each color into one of two groups.
    ///
    /// # Examples
    ///
    /// ```
    /// use console_display::color::{
    ///     Color,
    ///     RGBColor,
    /// };
    ///
    /// let colors =
    ///     [RGBColor::BLACK, RGBColor::WHITE, RGBColor::BLACK];
    ///
    /// let grouping = RGBColor::group(&colors);
    ///
    /// assert_ne!(grouping[0], grouping[1]);
    /// assert_ne!(grouping[1], grouping[2]);
    /// assert_eq!(grouping[0], grouping[2]);
    /// ```
    #[must_use]
    fn group<const N: usize>(colors: &[Self; N]) -> [bool; N] {
        let mut max = 0f32;
        let mut col1 = 0;
        let mut col2 = 0;
        konst::for_range! { i in 0..N =>
            konst::for_range! { j in (i + 1)..N =>
                let dist = Self::distance(&colors[i], &colors[j]);
                if dist > max {
                    max = dist;
                    col1 = i;
                    col2 = j;
                }
            }
        }
        let mut groups = [false; N];
        konst::for_range! { i in 0..N =>
            if Self::distance(&colors[col1], &colors[i]) >
                Self::distance(&colors[col2], &colors[i])
            {
                groups[i] = true;
            }
        }
        groups
    }
}
