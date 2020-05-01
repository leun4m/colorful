//! A crate for doing all sorts of stuff with colors
//!
//! # Color models
//!
//! - [RGB](https://en.wikipedia.org/wiki/RGB_color_model) - based on *red, green, blue*
//! - [HSV](https://en.wikipedia.org/wiki/HSL_and_HSV) - based on *hue, saturation, value*

/// A module containing different color_models models
pub mod color_models;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
