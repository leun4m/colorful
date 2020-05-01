mod presets;

/// Representation of a color stored as RGB channels.
/// Each channel is stored as u8 (0-255)
#[derive(Debug)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color {
            red: r,
            green: g,
            blue: b,
        }
    }

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
    use crate::color::{presets, Color};

    #[test]
    fn hex_6_presets() {
        assert_eq!(presets::WHITE, Color::from_hex("ffffff"));
        assert_eq!(presets::BLACK, Color::from_hex("000000"));
        assert_eq!(presets::RED, Color::from_hex("ff0000"));
        assert_eq!(presets::GREEN, Color::from_hex("00ff00"));
        assert_eq!(presets::BLUE, Color::from_hex("0000ff"));
    }

    #[test]
    fn hex_3_presets() {
        assert_eq!(presets::WHITE, Color::from_hex("fff"));
        assert_eq!(presets::BLACK, Color::from_hex("000"));
        assert_eq!(presets::RED, Color::from_hex("f00"));
        assert_eq!(presets::GREEN, Color::from_hex("0f0"));
        assert_eq!(presets::BLUE, Color::from_hex("00f"));
    }
}
