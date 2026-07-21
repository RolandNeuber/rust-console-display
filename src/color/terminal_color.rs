use crate::color::{
    ARGBColor,
    Color,
    RGBColor,
    Shadable,
};

/// Defines a color used for foreground and background coloring of text.
///
/// `Default` - Uses the default color provided by the terminal for foreground or background respectively.\
/// `ARGBColor` - Displays a color made of RGB components and an alpha/opacity channel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[derive_const(Default)]
pub enum TerminalColor {
    #[default]
    Default,
    ARGBColor(ARGBColor),
}

// TODO: Check if this impl can be const
impl Color for TerminalColor {
    fn blend(color_top: &Self, color_bottom: &Self) -> Self {
        if let Self::ARGBColor(color_top) = color_top &&
            let Self::ARGBColor(color_bottom) = color_bottom
        {
            return Self::ARGBColor(ARGBColor::blend(
                color_top,
                color_bottom,
            ));
        }
        if let Self::ARGBColor(color) = color_top &&
            color.opacity > 0
        {
            *color_top
        }
        else {
            *color_bottom
        }
    }

    fn color(
        text: &str,
        foreground_color: &Self,
        background_color: &Self,
    ) -> String {
        let mut codes = Vec::new();

        if let Self::ARGBColor(top_color) = foreground_color {
            if let Self::ARGBColor(bottom_color) = *background_color {
                let top_color = ARGBColor::blend(top_color, &bottom_color);
                codes.push(format!(
                    "\x1b[38;2;{};{};{}m",
                    top_color.color.r,
                    top_color.color.g,
                    top_color.color.b, // foreground color
                ));
            }
            codes.push(format!(
                "\x1b[38;2;{};{};{}m",
                top_color.color.r,
                top_color.color.g,
                top_color.color.b, // foreground color
            ));
        }
        if let Self::ARGBColor(background_color) = background_color {
            codes.push(format!(
                "\x1b[48;2;{};{};{}m",
                background_color.color.r,
                background_color.color.g,
                background_color.color.b, // background color
            ));
        }
        if codes.is_empty() {
            return text.to_owned();
        }
        format!("{}{text}{}", codes.join(""), "\x1b[0m")
    }

    fn distance(color1: &Self, color2: &Self) -> f32 {
        if let Self::ARGBColor(col1) = color1 &&
            let Self::ARGBColor(col2) = color2
        {
            return ARGBColor::distance(col1, col2);
        }
        0.
    }

    fn mix(colors: &[Self]) -> Self {
        let mut argb_colors = Vec::with_capacity(colors.len());
        for color in colors {
            argb_colors.push(match color {
                Self::Default => continue,
                Self::ARGBColor(argbcolor) => *argbcolor,
            });
        }
        if argb_colors.is_empty() {
            return Self::Default;
        }

        Self::ARGBColor(ARGBColor::mix(argb_colors.as_slice()))
    }
}

impl Shadable for TerminalColor {
    fn adjust_lightness(&self, amount: f32) -> Self {
        match self {
            Self::Default => Self::Default,
            Self::ARGBColor(argbcolor) => {
                Self::ARGBColor(argbcolor.adjust_lightness(amount))
            }
        }
    }
}

// TODO: Check if this impl can be const
impl From<RGBColor> for TerminalColor {
    fn from(value: RGBColor) -> Self {
        Self::ARGBColor(value.into())
    }
}

impl const From<ARGBColor> for TerminalColor {
    fn from(value: ARGBColor) -> Self {
        Self::ARGBColor(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::color::RGBColor;

    use super::*;

    #[test]
    fn color_none() {
        let msg = TerminalColor::color(
            "test",
            &TerminalColor::Default,
            &TerminalColor::Default,
        );
        assert_eq!(msg, "test");
    }

    #[test]
    fn color_default() {
        let msg = TerminalColor::color(
            "test",
            &RGBColor::RED.into(),
            &TerminalColor::Default,
        );
        assert!(msg.chars().count() > 4);
    }

    #[test]
    fn distance() {
        let dist_prio = TerminalColor::distance(
            &RGBColor::BLACK.into(),
            &TerminalColor::Default,
        );
        let dist_min = TerminalColor::distance(
            &RGBColor::BLACK.into(),
            &RGBColor::BLACK.into(),
        );
        let dist = TerminalColor::distance(
            &RGBColor::BLACK.into(),
            &RGBColor::RED.into(),
        );
        let dist_max = TerminalColor::distance(
            &RGBColor::BLACK.into(),
            &RGBColor::WHITE.into(),
        );

        assert!(dist_prio <= dist_min);
        assert!(dist_min < dist);
        assert!(dist < dist_max);
    }

    #[test]
    fn mix() {
        let colors = [RGBColor::BLACK, RGBColor::RED, RGBColor::GREEN]
            .map(Into::into);
        let mix = TerminalColor::mix(&colors);
        assert_eq!(
            mix,
            RGBColor {
                r: 255 / 3,
                g: 255 / 3,
                b: 0
            }
            .into()
        );
    }

    #[test]
    fn mix_empty() {
        let colors = [];
        let mix = TerminalColor::mix(&colors);
        assert_eq!(mix, TerminalColor::Default);
    }

    #[test]
    fn mix_only_default() {
        let colors = [TerminalColor::Default, TerminalColor::Default];
        let mix = TerminalColor::mix(&colors);
        assert_eq!(mix, TerminalColor::Default);
    }

    #[test]
    fn mix_default() {
        let colors = [
            RGBColor::BLACK.into(),
            TerminalColor::Default,
            RGBColor::WHITE.into(),
        ];
        let mix = TerminalColor::mix(&colors);
        assert_eq!(mix, RGBColor::GRAY.into());
    }

    #[test]
    fn blend_translucent_default() {
        let translucent_red = ARGBColor::mix(&[
            RGBColor::RED.into(),
            ARGBColor::TRANSPARENT,
        ])
        .into();
        let color = TerminalColor::blend(
            &translucent_red,
            &TerminalColor::Default,
        );
        assert_eq!(color, translucent_red);
    }

    #[test]
    fn blend_transparent_default() {
        let color = TerminalColor::blend(
            &ARGBColor::TRANSPARENT.into(),
            &TerminalColor::Default,
        );
        assert_eq!(color, TerminalColor::Default);
    }

    #[test]
    fn from_argb() {
        let color: TerminalColor = ARGBColor::TRANSPARENT.into();
        assert_eq!(
            color,
            TerminalColor::ARGBColor(ARGBColor::TRANSPARENT)
        );
    }
}
