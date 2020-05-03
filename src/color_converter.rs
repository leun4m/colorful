use crate::color_models::hsv::HSVColor;
use crate::color_models::rgb24::RGB24;
use crate::number_utils;

/// Converts the given `RGBColor` to an `HSVColor`
pub fn rgb_to_hex(rgb_color: &RGB24) -> HSVColor {
    let r = number_utils::as_float(rgb_color.red());
    let g = number_utils::as_float(rgb_color.green());
    let b = number_utils::as_float(rgb_color.blue());

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
    use crate::color_models::rgb24::presets;

    #[test]
    fn from_rgb() {
        assert_eq!((0.0, 0.0, 1.0), (rgb_to_hex(&presets::WHITE)).as_tuple());
        assert_eq!((0.0, 0.0, 0.0), (rgb_to_hex(&presets::BLACK)).as_tuple());
        assert_eq!((0.0, 1.0, 1.0), (rgb_to_hex(&presets::RED)).as_tuple());
        assert_eq!((120.0, 1.0, 1.0), (rgb_to_hex(&presets::GREEN)).as_tuple());
        assert_eq!((240.0, 1.0, 1.0), (rgb_to_hex(&presets::BLUE)).as_tuple());
    }
}
