use crate::color_models::rgb_color;

/// Calculates the maximum value of the given triple
pub fn get_max(a: f64, b: f64, c: f64) -> f64 {
    a.max(b.max(c))
}

/// Calculates the minimum value of the given triple
pub fn get_min(a: f64, b: f64, c: f64) -> f64 {
    a.min(b.min(c))
}

/// Converts *byte* to *float* representation
pub fn as_float(a: u8) -> f64 {
    a as f64 / rgb_color::MAX_VALUE as f64
}

pub fn save_convert_float_to_byte(float: f64) -> u8 {
    if float >= 1.0 {
        255
    } else if float < 0.0 {
        0
    } else {
        (float * 255.0) as u8
    }
}

pub fn as_byte_tuple(floats: (f64, f64, f64)) -> (u8, u8, u8) {
    (
        save_convert_float_to_byte(floats.0),
        save_convert_float_to_byte(floats.1),
        save_convert_float_to_byte(floats.2),
    )
}

/// Returns `true` if `a` and `b` are approximately equal considering the given epsilon
///
/// # Normal case
/// `approx_equal_f64(1.01, 1.02, 0.01) => false`
/// `approx_equal_f64(1.01, 1.02, 0.1) => true`
///
/// # Special cases
///
/// *In this special cases epsilon will be ignored*
///
/// - `NAN == NAN`
/// - `NAN != INFINITY`
/// - `INFINITY` == `INFINITY`
/// - `-INFINITY` == `-INFINITY`
/// - `INFINITY` != `-INFINITY`
pub fn approx_equal_f64(a: f64, b: f64, epsilon: f64) -> bool {
    // If exactly one of the values is not finite
    if (a.is_finite() && !b.is_finite()) || (!a.is_finite() && b.is_finite()) {
        false
    }
    // If both are not finite
    else if !a.is_finite() && !b.is_finite() {
        a.is_nan() && b.is_nan()
            || a.is_infinite()
                && b.is_infinite()
                && ((a.is_sign_positive() && b.is_sign_positive())
                    || a.is_sign_negative() && b.is_sign_negative())
    }
    // If both are finite
    else {
        println!("{}", (a - b).abs());
        (a - b).abs() < epsilon
    }
}

#[cfg(test)]
mod tests {
    use crate::color_models::utils::approx_equal_f64;
    use core::num::FpCategory::Infinite;
    use std::f64::{INFINITY, NAN};

    #[test]
    fn approx_equal_f64__nan_nan() {
        let a = NAN;
        let b = NAN;
        assert!(approx_equal_f64(a, b, 1.0));
        assert!(approx_equal_f64(b, a, 1.0));
    }

    #[test]
    fn approx_equal_f64__nan_normal() {
        let a = NAN;
        let b = 2.1458921;
        assert!(!approx_equal_f64(a, b, 1.0));
        assert!(!approx_equal_f64(b, a, 1.0));
    }

    #[test]
    fn approx_equal_f64__infinity_normal() {
        let a = INFINITY;
        let b = 1.7927498;
        assert!(!approx_equal_f64(a, b, 1.0));
        assert!(!approx_equal_f64(b, a, 1.0));
    }

    #[test]
    fn approx_equal_f64__minfinity_normal() {
        let a = -INFINITY;
        let b = 4.0548962;
        assert!(!approx_equal_f64(a, b, 1.0));
        assert!(!approx_equal_f64(b, a, 1.0));
    }

    #[test]
    fn approx_equal_f64__infinity_nan() {
        let a = INFINITY;
        let b = NAN;
        assert!(!approx_equal_f64(a, b, 1.0));
        assert!(!approx_equal_f64(b, a, 1.0));
    }

    #[test]
    fn approx_equal_f64__minfinity_nan() {
        let a = -INFINITY;
        let b = NAN;
        assert!(!approx_equal_f64(a, b, 1.0));
        assert!(!approx_equal_f64(b, a, 1.0));
    }

    #[test]
    fn approx_equal_f64__infinity_infinity() {
        let a = INFINITY;
        let b = INFINITY;
        assert!(approx_equal_f64(a, b, 1.0));
        assert!(approx_equal_f64(b, a, 1.0));
    }

    #[test]
    fn approx_equal_f64__minfinity_minfinity() {
        let a = -INFINITY;
        let b = -INFINITY;
        assert!(approx_equal_f64(a, b, 1.0));
        assert!(approx_equal_f64(b, a, 1.0));
    }

    #[test]
    fn approx_equal_f64__normal_normal_exact() {
        let a = 30.15657564;
        let b = 30.15657564;
        assert!(approx_equal_f64(a, b, 0.00001));
        assert!(approx_equal_f64(b, a, 0.00001));
    }

    #[test]
    fn approx_equal_f64__normal_mnormal_exact() {
        let a = 29.1124521;
        let b = -29.1124521;
        assert!(!approx_equal_f64(a, b, 0.00001));
        assert!(!approx_equal_f64(b, a, 0.00001));
    }

    #[test]
    fn approx_equal_f64__mnormal_mnormal_exact() {
        let a = -29.1124521;
        let b = -29.1124521;
        assert!(approx_equal_f64(a, b, 0.000_000_001));
        assert!(approx_equal_f64(b, a, 0.000_000_001));
    }

    #[test]
    fn approx_equal_f64__mnormal_mnormal() {
        let a = -29.12345;
        let b = -29.12346;

        assert!(!approx_equal_f64(a, b, 0.000_01));
        assert!(!approx_equal_f64(b, a, 0.000_01));

        assert!(approx_equal_f64(a, b, 0.0001));
        assert!(approx_equal_f64(b, a, 0.0001));
    }

    #[test]
    fn approx_equal_f64__normal_normal() {
        let a = 37.000_111_222_5;
        let b = 37.000_111_222_6;

        assert!(!approx_equal_f64(a, b, 0.000_000_000_01));
        assert!(!approx_equal_f64(b, a, 0.000_000_000_01));

        assert!(approx_equal_f64(a, b, 0.0001));
        assert!(approx_equal_f64(b, a, 0.0001));
    }

    #[test]
    fn approx_equal_f64__comment() {
        let a = 1.01;
        let b = 1.02;

        assert!(!approx_equal_f64(a, b, 0.01));
        assert!(!approx_equal_f64(b, a, 0.01));

        assert!(approx_equal_f64(a, b, 0.1));
        assert!(approx_equal_f64(b, a, 0.1));
    }
}
