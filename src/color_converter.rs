use crate::color_models::hsv::HSVColor;
use crate::color_models::rgb::rgb24::RGB24;
use crate::color_models::rgb::RGB;
use crate::number_utils;

/// Converts the given `RGBColor` to an `HSVColor`
pub fn rgb_to_hex(rgb_color: &RGB24) -> HSVColor {
    let r = number_utils::as_float(rgb_color.r());
    let g = number_utils::as_float(rgb_color.g());
    let b = number_utils::as_float(rgb_color.b());

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

    HSVColor::from_hsv(hue, saturation, value)
}

#[cfg(test)]
mod tests {
    use crate::color_converter::rgb_to_hex;
    use crate::color_models::rgb::rgb24::{BLACK, BLUE, GREEN, RED, WHITE};

    #[test]
    fn from_rgb() {
        assert_eq!((0.0, 0.0, 1.0), (rgb_to_hex(&WHITE)).as_tuple());
        assert_eq!((0.0, 0.0, 0.0), (rgb_to_hex(&BLACK)).as_tuple());
        assert_eq!((0.0, 1.0, 1.0), (rgb_to_hex(&RED)).as_tuple());
        assert_eq!((120.0, 1.0, 1.0), (rgb_to_hex(&GREEN)).as_tuple());
        assert_eq!((240.0, 1.0, 1.0), (rgb_to_hex(&BLUE)).as_tuple());
    }
}
