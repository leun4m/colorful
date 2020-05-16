use crate::models::hsv::{HSVColor, HSV};
use crate::models::rgb::rgb24::RGB24;
use crate::models::rgb::rgb48::RGB48;
use crate::models::rgb::RGBColor;
use crate::number_utils;

/// [HSV]: crate::models::hsv::HSV
/// [RGBColor]: crate::models::rgb::RGBColor
/// [RGB24]: crate::models::rgb::rgb24::RGB24
/// [RGB48]: crate::models::rgb::rgb24::RGB24

/// Converts the given [`RGBColor`] -> [`HSV`]
pub fn rgb_to_hsv<T>(rgb_color: &impl RGBColor<T>) -> HSV {
    let (r, g, b) = rgb_color.as_tuple_f64();

    let c_max = number_utils::get_max(r, g, b);
    let c_min = number_utils::get_min(r, g, b);
    let delta = c_max - c_min;

    let mut hue = if delta == 0.0 {
        0.0
    } else if c_max == r {
        60.0 * ((g - b) / delta)
    } else if c_max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    if hue < 0.0 {
        hue += 360.0
    }

    let saturation = if c_max == 0.0 { 0.0 } else { delta / c_max };
    let value = c_max;

    HSV::from_hsv(hue, saturation, value)
}

/// Converts the given [`HSV`] -> [`RGBColor`]
pub fn hsv_to_rgb<T, U>(hsv: &HSV) -> T
where
    T: RGBColor<U>,
{
    let h = (hsv.h() / 60.0) as u8;
    let f = hsv.h() / 60.0 - h as f64;
    let p = hsv.v() * (1.0 - hsv.s());
    let q = hsv.v() * (1.0 - hsv.s() * f);
    let t = hsv.v() * (1.0 - hsv.s() * (1.0 - f));

    let a = match h {
        0 | 6 => (hsv.v(), t, p),
        1 => (q, hsv.v(), p),
        2 => (p, hsv.v(), t),
        3 => (p, q, hsv.v()),
        4 => (t, p, hsv.v()),
        5 => (hsv.v(), p, q),
        _ => panic!("Impossible h: {}", h),
    };

    T::from_rgb_f64(a.0, a.1, a.2)
}

/// Converts the given [`RGB24`] -> [`RGB48`]
pub fn rgb24_to_rgb48(rgb: &RGB24) -> RGB48 {
    const FACTOR: u16 = RGB48::MAX / RGB24::MAX as u16;
    RGB48::from_rgb(
        rgb.r() as u16 * FACTOR,
        rgb.g() as u16 * FACTOR,
        rgb.b() as u16 * FACTOR,
    )
}

/// Converts the given [`RGB48`] -> [`RGB24`]
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
    use crate::models::hsv::{HSVColor, HSV};
    use crate::models::rgb::rgb24::RGB24;
    use crate::models::rgb::rgb48::RGB48;
    use crate::models::rgb::RGBColor;
    use crate::presets::X11Color;
    use std::fmt::Debug;
    use strum::IntoEnumIterator;

    fn assert_approx_equal_hsv(a: &HSV, b: &HSV) {
        const EPSILON: f64 = 0.02;

        if (a.h() - b.h()).abs() / HSV::H_MAX >= EPSILON
            || (a.s() - b.s()).abs() / HSV::S_MAX >= EPSILON
            || (a.v() - b.v()).abs() / HSV::V_MAX >= EPSILON
        {
            panic!("{:?} !~ {:?}", a, b);
        }
    }

    fn assert_approx_equal_rgb<T>(a: &T, b: &T) -> ()
    where
        T: RGBColor<u8> + Debug,
    {
        const EPSILON: i32 = 4;
        if (a.r() as i32 - b.r() as i32).abs() >= EPSILON
            || (a.g() as i32 - b.g() as i32).abs() >= EPSILON
            || (a.b() as i32 - b.b() as i32).abs() >= EPSILON
        {
            panic!("{:?} !~ {:?}", a, b);
        }
    }

    #[test]
    fn rgb_to_hsv_rgb24() {
        assert_eq!(HSV::WHITE, rgb_to_hsv(&RGB24::WHITE));
        assert_eq!(HSV::BLACK, rgb_to_hsv(&RGB24::BLACK));
        assert_eq!(HSV::RED, rgb_to_hsv(&RGB24::RED));
        assert_eq!(HSV::GREEN, rgb_to_hsv(&RGB24::GREEN));
        assert_eq!(HSV::BLUE, rgb_to_hsv(&RGB24::BLUE));
    }

    #[test]
    fn rgb_to_hsv_x11() {
        for color in X11Color::iter() {
            assert_approx_equal_hsv(&color.to_hsv(), &rgb_to_hsv(&color.to_rgb::<RGB48, u16>()));
        }
    }

    #[test]
    fn hsv_to_rgb_x11() {
        for color in X11Color::iter() {
            assert_approx_equal_rgb(&color.to_rgb(), &hsv_to_rgb::<RGB24, u8>(&color.to_hsv()));
        }
    }

    #[test]
    fn rgb_to_hsv_rgb48() {
        assert_eq!(HSV::WHITE, rgb_to_hsv(&RGB48::WHITE));
        assert_eq!(HSV::BLACK, rgb_to_hsv(&RGB48::BLACK));
        assert_eq!(HSV::RED, rgb_to_hsv(&RGB48::RED));
        assert_eq!(HSV::GREEN, rgb_to_hsv(&RGB48::GREEN));
        assert_eq!(HSV::BLUE, rgb_to_hsv(&RGB48::BLUE));
    }

    #[test]
    fn hsv_to_rgb_rgb24() {
        assert_eq!(RGB24::WHITE, hsv_to_rgb(&HSV::WHITE));
        assert_eq!(RGB24::BLACK, hsv_to_rgb(&HSV::BLACK));
        assert_eq!(RGB24::RED, hsv_to_rgb(&HSV::RED));
        assert_eq!(RGB24::GREEN, hsv_to_rgb(&HSV::GREEN));
        assert_eq!(RGB24::BLUE, hsv_to_rgb(&HSV::BLUE));

        assert_eq!(
            RGB24::from((255, 0, 128)),
            hsv_to_rgb(&HSV::from((330.0, 1.0, 1.0)))
        )
    }

    #[test]
    fn hsv_to_rgb_rgb48() {
        assert_eq!(RGB48::WHITE, hsv_to_rgb(&HSV::WHITE));
        assert_eq!(RGB48::BLACK, hsv_to_rgb(&HSV::BLACK));
        assert_eq!(RGB48::RED, hsv_to_rgb(&HSV::RED));
        assert_eq!(RGB48::GREEN, hsv_to_rgb(&HSV::GREEN));
        assert_eq!(RGB48::BLUE, hsv_to_rgb(&HSV::BLUE));
    }

    #[test]
    fn rgb24_to_rgb48_() {
        assert_eq!(RGB48::WHITE, rgb24_to_rgb48(&RGB24::WHITE));
        assert_eq!(RGB48::BLACK, rgb24_to_rgb48(&RGB24::BLACK));
        assert_eq!(RGB48::RED, rgb24_to_rgb48(&RGB24::RED));
        assert_eq!(RGB48::GREEN, rgb24_to_rgb48(&RGB24::GREEN));
        assert_eq!(RGB48::BLUE, rgb24_to_rgb48(&RGB24::BLUE));
    }

    #[test]
    fn rgb48_to_rgb24_() {
        assert_eq!(RGB24::WHITE, rgb48_to_rgb24(&RGB48::WHITE));
        assert_eq!(RGB24::BLACK, rgb48_to_rgb24(&RGB48::BLACK));
        assert_eq!(RGB24::RED, rgb48_to_rgb24(&RGB48::RED));
        assert_eq!(RGB24::GREEN, rgb48_to_rgb24(&RGB48::GREEN));
        assert_eq!(RGB24::BLUE, rgb48_to_rgb24(&RGB48::BLUE));
    }
}
