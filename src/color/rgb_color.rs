use crate::color::{
    Color,
    Shadable,
};

/// Defines an RGB color used for foreground and background coloring of text.
/// `r`, `g`, `b` are the red, green and blue components of the color respectively.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// TODO: Check if this impl can be const
impl Color for RGBColor {
    fn blend(color_top: &Self, _color_bottom: &Self) -> Self {
        *color_top
    }

    fn color(
        text: &str,
        foreground_color: &Self,
        background_color: &Self,
    ) -> String {
        format!(
            "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m{}\x1b[0m",
            foreground_color.r,
            foreground_color.g,
            foreground_color.b, // foreground color
            background_color.r,
            background_color.g,
            background_color.b, // background color
            text
        )
    }

    #[rustfmt::skip]
    #[allow(clippy::suboptimal_flops)]
    fn distance(color1: &Self, color2: &Self) -> f32 {
        (
            ((f32::from(color1.r) - f32::from(color2.r)) / 255.).powi(2) +
            ((f32::from(color1.g) - f32::from(color2.g)) / 255.).powi(2) +
            ((f32::from(color1.b) - f32::from(color2.b)) / 255.).powi(2)
        )
        .sqrt()
    }

    fn mix(colors: &[Self]) -> Self {
        let mut sum = (0, 0, 0);
        for color in colors {
            sum.0 += usize::from(color.r);
            sum.1 += usize::from(color.g);
            sum.2 += usize::from(color.b);
        }
        let colors_len = colors.len();

        #[allow(clippy::cast_possible_truncation)]
        Self {
            r: (sum.0 / colors_len).clamp(0, 255) as u8,
            g: (sum.1 / colors_len).clamp(0, 255) as u8,
            b: (sum.2 / colors_len).clamp(0, 255) as u8,
        }
    }
}

impl Shadable for RGBColor {
    fn adjust_lightness(&self, amount: f32) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        Self {
            r: (f32::from(self.r) * (1. + amount)) as u8,
            g: (f32::from(self.g) * (1. + amount)) as u8,
            b: (f32::from(self.b) * (1. + amount)) as u8,
        }
    }
}

impl RGBColor {
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
    pub const DARK_GRAY: Self = Self {
        r: 63,
        g: 63,
        b: 63,
    };
    pub const GRAY: Self = Self {
        r: 127,
        g: 127,
        b: 127,
    };
    pub const LIGHT_GRAY: Self = Self {
        r: 191,
        g: 191,
        b: 191,
    };
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
    };
    pub const RED: Self = Self { r: 255, g: 0, b: 0 };
    pub const GREEN: Self = Self { r: 0, g: 255, b: 0 };
    pub const BLUE: Self = Self { r: 0, g: 0, b: 255 };
    pub const YELLOW: Self = Self {
        r: 255,
        g: 255,
        b: 0,
    };
    pub const CYAN: Self = Self {
        r: 0,
        g: 255,
        b: 255,
    };
    pub const MAGENTA: Self = Self {
        r: 255,
        g: 0,
        b: 255,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{
        RngExt,
        rng,
    };

    #[test]
    fn blend() {
        let mut rng = rng();
        let top = RGBColor {
            r: rng.random_range(0..=255),
            g: rng.random_range(0..=255),
            b: rng.random_range(0..=255),
        };
        let bottom = RGBColor {
            r: rng.random_range(0..=255),
            g: rng.random_range(0..=255),
            b: rng.random_range(0..=255),
        };
        assert_eq!(top, RGBColor::blend(&top, &bottom));
    }

    #[test]
    fn color() {
        let msg =
            RGBColor::color("test", &RGBColor::RED, &RGBColor::BLACK);
        assert!(msg.chars().count() > 4);
    }

    #[test]
    fn group() {
        let colors = [RGBColor::BLACK, RGBColor::WHITE, RGBColor::BLACK];
        let grouping = RGBColor::group(&colors);
        assert_ne!(grouping[0], grouping[1]);
        assert_ne!(grouping[1], grouping[2]);
        assert_eq!(grouping[0], grouping[2]);
    }

    #[test]
    fn group_equal() {
        let colors = [RGBColor::BLACK, RGBColor::BLACK, RGBColor::BLACK];
        let grouping = RGBColor::group(&colors);
        assert_eq!(grouping[0], grouping[1]);
        assert_eq!(grouping[1], grouping[2]);
        assert_eq!(grouping[0], grouping[2]);
    }
}
