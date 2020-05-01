use crate::color::rgb_color::RGBColor;

pub const WHITE: RGBColor = RGBColor {
    r: 255,
    g: 255,
    b: 255,
};

pub const BLACK: RGBColor = RGBColor { r: 0, g: 0, b: 0 };

pub const RED: RGBColor = RGBColor { r: 255, g: 0, b: 0 };

pub const GREEN: RGBColor = RGBColor { r: 0, g: 255, b: 0 };

pub const BLUE: RGBColor = RGBColor { r: 0, g: 0, b: 255 };
