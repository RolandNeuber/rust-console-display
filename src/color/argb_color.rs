use crate::color::{
    Color,
    RGBColor,
    Shadable,
};

/// Defines an ARGB color used for foreground and background coloring of text.
/// `color` are the opaque RGB components of the color with an additional `opacity`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ARGBColor {
    pub opacity: u8,
    pub color: RGBColor,
}

// TODO: Check if this impl can be const
impl Color for ARGBColor {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn blend(color_top: &Self, color_bottom: &Self) -> Self {
        let opacity_top = f32::from(color_top.opacity) / 255.;
        let opacity_bottom = f32::from(color_bottom.opacity) / 255.;
        let opacity_res =
            opacity_bottom.mul_add(1. - opacity_top, opacity_top);
        let red = f32::from(color_top.color.r).mul_add(
            opacity_top,
            f32::from(color_bottom.color.r) *
                (1. - opacity_top) *
                opacity_bottom,
        ) / opacity_res;
        let green = f32::from(color_top.color.g).mul_add(
            opacity_top,
            f32::from(color_bottom.color.g) *
                (1. - opacity_top) *
                opacity_bottom,
        ) / opacity_res;
        let blue = f32::from(color_top.color.b).mul_add(
            opacity_top,
            f32::from(color_bottom.color.b) *
                (1. - opacity_top) *
                opacity_bottom,
        ) / opacity_res;
        Self {
            opacity: (opacity_res * 255.).clamp(0., 255.) as u8,
            color: RGBColor {
                r: red.clamp(0., 255.) as u8,
                g: green.clamp(0., 255.) as u8,
                b: blue.clamp(0., 255.) as u8,
            },
        }
    }

    fn color(
        text: &str,
        foreground_color: &Self,
        background_color: &Self,
    ) -> String {
        format!(
            "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m{}\x1b[0m",
            foreground_color.color.r,
            foreground_color.color.g,
            foreground_color.color.b, // foreground color
            background_color.color.r,
            background_color.color.g,
            background_color.color.b, // background color
            text
        )
    }

    fn distance(color1: &Self, color2: &Self) -> f32 {
        // Equivalent to d = sqrt(r²+g²+b²+a²)
        RGBColor::distance(&color1.color, &color2.color).hypot(
            (f32::from(color1.opacity) - f32::from(color2.opacity)) / 255.,
        )
    }

    fn mix(colors: &[Self]) -> Self {
        let mut sum_opacity = 0;
        for color in colors {
            sum_opacity += usize::from(color.opacity);
        }

        let colors_len = colors.len();

        #[allow(clippy::cast_possible_truncation)]
        Self {
            opacity: (sum_opacity / colors_len).clamp(0, 255) as u8,
            color: RGBColor::mix(
                &colors.iter().map(|x| x.color).collect::<Vec<_>>(),
            ),
        }
    }
}

impl Shadable for ARGBColor {
    fn adjust_lightness(&self, amount: f32) -> Self {
        Self {
            opacity: self.opacity,
            color: self.color.adjust_lightness(amount),
        }
    }
}

impl ARGBColor {
    pub const TRANSPARENT: Self = Self {
        opacity: 0,
        color: RGBColor::BLACK,
    };
}

impl const From<RGBColor> for ARGBColor {
    fn from(value: RGBColor) -> Self {
        Self {
            opacity: u8::MAX,
            color: value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color() {
        let msg = ARGBColor::color(
            "test",
            &RGBColor::RED.into(),
            &RGBColor::BLACK.into(),
        );
        assert!(msg.chars().count() > 4);
    }
}
