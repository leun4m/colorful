use crate::color_models::hsv::HSV;
use crate::color_models::rgb::RGB;
use crate::number_utils;

/// Converts the given `RGBColor` to an `HSVColor`
pub fn rgb_to_hsv<T>(rgb_color: &impl RGB<T>) -> HSV {
    let (r, g, b) = rgb_color.as_tuple_f64();

    let c_max = number_utils::get_max(r, g, b);
    let c_min = number_utils::get_min(r, g, b);
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

    HSV::from_hsv(hue, saturation, value)
}

#[cfg(test)]
mod tests {
    use crate::color_converter::rgb_to_hsv;
    use crate::color_models::rgb::rgb24::{BLACK, BLUE, GREEN, RED, WHITE};

    #[test]
    fn from_rgb() {
        assert_eq!((0.0, 0.0, 1.0), (rgb_to_hsv(&WHITE)).as_tuple());
        assert_eq!((0.0, 0.0, 0.0), (rgb_to_hsv(&BLACK)).as_tuple());
        assert_eq!((0.0, 1.0, 1.0), (rgb_to_hsv(&RED)).as_tuple());
        assert_eq!((120.0, 1.0, 1.0), (rgb_to_hsv(&GREEN)).as_tuple());
        assert_eq!((240.0, 1.0, 1.0), (rgb_to_hsv(&BLUE)).as_tuple());
    }
}
