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
            let r = &hex[0..2];
            let g = &hex[2..4];
            let b = &hex[4..6];
            Color::from_rgb(
                Color::convert_u8_hex(r),
                Color::convert_u8_hex(g),
                Color::convert_u8_hex(b),
            )
        } else {
            panic!("HEX number has invalid length: {}", length);
        }
    }

    fn convert_u8_hex(hex: &str) -> u8 {
        u8::from_str_radix(hex, 16).expect(format!("HEX no valid u8: {}", hex).as_str())
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    use crate::common_colors::CommonColors;

    #[test]
    fn hex_white() {
        let color = Color::from_hex("ffffff");
        assert_eq!(CommonColors::WHITE.as_color(), color);
    }
}
