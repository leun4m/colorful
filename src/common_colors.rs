use crate::color::Color;

#[derive(Debug)]
pub enum CommonColors {
    WHITE,
    BLACK,
}

impl CommonColors {
    pub fn as_color(&self) -> Color {
        match &self {
            CommonColors::WHITE => Color::from_rgb(255, 255, 255),
            CommonColors::BLACK => Color::from_rgb(0, 0, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::common_colors::CommonColors;

    #[test]
    fn white_is_white() {
        assert_eq!(
            Color::from_rgb(255, 255, 255),
            CommonColors::WHITE.as_color(),
        )
    }
}
