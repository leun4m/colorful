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

/// Converts a float to a byte
///
/// # Expects
/// - values as fractions from 0.0 to 1.0
/// - values > 1 will be treated as 1
/// - values < 0 will be treated as 0
/// - NAN => 0
pub fn save_convert_float_to_byte(float: f64) -> u8 {
    if float >= 1.0 {
        u8::MAX
    } else if float <= 0.0 {
        0
    } else {
        (float * u8::MAX as f64) as u8
    }
}

/// Converts a float triple to byte triple
///
/// **Expects fractions!**
///
/// For more info read comment `save_convert_float_to_byte`
pub fn to_byte_tuple(floats: (f64, f64, f64)) -> (u8, u8, u8) {
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
        (a - b).abs() < epsilon
    }
}

#[cfg(test)]
mod tests {
    use crate::color_models::number_utils::{
        approx_equal_f64, as_float, get_max, get_min, save_convert_float_to_byte, to_byte_tuple,
    };
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

    #[test]
    fn to_byte_tuple__normal() {
        let tuple = (0.0, 0.5, 1.0);
        let result = to_byte_tuple(tuple);
        assert_eq!((0, 127, 255), result);
    }

    #[test]
    fn to_byte_tuple__values_lower_zero() {
        let tuple = (-0.5, -0.9, -1042.4);
        let result = to_byte_tuple(tuple);
        assert_eq!((0, 0, 0), result);
    }

    #[test]
    fn to_byte_tuple__values_higher_one() {
        let tuple = (1.5, 1.9, 1042.4);
        let result = to_byte_tuple(tuple);
        assert_eq!((255, 255, 255), result);
    }

    #[test]
    fn to_byte_tuple__values_infinite() {
        let tuple = (INFINITY, -INFINITY, NAN);
        let result = to_byte_tuple(tuple);
        assert_eq!((255, 0, 0), result);
    }

    #[test]
    fn save_convert_float_to_byte__normal() {
        assert_eq!(0, save_convert_float_to_byte(0.0));
        assert_eq!(51, save_convert_float_to_byte(0.2));
        assert_eq!(127, save_convert_float_to_byte(0.5));
        assert_eq!(255, save_convert_float_to_byte(1.0));
    }

    #[test]
    fn save_convert_float_to_byte__lower_zero() {
        assert_eq!(0, save_convert_float_to_byte(-0.0));
        assert_eq!(0, save_convert_float_to_byte(-0.1));
        assert_eq!(0, save_convert_float_to_byte(-12.5));
        assert_eq!(0, save_convert_float_to_byte(-1124.0));
    }

    #[test]
    fn save_convert_float_to_byte__higher_one() {
        assert_eq!(255, save_convert_float_to_byte(1.0));
        assert_eq!(255, save_convert_float_to_byte(1.1));
        assert_eq!(255, save_convert_float_to_byte(112.5));
        assert_eq!(255, save_convert_float_to_byte(1204.0));
    }

    #[test]
    fn save_convert_float_to_byte__infinite() {
        assert_eq!(0, save_convert_float_to_byte(NAN));
        assert_eq!(0, save_convert_float_to_byte(-INFINITY));
        assert_eq!(255, save_convert_float_to_byte(INFINITY));
    }

    #[test]
    fn get_max__normal() {
        assert_eq!(1.2, get_max(0.0, 0.1, 1.2));
        assert_eq!(1.2, get_max(0.0, 1.2, 1.0));
        assert_eq!(1.2, get_max(1.2, 0.1, 1.0));
    }

    #[test]
    fn get_max__infinite() {
        assert_eq!(1.2, get_max(0.0, NAN, 1.2));
        assert_eq!(INFINITY, get_max(0.0, INFINITY, 1.0));
        assert_eq!(-INFINITY, get_max(-INFINITY, -INFINITY, NAN));
    }

    #[test]
    fn get_min__normal() {
        assert_eq!(0.0, get_min(0.0, 0.1, 1.2));
        assert_eq!(-5.0, get_min(-5.0, 1.2, 1.0));
        assert_eq!(0.1, get_min(1.2, 0.1, 1.0));
    }

    #[test]
    fn get_min__infinite() {
        assert_eq!(0.0, get_min(0.0, NAN, 1.2));
        assert_eq!(0.0, get_min(0.0, INFINITY, 1.0));
        assert_eq!(INFINITY, get_min(INFINITY, INFINITY, NAN));
    }

    #[test]
    fn as_float__normal() {
        assert!(approx_equal_f64(0.0, as_float(0), 0.01));
        assert!(approx_equal_f64(0.2, as_float(51), 0.01));
        assert!(approx_equal_f64(0.5, as_float(127), 0.01));
        assert!(approx_equal_f64(1.0, as_float(255), 0.01));
    }
}
