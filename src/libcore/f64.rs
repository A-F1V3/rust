// NB: transitionary, de-mode-ing.
#[forbid(deprecated_mode)];
#[forbid(deprecated_pattern)];

//! Operations and constants for `f64`

// PORT

use cmath::c_double::*;
use cmath::c_double_targ_consts::*;

// These are not defined inside consts:: for consistency with
// the integer types

// Even though this module exports everything defined in it,
// because it contains re-exports, we also have to explicitly
// export locally defined things. That's a bit annoying.
export add, sub, mul, div, rem, lt, le, eq, ne, ge, gt;
export is_positive, is_negative, is_nonpositive, is_nonnegative;
export is_zero, is_infinite, is_finite;
export NaN, is_NaN, infinity, neg_infinity;
export consts;
export logarithm;
export acos, asin, atan, atan2, cbrt, ceil, copysign, cos, cosh, floor;
export erf, erfc, exp, expm1, exp2, abs, abs_sub;
export mul_add, fmax, fmin, nextafter, frexp, hypot, ldexp;
export lgamma, ln, log_radix, ln1p, log10, log2, ilog_radix;
export modf, pow, round, sin, sinh, sqrt, tan, tanh, tgamma, trunc;
export signbit;
export epsilon;

export j0, j1, jn, y0, y1, yn;

export num;

// PORT check per architecture

// FIXME (#1433): obtain these in a different way

pub const radix: uint = 2u;

pub const mantissa_digits: uint = 53u;
pub const digits: uint = 15u;

pub const epsilon: f64 = 2.2204460492503131e-16_f64;

pub const min_value: f64 = 2.2250738585072014e-308_f64;
pub const max_value: f64 = 1.7976931348623157e+308_f64;

pub const min_exp: int = -1021;
pub const max_exp: int = 1024;

pub const min_10_exp: int = -307;
pub const max_10_exp: int = 308;

pub const NaN: f64 = 0.0_f64/0.0_f64;

pub const infinity: f64 = 1.0_f64/0.0_f64;

pub const neg_infinity: f64 = -1.0_f64/0.0_f64;

pub pure fn is_NaN(f: f64) -> bool { f != f }

pub pure fn add(x: f64, y: f64) -> f64 { return x + y; }

pub pure fn sub(x: f64, y: f64) -> f64 { return x - y; }

pub pure fn mul(x: f64, y: f64) -> f64 { return x * y; }

pub pure fn div(x: f64, y: f64) -> f64 { return x / y; }

pub pure fn rem(x: f64, y: f64) -> f64 { return x % y; }

pub pure fn lt(x: f64, y: f64) -> bool { return x < y; }

pub pure fn le(x: f64, y: f64) -> bool { return x <= y; }

pub pure fn eq(x: f64, y: f64) -> bool { return x == y; }

pub pure fn ne(x: f64, y: f64) -> bool { return x != y; }

pub pure fn ge(x: f64, y: f64) -> bool { return x >= y; }

pub pure fn gt(x: f64, y: f64) -> bool { return x > y; }

pub pure fn sqrt(x: f64) -> f64 {
    cmath::c_double::sqrt(x as libc::c_double) as f64
}

/// Returns true if `x` is a positive number, including +0.0f640 and +Infinity
pub pure fn is_positive(x: f64) -> bool
    { return x > 0.0f64 || (1.0f64/x) == infinity; }

/// Returns true if `x` is a negative number, including -0.0f640 and -Infinity
pub pure fn is_negative(x: f64) -> bool
    { return x < 0.0f64 || (1.0f64/x) == neg_infinity; }

/**
 * Returns true if `x` is a negative number, including -0.0f640 and -Infinity
 *
 * This is the same as `f64::is_negative`.
 */
pub pure fn is_nonpositive(x: f64) -> bool {
  return x < 0.0f64 || (1.0f64/x) == neg_infinity;
}

/**
 * Returns true if `x` is a positive number, including +0.0f640 and +Infinity
 *
 * This is the same as `f64::positive`.
 */
pub pure fn is_nonnegative(x: f64) -> bool {
  return x > 0.0f64 || (1.0f64/x) == infinity;
}

/// Returns true if `x` is a zero number (positive or negative zero)
pub pure fn is_zero(x: f64) -> bool {
    return x == 0.0f64 || x == -0.0f64;
}

/// Returns true if `x`is an infinite number
pub pure fn is_infinite(x: f64) -> bool {
    return x == infinity || x == neg_infinity;
}

/// Returns true if `x`is a finite number
pub pure fn is_finite(x: f64) -> bool {
    return !(is_NaN(x) || is_infinite(x));
}

// FIXME (#1999): add is_normal, is_subnormal, and fpclassify

/* Module: consts */
pub mod consts {
    // FIXME (requires Issue #1433 to fix): replace with mathematical
    // constants from cmath.
    /// Archimedes' constant
    pub const pi: f64 = 3.14159265358979323846264338327950288_f64;

    /// pi/2.0
    pub const frac_pi_2: f64 = 1.57079632679489661923132169163975144_f64;

    /// pi/4.0
    pub const frac_pi_4: f64 = 0.785398163397448309615660845819875721_f64;

    /// 1.0/pi
    pub const frac_1_pi: f64 = 0.318309886183790671537767526745028724_f64;

    /// 2.0/pi
    pub const frac_2_pi: f64 = 0.636619772367581343075535053490057448_f64;

    /// 2.0/sqrt(pi)
    pub const frac_2_sqrtpi: f64 = 1.12837916709551257389615890312154517_f64;

    /// sqrt(2.0)
    pub const sqrt2: f64 = 1.41421356237309504880168872420969808_f64;

    /// 1.0/sqrt(2.0)
    pub const frac_1_sqrt2: f64 = 0.707106781186547524400844362104849039_f64;

    /// Euler's number
    pub const e: f64 = 2.71828182845904523536028747135266250_f64;

    /// log2(e)
    pub const log2_e: f64 = 1.44269504088896340735992468100189214_f64;

    /// log10(e)
    pub const log10_e: f64 = 0.434294481903251827651128918916605082_f64;

    /// ln(2.0)
    pub const ln_2: f64 = 0.693147180559945309417232121458176568_f64;

    /// ln(10.0)
    pub const ln_10: f64 = 2.30258509299404568401799145468436421_f64;
}

pub pure fn signbit(x: f64) -> int {
    if is_negative(x) { return 1; } else { return 0; }
}

pub pure fn logarithm(n: f64, b: f64) -> f64 {
    return log2(n) / log2(b);
}

impl f64: num::Num {
    pure fn add(other: &f64)    -> f64 { return self + *other; }
    pure fn sub(other: &f64)    -> f64 { return self - *other; }
    pure fn mul(other: &f64)    -> f64 { return self * *other; }
    pure fn div(other: &f64)    -> f64 { return self / *other; }
    pure fn modulo(other: &f64) -> f64 { return self % *other; }
    pure fn neg()                -> f64 { return -self;        }

    pure fn to_int()         -> int { return self as int; }
    static pure fn from_int(n: int) -> f64 { return n as f64;    }
}

//
// Local Variables:
// mode: rust
// fill-column: 78;
// indent-tabs-mode: nil
// c-basic-offset: 4
// buffer-file-coding-system: utf-8-unix
// End:
//
