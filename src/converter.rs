use crate::models::hsv::HSV;
use crate::models::rgb::rgb24::RGB24;
use crate::models::rgb::rgb48::RGB48;
use crate::models::rgb::RGB;
use crate::number_utils;

/// Converts the given `RGB` -> `HSV`
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

    // eprintln!("r: {} g: {} b: {}", r, g, b);
    // eprintln!(
    //     "/// cMin:{} cMax:{} delta: {} hue:{} ///",
    //     c_min, c_max, delta, hue
    // );

    let saturation = if c_max > 0.0 { delta / c_max } else { 0.0 };
    let value = c_max;

    HSV::from_hsv(hue, saturation, value)
}

/// Converts the given `HSV` -> `RGB`
///
/// This uses the formula provided by [Wikipedia](https://en.wikipedia.org/wiki/HSL_and_HSV#HSV_to_RGB)
/// (2020-05-03)
pub fn hsv_to_rgb<T, U>(hsv: &HSV) -> T
where
    T: RGB<U>,
{
    let chroma = hsv.v() * hsv.s();
    let h = hsv.h() / 60.0;
    let x = chroma * (1.0 - (h % 2.0 - 1.0).abs());

    let rgb = calc_hsv(h, chroma, x);
    let m = hsv.v() - chroma;

    T::from_rgb_f64(rgb.0 + m, rgb.1 + m, rgb.2 + m)
}

fn calc_hsv(h: f64, chroma: f64, x: f64) -> (f64, f64, f64) {
    if h.is_nan() {
        (0.0, 0.0, 0.0)
    } else if 0.0 <= h && h <= 1.0 {
        (chroma, x, 0.0)
    } else if 1.0 < h && h <= 2.0 {
        (x, chroma, 0.0)
    } else if 2.0 < h && h <= 3.0 {
        (0.0, chroma, x)
    } else if 3.0 < h && h <= 4.0 {
        (0.0, x, chroma)
    } else if 4.0 < h && h <= 5.0 {
        (x, 0.0, chroma)
    } else if 5.0 < h && h <= 6.0 {
        (chroma, 0.0, x)
    } else {
        (0.0, 0.0, 0.0)
    }
}
/// Converts the given `RGB24` -> `RGB48`
pub fn rgb24_to_rgb48(rgb: &RGB24) -> RGB48 {
    const FACTOR: u16 = RGB48::MAX / RGB24::MAX as u16;
    RGB48::from_rgb(
        rgb.r() as u16 * FACTOR,
        rgb.g() as u16 * FACTOR,
        rgb.b() as u16 * FACTOR,
    )
}

/// Converts the given `RGB48` -> `RGB24`
pub fn rgb48_to_rgb24(rgb: &RGB48) -> RGB24 {
    const DIVIDER: u16 = RGB48::MAX / RGB24::MAX as u16;
    RGB24::from_rgb(
        (rgb.r() as u16 / DIVIDER) as u8,
        (rgb.g() as u16 / DIVIDER) as u8,
        (rgb.b() as u16 / DIVIDER) as u8,
    )
}

#[cfg(test)]
mod tests {
    use crate::converter::{hsv_to_rgb, rgb24_to_rgb48, rgb48_to_rgb24, rgb_to_hsv};
    use crate::models::hsv;
    use crate::models::hsv::HSV;
    use crate::models::rgb::rgb24;
    use crate::models::rgb::rgb24::RGB24;
    use crate::models::rgb::rgb48;
    use crate::models::rgb::rgb48::RGB48;
    use crate::presets::X11Color;
    use strum::IntoEnumIterator;

    fn approx_equal_hsv(a: &HSV, b: &HSV) -> bool {
        const EPSILON: f64 = 0.1;

        if (a.h() - b.h()).abs() / hsv::H_MAX < EPSILON
            && (a.s() - b.s()).abs() / hsv::S_MAX < EPSILON
            && (a.v() - b.v()).abs() / hsv::V_MAX < EPSILON
        {
            true
        } else {
            println!("{:?} !~ {:?}", a, b);
            false
        }
    }

    #[test]
    fn rgb_to_hsv_rgb24() {
        assert_eq!(hsv::WHITE, rgb_to_hsv(&rgb24::WHITE));
        assert_eq!(hsv::BLACK, rgb_to_hsv(&rgb24::BLACK));
        assert_eq!(hsv::RED, rgb_to_hsv(&rgb24::RED));
        assert_eq!(hsv::GREEN, rgb_to_hsv(&rgb24::GREEN));
        assert_eq!(hsv::BLUE, rgb_to_hsv(&rgb24::BLUE));
    }

    #[test]
    fn rgb_to_hsv_x11() {
        let mut a = 0;
        for color in X11Color::iter() {
            // assert!(
            //     approx_equal_hsv(&color.to_hsv(), &rgb_to_hsv(&color.to_rgb::<RGB48, u16>())),
            //     "{:?}",
            //     color
            // );
            if !approx_equal_hsv(&color.to_hsv(), &rgb_to_hsv(&color.to_rgb::<RGB48, u16>())) {
                a += 1;
            }
        }
        assert_eq!(a, 0)
    }

    #[test]
    fn rgb_to_hsv_rgb48() {
        assert_eq!(hsv::WHITE, rgb_to_hsv(&rgb48::WHITE));
        assert_eq!(hsv::BLACK, rgb_to_hsv(&rgb48::BLACK));
        assert_eq!(hsv::RED, rgb_to_hsv(&rgb48::RED));
        assert_eq!(hsv::GREEN, rgb_to_hsv(&rgb48::GREEN));
        assert_eq!(hsv::BLUE, rgb_to_hsv(&rgb48::BLUE));
    }

    #[test]
    fn hsv_to_rgb_rgb24() {
        assert_eq!(rgb24::WHITE, hsv_to_rgb(&hsv::WHITE));
        assert_eq!(rgb24::BLACK, hsv_to_rgb(&hsv::BLACK));
        assert_eq!(rgb24::RED, hsv_to_rgb(&hsv::RED));
        assert_eq!(rgb24::GREEN, hsv_to_rgb(&hsv::GREEN));
        assert_eq!(rgb24::BLUE, hsv_to_rgb(&hsv::BLUE));

        assert_eq!(
            RGB24::from((255, 0, 128)),
            hsv_to_rgb(&HSV::from((330.0, 1.0, 1.0)))
        )
    }

    #[test]
    fn hsv_to_rgb_rgb48() {
        assert_eq!(rgb48::WHITE, hsv_to_rgb(&hsv::WHITE));
        assert_eq!(rgb48::BLACK, hsv_to_rgb(&hsv::BLACK));
        assert_eq!(rgb48::RED, hsv_to_rgb(&hsv::RED));
        assert_eq!(rgb48::GREEN, hsv_to_rgb(&hsv::GREEN));
        assert_eq!(rgb48::BLUE, hsv_to_rgb(&hsv::BLUE));
    }

    #[test]
    fn rgb24_to_rgb48_() {
        assert_eq!(rgb48::WHITE, rgb24_to_rgb48(&rgb24::WHITE));
        assert_eq!(rgb48::BLACK, rgb24_to_rgb48(&rgb24::BLACK));
        assert_eq!(rgb48::RED, rgb24_to_rgb48(&rgb24::RED));
        assert_eq!(rgb48::GREEN, rgb24_to_rgb48(&rgb24::GREEN));
        assert_eq!(rgb48::BLUE, rgb24_to_rgb48(&rgb24::BLUE));
    }

    #[test]
    fn rgb48_to_rgb24_() {
        assert_eq!(rgb24::WHITE, rgb48_to_rgb24(&rgb48::WHITE));
        assert_eq!(rgb24::BLACK, rgb48_to_rgb24(&rgb48::BLACK));
        assert_eq!(rgb24::RED, rgb48_to_rgb24(&rgb48::RED));
        assert_eq!(rgb24::GREEN, rgb48_to_rgb24(&rgb48::GREEN));
        assert_eq!(rgb24::BLUE, rgb48_to_rgb24(&rgb48::BLUE));
    }
}
