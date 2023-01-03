use crate::models::hsv::HSV;
use crate::models::rgb::RGBColor;
use crate::models::Color;
use crate::{converter, number_utils, RGB24};
use std::fmt::{Display, Formatter, Result};

/// 48-bit RGB color
///
/// This is a *deep color*, meaning every color channel consists of `16-bit` (0 - 65535).
///
#[derive(Copy, Clone, Debug, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RGB48 {
    r: u16,
    g: u16,
    b: u16,
}

impl RGB48 {
    /// Converts [`RGB48`] -> [`RGB24`]
    ///
    /// # Careful
    /// This is a lossy conversion
    pub fn to_rgb48(&self) -> RGB24 {
        converter::rgb48_to_rgb24(self)
    }
}

impl RGBColor<u16> for RGB48 {
    const MIN: u16 = u16::MIN;

    const MAX: u16 = u16::MAX;

    const WHITE: Self = Self {
        r: u16::MAX,
        g: u16::MAX,
        b: u16::MAX,
    };

    const BLACK: Self = Self {
        r: u16::MIN,
        g: u16::MIN,
        b: u16::MIN,
    };

    const RED: Self = Self {
        r: u16::MAX,
        g: u16::MIN,
        b: u16::MIN,
    };

    const GREEN: Self = Self {
        r: u16::MIN,
        g: u16::MAX,
        b: u16::MIN,
    };

    const BLUE: Self = Self {
        r: u16::MIN,
        g: u16::MIN,
        b: u16::MAX,
    };

    fn from_rgb(r: u16, g: u16, b: u16) -> Self {
        Self { r, g, b }
    }

    fn from_rgb_f64(r: f64, g: f64, b: f64) -> Self {
        Self::from_rgb(
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

    fn as_tuple(&self) -> (u16, u16, u16) {
        (self.r, self.g, self.b)
    }

    fn as_tuple_f64(&self) -> (f64, f64, f64) {
        (
            self.r as f64 / RGB48::MAX as f64,
            self.g as f64 / RGB48::MAX as f64,
            self.b as f64 / RGB48::MAX as f64,
        )
    }

    fn to_hsv(&self) -> HSV {
        converter::rgb_to_hsv(self)
    }
}

impl From<(u16, u16, u16)> for RGB48 {
    /// Creates a new `RGB48` from the given tuple.
    ///
    /// Works similar to [from_rgb](#method.from_rgb)
    fn from(rgb: (u16, u16, u16)) -> Self {
        Self::from_rgb(rgb.0, rgb.1, rgb.2)
    }
}

impl From<(f64, f64, f64)> for RGB48 {
    /// Creates a new `RGB48` from the given tuple of floating point values
    ///
    /// Works similar to [from_rgb_f64](#method.from_rgb_f64)
    fn from(rgb: (f64, f64, f64)) -> Self {
        Self::from_rgb_f64(rgb.0, rgb.1, rgb.2)
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
        self == &Self::WHITE
    }

    fn is_black(&self) -> bool {
        self == &Self::BLACK
    }
}

impl Default for RGB48 {
    /// Creates a new `RGB`, setting all values to zero
    ///
    /// This is *black*.
    fn default() -> Self {
        Self::BLACK
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_() {
        assert_eq!(RGB48::BLACK, RGB48::default());
    }

    #[test]
    fn set_r_() {
        let mut color = RGB48::default();
        assert_eq!(0, color.r());
        color.set_r(3);
        assert_eq!(3, color.r());
        assert_eq!(0, color.g());
        assert_eq!(0, color.b());
    }

    #[test]
    fn set_g_() {
        let mut color = RGB48::default();
        assert_eq!(0, color.g());
        color.set_g(42);
        assert_eq!(0, color.r());
        assert_eq!(42, color.g());
        assert_eq!(0, color.b());
    }

    #[test]
    fn set_b_() {
        let mut color = RGB48::default();
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
