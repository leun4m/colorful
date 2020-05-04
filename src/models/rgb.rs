use crate::models::hsv::HSV;

/// The RGB color model (24-bit)
pub mod rgb24;
/// The RGB color model (48-bit)
pub mod rgb48;

/// Representation of an RGB Color
///
/// Suitable for different color depths
///
/// `T` is meant to be the base type for each channel
///
pub trait RGB<T> {
    /// The minimal value for a channel (0%)
    const MIN: T;

    /// The maximal value for a channel (100%)
    const MAX: T;

    /// Creates a new RGB color, setting all values to zero.
    ///
    /// This is *black*.
    fn new() -> Self;

    /// Creates a new RGB color
    ///
    /// # Arguments
    /// - `r`: red
    /// - `g`: green    
    /// - `b`: blue
    fn from_rgb(r: T, g: T, b: T) -> Self;

    /// Creates a new RGB color from the given floating point values.
    ///
    /// # Arguments
    /// - `r`: red
    /// - `g`: green
    /// - `b`: blue
    ///
    /// # Please note
    /// Expects values from 0.0 to 1.0 (both inclusive)
    /// - Any values > 1 will be treated as 1
    /// - Any values < 0 it will be treated as 0
    fn from_rgb_f64(r: f64, g: f64, b: f64) -> Self;

    /// Returns the value of channel **R** (red)
    fn r(&self) -> T;

    /// Returns the value of channel **G** (green)
    fn g(&self) -> T;

    /// Returns the value of channel **B** (blue)
    fn b(&self) -> T;

    /// Sets the value of channel **R** (red)
    fn set_r(&mut self, r: T);

    /// Sets the value of channel **G** (green)
    fn set_g(&mut self, g: T);

    /// Sets the value of channel **B** (blue)
    fn set_b(&mut self, b: T);

    /// Converts this to an RGB tuple
    fn as_tuple(&self) -> (T, T, T) {
        (self.r(), self.g(), self.b())
    }

    /// Converts this to an RGB Tuple using fractions
    fn as_tuple_f64(&self) -> (f64, f64, f64);

    /// Converts this to `HSV`
    fn to_hsv(&self) -> HSV;
}
