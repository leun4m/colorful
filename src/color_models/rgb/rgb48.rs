use crate::color_models::rgb::RGB;
use crate::color_models::Color;
use crate::number_utils;
use std::fmt::{Display, Formatter, Result};

/// Representation of a color model stor as RGB channels.
///
/// This is the most widespread variant of RGB called
/// [True color (24-bit)](https://en.wikipedia.org/wiki/Color_depth#True_color_(24-bit))
/// meaning every color channel consists of `8-bit` (0-255).
///
#[derive(Debug)]
pub struct RGB48 {
    r: u16,
    g: u16,
    b: u16,
}

/// 100% white as `RGB48`
pub const WHITE: RGB48 = RGB48 {
    r: u16::MAX,
    g: u16::MAX,
    b: u16::MAX,
};

/// 100% black as `RGB48`
pub const BLACK: RGB48 = RGB48 {
    r: u16::MIN,
    g: u16::MIN,
    b: u16::MIN,
};

/// 100% red as `RGB48`
pub const RED: RGB48 = RGB48 {
    r: u16::MAX,
    g: u16::MIN,
    b: u16::MIN,
};

/// 100% green as `RGB48`
pub const GREEN: RGB48 = RGB48 {
    r: u16::MIN,
    g: u16::MAX,
    b: u16::MIN,
};

/// 100% blue as `RGB48`
pub const BLUE: RGB48 = RGB48 {
    r: u16::MIN,
    g: u16::MIN,
    b: u16::MAX,
};

impl RGB<u16> for RGB48 {
    fn new() -> Self {
        RGB48::from_rgb(0, 0, 0)
    }

    fn from_rgb(r: u16, g: u16, b: u16) -> Self {
        RGB48 { r, g, b }
    }

    fn from_rgb_f64(r: f64, g: f64, b: f64) -> Self {
        RGB48::from_rgb(
            number_utils::to_u16_repr(r),
            number_utils::to_u16_repr(g),
            number_utils::to_u16_repr(b),
        )
    }

    fn r(&self) -> u16 {
        self.r
    }

    fn g(&self) -> u16 {
        self.g
    }

    fn b(&self) -> u16 {
        self.b
    }

    fn set_r(&mut self, r: u16) {
        self.r = r;
    }

    fn set_g(&mut self, g: u16) {
        self.g = g;
    }

    fn set_b(&mut self, b: u16) {
        self.b = b;
    }

    fn min() -> u16 {
        u16::MIN
    }

    fn max() -> u16 {
        u16::MAX
    }

    fn as_tuple(&self) -> (u16, u16, u16) {
        (self.r, self.g, self.b)
    }

    fn as_tuple_f64(&self) -> (f64, f64, f64) {
        number_utils::as_float_tuple_u16(self.as_tuple())
    }
}

impl From<(u16, u16, u16)> for RGB48 {
    /// Creates a new `RGB48` from the given tuple.
    ///
    /// Works similar to [from_rgb](#method.from_rgb)
    fn from(rgb: (u16, u16, u16)) -> Self {
        RGB48::from_rgb(rgb.0, rgb.1, rgb.2)
    }
}

impl From<(f64, f64, f64)> for RGB48 {
    /// Creates a new `RGB48` from the given tuple of floating point values
    ///
    /// Works similar to [from_rgb_f64](#method.from_rgb_f64)
    fn from(rgb: (f64, f64, f64)) -> Self {
        RGB48::from_rgb_f64(rgb.0, rgb.1, rgb.2)
    }
}

impl Display for RGB48 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "(R:{}, G:{}, B:{})", self.r, self.g, self.b)
    }
}

impl PartialEq for RGB48 {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

impl Color for RGB48 {
    fn is_white(&self) -> bool {
        self == &WHITE
    }

    fn is_black(&self) -> bool {
        self == &BLACK
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_() {
        assert_eq!(BLACK, RGB48::new());
    }

    #[test]
    fn set_r_() {
        let mut color = RGB48::new();
        assert_eq!(0, color.r());
        color.set_r(3);
        assert_eq!(3, color.r());
        assert_eq!(0, color.g());
        assert_eq!(0, color.b());
    }

    #[test]
    fn set_g_() {
        let mut color = RGB48::new();
        assert_eq!(0, color.g());
        color.set_g(42);
        assert_eq!(0, color.r());
        assert_eq!(42, color.g());
        assert_eq!(0, color.b());
    }

    #[test]
    fn set_b_() {
        let mut color = RGB48::new();
        assert_eq!(0, color.b());
        color.set_b(127);
        assert_eq!(0, color.r());
        assert_eq!(0, color.g());
        assert_eq!(127, color.b());
    }

    #[test]
    fn as_tuple_() {
        let color = RGB48::from((1, 27, 49));
        assert_eq!((1, 27, 49), color.as_tuple());
    }
}
