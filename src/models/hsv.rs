use crate::models::rgb::RGB;
use crate::models::Color;
use crate::{converter, RGB24};
use crate::{number_utils, RGB48};

/// HSV color - based on *hue, saturation, value*
///
/// Each channel is stored as `f64`
///
/// - `h` in degrees (0.0 - 360.0)
/// - `s` as fraction (0.0 - 1.0)
/// - `v` as fraction (0.0 - 1.0)
///
/// [RGB]: crate::models::rgb::RGB
/// [RGB24]: crate::models::rgb::rgb24::RGB
/// [RGB48]: crate::models::rgb::rgb48::RGB48
#[derive(Debug)]
pub struct HSV {
    h: f64,
    s: f64,
    v: f64,
}

impl HSV {
    /// Used for the precision of equality between to HSV Colors
    pub const EPSILON: f64 = 0.000_000_1;

    /// 100% white
    pub const WHITE: HSV = HSV {
        h: 0.0,
        s: 0.0,
        v: 1.0,
    };

    /// 100% black
    pub const BLACK: HSV = HSV {
        h: 0.0,
        s: 0.0,
        v: 0.0,
    };

    /// 100% red
    pub const RED: HSV = HSV {
        h: 0.0,
        s: 1.0,
        v: 1.0,
    };

    /// 100% green
    pub const GREEN: HSV = HSV {
        h: 120.0,
        s: 1.0,
        v: 1.0,
    };

    /// 100% blue
    pub const BLUE: HSV = HSV {
        h: 240.0,
        s: 1.0,
        v: 1.0,
    };

    /// The minimum for channel **hue**
    pub const H_MIN: f64 = 0.0;
    /// The minimum for channel **saturation**
    pub const S_MIN: f64 = 0.0;
    /// The minimum for channel **value**
    pub const V_MIN: f64 = 0.0;
    /// The maximum for channel **hue**
    pub const H_MAX: f64 = 360.0;
    /// The maximum for channel **saturation**
    pub const S_MAX: f64 = 1.0;
    /// The maximum for channel **value**
    pub const V_MAX: f64 = 1.0;

    /// Creates a new `HSV`, setting all values to zero
    ///
    /// This is *black*.
    pub fn new() -> Self {
        HSV::from_hsv(0.0, 0.0, 0.0)
    }

    /// Creates a new `HSV` from the given floating point values.
    ///
    /// # Parameters
    /// - `h`: **hue**. Expects `0 <= h < 360`.
    ///     Values outside of that range will be transformed using modulo.
    /// - `s`: **saturation**. Expects `0 <= s <= 1`.
    ///     Values greater than 1 will be straightened to 1. Values lower than 0 will be straightened to 0.
    /// - `v`: **value**. Expects `0 <= s <= 1`.
    ///     Values greater than 1 will be straightened to 1. Values lower than 0 will be straightened to 0.
    ///
    /// # Panics
    /// - if one of the values is NaN
    /// - if `h` is infinite
    pub fn from_hsv(h: f64, s: f64, v: f64) -> Self {
        assert!(
            !h.is_nan() && !s.is_nan() && !v.is_nan(),
            "At least one of the given values is NAN"
        );
        assert!(h.is_finite(), "h must be finite!");

        HSV {
            h: h.rem_euclid(HSV::H_MAX),
            s: number_utils::convert_to_range(s, HSV::S_MIN, HSV::S_MAX),
            v: number_utils::convert_to_range(v, HSV::V_MIN, HSV::V_MAX),
        }
    }

    /// Creates a new `HSV` from the given `u8` values.
    ///
    /// Computes the range of 0 - 255 to the HSV range.
    ///
    /// # Parameters
    /// - `h`: **hue**. This will map *0 to 0* and *255 to 360* and everything between accordingly.
    /// - `s`: **saturation**. This will map *0 to 0* and *255 to 1* and everything between accordingly.
    /// - `v`: **value**. This will map *0 to 0* and *255 to 1* and everything between accordingly.
    ///
    /// # Examples
    /// - (0, 0, 0) => (0.0, 0.0, 0.0)
    /// - (255, 255, 255) => (360.0, 1.0, 1.0)
    /// - (51, 51, 51) => (72.0, 0.2, 0.2)
    pub fn from_hsv_u8(h: u8, s: u8, v: u8) -> Self {
        HSV::from_hsv(
            h as f64 / u8::MAX as f64 * HSV::H_MAX,
            s as f64 / u8::MAX as f64 * HSV::S_MAX,
            v as f64 / u8::MAX as f64 * HSV::V_MAX,
        )
    }

    /// Converts values to tuple
    ///
    /// # Returns
    /// Values as tuple (H, S, V)
    pub fn as_tuple(&self) -> (f64, f64, f64) {
        (self.h, self.s, self.v)
    }

    /// Converts values to u8 tuple
    ///
    /// # Examples
    /// - (0.0, 0.0, 0.0) => (0, 0, 0)
    /// - (360.0, 1.0, 1.0) => (255, 255, 255)
    /// - (72.0, 0.2, 0.2) => (51, 51, 51)
    ///
    /// # Returns
    /// Values as tuple (H, S, V)
    pub fn as_tuple_u8(&self) -> (u8, u8, u8) {
        print!("{}", self.h);
        (
            (self.h / Self::H_MAX * u8::MAX as f64) as u8,
            (self.s / Self::S_MAX * u8::MAX as f64) as u8,
            (self.v / Self::V_MAX * u8::MAX as f64) as u8,
        )
    }

    /// Converts this to [`RGB`]
    pub fn to_rgb<T, U>(&self) -> T
    where
        T: RGB<U>,
    {
        converter::hsv_to_rgb(self)
    }

    /// Converts this to [`RGB24`]
    pub fn to_rgb24(&self) -> RGB24 {
        converter::hsv_to_rgb::<RGB24, u8>(self)
    }

    /// Converts this to [`RGB48`]
    pub fn to_rgb48(&self) -> RGB48 {
        converter::hsv_to_rgb::<RGB48, u16>(self)
    }

    /// Returns value of channel **hue**
    pub fn h(&self) -> f64 {
        self.h
    }

    /// Returns value of channel **saturation**
    pub fn s(&self) -> f64 {
        self.s
    }

    /// Returns value of channel **value**
    pub fn v(&self) -> f64 {
        self.v
    }

    /// Sets value of channel **hue**
    pub fn set_h(&mut self, h: f64) {
        self.h = h;
    }

    /// Sets value of channel **saturation**
    pub fn set_s(&mut self, s: f64) {
        self.s = s;
    }

    /// Sets value of channel **value**
    pub fn set_v(&mut self, v: f64) {
        self.v = v;
    }
}

impl From<(f64, f64, f64)> for HSV {
    fn from(hsv: (f64, f64, f64)) -> Self {
        HSV::from_hsv(hsv.0, hsv.1, hsv.2)
    }
}

impl Color for HSV {
    fn is_white(&self) -> bool {
        self == &HSV::WHITE
    }

    fn is_black(&self) -> bool {
        self == &HSV::BLACK
    }
}

impl PartialEq for HSV {
    /// Checks if both colors are equal.
    ///
    /// Since this uses f64 it will check against [EPSILON](HSV::EPSILON)
    fn eq(&self, other: &Self) -> bool {
        // Compare floating points
        number_utils::approx_equal_f64(self.h, other.h, HSV::EPSILON)
            && number_utils::approx_equal_f64(self.s, other.s, HSV::EPSILON)
            && number_utils::approx_equal_f64(self.v, other.v, HSV::EPSILON)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::hsv::HSV;
    use crate::models::Color;

    #[test]
    fn getter_setter() {
        let mut color = HSV::new();
        assert_eq!(0.0, color.h());
        assert_eq!(0.0, color.s());
        assert_eq!(0.0, color.v());
        color.set_h(120.0);
        color.set_s(0.5);
        color.set_v(1.0);
        assert_eq!(120.0, color.h());
        assert_eq!(0.5, color.s());
        assert_eq!(1.0, color.v());
    }

    #[test]
    fn from_hsv_u8_works() {
        assert_eq!(
            HSV::from_hsv(0.0, 0.0, 0.0),
            HSV::from_hsv_u8(u8::MIN, u8::MIN, u8::MIN)
        );
        assert_eq!(HSV::from_hsv(72.0, 0.2, 0.2), HSV::from_hsv_u8(51, 51, 51));
        assert_eq!(
            HSV::from_hsv(360.0, 1.0, 1.0),
            HSV::from_hsv_u8(u8::MAX, u8::MAX, u8::MAX)
        );
    }

    #[test]
    fn as_tuple_u8_works() {
        assert_eq!(
            (0, 0, 0),
            HSV::from_hsv(HSV::H_MIN, HSV::S_MIN, HSV::V_MIN).as_tuple_u8()
        );
        assert_eq!(
            (0, 255, 255),
            HSV::from_hsv(HSV::H_MAX, HSV::S_MAX, HSV::V_MAX).as_tuple_u8()
        );
        assert_eq!(
            (127, 127, 51),
            HSV::from_hsv(HSV::H_MAX / 2.0, HSV::S_MAX / 2.0, HSV::V_MAX / 5.0).as_tuple_u8()
        );
    }

    #[test]
    fn white_black() {
        assert!(HSV::WHITE.is_white());
        assert!(HSV::BLACK.is_black());
    }

    #[test]
    fn from_f64_tuple() {
        assert_eq!(HSV::from_hsv(0.5, 0.8, 0.9), HSV::from((0.5, 0.8, 0.9)))
    }

    #[test]
    #[should_panic]
    fn from_hsv_nan_panic() {
        println!("{}", f64::NAN);
        HSV::from_hsv(f64::NAN, 1.0, 1.0);
    }

    #[test]
    fn from_hsv_value_transform() {
        assert_eq!(
            HSV::from_hsv(HSV::H_MAX - 1.0, HSV::S_MIN, HSV::V_MIN),
            HSV::from_hsv(HSV::H_MIN - 1.0, HSV::S_MIN - 1.0, HSV::V_MIN - 1.0)
        );
        assert_eq!(
            HSV::from_hsv(HSV::H_MIN + 1.0, HSV::S_MAX, HSV::V_MAX),
            HSV::from_hsv(HSV::H_MAX + 1.0, HSV::S_MAX + 1.0, HSV::V_MAX + 1.0)
        );

        assert_eq!(
            HSV::from_hsv(HSV::H_MIN, HSV::S_MIN, HSV::V_MIN),
            HSV::from_hsv(HSV::H_MIN, f64::NEG_INFINITY, f64::NEG_INFINITY)
        );

        assert_eq!(
            HSV::from_hsv(HSV::H_MAX, HSV::S_MAX, HSV::V_MAX),
            HSV::from_hsv(HSV::H_MAX, f64::INFINITY, f64::INFINITY)
        );
    }

    #[test]
    #[should_panic(expected = "h must be finite")]
    fn from_hsv_value_neg_infinite_h() {
        HSV::from_hsv(f64::NEG_INFINITY, HSV::S_MIN, HSV::V_MIN);
    }

    #[test]
    #[should_panic(expected = "h must be finite")]
    fn from_hsv_value_infinite_h() {
        HSV::from_hsv(f64::INFINITY, HSV::S_MIN, HSV::V_MIN);
    }
}
