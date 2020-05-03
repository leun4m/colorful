//! A crate for doing all sorts of stuff with colors
//!
//! # Color models
//!
//! - RGB [(Wikipedia)](https://en.wikipedia.org/wiki/RGB_color_model) - based on *red, green, blue*
//! - HSV [(Wikipedia)](https://en.wikipedia.org/wiki/HSL_and_HSV) - based on *hue, saturation, value*

/// Contains different color models
pub mod color_models;

/// Contains the calculations for conversion between color models
pub mod color_converter;

/// Contains various util methods for the work with numbers
mod number_utils;

#[cfg(test)]
mod tests {
    use crate::color_models::rgb::rgb24::RGB24;

    #[test]
    fn rgb() {
        let color = RGB24::from((0, 255, 127));
    }
}
