use crate::color_models::rgb_color::RGBColor;
use crate::color_models::{utils, Color};

/// Representation of a color_models stored as HSV channels.
///
/// Each channel is stored as `f64`
/// `h` in degrees (0 - 360)
/// `s, v` as fraction (0 - 1.0)
#[derive(Debug)]
pub struct HSVColor {
    h: f64,
    s: f64,
    v: f64,
}

/// White as `HSVColor`
pub const WHITE: HSVColor = HSVColor {
    h: 0.0,
    s: 0.0,
    v: 1.0,
};

/// Black as `HSVColor`
pub const BLACK: HSVColor = HSVColor {
    h: 0.0,
    s: 0.0,
    v: 0.0,
};

impl HSVColor {
    pub fn from_hsv(h: f64, s: f64, v: f64) -> HSVColor {
        HSVColor { h, s, v }
    }

    pub fn as_tuple(&self) -> (f64, f64, f64) {
        (self.h, self.s, self.v)
    }
}

impl From<RGBColor> for HSVColor {
    /// Converts `RGBColor` to a `HSVColor`
    fn from(rgb_color: RGBColor) -> Self {
        let r = utils::as_float(rgb_color.red());
        let g = utils::as_float(rgb_color.green());
        let b = utils::as_float(rgb_color.blue());

        let c_max = utils::get_max(r, g, b);
        let c_min = utils::get_min(r, g, b);
        let delta = c_max - c_min;

        let hue = if delta == 0.0 {
            0.0
        } else if r >= b && r >= g {
            60.0 * (((g - b) / delta) % 6.0)
        } else if g >= r && g >= b {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        let saturation = if c_max > 0.0 { delta / c_max } else { 0.0 };
        let value = c_max;

        HSVColor::from_hsv(hue, saturation, value)
    }
}

impl Color for HSVColor {
    fn is_white(&self) -> bool {
        self == &WHITE
    }

    fn is_black(&self) -> bool {
        self == &BLACK
    }
}

impl PartialEq for HSVColor {
    fn eq(&self, other: &Self) -> bool {
        const EPSILON: f64 = 0.000_001;
        // Compare floating points
        utils::approx_equal_f64(self.h, other.h, EPSILON)
            && utils::approx_equal_f64(self.s, other.s, EPSILON)
            && utils::approx_equal_f64(self.v, other.v, EPSILON)
    }
}

#[cfg(test)]
mod tests {
    use crate::color_models::hsv_color::HSVColor;
    use crate::color_models::rgb_color::presets;

    #[test]
    fn presets() {
        assert_eq!((0.0, 0.0, 1.0), (HSVColor::from(presets::WHITE)).as_tuple());
        assert_eq!((0.0, 0.0, 0.0), (HSVColor::from(presets::BLACK)).as_tuple());
        assert_eq!((0.0, 1.0, 1.0), (HSVColor::from(presets::RED)).as_tuple());
        assert_eq!(
            (120.0, 1.0, 1.0),
            (HSVColor::from(presets::GREEN)).as_tuple()
        );
        assert_eq!(
            (240.0, 1.0, 1.0),
            (HSVColor::from(presets::BLUE)).as_tuple()
        );
    }
}
