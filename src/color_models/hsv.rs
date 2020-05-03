use crate::color_models::Color;
use crate::number_utils;

/// Representation of a color_models stored as HSV channels.
///
/// Each channel is stored as `f64`
///
/// - **H** in degrees (0.0 - 360.0)
/// - **S/V** as fraction (0.0 - 1.0)
#[derive(Debug)]
pub struct HSVColor {
    h: f64,
    s: f64,
    v: f64,
}

/// Used for the precision of equality between to HSV Colors
pub const EPSILON: f64 = 0.000_000_1;

/// White as `HSVColor`
pub const WHITE: HSVColor = HSVColor {
    h: 0.0,
    s: 0.0,
    v: 1.0,
};

/// Black as `HSVColor`
pub const BLACK: HSVColor = HSVColor {
    h: 0.0,
    s: 0.0,
    v: 0.0,
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

impl HSVColor {
    /// Creates a new `HSVColor`, setting all values to zero
    ///
    /// This is *black*.
    pub fn new() -> Self {
        HSVColor::from_hsv(0.0, 0.0, 0.0)
    }

    /// Creates a new `HSVColor` from the given floating point values.
    ///
    /// # Arguments
    /// **H**
    /// Will transform any values above 360 to 360
    /// Will transform any values below 0 to 0
    ///
    /// # Panics
    /// Will panic if at least one of the values is NaN!
    pub fn from_hsv(h: f64, s: f64, v: f64) -> Self {
        assert!(
            !h.is_nan() && !s.is_nan() && !v.is_nan(),
            "At least one of the given values is NAN"
        );

        HSVColor {
            h: number_utils::convert_to_range(h, H_MIN, H_MAX),
            s: number_utils::convert_to_range(s, S_MIN, S_MAX),
            v: number_utils::convert_to_range(v, V_MIN, V_MAX),
        }
    }

    /// Creates a new `HSVColor` from the given `u8` values.
    ///
    /// Computes the range of 0 - 255 to the value:
    ///
    /// # Examples
    /// - (0, 0, 0) => (0.0, 0.0, 0.0)
    /// - (255, 255, 255) => (360.0, 1.0, 1.0)
    /// - (51, 51, 51) => (72.0, 0.2, 0.2)
    ///
    pub fn from_hsv_u8(h: u8, s: u8, v: u8) -> Self {
        HSVColor::from_hsv(
            h as f64 / u8::MAX as f64 * H_MAX,
            s as f64 / u8::MAX as f64 * S_MAX,
            v as f64 / u8::MAX as f64 * V_MAX,
        )
    }

    /// Returns values as tuple (H, S, V)
    pub fn as_tuple(&self) -> (f64, f64, f64) {
        (self.h, self.s, self.v)
    }

    /// Returns values as tuple (H, S, V)
    ///
    /// # Examples
    /// - (0.0, 0.0, 0.0) => (0, 0, 0)
    /// - (360.0, 1.0, 1.0) => (255, 255, 255)
    /// - (72.0, 0.2, 0.2) => (51, 51, 51)
    pub fn as_tuple_u8(&self) -> (u8, u8, u8) {
        (
            (self.h / H_MAX * u8::MAX as f64) as u8,
            (self.s / S_MAX * u8::MAX as f64) as u8,
            (self.v / V_MAX * u8::MAX as f64) as u8,
        )
    }

    /// Returns value of channel **hue**
    pub fn h(&self) -> f64 {
        self.h
    }

    /// Returns value of channel **saturation+*
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

impl From<(f64, f64, f64)> for HSVColor {
    fn from(hsv: (f64, f64, f64)) -> Self {
        HSVColor::from_hsv(hsv.0, hsv.1, hsv.2)
    }
}

impl Color for HSVColor {
    fn is_white(&self) -> bool {
        self == &WHITE
    }

    fn is_black(&self) -> bool {
        self == &BLACK
    }
}

impl PartialEq for HSVColor {
    /// Checks if both colors are equal.
    ///
    /// Since this uses f64 it will check against [EPSILON](constant.EPSILON.html)
    fn eq(&self, other: &Self) -> bool {
        // Compare floating points
        number_utils::approx_equal_f64(self.h, other.h, EPSILON)
            && number_utils::approx_equal_f64(self.s, other.s, EPSILON)
            && number_utils::approx_equal_f64(self.v, other.v, EPSILON)
    }
}

#[cfg(test)]
mod tests {
    use crate::color_models::hsv::{
        HSVColor, BLACK, H_MAX, H_MIN, S_MAX, S_MIN, V_MAX, V_MIN, WHITE,
    };
    use crate::color_models::rgb24::presets;
    use crate::color_models::Color;

    #[test]
    fn getter_setter() {
        let mut color = HSVColor::new();
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
            HSVColor::from_hsv(0.0, 0.0, 0.0),
            HSVColor::from_hsv_u8(u8::MIN, u8::MIN, u8::MIN)
        );
        assert_eq!(
            HSVColor::from_hsv(72.0, 0.2, 0.2),
            HSVColor::from_hsv_u8(51, 51, 51)
        );
        assert_eq!(
            HSVColor::from_hsv(360.0, 1.0, 1.0),
            HSVColor::from_hsv_u8(u8::MAX, u8::MAX, u8::MAX)
        );
    }

    #[test]
    fn as_tuple_u8_works() {
        assert_eq!(
            (0, 0, 0),
            HSVColor::from_hsv(H_MIN, S_MIN, V_MIN).as_tuple_u8()
        );
        assert_eq!(
            (255, 255, 255),
            HSVColor::from_hsv(H_MAX, S_MAX, V_MAX).as_tuple_u8()
        );
        assert_eq!(
            (127, 127, 51),
            HSVColor::from_hsv(H_MAX / 2.0, S_MAX / 2.0, V_MAX / 5.0).as_tuple_u8()
        );
    }

    #[test]
    fn white_black() {
        assert!(WHITE.is_white());
        assert!(BLACK.is_black());
    }

    #[test]
    fn from_f64_tuple() {
        assert_eq!(
            HSVColor::from_hsv(0.5, 0.8, 0.9),
            HSVColor::from((0.5, 0.8, 0.9))
        )
    }

    #[test]
    #[should_panic]
    fn from_hsv_nan_panic() {
        println!("{}", f64::NAN);
        HSVColor::from_hsv(f64::NAN, 1.0, 1.0);
    }

    #[test]
    fn from_hsv_value_transform() {
        assert_eq!(
            HSVColor::from_hsv(H_MIN, S_MIN, V_MIN),
            HSVColor::from_hsv(H_MIN - 1.0, S_MIN - 1.0, V_MIN - 1.0)
        );
        assert_eq!(
            HSVColor::from_hsv(H_MAX, S_MAX, V_MAX),
            HSVColor::from_hsv(H_MAX + 1.0, S_MAX + 1.0, V_MAX + 1.0)
        );

        assert_eq!(
            HSVColor::from_hsv(H_MIN, S_MIN, V_MIN),
            HSVColor::from_hsv(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY)
        );
        assert_eq!(
            HSVColor::from_hsv(H_MAX, S_MAX, V_MAX),
            HSVColor::from_hsv(f64::INFINITY, f64::INFINITY, f64::INFINITY)
        );
    }
}
