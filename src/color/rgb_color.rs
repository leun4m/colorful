use crate::color::utils;

pub mod presets;

/// Representation of a color stored as RGB channels.
///
/// Each channel is stored as `u8` (0-255)
#[derive(Debug)]
pub struct RGBColor {
    r: u8,
    g: u8,
    b: u8,
}

pub const BASE: u32 = 255;

impl RGBColor {
    /// Creates `Color` from the given integer values.
    ///
    /// `r`: red, `g`: green, `b`: blue
    pub fn from_rgb(r: u8, g: u8, b: u8) -> RGBColor {
        RGBColor { r, g, b }
    }

    /// Creates `Color` from the given decimal values.
    ///
    /// Expects values from 0.0 to 1.0 (both inclusive)
    /// - If a value > 1 it will be treated as 1
    /// - If a value < 0 it will be treated as 0
    pub fn from_rgb_float(r: f64, g: f64, b: f64) -> RGBColor {
        RGBColor {
            r: utils::save_convert_float_to_byte(r),
            g: utils::save_convert_float_to_byte(g),
            b: utils::save_convert_float_to_byte(b),
        }
    }

    /// Creates `Color` from the given tuple.
    ///
    /// (r, g, b)
    pub fn from_rgb_tuple(tuple: (u8, u8, u8)) -> RGBColor {
        RGBColor::from_rgb(tuple.0, tuple.1, tuple.2)
    }

    /// Creates `Color` from the given hex string.
    ///
    /// Accepts strings only with the following formats and length:
    /// - `f0f0f0` (`rrggbb`)
    /// - `fff` (`rgb`)
    pub fn from_hex(hex: &str) -> RGBColor {
        let length = hex.chars().count();
        if length == 6 {
            RGBColor::from_any_hex(hex, 16 * 16)
        } else if length == 3 {
            RGBColor::from_any_hex(hex, 16)
        } else {
            panic!("HEX number has invalid length: {}", length);
        }
    }

    /// Returns the value for channel red
    pub fn get_red(&self) -> u8 {
        self.r
    }

    /// Returns the value for channel green
    pub fn get_green(&self) -> u8 {
        self.g
    }

    /// Returns the value for channel blue
    pub fn get_blue(&self) -> u8 {
        self.b
    }

    /// Converts `Color` to a RGB Tuple
    pub fn to_rgb_tuple(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    /// Converts `Color` to a HSV Tuple
    ///
    /// The tuple has the usual format:
    /// (hue, saturation, value)
    ///
    /// h in degrees (0 - 360)
    /// s, v in percent (0 - 1.0)
    pub fn to_hsv(&self) -> (f64, f64, f64) {
        let r = utils::as_float(self.r);
        let g = utils::as_float(self.g);
        let b = utils::as_float(self.b);

        let c_max = utils::get_max(r, g, b);
        let c_min = utils::get_min(r, g, b);
        let delta = c_max - c_min;

        let hue = if delta == 0.0 {
            0.0
        } else if r >= b && r >= g {
            60.0 * (((g - b) / delta) % 6.0)
        } else if g >= r && g >= b {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        let saturation = if c_max > 0.0 { delta / c_max } else { 0.0 };
        let value = c_max;

        (hue, saturation, value)
    }

    /// Converts `Color` to a `HEX` String (6 digits)
    ///
    /// e.g. white => `"ffffff"`
    pub fn to_hex(&self) -> String {
        let sum: u32 = ((self.r as u32) << 16) + ((self.g as u32) << 8) + (self.b as u32);
        format!("{:06x}", sum)
    }

    /// Converts `Color` to a 3 digit `HEX` String
    ///
    /// e.g. white => `"fff"`
    ///
    /// **Warning:** This is a *lossy* compression.
    /// It will round to the nearest value
    pub fn to_hex_3(&self) -> String {
        let r = (self.r as f64 / BASE as f64 * 15 as f64).round() as u32;
        let g = (self.g as f64 / BASE as f64 * 15 as f64).round() as u32;
        let b = (self.b as f64 / BASE as f64 * 15 as f64).round() as u32;

        let sum: u32 = (r << 8) + (g << 4) + b;
        format!("{:03x}", sum)
    }

    fn from_any_hex(hex: &str, base: u32) -> RGBColor {
        let factor = 255 / (base - 1);
        let bit_move = (base as f64).log2() as u32;
        let mut value =
            u32::from_str_radix(hex, 16).expect(format!("HEX is invalid: {}", hex).as_str());

        let b = ((value % base) * factor) as u8;
        value >>= bit_move;
        let g = ((value % base) * factor) as u8;
        value >>= bit_move;
        let r = ((value % base) * factor) as u8;

        RGBColor::from_rgb(r, g, b)
    }
}

impl PartialEq for RGBColor {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod from_hex {
        use super::*;

        #[test]
        fn h6_presets() {
            assert_eq!(presets::WHITE, RGBColor::from_hex("ffffff"));
            assert_eq!(presets::BLACK, RGBColor::from_hex("000000"));
            assert_eq!(presets::RED, RGBColor::from_hex("ff0000"));
            assert_eq!(presets::GREEN, RGBColor::from_hex("00ff00"));
            assert_eq!(presets::BLUE, RGBColor::from_hex("0000ff"));
        }

        #[test]
        fn h6_custom() {
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
        fn h3_presets() {
            assert_eq!(presets::WHITE, RGBColor::from_hex("fff"));
            assert_eq!(presets::BLACK, RGBColor::from_hex("000"));
            assert_eq!(presets::RED, RGBColor::from_hex("f00"));
            assert_eq!(presets::GREEN, RGBColor::from_hex("0f0"));
            assert_eq!(presets::BLUE, RGBColor::from_hex("00f"));
        }

        #[test]
        fn h3_custom() {
            assert_eq!(RGBColor::from_rgb(255, 51, 153), RGBColor::from_hex("f39"));
            assert_eq!(RGBColor::from_rgb(153, 255, 51), RGBColor::from_hex("9f3"));
            assert_eq!(RGBColor::from_rgb(51, 153, 255), RGBColor::from_hex("39f"));
        }

        #[test]
        fn h3_gray() {
            assert_eq!(RGBColor::from_rgb(17, 17, 17), RGBColor::from_hex("111"));
            assert_eq!(RGBColor::from_rgb(34, 34, 34), RGBColor::from_hex("222"));
            assert_eq!(RGBColor::from_rgb(51, 51, 51), RGBColor::from_hex("333"));
        }
    }

    mod from_rgb_tuple {
        use super::*;

        #[test]
        fn custom() {
            assert_eq!(
                RGBColor::from_rgb(1, 2, 3),
                RGBColor::from_rgb_tuple((1, 2, 3))
            );
            assert_eq!(
                RGBColor::from_rgb(255, 0, 127),
                RGBColor::from_rgb_tuple((255, 0, 127))
            );
        }
    }

    mod from_rgb_float {
        use super::*;

        #[test]
        fn custom() {
            assert_eq!("ffffff", RGBColor::from_rgb_float(1.0, 1.0, 1.0).to_hex());
            assert_eq!("000000", RGBColor::from_rgb_float(0.0, 0.0, 0.0).to_hex());
            assert_eq!("7f7f7f", RGBColor::from_rgb_float(0.5, 0.5, 0.5).to_hex());
            assert_eq!("333333", RGBColor::from_rgb_float(0.2, 0.2, 0.2).to_hex());
        }

        #[test]
        fn more_than_one() {
            assert_eq!("ff0000", RGBColor::from_rgb_float(2.0, 0.0, 0.0).to_hex());
        }

        #[test]
        fn less_than_zero() {
            assert_eq!("0000cc", RGBColor::from_rgb_float(-0.5, -3.0, 0.8).to_hex());
        }
    }

    mod to_rgb_tuple {
        use super::*;

        #[test]
        fn presets() {
            assert_eq!((255, 255, 255), presets::WHITE.to_rgb_tuple());
            assert_eq!((0, 0, 0), presets::BLACK.to_rgb_tuple());
        }

        #[test]
        fn custom() {
            assert_eq!((2, 20, 200), RGBColor::from_rgb(2, 20, 200).to_rgb_tuple());
            assert_eq!((42, 13, 5), RGBColor::from_rgb(42, 13, 5).to_rgb_tuple());
            assert_eq!((80, 252, 1), RGBColor::from_rgb(80, 252, 1).to_rgb_tuple());
        }
    }

    mod to_hex {
        use super::*;

        #[test]
        fn presets() {
            assert_eq!("ffffff", presets::WHITE.to_hex());
            assert_eq!("000000", presets::BLACK.to_hex());
            assert_eq!("ff0000", presets::RED.to_hex());
            assert_eq!("00ff00", presets::GREEN.to_hex());
            assert_eq!("0000ff", presets::BLUE.to_hex());
        }

        #[test]
        fn h3_custom() {
            assert_eq!("ff3399", RGBColor::from_hex("f39").to_hex());
            assert_eq!("225511", RGBColor::from_hex("251").to_hex());
            assert_eq!("aa3322", RGBColor::from_hex("a32").to_hex());
        }
    }

    mod to_hex_3 {
        use super::*;

        #[test]
        fn presets() {
            assert_eq!("fff", presets::WHITE.to_hex_3());
            assert_eq!("000", presets::BLACK.to_hex_3());
            assert_eq!("f00", presets::RED.to_hex_3());
            assert_eq!("0f0", presets::GREEN.to_hex_3());
            assert_eq!("00f", presets::BLUE.to_hex_3());
        }

        #[test]
        fn custom() {
            assert_eq!("eee", RGBColor::from_hex("f0f0f0").to_hex_3());
            assert_eq!("123", RGBColor::from_hex("102030").to_hex_3());
            assert_eq!("9ce", RGBColor::from_hex("a0c4ed").to_hex_3());
        }
    }

    mod to_hsv {
        use super::*;

        #[test]
        fn presets() {
            assert_eq!((0.0, 0.0, 1.0), presets::WHITE.to_hsv());
            assert_eq!((0.0, 0.0, 0.0), presets::BLACK.to_hsv());
            assert_eq!((0.0, 1.0, 1.0), presets::RED.to_hsv());
            assert_eq!((120.0, 1.0, 1.0), presets::GREEN.to_hsv());
            assert_eq!((240.0, 1.0, 1.0), presets::BLUE.to_hsv());
        }
    }
}
