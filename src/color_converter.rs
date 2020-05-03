use crate::color_models::hsv::HSV;
use crate::color_models::rgb::rgb24::RGB24;
use crate::color_models::rgb::rgb48::RGB48;
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

/// Converts the given `RGB24` -> `RGB48`
pub fn rgb24_to_rgb48(rgb: &RGB24) -> RGB48 {
    const FACTOR: u16 = RGB48::MAX / RGB24::MAX as u16;
    RGB48::from_rgb(
        (rgb.r() as u16 * FACTOR),
        (rgb.g() as u16 * FACTOR),
        (rgb.b() as u16 * FACTOR),
    )
}

/// Converts the given `RGB48` -> `RGB24`
pub fn rgb48_to_rgb24(rgb: &RGB24) -> RGB24 {
    const DIVIDER: u16 = RGB48::MAX / RGB24::MAX as u16;
    RGB24::from_rgb(
        (rgb.r() as u16 / DIVIDER) as u8,
        (rgb.g() as u16 / DIVIDER) as u8,
        (rgb.b() as u16 / DIVIDER) as u8,
    )
}

#[cfg(test)]
mod tests {
    use crate::color_converter::{rgb24_to_rgb48, rgb_to_hsv};
    use crate::color_models::rgb::rgb24;
    use crate::color_models::rgb::rgb24::{BLACK, BLUE, GREEN, RED, WHITE};
    use crate::color_models::rgb::rgb48::{self, RGB48};

    #[test]
    fn from_rgb() {
        assert_eq!((0.0, 0.0, 1.0), (rgb_to_hsv(&WHITE)).as_tuple());
        assert_eq!((0.0, 0.0, 0.0), (rgb_to_hsv(&BLACK)).as_tuple());
        assert_eq!((0.0, 1.0, 1.0), (rgb_to_hsv(&RED)).as_tuple());
        assert_eq!((120.0, 1.0, 1.0), (rgb_to_hsv(&GREEN)).as_tuple());
        assert_eq!((240.0, 1.0, 1.0), (rgb_to_hsv(&BLUE)).as_tuple());
    }

    #[test]
    fn rgb24_to_rgb48_() {
        assert_eq!(rgb48::WHITE, rgb24_to_rgb48(&rgb24::WHITE));
        assert_eq!(rgb48::BLACK, rgb24_to_rgb48(&rgb24::BLACK));
        assert_eq!(rgb48::RED, rgb24_to_rgb48(&rgb24::RED));
        assert_eq!(rgb48::GREEN, rgb24_to_rgb48(&rgb24::GREEN));
        assert_eq!(rgb48::BLUE, rgb24_to_rgb48(&rgb24::BLUE));
    }
}
