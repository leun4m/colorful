/// The HSV color model
pub mod hsv;
/// The RGB (24-bit) color model
pub mod rgb24;

/// Collection of basic methods every color (regardless of model) should have
pub trait Color {
    /// Returns if color is (absolute) white
    fn is_white(&self) -> bool;
    /// Returns if color is (absolute) black
    fn is_black(&self) -> bool;
}
