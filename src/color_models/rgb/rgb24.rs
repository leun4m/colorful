use crate::color_converter;
use crate::color_models::hsv::HSVColor;
use crate::color_models::rgb::RGB;
use crate::color_models::Color;
use crate::number_utils;
use std::fmt::{Display, Formatter, Result};

/// Representation of a color model stored as RGB channels.
///
/// This is the most widespread variant of RGB called
/// [True color (24-bit)](https://en.wikipedia.org/wiki/Color_depth#True_color_(24-bit))
/// meaning every color channel consists of `8-bit` (0-255).
///
#[derive(Debug)]
pub struct RGB24 {
    r: u8,
    g: u8,
    b: u8,
}

/// 100% white as `RGB24`
pub const WHITE: RGB24 = RGB24 {
    r: u8::MAX,
    g: u8::MAX,
    b: u8::MAX,
};

/// 100% black as `RGB24`
pub const BLACK: RGB24 = RGB24 {
    r: u8::MIN,
    g: u8::MIN,
    b: u8::MIN,
};
/// 100% red as `RGB24`
pub const RED: RGB24 = RGB24 {
    r: u8::MAX,
    g: u8::MIN,
    b: u8::MIN,
};

/// 100% green as `RGB24`
pub const GREEN: RGB24 = RGB24 {
    r: u8::MIN,
    g: u8::MAX,
    b: u8::MIN,
};

/// 100% blue as `RGB24`
pub const BLUE: RGB24 = RGB24 {
    r: u8::MIN,
    g: u8::MIN,
    b: u8::MAX,
};

impl RGB24 {
    /// Creates a new `RGB24` from the given hex string.
    ///
    /// # Arguments
    /// - `hex`: the hexadecimal string to be converted
    ///
    /// # Please note
    /// 1. Accepts strings only with the following format and length:
    ///     - `aabbcc` (`rrggbb`)
    ///     - `abc` (`rgb24`)
    /// 2. Make sure the Hex contains only valid (hexademical) digits:
    /// `0123456789abcdef`
    ///
    /// It will `panic` otherwise!
    pub fn from_hex(hex: &str) -> Self {
        let length = hex.chars().count();
        let value =
            u32::from_str_radix(hex, 16).expect(format!("HEX is invalid: {}", hex).as_str());

        if length == 6 {
            RGB24::from_int(value, 256)
        } else if length == 3 {
            RGB24::from_int(value, 16)
        } else {
            panic!("HEX number has invalid length: {}", length);
        }
    }

    /// Converts `RGB24` to a `HEX` String (6 digits)
    ///
    /// e.g. white => `"ffffff"`
    pub fn to_hex(&self) -> String {
        let sum: u32 = ((self.r as u32) << 16) + ((self.g as u32) << 8) + (self.b as u32);
        format!("{:06x}", sum)
    }

    /// Converts `RGB24` to a 3 digit `HEX` String
    ///
    /// e.g. white => `"fff"`
    ///
    /// **Warning:** This is a *lossy* compression.
    /// It will round to the nearest value
    pub fn to_hex_short(&self) -> String {
        let r = (self.r as f64 / RGB24::max() as f64 * 15 as f64).round() as u32;
        let g = (self.g as f64 / RGB24::max() as f64 * 15 as f64).round() as u32;
        let b = (self.b as f64 / RGB24::max() as f64 * 15 as f64).round() as u32;

        let sum: u32 = (r << 8) + (g << 4) + b;
        format!("{:03x}", sum)
    }

    /// Converts an integer to the corresponding RGB Color
    ///
    /// **Important:** Works only for specific bases:
    /// 4, 16, 256
    ///
    /// # Panics
    /// Will panic if another base is provided!
    fn from_int(mut value: u32, base: u32) -> Self {
        assert!(
            base == 4 || base == 16 || base == 256,
            "base must be one of these [4, 16, 256] but is instead {}",
            base
        );

        let factor = RGB24::max() as u32 / (base - 1);
        let bit_move = (base as f64).log2() as u32;

        let b = ((value % base) * factor) as u8;
        value >>= bit_move;
        let g = ((value % base) * factor) as u8;
        value >>= bit_move;
        let r = ((value % base) * factor) as u8;

        RGB24::from_rgb(r, g, b)
    }
}

impl RGB<u8> for RGB24 {
    fn new() -> Self {
        RGB24::from_rgb(0, 0, 0)
    }

    fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        RGB24 { r, g, b }
    }

    fn from_rgb_f64(r: f64, g: f64, b: f64) -> Self {
        RGB24::from_rgb(
            number_utils::to_u8_repr(r),
            number_utils::to_u8_repr(g),
            number_utils::to_u8_repr(b),
        )
    }

    fn r(&self) -> u8 {
        self.r
    }

    fn g(&self) -> u8 {
        self.g
    }

    fn b(&self) -> u8 {
        self.b
    }

    fn set_r(&mut self, r: u8) {
        self.r = r
    }

    fn set_g(&mut self, g: u8) {
        self.g = g
    }

    fn set_b(&mut self, b: u8) {
        self.b = b
    }

    fn min() -> u8 {
        u8::MIN
    }

    fn max() -> u8 {
        u8::MAX
    }

    fn as_tuple_f64(&self) -> (f64, f64, f64) {
        (
            self.r as f64 / RGB24::max() as f64,
            self.g as f64 / RGB24::max() as f64,
            self.b as f64 / RGB24::max() as f64,
        )
    }
}

impl From<(u8, u8, u8)> for RGB24 {
    /// Creates a new `RGB24` from the given tuple.
    ///
    /// Works similar to [from_rgb](#method.from_rgb)
    fn from(rgb: (u8, u8, u8)) -> Self {
        RGB24::from_rgb(rgb.0, rgb.1, rgb.2)
    }
}

impl From<(f64, f64, f64)> for RGB24 {
    /// Creates a new `RGB24` from the given tuple of floating point values
    ///
    /// Works similar to [from_rgb_f64](#method.from_rgb_f64)
    fn from(rgb: (f64, f64, f64)) -> Self {
        RGB24::from_rgb_f64(rgb.0, rgb.1, rgb.2)
    }
}

impl Display for RGB24 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "(R:{}, G:{}, B:{})", self.r, self.g, self.b)
    }
}

impl PartialEq for RGB24 {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

impl Color for RGB24 {
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
        assert_eq!(BLACK, RGB24::new());
    }

    #[test]
    fn set_red_() {
        let mut color = RGB24::new();
        assert_eq!(0, color.r());
        color.set_r(3);
        assert_eq!(3, color.r());
        assert_eq!(0, color.g());
        assert_eq!(0, color.b());
    }

    #[test]
    fn set_green_() {
        let mut color = RGB24::new();
        assert_eq!(0, color.g());
        color.set_g(42);
        assert_eq!(0, color.r());
        assert_eq!(42, color.g());
        assert_eq!(0, color.b());
    }

    #[test]
    fn set_blue_() {
        let mut color = RGB24::new();
        assert_eq!(0, color.b());
        color.set_b(127);
        assert_eq!(0, color.r());
        assert_eq!(0, color.g());
        assert_eq!(127, color.b());
    }

    #[test]
    fn as_tuple_() {
        let color = RGB24::from((1, 27, 49));
        assert_eq!((1, 27, 49), color.as_tuple());
    }

    #[test]
    fn from_int_4() {
        assert_eq!(BLACK, RGB24::from_int(0, 4));
        assert_eq!(WHITE, RGB24::from_int(u32::pow(4, 3) - 1, 4));

        assert_eq!(BLACK, RGB24::from_int(0, 16));
        assert_eq!(WHITE, RGB24::from_int(u32::pow(16, 3) - 1, 16));

        assert_eq!(BLACK, RGB24::from_int(0, 256));
        assert_eq!(WHITE, RGB24::from_int(u32::pow(256, 3) - 1, 256));
    }

    #[test]
    #[should_panic]
    fn from_int_fail() {
        RGB24::from_int(0, 3);
        RGB24::from_int(0, 9);
        RGB24::from_int(0, 12);
    }

    #[test]
    fn from_hex_h6_presets() {
        assert_eq!(WHITE, RGB24::from_hex("ffffff"));
        assert_eq!(BLACK, RGB24::from_hex("000000"));
        assert_eq!(RED, RGB24::from_hex("ff0000"));
        assert_eq!(GREEN, RGB24::from_hex("00ff00"));
        assert_eq!(BLUE, RGB24::from_hex("0000ff"));
    }

    #[test]
    fn from_hex_h6_custom() {
        assert_eq!(RGB24::from_rgb(166, 65, 21), RGB24::from_hex("A64115"));
        assert_eq!(RGB24::from_rgb(21, 166, 65), RGB24::from_hex("15A641"));
        assert_eq!(RGB24::from_rgb(65, 21, 166), RGB24::from_hex("4115A6"));
    }

    #[test]
    #[should_panic]
    fn from_hex_too_long() {
        RGB24::from_hex("abcdefg");
    }

    #[test]
    #[should_panic]
    fn from_hex_too_short() {
        RGB24::from_hex("ab");
    }

    #[test]
    #[should_panic]
    fn from_hex_weird_chars() {
        RGB24::from_hex("axx");
    }

    #[test]
    fn from_hex_h3_presets() {
        assert_eq!(WHITE, RGB24::from_hex("fff"));
        assert_eq!(BLACK, RGB24::from_hex("000"));
        assert_eq!(RED, RGB24::from_hex("f00"));
        assert_eq!(GREEN, RGB24::from_hex("0f0"));
        assert_eq!(BLUE, RGB24::from_hex("00f"));
    }

    #[test]
    fn from_hex_h3_custom() {
        assert_eq!(RGB24::from_rgb(255, 51, 153), RGB24::from_hex("f39"));
        assert_eq!(RGB24::from_rgb(153, 255, 51), RGB24::from_hex("9f3"));
        assert_eq!(RGB24::from_rgb(51, 153, 255), RGB24::from_hex("39f"));
    }

    #[test]
    fn from_hex_h3_gray() {
        assert_eq!(RGB24::from_rgb(17, 17, 17), RGB24::from_hex("111"));
        assert_eq!(RGB24::from_rgb(34, 34, 34), RGB24::from_hex("222"));
        assert_eq!(RGB24::from_rgb(51, 51, 51), RGB24::from_hex("333"));
    }

    #[test]
    fn from_rgb_tuple_custom() {
        assert_eq!(RGB24::from_rgb(1, 2, 3), RGB24::from((1, 2, 3)));
        assert_eq!(RGB24::from_rgb(255, 0, 127), RGB24::from((255, 0, 127)));
    }

    #[test]
    fn from_rgb_float_custom() {
        assert_eq!("ffffff", RGB24::from_rgb_f64(1.0, 1.0, 1.0).to_hex());
        assert_eq!("000000", RGB24::from_rgb_f64(0.0, 0.0, 0.0).to_hex());
        assert_eq!("7f7f7f", RGB24::from_rgb_f64(0.5, 0.5, 0.5).to_hex());
        assert_eq!("333333", RGB24::from_rgb_f64(0.2, 0.2, 0.2).to_hex());
    }

    #[test]
    fn from_rgb_float_more_than_one() {
        assert_eq!("ff0000", RGB24::from_rgb_f64(2.0, 0.0, 0.0).to_hex());
    }

    #[test]
    fn from_rgb_float_less_than_zero() {
        assert_eq!("0000cc", RGB24::from_rgb_f64(-0.5, -3.0, 0.8).to_hex());
    }

    #[test]
    fn to_rgb_tuple_presets() {
        assert_eq!((255, 255, 255), WHITE.as_tuple());
        assert_eq!((0, 0, 0), BLACK.as_tuple());
    }

    #[test]
    fn to_rgb_tuple_custom() {
        assert_eq!((2, 20, 200), RGB24::from_rgb(2, 20, 200).as_tuple());
        assert_eq!((42, 13, 5), RGB24::from_rgb(42, 13, 5).as_tuple());
        assert_eq!((80, 252, 1), RGB24::from_rgb(80, 252, 1).as_tuple());
    }

    #[test]
    fn to_hex_presets() {
        assert_eq!("ffffff", WHITE.to_hex());
        assert_eq!("000000", BLACK.to_hex());
        assert_eq!("ff0000", RED.to_hex());
        assert_eq!("00ff00", GREEN.to_hex());
        assert_eq!("0000ff", BLUE.to_hex());
    }

    #[test]
    fn to_hex_h3_custom() {
        assert_eq!("ff3399", RGB24::from_hex("f39").to_hex());
        assert_eq!("225511", RGB24::from_hex("251").to_hex());
        assert_eq!("aa3322", RGB24::from_hex("a32").to_hex());
    }

    #[test]
    fn to_hex_short_presets() {
        assert_eq!("fff", WHITE.to_hex_short());
        assert_eq!("000", BLACK.to_hex_short());
        assert_eq!("f00", RED.to_hex_short());
        assert_eq!("0f0", GREEN.to_hex_short());
        assert_eq!("00f", BLUE.to_hex_short());
    }

    #[test]
    fn to_hex_short_custom() {
        assert_eq!("eee", RGB24::from_hex("f0f0f0").to_hex_short());
        assert_eq!("123", RGB24::from_hex("102030").to_hex_short());
        assert_eq!("9ce", RGB24::from_hex("a0c4ed").to_hex_short());
    }

    #[test]
    fn from_f64_tuple() {
        assert_eq!(
            RGB24::from_rgb_f64(0.5, 0.4, 0.7),
            RGB24::from((0.5, 0.4, 0.7))
        )
    }

    #[test]
    fn fmt_() {
        assert_eq!("(R:0, G:0, B:0)", format!("{}", BLACK));
        assert_eq!("(R:255, G:255, B:255)", format!("{}", WHITE));
        assert_eq!("(R:0, G:255, B:0)", format!("{}", GREEN));
    }

    #[test]
    fn is_white_() {
        assert!(WHITE.is_white())
    }

    #[test]
    fn is_black_() {
        assert!(BLACK.is_black())
    }
}
