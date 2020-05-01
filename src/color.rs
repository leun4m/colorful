mod presets;

/// Representation of a color stored as RGB channels.
///
/// Each channel is stored as `u8` (0-255)
#[derive(Debug)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    /// Creates `Color` from the given values.
    ///
    /// `r`: red, `g`: green, `b`: blue
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color {
            red: r,
            green: g,
            blue: b,
        }
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
        self.red == other.red && self.green == other.green && self.blue == other.blue
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
}
