/// Calculates the maximum value of the given tuple
pub fn get_max(a: f64, b: f64, c: f64) -> f64 {
    a.max(b.max(c))
}

/// Calculates the minimum value of the given tuple
pub fn get_min(a: f64, b: f64, c: f64) -> f64 {
    a.min(b.min(c))
}

/// Maps `f64` to `u8`
///
/// # Expects
/// - values as fractions from 0.0 to 1.0
/// - values > 1 will be treated as 1
/// - values < 0 will be treated as 0
/// - NAN => 0
pub fn to_u8_repr(float: f64) -> u8 {
    if float >= 1.0 {
        u8::MAX
    } else if float <= 0.0 {
        0
    } else {
        (float * u8::MAX as f64).round() as u8
    }
}

/// Maps `f64` to `u16`
///
/// # Expects
/// - values as fractions from 0.0 to 1.0
/// - values > 1 will be treated as 1
/// - values < 0 will be treated as 0
/// - NAN => 0
pub fn to_u16_repr(float: f64) -> u16 {
    if float >= 1.0 {
        u16::MAX
    } else if float <= 0.0 {
        0
    } else {
        (float * u16::MAX as f64).round() as u16
    }
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
/// - `NAN != f64::INFINITY`
/// - `f64::INFINITY` == `f64::INFINITY`
/// - `f64::NEG_INFINITY` == `f64::NEG_INFINITY`
/// - `f64::INFINITY` != `f64::NEG_INFINITY`
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
        (a - b).abs() < epsilon
    }
}

/// Converts any number to the given range.
///
/// # Rules
/// - value < min => min
/// - value > max => max
/// - else => value
pub fn convert_to_range(a: f64, min: f64, max: f64) -> f64 {
    if a <= min {
        min
    } else if a >= max {
        max
    } else {
        a
    }
}

#[cfg(test)]
mod tests {
    use crate::number_utils::{approx_equal_f64, get_max, get_min, to_u8_repr};

    #[test]
    fn approx_equal_f64_nan_nan() {
        let a = f64::NAN;
        let b = f64::NAN;
        assert!(approx_equal_f64(a, b, 1.0));
        assert!(approx_equal_f64(b, a, 1.0));
    }

    #[test]
    fn approx_equal_f64_nan_normal() {
        let a = f64::NAN;
        let b = 2.1458921;
        assert!(!approx_equal_f64(a, b, 1.0));
        assert!(!approx_equal_f64(b, a, 1.0));
    }

    #[test]
    fn approx_equal_f64_infinity_normal() {
        let a = f64::INFINITY;
        let b = 1.7927498;
        assert!(!approx_equal_f64(a, b, 1.0));
        assert!(!approx_equal_f64(b, a, 1.0));
    }

    #[test]
    fn approx_equal_f64_minfinity_normal() {
        let a = f64::NEG_INFINITY;
        let b = 4.0548962;
        assert!(!approx_equal_f64(a, b, 1.0));
        assert!(!approx_equal_f64(b, a, 1.0));
    }

    #[test]
    fn approx_equal_f64_infinity_nan() {
        let a = f64::INFINITY;
        let b = f64::NAN;
        assert!(!approx_equal_f64(a, b, 1.0));
        assert!(!approx_equal_f64(b, a, 1.0));
    }

    #[test]
    fn approx_equal_f64_minfinity_nan() {
        let a = f64::NEG_INFINITY;
        let b = f64::NAN;
        assert!(!approx_equal_f64(a, b, 1.0));
        assert!(!approx_equal_f64(b, a, 1.0));
    }

    #[test]
    fn approx_equal_f64_infinity_infinity() {
        let a = f64::INFINITY;
        let b = f64::INFINITY;
        assert!(approx_equal_f64(a, b, 1.0));
        assert!(approx_equal_f64(b, a, 1.0));
    }

    #[test]
    fn approx_equal_f64_neg_infinity_neg_infinity() {
        let a = f64::NEG_INFINITY;
        let b = f64::NEG_INFINITY;
        assert!(approx_equal_f64(a, b, 1.0));
        assert!(approx_equal_f64(b, a, 1.0));
    }

    #[test]
    fn approx_equal_f64_normal_normal_exact() {
        let a = 30.15657564;
        let b = 30.15657564;
        assert!(approx_equal_f64(a, b, 0.00001));
        assert!(approx_equal_f64(b, a, 0.00001));
    }

    #[test]
    fn approx_equal_f64_normal_neg_normal_exact() {
        let a = 29.1124521;
        let b = -29.1124521;
        assert!(!approx_equal_f64(a, b, 0.00001));
        assert!(!approx_equal_f64(b, a, 0.00001));
    }

    #[test]
    fn approx_equal_f64_neg_normal_neg_normal_exact() {
        let a = -29.1124521;
        let b = -29.1124521;
        assert!(approx_equal_f64(a, b, 0.000_000_001));
        assert!(approx_equal_f64(b, a, 0.000_000_001));
    }

    #[test]
    fn approx_equal_f64_neg_normal_neg_normal() {
        let a = -29.12345;
        let b = -29.12346;

        assert!(!approx_equal_f64(a, b, 0.000_01));
        assert!(!approx_equal_f64(b, a, 0.000_01));

        assert!(approx_equal_f64(a, b, 0.0001));
        assert!(approx_equal_f64(b, a, 0.0001));
    }

    #[test]
    fn approx_equal_f64_normal_normal() {
        let a = 37.000_111_222_5;
        let b = 37.000_111_222_6;

        assert!(!approx_equal_f64(a, b, 0.000_000_000_01));
        assert!(!approx_equal_f64(b, a, 0.000_000_000_01));

        assert!(approx_equal_f64(a, b, 0.0001));
        assert!(approx_equal_f64(b, a, 0.0001));
    }

    #[test]
    fn approx_equal_f64_comment() {
        let a = 1.01;
        let b = 1.02;

        assert!(!approx_equal_f64(a, b, 0.01));
        assert!(!approx_equal_f64(b, a, 0.01));

        assert!(approx_equal_f64(a, b, 0.1));
        assert!(approx_equal_f64(b, a, 0.1));
    }

    #[test]
    fn save_convert_float_to_byte_normal() {
        assert_eq!(0, to_u8_repr(0.0));
        assert_eq!(51, to_u8_repr(0.2));
        assert_eq!(128, to_u8_repr(0.5));
        assert_eq!(255, to_u8_repr(1.0));
    }

    #[test]
    fn save_convert_float_to_byte_lower_zero() {
        assert_eq!(0, to_u8_repr(-0.0));
        assert_eq!(0, to_u8_repr(-0.1));
        assert_eq!(0, to_u8_repr(-12.5));
        assert_eq!(0, to_u8_repr(-1124.0));
    }

    #[test]
    fn save_convert_float_to_byte_higher_one() {
        assert_eq!(255, to_u8_repr(1.0));
        assert_eq!(255, to_u8_repr(1.1));
        assert_eq!(255, to_u8_repr(112.5));
        assert_eq!(255, to_u8_repr(1204.0));
    }

    #[test]
    fn save_convert_float_to_byte_infinite() {
        assert_eq!(0, to_u8_repr(f64::NAN));
        assert_eq!(0, to_u8_repr(f64::NEG_INFINITY));
        assert_eq!(255, to_u8_repr(f64::INFINITY));
    }

    #[test]
    fn get_max_normal() {
        assert_eq!(1.2, get_max(0.0, 0.1, 1.2));
        assert_eq!(1.2, get_max(0.0, 1.2, 1.0));
        assert_eq!(1.2, get_max(1.2, 0.1, 1.0));
    }

    #[test]
    fn get_max_infinite() {
        assert_eq!(1.2, get_max(0.0, f64::NAN, 1.2));
        assert_eq!(f64::INFINITY, get_max(0.0, f64::INFINITY, 1.0));
        assert_eq!(
            f64::NEG_INFINITY,
            get_max(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NAN)
        );
    }

    #[test]
    fn get_min_normal() {
        assert_eq!(0.0, get_min(0.0, 0.1, 1.2));
        assert_eq!(-5.0, get_min(-5.0, 1.2, 1.0));
        assert_eq!(0.1, get_min(1.2, 0.1, 1.0));
    }

    #[test]
    fn get_min_infinite() {
        assert_eq!(0.0, get_min(0.0, f64::NAN, 1.2));
        assert_eq!(0.0, get_min(0.0, f64::INFINITY, 1.0));
        assert_eq!(
            f64::INFINITY,
            get_min(f64::INFINITY, f64::INFINITY, f64::NAN)
        );
    }
}
