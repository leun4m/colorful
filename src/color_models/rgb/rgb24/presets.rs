use crate::color_models::rgb::rgb24::RGB24;

/// Perfect white: `#fff`
pub const WHITE: RGB24 = RGB24 {
    r: 255,
    g: 255,
    b: 255,
};

/// Deep black, nothing is darker: `#000`
pub const BLACK: RGB24 = RGB24 { r: 0, g: 0, b: 0 };

/// The default red: `#f00`
pub const RED: RGB24 = RGB24 { r: 255, g: 0, b: 0 };

/// The default green: `#0f0`
pub const GREEN: RGB24 = RGB24 { r: 0, g: 255, b: 0 };

/// The default blue: `#00f`
pub const BLUE: RGB24 = RGB24 { r: 0, g: 0, b: 255 };
