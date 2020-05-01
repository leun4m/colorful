/// Representation of a color_models stored as HSV channels.
///
/// Each channel is stored as `f64`
/// h in degrees (0 - 360)
/// s, v in percent (0 - 1.0)
#[derive(Debug)]
pub struct HSVColor {
    h: f64,
    s: f64,
    v: f64,
}

impl HSVColor {
    pub fn from_hsv(h: f64, s: f64, v: f64) -> HSVColor {
        HSVColor { h, s, v }
    }

    pub fn as_tuple(&self) -> (f64, f64, f64) {
        (self.h, self.s, self.v)
    }
}
