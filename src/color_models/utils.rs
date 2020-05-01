use crate::color_models::rgb_color;

/// Calculates the maximum value of the given triple
pub fn get_max(a: f64, b: f64, c: f64) -> f64 {
    a.max(b.max(c))
}

/// Calculates the minimum value of the given triple
pub fn get_min(a: f64, b: f64, c: f64) -> f64 {
    a.min(b.min(c))
}

/// Converts *byte* to *float* representation
pub fn as_float(a: u8) -> f64 {
    a as f64 / rgb_color::BASE as f64
}

pub fn save_convert_float_to_byte(float: f64) -> u8 {
    if float >= 1.0 {
        255
    } else if float < 0.0 {
        0
    } else {
        (float * 255.0) as u8
    }
}

pub fn as_byte_tuple(floats: (f64, f64, f64)) -> (u8, u8, u8) {
    (
        save_convert_float_to_byte(floats.0),
        save_convert_float_to_byte(floats.1),
        save_convert_float_to_byte(floats.2),
    )
}
