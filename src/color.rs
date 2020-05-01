pub mod presets;

/// Representation of a color stored as RGB channels.
///
/// Each channel is stored as `u8` (0-255)
#[derive(Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    /// Creates `Color` from the given integer values.
    ///
    /// `r`: red, `g`: green, `b`: blue
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    /// Creates `Color` from the given decimal values.
    ///
    /// Expects values from 0.0 to 1.0 (both inclusive)
    /// - If a value > 1 it will be treated as 1
    /// - If a value < 0 it will be treated as 0
    pub fn from_rgb_float(r: f64, g: f64, b: f64) -> Color {
        Color {
            r: Color::save_convert_float_to_byte(r),
            g: Color::save_convert_float_to_byte(g),
            b: Color::save_convert_float_to_byte(b),
        }
    }

    fn save_convert_float_to_byte(float: f64) -> u8 {
        if float >= 1.0 {
            255
        } else if float < 0.0 {
            0
        } else {
            (float * 255.0) as u8
        }
    }

    /// Creates `Color` from the given tuple.
    ///
    /// (r, g, b)
    pub fn from_rgb_tuple(tuple: (u8, u8, u8)) -> Color {
        Color::from_rgb(tuple.0, tuple.1, tuple.2)
    }

    /// Creates `Color` from the given hex string.
    ///
    /// Accepts strings only with the following formats and length:
    /// - `f0f0f0` (`rrggbb`)
    /// - `fff` (`rgb`)
    pub fn from_hex(hex: &str) -> Color {
        let length = hex.chars().count();
        if length == 6 {
            Color::from_any_hex(hex, 16 * 16)
        } else if length == 3 {
            Color::from_any_hex(hex, 16)
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
    /// It will basically omit any second value:
    /// `"a0c4ed"` => `"ace"`
    pub fn to_hex_3(&self) -> String {
        let sum: u32 = ((self.r as u32) << 4) + ((self.g as u32) << 2) + (self.b as u32);
        format!("{:03x}", sum)
    }

    fn from_any_hex(hex: &str, base: u32) -> Color {
        let factor = 255 / (base - 1);
        let bit_move = (base as f64).log2() as u32;
        let mut value =
            u32::from_str_radix(hex, 16).expect(format!("HEX is invalid: {}", hex).as_str());

        let b = ((value % base) * factor) as u8;
        value >>= bit_move;
        let g = ((value % base) * factor) as u8;
        value >>= bit_move;
        let r = ((value % base) * factor) as u8;

        Color::from_rgb(r, g, b)
    }
}

impl PartialEq for Color {
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
            assert_eq!(presets::WHITE, Color::from_hex("ffffff"));
            assert_eq!(presets::BLACK, Color::from_hex("000000"));
            assert_eq!(presets::RED, Color::from_hex("ff0000"));
            assert_eq!(presets::GREEN, Color::from_hex("00ff00"));
            assert_eq!(presets::BLUE, Color::from_hex("0000ff"));
        }

        #[test]
        fn h6_custom() {
            assert_eq!(Color::from_rgb(166, 65, 21), Color::from_hex("A64115"));
            assert_eq!(Color::from_rgb(21, 166, 65), Color::from_hex("15A641"));
            assert_eq!(Color::from_rgb(65, 21, 166), Color::from_hex("4115A6"));
        }

        #[test]
        fn h3_presets() {
            assert_eq!(presets::WHITE, Color::from_hex("fff"));
            assert_eq!(presets::BLACK, Color::from_hex("000"));
            assert_eq!(presets::RED, Color::from_hex("f00"));
            assert_eq!(presets::GREEN, Color::from_hex("0f0"));
            assert_eq!(presets::BLUE, Color::from_hex("00f"));
        }

        #[test]
        fn h3_custom() {
            assert_eq!(Color::from_rgb(255, 51, 153), Color::from_hex("f39"));
            assert_eq!(Color::from_rgb(153, 255, 51), Color::from_hex("9f3"));
            assert_eq!(Color::from_rgb(51, 153, 255), Color::from_hex("39f"));
        }

        #[test]
        fn h3_gray() {
            assert_eq!(Color::from_rgb(17, 17, 17), Color::from_hex("111"));
            assert_eq!(Color::from_rgb(34, 34, 34), Color::from_hex("222"));
            assert_eq!(Color::from_rgb(51, 51, 51), Color::from_hex("333"));
        }
    }

    mod from_rgb_tuple {
        use super::*;

        #[test]
        fn custom() {
            assert_eq!(Color::from_rgb(1, 2, 3), Color::from_rgb_tuple((1, 2, 3)));
            assert_eq!(
                Color::from_rgb(255, 0, 127),
                Color::from_rgb_tuple((255, 0, 127))
            );
        }
    }

    mod from_rgb_float {
        use super::*;

        #[test]
        fn custom() {
            assert_eq!("ffffff", Color::from_rgb_float(1.0, 1.0, 1.0).to_hex());
            assert_eq!("000000", Color::from_rgb_float(0.0, 0.0, 0.0).to_hex());
            assert_eq!("7f7f7f", Color::from_rgb_float(0.5, 0.5, 0.5).to_hex());
            assert_eq!("333333", Color::from_rgb_float(0.2, 0.2, 0.2).to_hex());
        }

        #[test]
        fn more_than_one() {
            assert_eq!("ff0000", Color::from_rgb_float(2.0, 0.0, 0.0).to_hex());
        }

        #[test]
        fn less_than_zero() {
            assert_eq!("0000cc", Color::from_rgb_float(-0.5, -3.0, 0.8).to_hex());
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
            assert_eq!((2, 20, 200), Color::from_rgb(2, 20, 200).to_rgb_tuple());
            assert_eq!((42, 13, 5), Color::from_rgb(42, 13, 5).to_rgb_tuple());
            assert_eq!((80, 252, 1), Color::from_rgb(80, 252, 1).to_rgb_tuple());
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
            assert_eq!("ff3399", Color::from_hex("f39").to_hex());
            assert_eq!("225511", Color::from_hex("251").to_hex());
            assert_eq!("aa3322", Color::from_hex("a32").to_hex());
        }
    }

    mod to_hex_3 {
        use super::*;

        fn presets() {
            assert_eq!("fff", presets::WHITE.to_hex_3());
            assert_eq!("000", presets::BLACK.to_hex_3());
            assert_eq!("f00", presets::RED.to_hex_3());
            assert_eq!("0f0", presets::GREEN.to_hex_3());
            assert_eq!("00f", presets::BLUE.to_hex_3());
        }

        fn custom() {
            assert_eq!("fff", Color::from_hex("f0f0f0").to_hex_3());
            assert_eq!("123", Color::from_hex("102030").to_hex_3());
            assert_eq!("ace", Color::from_hex("a0c4ed").to_hex_3());
        }
    }
}
