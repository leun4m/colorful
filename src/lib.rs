//! A crate for doing all sorts of stuff with colors
//!
//! # Color models
//!
//! - RGB [(Wikipedia)](https://en.wikipedia.org/wiki/RGB_color_model) - based on *red, green, blue*
//! - HSV [(Wikipedia)](https://en.wikipedia.org/wiki/HSL_and_HSV) - based on *hue, saturation, value*
//!
//! # Please note
//!
//! This library is still under heavy construction
//!

extern crate strum;
extern crate strum_macros;

/// Contains different color models
pub mod models;

/// Contains the calculations for conversion between color models
pub mod converter;

/// Contains a set of common predefined colors
pub mod presets;

/// Contains various util methods for the work with numbers
mod number_utils;

#[cfg(test)]
mod tests {
    use crate::models::rgb::rgb24::RGB24;
    use crate::models::rgb::RGB;

    #[test]
    fn rgb() {
        let color_a = RGB24::from((0, 255, 127));
        let color_b = color_a.to_hsv().to_rgb();
        assert_eq!(color_a, color_b);
    }
}
