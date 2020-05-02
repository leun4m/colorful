/// A module for the HSV color model
pub mod hsv_color;
/// A module for the RGB color model
pub mod rgb_color;

mod number_utils;

pub trait Color {
    /// Returns if color is (absolute) white
    fn is_white(&self) -> bool;
    /// Returns if color is (absolute) black
    fn is_black(&self) -> bool;
}
