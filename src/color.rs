/// Representation of a color stored as RGB channels.
/// Each channel is stored as u8 (0-255)
#[derive(Debug)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl Color {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color {
            red: r,
            green: g,
            blue: b
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}