//! Math backend implementations for different numeric types and feature configurations.

use super::traits::MathBackend;

/// Floating-point backend using micromath
#[cfg(feature = "floating-point")]
pub struct FloatingPointBackend;

#[cfg(feature = "floating-point")]
impl MathBackend<f32> for FloatingPointBackend {
    #[inline]
    fn sqrt(&self, x: f32) -> f32 {
        micromath::F32Ext::sqrt(x)
    }

    #[inline]
    fn abs(&self, x: f32) -> f32 {
        if x < 0.0 {
            -x
        } else {
            x
        }
    }

    #[inline]
    fn min(&self, a: f32, b: f32) -> f32 {
        if a < b {
            a
        } else {
            b
        }
    }

    #[inline]
    fn max(&self, a: f32, b: f32) -> f32 {
        if a > b {
            a
        } else {
            b
        }
    }

    #[inline]
    fn floor(&self, x: f32) -> f32 {
        micromath::F32Ext::floor(x)
    }

    #[inline]
    fn ceil(&self, x: f32) -> f32 {
        micromath::F32Ext::ceil(x)
    }

    #[inline]
    fn pow(&self, x: f32, y: f32) -> f32 {
        micromath::F32Ext::powf(x, y)
    }

    #[inline]
    fn ln(&self, x: f32) -> f32 {
        micromath::F32Ext::ln(x)
    }

    #[inline]
    fn log10(&self, x: f32) -> f32 {
        micromath::F32Ext::log10(x)
    }

    #[inline]
    fn sin(&self, x: f32) -> f32 {
        micromath::F32Ext::sin(x)
    }

    #[inline]
    fn cos(&self, x: f32) -> f32 {
        micromath::F32Ext::cos(x)
    }

    #[inline]
    fn tan(&self, x: f32) -> f32 {
        micromath::F32Ext::tan(x)
    }

    #[inline]
    fn to_radians(&self, degrees: f32) -> f32 {
        degrees * (core::f32::consts::PI / 180.0)
    }

    #[inline]
    fn to_degrees(&self, radians: f32) -> f32 {
        radians * (180.0 / core::f32::consts::PI)
    }

    #[inline]
    fn atan2(&self, y: f32, x: f32) -> f32 {
        micromath::F32Ext::atan2(y, x)
    }
}

/// Libm backend for floating-point operations
#[cfg(feature = "libm-math")]
pub struct LibmBackend;

#[cfg(feature = "libm-math")]
impl MathBackend<f32> for LibmBackend {
    #[inline]
    fn sqrt(&self, x: f32) -> f32 {
        libm::sqrtf(x)
    }

    #[inline]
    fn abs(&self, x: f32) -> f32 {
        libm::fabsf(x)
    }

    #[inline]
    fn min(&self, a: f32, b: f32) -> f32 {
        libm::fminf(a, b)
    }

    #[inline]
    fn max(&self, a: f32, b: f32) -> f32 {
        libm::fmaxf(a, b)
    }

    #[inline]
    fn floor(&self, x: f32) -> f32 {
        libm::floorf(x)
    }

    #[inline]
    fn ceil(&self, x: f32) -> f32 {
        libm::ceilf(x)
    }

    #[inline]
    fn pow(&self, x: f32, y: f32) -> f32 {
        libm::powf(x, y)
    }

    #[inline]
    fn ln(&self, x: f32) -> f32 {
        libm::logf(x)
    }

    #[inline]
    fn log10(&self, x: f32) -> f32 {
        libm::log10f(x)
    }

    #[inline]
    fn sin(&self, x: f32) -> f32 {
        libm::sinf(x)
    }

    #[inline]
    fn cos(&self, x: f32) -> f32 {
        libm::cosf(x)
    }

    #[inline]
    fn tan(&self, x: f32) -> f32 {
        libm::tanf(x)
    }

    #[inline]
    fn to_radians(&self, degrees: f32) -> f32 {
        degrees * (core::f32::consts::PI / 180.0)
    }

    #[inline]
    fn to_degrees(&self, radians: f32) -> f32 {
        radians * (180.0 / core::f32::consts::PI)
    }

    #[inline]
    fn atan2(&self, y: f32, x: f32) -> f32 {
        libm::atan2f(y, x)
    }
}

/// Fixed-point backend using the fixed crate
#[cfg(feature = "fixed-point")]
pub struct FixedPointBackend;

#[cfg(feature = "fixed-point")]
impl MathBackend<fixed::types::I16F16> for FixedPointBackend {
    #[inline]
    fn sqrt(&self, x: fixed::types::I16F16) -> fixed::types::I16F16 {
        // Simple Newton-Raphson method for square root
        if x <= fixed::types::I16F16::ZERO {
            return fixed::types::I16F16::ZERO;
        }

        let mut guess = x / 2;
        for _ in 0..8 {
            // 8 iterations should be sufficient for I16F16
            let new_guess = (guess + x / guess) / 2;
            if (new_guess - guess).abs() < fixed::types::I16F16::from_num(0.001) {
                break;
            }
            guess = new_guess;
        }
        guess
    }

    #[inline]
    fn abs(&self, x: fixed::types::I16F16) -> fixed::types::I16F16 {
        x.abs()
    }

    #[inline]
    fn min(&self, a: fixed::types::I16F16, b: fixed::types::I16F16) -> fixed::types::I16F16 {
        if a < b {
            a
        } else {
            b
        }
    }

    #[inline]
    fn max(&self, a: fixed::types::I16F16, b: fixed::types::I16F16) -> fixed::types::I16F16 {
        if a > b {
            a
        } else {
            b
        }
    }

    #[inline]
    fn floor(&self, x: fixed::types::I16F16) -> fixed::types::I16F16 {
        x.floor()
    }

    #[inline]
    fn ceil(&self, x: fixed::types::I16F16) -> fixed::types::I16F16 {
        x.ceil()
    }

    #[inline]
    fn pow(&self, x: fixed::types::I16F16, y: fixed::types::I16F16) -> fixed::types::I16F16 {
        // Simple integer power for fixed-point
        if y == fixed::types::I16F16::ZERO {
            return fixed::types::I16F16::ONE;
        }

        let y_int: i32 = y.to_num();
        if y_int < 0 {
            return fixed::types::I16F16::ONE / self.pow(x, -y);
        }

        let mut result = fixed::types::I16F16::ONE;
        let mut base = x;
        let mut exp = y_int as u32;

        while exp > 0 {
            if exp & 1 == 1 {
                result *= base;
            }
            base = base * base;
            exp >>= 1;
        }
        result
    }

    #[inline]
    fn ln(&self, x: fixed::types::I16F16) -> fixed::types::I16F16 {
        // Simplified natural log approximation for fixed-point
        if x <= fixed::types::I16F16::ZERO {
            return fixed::types::I16F16::from_num(-100.0); // Approximate -infinity
        }

        // Use Taylor series approximation around x=1
        let one = fixed::types::I16F16::ONE;
        let t = x - one;

        // ln(1+t) ≈ t - t²/2 + t³/3 - t⁴/4 + ...
        let t2 = t * t;
        let t3 = t2 * t;
        let t4 = t3 * t;

        t - t2 / 2 + t3 / 3 - t4 / 4
    }

    #[inline]
    fn log10(&self, x: fixed::types::I16F16) -> fixed::types::I16F16 {
        // log10(x) = ln(x) / ln(10)
        let ln_x = self.ln(x);
        let ln_10 = fixed::types::I16F16::from_num(core::f32::consts::LN_10); // ln(10)
        ln_x / ln_10
    }

    #[inline]
    fn sin(&self, x: fixed::types::I16F16) -> fixed::types::I16F16 {
        // Taylor series approximation for sine
        let pi = fixed::types::I16F16::from_num(core::f32::consts::PI);
        let two_pi = pi * 2;

        // Normalize angle to [-π, π]
        let mut angle = x;
        while angle > pi {
            angle -= two_pi;
        }
        while angle < -pi {
            angle += two_pi;
        }

        // sin(x) ≈ x - x³/6 + x⁵/120 - x⁷/5040
        let x2 = angle * angle;
        let x3 = x2 * angle;
        let x5 = x3 * x2;
        let x7 = x5 * x2;

        angle - x3 / 6 + x5 / 120 - x7 / 5040
    }

    #[inline]
    fn cos(&self, x: fixed::types::I16F16) -> fixed::types::I16F16 {
        // cos(x) = sin(x + π/2)
        let pi_2 = fixed::types::I16F16::from_num(core::f32::consts::FRAC_PI_2); // π/2
        self.sin(x + pi_2)
    }

    #[inline]
    fn tan(&self, x: fixed::types::I16F16) -> fixed::types::I16F16 {
        // tan(x) = sin(x) / cos(x)
        let sin_x = self.sin(x);
        let cos_x = self.cos(x);

        if cos_x.abs() < fixed::types::I16F16::from_num(0.001) {
            // Avoid division by zero
            if sin_x >= fixed::types::I16F16::ZERO {
                fixed::types::I16F16::from_num(100.0) // Approximate +infinity
            } else {
                fixed::types::I16F16::from_num(-100.0) // Approximate -infinity
            }
        } else {
            sin_x / cos_x
        }
    }

    #[inline]
    fn to_radians(&self, degrees: fixed::types::I16F16) -> fixed::types::I16F16 {
        let pi = fixed::types::I16F16::from_num(core::f32::consts::PI);
        degrees * pi / fixed::types::I16F16::from_num(180.0)
    }

    #[inline]
    fn to_degrees(&self, radians: fixed::types::I16F16) -> fixed::types::I16F16 {
        let pi = fixed::types::I16F16::from_num(core::f32::consts::PI);
        radians * fixed::types::I16F16::from_num(180.0) / pi
    }

    #[inline]
    fn atan2(&self, y: fixed::types::I16F16, x: fixed::types::I16F16) -> fixed::types::I16F16 {
        // Implement atan2 using atan approximation
        let zero = fixed::types::I16F16::ZERO;
        let pi = fixed::types::I16F16::from_num(core::f32::consts::PI);
        let pi_2 = pi / 2;

        if x == zero {
            if y > zero {
                return pi_2;
            } else if y < zero {
                return -pi_2;
            } else {
                return zero; // undefined, but return 0
            }
        }

        // Use atan(y/x) and adjust for quadrant
        let ratio = y / x;
        let atan_ratio = self.atan_approx(ratio);

        if x > zero {
            atan_ratio
        } else if y >= zero {
            atan_ratio + pi
        } else {
            atan_ratio - pi
        }
    }
}

#[cfg(feature = "fixed-point")]
impl FixedPointBackend {
    /// Approximate atan using polynomial approximation
    #[allow(clippy::only_used_in_recursion)]
    fn atan_approx(&self, x: fixed::types::I16F16) -> fixed::types::I16F16 {
        let abs_x = x.abs();
        let pi_4 = fixed::types::I16F16::from_num(core::f32::consts::FRAC_PI_4); // π/4

        // For |x| <= 1, use: atan(x) ≈ x - x³/3 + x⁵/5 - x⁷/7
        if abs_x <= fixed::types::I16F16::ONE {
            let x2 = x * x;
            let x3 = x2 * x;
            let x5 = x3 * x2;
            let x7 = x5 * x2;

            x - x3 / 3 + x5 / 5 - x7 / 7
        } else {
            // For |x| > 1, use: atan(x) = π/2 - atan(1/x)
            let inv_x = fixed::types::I16F16::ONE / x;
            let atan_inv = self.atan_approx(inv_x);

            if x > fixed::types::I16F16::ZERO {
                pi_4 * 2 - atan_inv
            } else {
                -pi_4 * 2 - atan_inv
            }
        }
    }
}

/// Integer-only backend for the most constrained environments
#[cfg(feature = "integer-math")]
pub struct IntegerBackend;

#[cfg(feature = "integer-math")]
impl MathBackend<i32> for IntegerBackend {
    #[inline]
    fn sqrt(&self, x: i32) -> i32 {
        if x <= 0 {
            return 0;
        }

        // Integer square root using binary search
        let mut left = 0i32;
        let mut right = x;
        let mut result = 0i32;

        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_squared = mid.saturating_mul(mid);

            if mid_squared == x {
                return mid;
            } else if mid_squared < x {
                left = mid + 1;
                result = mid;
            } else {
                right = mid - 1;
            }
        }

        result
    }

    #[inline]
    fn abs(&self, x: i32) -> i32 {
        x.abs()
    }

    #[inline]
    fn min(&self, a: i32, b: i32) -> i32 {
        if a < b {
            a
        } else {
            b
        }
    }

    #[inline]
    fn max(&self, a: i32, b: i32) -> i32 {
        if a > b {
            a
        } else {
            b
        }
    }

    #[inline]
    fn floor(&self, x: i32) -> i32 {
        x // Integers are already "floored"
    }

    #[inline]
    fn ceil(&self, x: i32) -> i32 {
        x // Integers are already "ceiled"
    }

    #[inline]
    fn pow(&self, x: i32, y: i32) -> i32 {
        if y == 0 {
            return 1;
        }
        if y < 0 {
            return 0; // Integer division would result in 0 for most cases
        }

        let mut result = 1i32;
        let mut base = x;
        let mut exp = y as u32;

        while exp > 0 {
            if exp & 1 == 1 {
                result = result.saturating_mul(base);
            }
            base = base.saturating_mul(base);
            exp >>= 1;
        }
        result
    }

    #[inline]
    fn ln(&self, x: i32) -> i32 {
        // Very rough integer approximation of natural log
        if x <= 0 {
            return -1000; // Approximate -infinity
        }

        // Simple lookup table for small values
        match x {
            1 => 0,
            2..=3 => 693,    // ln(2) * 1000 ≈ 693
            4..=7 => 1386,   // ln(4) * 1000 ≈ 1386
            8..=15 => 2079,  // ln(8) * 1000 ≈ 2079
            16..=31 => 2772, // ln(16) * 1000 ≈ 2772
            _ => {
                // For larger values, use bit length approximation
                let bit_len = 32 - x.leading_zeros() as i32;
                bit_len * 693 // Approximate ln(2^n) = n * ln(2)
            }
        }
    }

    #[inline]
    fn log10(&self, x: i32) -> i32 {
        // log10(x) = ln(x) / ln(10)
        let ln_x = self.ln(x);
        ln_x * 1000 / 2303 // ln(10) * 1000 ≈ 2303
    }

    #[inline]
    fn sin(&self, x: i32) -> i32 {
        // Very rough integer sine approximation using lookup table
        // Input is assumed to be in milliradians (radians * 1000)
        let pi_1000 = 3142; // π * 1000
        let two_pi_1000 = 6284; // 2π * 1000

        // Normalize to [0, 2π)
        let mut angle = x % two_pi_1000;
        if angle < 0 {
            angle += two_pi_1000;
        }

        // Simple piecewise linear approximation
        if angle <= pi_1000 / 2 {
            // First quadrant: sin increases from 0 to 1
            (angle * 1000) / (pi_1000 / 2)
        } else if angle <= pi_1000 {
            // Second quadrant: sin decreases from 1 to 0
            ((pi_1000 - angle) * 1000) / (pi_1000 / 2)
        } else if angle <= 3 * pi_1000 / 2 {
            // Third quadrant: sin decreases from 0 to -1
            -((angle - pi_1000) * 1000) / (pi_1000 / 2)
        } else {
            // Fourth quadrant: sin increases from -1 to 0
            -((two_pi_1000 - angle) * 1000) / (pi_1000 / 2)
        }
    }

    #[inline]
    fn cos(&self, x: i32) -> i32 {
        // cos(x) = sin(x + π/2)
        let pi_2_1000 = 1571; // π/2 * 1000
        self.sin(x + pi_2_1000)
    }

    #[inline]
    fn tan(&self, x: i32) -> i32 {
        let sin_x = self.sin(x);
        let cos_x = self.cos(x);

        if cos_x.abs() < 10 {
            // Avoid division by very small numbers
            if sin_x >= 0 {
                100000 // Large positive number
            } else {
                -100000 // Large negative number
            }
        } else {
            (sin_x * 1000) / cos_x
        }
    }

    #[inline]
    fn to_radians(&self, degrees: i32) -> i32 {
        // Convert degrees to milliradians
        (degrees * 3142) / (180 * 1000)
    }

    #[inline]
    fn to_degrees(&self, radians: i32) -> i32 {
        // Convert milliradians to degrees
        (radians * 180 * 1000) / 3142
    }

    #[inline]
    fn atan2(&self, y: i32, x: i32) -> i32 {
        // Integer atan2 implementation
        // Returns angle in milliradians (radians * 1000)
        let pi_1000 = 3142; // π * 1000
        let pi_2_1000 = 1571; // π/2 * 1000

        if x == 0 {
            if y > 0 {
                return pi_2_1000;
            } else if y < 0 {
                return -pi_2_1000;
            } else {
                return 0; // undefined, but return 0
            }
        }

        // Simple quadrant-based approximation
        let abs_y = y.abs();
        let abs_x = x.abs();

        // Use a simple lookup table approach for basic angles
        let angle = if abs_x >= abs_y {
            // More horizontal than vertical
            (abs_y * pi_2_1000) / abs_x / 2 // Rough approximation
        } else {
            // More vertical than horizontal
            pi_2_1000 - (abs_x * pi_2_1000) / abs_y / 2
        };

        // Adjust for quadrant
        if x > 0 && y >= 0 {
            angle // First quadrant
        } else if x <= 0 && y > 0 {
            pi_1000 - angle // Second quadrant
        } else if x < 0 && y <= 0 {
            -pi_1000 + angle // Third quadrant
        } else {
            -angle // Fourth quadrant
        }
    }
}

/// CORDIC backend for trigonometric functions
#[cfg(feature = "cordic-math")]
pub struct CordicBackend;

#[cfg(feature = "cordic-math")]
impl MathBackend<fixed::types::I16F16> for CordicBackend {
    #[inline]
    fn sqrt(&self, x: fixed::types::I16F16) -> fixed::types::I16F16 {
        // Use the fixed-point backend for non-trig functions
        let fixed_backend = FixedPointBackend;
        fixed_backend.sqrt(x)
    }

    #[inline]
    fn abs(&self, x: fixed::types::I16F16) -> fixed::types::I16F16 {
        x.abs()
    }

    #[inline]
    fn min(&self, a: fixed::types::I16F16, b: fixed::types::I16F16) -> fixed::types::I16F16 {
        if a < b {
            a
        } else {
            b
        }
    }

    #[inline]
    fn max(&self, a: fixed::types::I16F16, b: fixed::types::I16F16) -> fixed::types::I16F16 {
        if a > b {
            a
        } else {
            b
        }
    }

    #[inline]
    fn floor(&self, x: fixed::types::I16F16) -> fixed::types::I16F16 {
        x.floor()
    }

    #[inline]
    fn ceil(&self, x: fixed::types::I16F16) -> fixed::types::I16F16 {
        x.ceil()
    }

    #[inline]
    fn pow(&self, x: fixed::types::I16F16, y: fixed::types::I16F16) -> fixed::types::I16F16 {
        let fixed_backend = FixedPointBackend;
        fixed_backend.pow(x, y)
    }

    #[inline]
    fn ln(&self, x: fixed::types::I16F16) -> fixed::types::I16F16 {
        let fixed_backend = FixedPointBackend;
        fixed_backend.ln(x)
    }

    #[inline]
    fn log10(&self, x: fixed::types::I16F16) -> fixed::types::I16F16 {
        let fixed_backend = FixedPointBackend;
        fixed_backend.log10(x)
    }

    #[inline]
    fn sin(&self, x: fixed::types::I16F16) -> fixed::types::I16F16 {
        // Use CORDIC for trigonometric functions
        let (sin_val, _cos_val) = cordic::sin_cos(x);
        sin_val
    }

    #[inline]
    fn cos(&self, x: fixed::types::I16F16) -> fixed::types::I16F16 {
        let (_sin_val, cos_val) = cordic::sin_cos(x);
        cos_val
    }

    #[inline]
    fn tan(&self, x: fixed::types::I16F16) -> fixed::types::I16F16 {
        let sin_x = self.sin(x);
        let cos_x = self.cos(x);

        if cos_x.abs() < fixed::types::I16F16::from_num(0.001) {
            if sin_x >= fixed::types::I16F16::ZERO {
                fixed::types::I16F16::from_num(100.0)
            } else {
                fixed::types::I16F16::from_num(-100.0)
            }
        } else {
            sin_x / cos_x
        }
    }

    #[inline]
    fn to_radians(&self, degrees: fixed::types::I16F16) -> fixed::types::I16F16 {
        let pi = fixed::types::I16F16::from_num(core::f32::consts::PI);
        degrees * pi / fixed::types::I16F16::from_num(180.0)
    }

    #[inline]
    fn to_degrees(&self, radians: fixed::types::I16F16) -> fixed::types::I16F16 {
        let pi = fixed::types::I16F16::from_num(core::f32::consts::PI);
        radians * fixed::types::I16F16::from_num(180.0) / pi
    }

    #[inline]
    fn atan2(&self, y: fixed::types::I16F16, x: fixed::types::I16F16) -> fixed::types::I16F16 {
        // Use the fixed-point backend for atan2
        let fixed_backend = FixedPointBackend;
        fixed_backend.atan2(y, x)
    }
}

// Default backend selection with priority order
#[cfg(feature = "floating-point")]
pub use self::FloatingPointBackend as DefaultBackend;

#[cfg(all(feature = "libm-math", not(feature = "floating-point")))]
pub use self::LibmBackend as DefaultBackend;

#[cfg(all(
    feature = "fixed-point",
    not(any(feature = "floating-point", feature = "libm-math"))
))]
pub use self::FixedPointBackend as DefaultBackend;

#[cfg(all(
    feature = "cordic-math",
    not(any(
        feature = "floating-point",
        feature = "libm-math",
        feature = "fixed-point"
    ))
))]
pub use self::CordicBackend as DefaultBackend;

#[cfg(all(
    feature = "integer-math",
    not(any(
        feature = "floating-point",
        feature = "libm-math",
        feature = "fixed-point",
        feature = "cordic-math"
    ))
))]
pub use self::IntegerBackend as DefaultBackend;

// Final fallback for when no features are explicitly enabled
#[cfg(not(any(
    feature = "floating-point",
    feature = "libm-math",
    feature = "fixed-point",
    feature = "cordic-math",
    feature = "integer-math"
)))]
/// Fallback math backend that provides basic implementations without external dependencies
pub struct FallbackBackend;

#[cfg(not(any(
    feature = "floating-point",
    feature = "libm-math",
    feature = "fixed-point",
    feature = "cordic-math",
    feature = "integer-math"
)))]
impl MathBackend<f32> for FallbackBackend {
    fn sqrt(&self, x: f32) -> f32 {
        // Basic integer square root approximation
        if x <= 0.0 {
            return 0.0;
        }
        let x_int = x as i32;
        if x_int <= 0 {
            return 0.0;
        }

        // Simple integer sqrt using binary search
        let mut left = 0i32;
        let mut right = x_int;
        let mut result = 0i32;

        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_squared = mid.saturating_mul(mid);

            if mid_squared <= x_int {
                result = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        result as f32
    }
    fn abs(&self, x: f32) -> f32 {
        if x < 0.0 {
            -x
        } else {
            x
        }
    }
    fn min(&self, a: f32, b: f32) -> f32 {
        if a < b {
            a
        } else {
            b
        }
    }
    fn max(&self, a: f32, b: f32) -> f32 {
        if a > b {
            a
        } else {
            b
        }
    }
    fn floor(&self, x: f32) -> f32 {
        (x as i32) as f32
    }
    fn ceil(&self, x: f32) -> f32 {
        let int_part = x as i32;
        if x > int_part as f32 {
            (int_part + 1) as f32
        } else {
            int_part as f32
        }
    }
    fn pow(&self, x: f32, y: f32) -> f32 {
        // Simple integer power approximation
        if y == 0.0 {
            return 1.0;
        }
        if y == 1.0 {
            return x;
        }
        if y == 2.0 {
            return x * x;
        }
        // For other powers, return x for simplicity in fallback
        x
    }
    fn ln(&self, _x: f32) -> f32 {
        0.0
    } // Stub implementation
    fn log10(&self, _x: f32) -> f32 {
        0.0
    } // Stub implementation
    fn sin(&self, _x: f32) -> f32 {
        0.0
    } // Stub implementation
    fn cos(&self, _x: f32) -> f32 {
        1.0
    } // Stub implementation
    fn tan(&self, _x: f32) -> f32 {
        0.0
    } // Stub implementation
    fn atan2(&self, _y: f32, _x: f32) -> f32 {
        0.0
    } // Stub implementation
    fn to_radians(&self, degrees: f32) -> f32 {
        degrees * 0.017453292
    } // Simple approximation
    fn to_degrees(&self, radians: f32) -> f32 {
        radians * 57.29578
    } // Simple approximation
}

#[cfg(not(any(
    feature = "floating-point",
    feature = "libm-math",
    feature = "fixed-point",
    feature = "cordic-math",
    feature = "integer-math"
)))]
pub use self::FallbackBackend as DefaultBackend;
