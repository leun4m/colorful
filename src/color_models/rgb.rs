use crate::color_models::{number_utils, Color};
use std::fmt::{Display, Formatter, Result};

/// Contains predefined colors
pub mod presets;

/// Representation of a color model stored as RGB channels.
///
/// Each channel is stored as `u8` (0-255)
#[derive(Debug)]
pub struct RGBColor {
    r: u8,
    g: u8,
    b: u8,
}

/// The maximum value for each channel
pub const CHANNEL_MAX: u32 = 255;

/// White as `RGBColor`
pub const WHITE: RGBColor = presets::WHITE;

/// Black as `RGBColor`
pub const BLACK: RGBColor = presets::BLACK;

impl RGBColor {
    /// Creates a new `RGBColor`, setting all values to zero.
    ///
    /// This is *black*.
    pub fn new() -> Self {
        RGBColor::from_rgb(0, 0, 0)
    }

    /// Creates a new `RGBColor` from the given integer values.
    ///
    /// # Arguments
    /// - `r`: red
    /// - `g`: green    
    /// - `b`: blue
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        RGBColor { r, g, b }
    }

    /// Creates a new `RGBColor` from the given floating point values.
    ///
    /// # Arguments
    /// - `r`: red
    /// - `g`: green
    /// - `b`: blue
    ///
    /// # Please note
    /// Expects values from 0.0 to 1.0 (both inclusive)
    /// - Any values > 1 will be treated as 1
    /// - Any values < 0 it will be treated as 0
    pub fn from_rgb_f64(r: f64, g: f64, b: f64) -> Self {
        RGBColor::from_rgb(
            number_utils::to_byte_repr(r),
            number_utils::to_byte_repr(g),
            number_utils::to_byte_repr(b),
        )
    }

    /// Creates a new `RGBColor` from the given hex string.
    ///
    /// # Arguments
    /// - `hex`: the hexadecimal string to be converted
    ///
    /// # Please note
    /// 1. Accepts strings only with the following format and length:
    ///     - `aabbcc` (`rrggbb`)
    ///     - `abc` (`rgb`)
    /// 2. Make sure the Hex contains only valid (hexademical) digits:
    /// `0123456789abcdef`
    ///
    /// It will `panic` otherwise!
    pub fn from_hex(hex: &str) -> Self {
        let length = hex.chars().count();
        let value =
            u32::from_str_radix(hex, 16).expect(format!("HEX is invalid: {}", hex).as_str());

        if length == 6 {
            RGBColor::from_int(value, 256)
        } else if length == 3 {
            RGBColor::from_int(value, 16)
        } else {
            panic!("HEX number has invalid length: {}", length);
        }
    }

    /// Returns the value of channel red
    pub fn red(&self) -> u8 {
        self.r
    }

    /// Returns the value of channel green
    pub fn green(&self) -> u8 {
        self.g
    }

    /// Returns the value of channel blue
    pub fn blue(&self) -> u8 {
        self.b
    }

    /// Sets the value of channel red
    pub fn set_red(&mut self, r: u8) {
        self.r = r
    }

    /// Sets the value of channel green
    pub fn set_green(&mut self, g: u8) {
        self.g = g
    }

    /// Sets the value of channel blue
    pub fn set_blue(&mut self, b: u8) {
        self.b = b
    }

    /// Converts `RGBColor` to an RGB Tuple
    pub fn as_tuple(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    /// Converts `RGBColor` to an RGB Tuple using fractions
    pub fn as_tuple_f64(&self) -> (f64, f64, f64) {
        number_utils::as_float_tuple(self.as_tuple())
    }

    /// Converts `RGBColor` to a `HEX` String (6 digits)
    ///
    /// e.g. white => `"ffffff"`
    pub fn to_hex(&self) -> String {
        let sum: u32 = ((self.r as u32) << 16) + ((self.g as u32) << 8) + (self.b as u32);
        format!("{:06x}", sum)
    }

    /// Converts `RGBColor` to a 3 digit `HEX` String
    ///
    /// e.g. white => `"fff"`
    ///
    /// **Warning:** This is a *lossy* compression.
    /// It will round to the nearest value
    pub fn to_hex_short(&self) -> String {
        let r = (self.r as f64 / CHANNEL_MAX as f64 * 15 as f64).round() as u32;
        let g = (self.g as f64 / CHANNEL_MAX as f64 * 15 as f64).round() as u32;
        let b = (self.b as f64 / CHANNEL_MAX as f64 * 15 as f64).round() as u32;

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

        let factor = CHANNEL_MAX / (base - 1);
        let bit_move = (base as f64).log2() as u32;

        let b = ((value % base) * factor) as u8;
        value >>= bit_move;
        let g = ((value % base) * factor) as u8;
        value >>= bit_move;
        let r = ((value % base) * factor) as u8;

        RGBColor::from_rgb(r, g, b)
    }
}

impl From<(u8, u8, u8)> for RGBColor {
    /// Creates a new `RGBColor` from the given tuple.
    ///
    /// Works similar to [from_rgb](#method.from_rgb)
    fn from(rgb: (u8, u8, u8)) -> Self {
        RGBColor::from_rgb(rgb.0, rgb.1, rgb.2)
    }
}

impl From<(f64, f64, f64)> for RGBColor {
    /// Creates a new `RGBColor` from the given tuple of floating point values
    ///
    /// Works similar to [from_rgb_f64](#method.from_rgb_f64)
    fn from(rgb: (f64, f64, f64)) -> Self {
        RGBColor::from_rgb_f64(rgb.0, rgb.1, rgb.2)
    }
}

impl Display for RGBColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "(R:{}, G:{}, B:{})", self.r, self.g, self.b)
    }
}

impl PartialEq for RGBColor {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

impl Color for RGBColor {
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
        assert_eq!(BLACK, RGBColor::new());
    }

    #[test]
    fn set_red_() {
        let mut color = RGBColor::new();
        assert_eq!(0, color.red());
        color.set_red(3);
        assert_eq!(3, color.red());
        assert_eq!(0, color.green());
        assert_eq!(0, color.blue());
    }

    #[test]
    fn set_green_() {
        let mut color = RGBColor::new();
        assert_eq!(0, color.green());
        color.set_green(42);
        assert_eq!(0, color.red());
        assert_eq!(42, color.green());
        assert_eq!(0, color.blue());
    }

    #[test]
    fn set_blue_() {
        let mut color = RGBColor::new();
        assert_eq!(0, color.blue());
        color.set_blue(127);
        assert_eq!(0, color.red());
        assert_eq!(0, color.green());
        assert_eq!(127, color.blue());
    }

    #[test]
    fn as_tuple_() {
        let color = RGBColor::from((1, 27, 49));
        assert_eq!((1, 27, 49), color.as_tuple());
    }

    #[test]
    fn from_int_4() {
        assert_eq!(BLACK, RGBColor::from_int(0, 4));
        assert_eq!(WHITE, RGBColor::from_int(u32::pow(4, 3) - 1, 4));

        assert_eq!(BLACK, RGBColor::from_int(0, 16));
        assert_eq!(WHITE, RGBColor::from_int(u32::pow(16, 3) - 1, 16));

        assert_eq!(BLACK, RGBColor::from_int(0, 256));
        assert_eq!(WHITE, RGBColor::from_int(u32::pow(256, 3) - 1, 256));
    }

    #[test]
    #[should_panic]
    fn from_int_fail() {
        RGBColor::from_int(0, 3);
        RGBColor::from_int(0, 9);
        RGBColor::from_int(0, 12);
    }

    #[test]
    fn from_hex_h6_presets() {
        assert_eq!(presets::WHITE, RGBColor::from_hex("ffffff"));
        assert_eq!(presets::BLACK, RGBColor::from_hex("000000"));
        assert_eq!(presets::RED, RGBColor::from_hex("ff0000"));
        assert_eq!(presets::GREEN, RGBColor::from_hex("00ff00"));
        assert_eq!(presets::BLUE, RGBColor::from_hex("0000ff"));
    }

    #[test]
    fn from_hex_h6_custom() {
        assert_eq!(
            RGBColor::from_rgb(166, 65, 21),
            RGBColor::from_hex("A64115")
        );
        assert_eq!(
            RGBColor::from_rgb(21, 166, 65),
            RGBColor::from_hex("15A641")
        );
        assert_eq!(
            RGBColor::from_rgb(65, 21, 166),
            RGBColor::from_hex("4115A6")
        );
    }

    #[test]
    #[should_panic]
    fn from_hex_too_long() {
        RGBColor::from_hex("abcdefg");
    }

    #[test]
    #[should_panic]
    fn from_hex_too_short() {
        RGBColor::from_hex("ab");
    }

    #[test]
    #[should_panic]
    fn from_hex_weird_chars() {
        RGBColor::from_hex("axx");
    }

    #[test]
    fn from_hex_h3_presets() {
        assert_eq!(presets::WHITE, RGBColor::from_hex("fff"));
        assert_eq!(presets::BLACK, RGBColor::from_hex("000"));
        assert_eq!(presets::RED, RGBColor::from_hex("f00"));
        assert_eq!(presets::GREEN, RGBColor::from_hex("0f0"));
        assert_eq!(presets::BLUE, RGBColor::from_hex("00f"));
    }

    #[test]
    fn from_hex_h3_custom() {
        assert_eq!(RGBColor::from_rgb(255, 51, 153), RGBColor::from_hex("f39"));
        assert_eq!(RGBColor::from_rgb(153, 255, 51), RGBColor::from_hex("9f3"));
        assert_eq!(RGBColor::from_rgb(51, 153, 255), RGBColor::from_hex("39f"));
    }

    #[test]
    fn from_hex_h3_gray() {
        assert_eq!(RGBColor::from_rgb(17, 17, 17), RGBColor::from_hex("111"));
        assert_eq!(RGBColor::from_rgb(34, 34, 34), RGBColor::from_hex("222"));
        assert_eq!(RGBColor::from_rgb(51, 51, 51), RGBColor::from_hex("333"));
    }

    #[test]
    fn from_rgb_tuple_custom() {
        assert_eq!(RGBColor::from_rgb(1, 2, 3), RGBColor::from((1, 2, 3)));
        assert_eq!(
            RGBColor::from_rgb(255, 0, 127),
            RGBColor::from((255, 0, 127))
        );
    }

    #[test]
    fn from_rgb_float_custom() {
        assert_eq!("ffffff", RGBColor::from_rgb_f64(1.0, 1.0, 1.0).to_hex());
        assert_eq!("000000", RGBColor::from_rgb_f64(0.0, 0.0, 0.0).to_hex());
        assert_eq!("7f7f7f", RGBColor::from_rgb_f64(0.5, 0.5, 0.5).to_hex());
        assert_eq!("333333", RGBColor::from_rgb_f64(0.2, 0.2, 0.2).to_hex());
    }

    #[test]
    fn from_rgb_float_more_than_one() {
        assert_eq!("ff0000", RGBColor::from_rgb_f64(2.0, 0.0, 0.0).to_hex());
    }

    #[test]
    fn from_rgb_float_less_than_zero() {
        assert_eq!("0000cc", RGBColor::from_rgb_f64(-0.5, -3.0, 0.8).to_hex());
    }

    #[test]
    fn to_rgb_tuple_presets() {
        assert_eq!((255, 255, 255), presets::WHITE.as_tuple());
        assert_eq!((0, 0, 0), presets::BLACK.as_tuple());
    }

    #[test]
    fn to_rgb_tuple_custom() {
        assert_eq!((2, 20, 200), RGBColor::from_rgb(2, 20, 200).as_tuple());
        assert_eq!((42, 13, 5), RGBColor::from_rgb(42, 13, 5).as_tuple());
        assert_eq!((80, 252, 1), RGBColor::from_rgb(80, 252, 1).as_tuple());
    }

    #[test]
    fn to_hex_presets() {
        assert_eq!("ffffff", presets::WHITE.to_hex());
        assert_eq!("000000", presets::BLACK.to_hex());
        assert_eq!("ff0000", presets::RED.to_hex());
        assert_eq!("00ff00", presets::GREEN.to_hex());
        assert_eq!("0000ff", presets::BLUE.to_hex());
    }

    #[test]
    fn to_hex_h3_custom() {
        assert_eq!("ff3399", RGBColor::from_hex("f39").to_hex());
        assert_eq!("225511", RGBColor::from_hex("251").to_hex());
        assert_eq!("aa3322", RGBColor::from_hex("a32").to_hex());
    }

    #[test]
    fn to_hex_short_presets() {
        assert_eq!("fff", presets::WHITE.to_hex_short());
        assert_eq!("000", presets::BLACK.to_hex_short());
        assert_eq!("f00", presets::RED.to_hex_short());
        assert_eq!("0f0", presets::GREEN.to_hex_short());
        assert_eq!("00f", presets::BLUE.to_hex_short());
    }

    #[test]
    fn to_hex_short_custom() {
        assert_eq!("eee", RGBColor::from_hex("f0f0f0").to_hex_short());
        assert_eq!("123", RGBColor::from_hex("102030").to_hex_short());
        assert_eq!("9ce", RGBColor::from_hex("a0c4ed").to_hex_short());
    }

    #[test]
    fn from_f64_tuple() {
        assert_eq!(
            RGBColor::from_rgb_f64(0.5, 0.4, 0.7),
            RGBColor::from((0.5, 0.4, 0.7))
        )
    }

    #[test]
    fn fmt_() {
        assert_eq!("(R:0, G:0, B:0)", format!("{}", BLACK));
        assert_eq!("(R:255, G:255, B:255)", format!("{}", WHITE));
        assert_eq!("(R:0, G:255, B:0)", format!("{}", presets::GREEN));
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
