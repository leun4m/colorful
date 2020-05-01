use crate::color_models::rgb_color::RGBColor;

/// Perfect white: `#fff`
pub const WHITE: RGBColor = RGBColor {
    r: 255,
    g: 255,
    b: 255,
};

/// Deep black, nothing is darker: `#000`
pub const BLACK: RGBColor = RGBColor { r: 0, g: 0, b: 0 };

/// The default red: `#f00`
pub const RED: RGBColor = RGBColor { r: 255, g: 0, b: 0 };

/// The default green: `#0f0`
pub const GREEN: RGBColor = RGBColor { r: 0, g: 255, b: 0 };

/// The default blue: `#00f`
pub const BLUE: RGBColor = RGBColor { r: 0, g: 0, b: 255 };
