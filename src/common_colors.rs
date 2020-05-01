use crate::color::Color;

enum CommonColors {
    WHITE,
    BLACK
}

impl CommonColors {
    pub fn as_color(&self) -> Color {
        match &self {
            CommonColors::WHITE => Color::from_rgb(255, 255, 255),
            CommonColors::BLACK => Color::from_rgb(0,0,0)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common_colors::CommonColors;
    use crate::color::Color;

    #[test]
    fn white_is_white() {
        assert_eq!(Color::from_rgb(255,255,255), CommonColors::WHITE.as_color(),)
    }
}