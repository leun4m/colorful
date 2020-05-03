use crate::color_models::rgb::rgb24::RGB24;

/// Perfect white: `#fff`
pub const WHITE: RGB24 = RGB24 {
    r: 255,
    g: 255,
    b: 255,
};

/// Deep black, nothing is darker: `#000`
pub const BLACK: RGB24 = RGB24 { r: 0, g: 0, b: 0 };
