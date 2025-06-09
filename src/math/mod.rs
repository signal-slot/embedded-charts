//! Math abstraction layer for no_std compatibility.
//!
//! This module provides trait-based math operations that can be backed by different
//! implementations depending on the target environment and feature flags:
//!
//! - `floating-point`: Full floating-point math using micromath
//! - `libm-math`: Alternative floating-point using libm
//! - `fixed-point`: Fixed-point arithmetic using the fixed crate
//! - `integer-math`: Integer-only math for the most constrained environments
//! - `cordic-math`: CORDIC-based trigonometric functions

pub mod backends;
pub mod traits;

// Re-export the main traits
pub use traits::{FloatLike, MathOps, TrigOps};

// Re-export backend implementations
pub use backends::*;

/// Primary numeric type used throughout the library
#[cfg(feature = "floating-point")]
pub type Number = f32;

#[cfg(all(feature = "fixed-point", not(feature = "floating-point")))]
/// Primary numeric type for fixed-point math backend
pub type Number = fixed::types::I16F16;

#[cfg(all(
    feature = "integer-math",
    not(any(feature = "floating-point", feature = "fixed-point"))
))]
/// Primary numeric type for integer-only math backend
pub type Number = i32;

#[cfg(not(any(
    feature = "floating-point",
    feature = "fixed-point",
    feature = "integer-math"
)))]
/// Default numeric type for mathematical operations
pub type Number = f32;

/// Math operations provider - selects the appropriate backend based on features
pub struct Math;

impl Math {
    /// Get the appropriate math backend for the current feature configuration
    #[cfg(feature = "floating-point")]
    pub fn backend() -> backends::FloatingPointBackend {
        backends::FloatingPointBackend
    }

    #[cfg(all(feature = "libm-math", not(feature = "floating-point")))]
    pub fn backend() -> backends::LibmBackend {
        backends::LibmBackend
    }

    #[cfg(all(
        feature = "fixed-point",
        not(any(feature = "floating-point", feature = "libm-math"))
    ))]
    /// Get the fixed-point math backend instance
    pub fn backend() -> backends::FixedPointBackend {
        backends::FixedPointBackend
    }

    #[cfg(all(
        feature = "cordic-math",
        not(any(
            feature = "floating-point",
            feature = "libm-math",
            feature = "fixed-point"
        ))
    ))]
    pub fn backend() -> backends::CordicBackend {
        backends::CordicBackend
    }

    #[cfg(all(
        feature = "integer-math",
        not(any(
            feature = "floating-point",
            feature = "libm-math",
            feature = "fixed-point",
            feature = "cordic-math"
        ))
    ))]
    /// Get the integer math backend for constrained environments
    pub fn backend() -> backends::IntegerBackend {
        backends::IntegerBackend
    }

    #[cfg(not(any(
        feature = "floating-point",
        feature = "libm-math",
        feature = "fixed-point",
        feature = "cordic-math",
        feature = "integer-math"
    )))]
    /// Get the fallback math backend when no specific feature is enabled
    pub fn backend() -> backends::FallbackBackend {
        backends::FallbackBackend
    }
}

/// Convenience functions for common math operations
impl Math {
    /// Calculate the square root of a number
    #[inline]
    pub fn sqrt(x: Number) -> Number {
        use crate::math::traits::MathBackend;
        Self::backend().sqrt(x)
    }

    /// Calculate the absolute value of a number
    #[inline]
    pub fn abs(x: Number) -> Number {
        use crate::math::traits::MathBackend;
        Self::backend().abs(x)
    }

    /// Calculate the minimum of two numbers
    #[inline]
    pub fn min(a: Number, b: Number) -> Number {
        use crate::math::traits::MathBackend;
        Self::backend().min(a, b)
    }

    /// Calculate the maximum of two numbers
    #[inline]
    pub fn max(a: Number, b: Number) -> Number {
        use crate::math::traits::MathBackend;
        Self::backend().max(a, b)
    }

    /// Calculate the floor of a number
    #[inline]
    pub fn floor(x: Number) -> Number {
        use crate::math::traits::MathBackend;
        Self::backend().floor(x)
    }

    /// Calculate the ceiling of a number
    #[inline]
    pub fn ceil(x: Number) -> Number {
        use crate::math::traits::MathBackend;
        Self::backend().ceil(x)
    }

    /// Calculate x raised to the power of y
    #[inline]
    pub fn pow(x: Number, y: Number) -> Number {
        use crate::math::traits::MathBackend;
        Self::backend().pow(x, y)
    }

    /// Calculate the natural logarithm
    #[inline]
    pub fn ln(x: Number) -> Number {
        use crate::math::traits::MathBackend;
        Self::backend().ln(x)
    }

    /// Calculate the base-10 logarithm
    #[inline]
    pub fn log10(x: Number) -> Number {
        use crate::math::traits::MathBackend;
        Self::backend().log10(x)
    }

    /// Calculate the sine of an angle in radians
    #[inline]
    pub fn sin(x: Number) -> Number {
        use crate::math::traits::MathBackend;
        Self::backend().sin(x)
    }

    /// Calculate the cosine of an angle in radians
    #[inline]
    pub fn cos(x: Number) -> Number {
        use crate::math::traits::MathBackend;
        Self::backend().cos(x)
    }

    /// Calculate the tangent of an angle in radians
    #[inline]
    pub fn tan(x: Number) -> Number {
        use crate::math::traits::MathBackend;
        Self::backend().tan(x)
    }

    /// Convert degrees to radians
    #[inline]
    pub fn to_radians(degrees: Number) -> Number {
        use crate::math::traits::MathBackend;
        Self::backend().to_radians(degrees)
    }

    /// Convert radians to degrees
    #[inline]
    pub fn to_degrees(radians: Number) -> Number {
        use crate::math::traits::MathBackend;
        Self::backend().to_degrees(radians)
    }

    /// Calculate atan2(y, x) - the angle from the positive x-axis to the point (x, y)
    #[inline]
    pub fn atan2(y: Number, x: Number) -> Number {
        use crate::math::traits::MathBackend;
        Self::backend().atan2(y, x)
    }
}

/// Type conversion utilities for different numeric types
pub trait NumericConversion<T> {
    /// Convert from the source type to Number
    fn to_number(self) -> Number;
    /// Convert from Number to the target type
    fn from_number(n: Number) -> T;
}

// Implement conversions for common types
impl NumericConversion<f32> for f32 {
    #[inline]
    fn to_number(self) -> Number {
        #[cfg(feature = "floating-point")]
        return self;

        #[cfg(all(feature = "fixed-point", not(feature = "floating-point")))]
        return fixed::types::I16F16::from_num(self);

        #[cfg(all(
            feature = "integer-math",
            not(any(feature = "floating-point", feature = "fixed-point"))
        ))]
        return (self * 1000.0) as i32; // Scale by 1000 for integer representation

        #[cfg(not(any(
            feature = "floating-point",
            feature = "fixed-point",
            feature = "integer-math"
        )))]
        return self;
    }

    #[inline]
    fn from_number(n: Number) -> f32 {
        #[cfg(feature = "floating-point")]
        return n;

        #[cfg(all(feature = "fixed-point", not(feature = "floating-point")))]
        return n.to_num();

        #[cfg(all(
            feature = "integer-math",
            not(any(feature = "floating-point", feature = "fixed-point"))
        ))]
        return n as f32 / 1000.0; // Unscale from integer representation

        #[cfg(not(any(
            feature = "floating-point",
            feature = "fixed-point",
            feature = "integer-math"
        )))]
        return n;
    }
}

impl NumericConversion<i32> for i32 {
    #[inline]
    fn to_number(self) -> Number {
        #[cfg(feature = "floating-point")]
        return self as f32;

        #[cfg(all(feature = "fixed-point", not(feature = "floating-point")))]
        return fixed::types::I16F16::from_num(self);

        #[cfg(all(
            feature = "integer-math",
            not(any(feature = "floating-point", feature = "fixed-point"))
        ))]
        return self * 1000; // Scale by 1000 for precision

        #[cfg(not(any(
            feature = "floating-point",
            feature = "fixed-point",
            feature = "integer-math"
        )))]
        return self as f32;
    }

    #[inline]
    fn from_number(n: Number) -> i32 {
        #[cfg(feature = "floating-point")]
        return n as i32;

        #[cfg(all(feature = "fixed-point", not(feature = "floating-point")))]
        return n.to_num();

        #[cfg(all(
            feature = "integer-math",
            not(any(feature = "floating-point", feature = "fixed-point"))
        ))]
        return n / 1000; // Unscale from integer representation

        #[cfg(not(any(
            feature = "floating-point",
            feature = "fixed-point",
            feature = "integer-math"
        )))]
        return n as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(not(feature = "integer-math"))] // Skip for integer-math to avoid precision issues
    fn test_basic_math_operations() {
        let a = 4.0f32.to_number();
        let b = 2.0f32.to_number();

        let sqrt_result = Math::sqrt(a);
        let min_result = Math::min(a, b);
        let max_result = Math::max(a, b);

        // Convert back to f32 for comparison
        assert!((f32::from_number(sqrt_result) - 2.0).abs() < 0.1);
        assert!((f32::from_number(min_result) - 2.0).abs() < 0.1);
        assert!((f32::from_number(max_result) - 4.0).abs() < 0.1);
    }

    #[test]
    fn test_trigonometric_functions() {
        let angle = 0.0f32.to_number();

        let sin_result = Math::sin(angle);
        let cos_result = Math::cos(angle);

        // Convert back to f32 for comparison
        assert!((f32::from_number(sin_result) - 0.0).abs() < 0.1);
        assert!((f32::from_number(cos_result) - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_numeric_conversions() {
        let original = core::f32::consts::PI;
        let converted = original.to_number();
        let back = f32::from_number(converted);

        // Should be approximately equal (allowing for precision loss in integer modes)
        assert!((original - back).abs() < 0.1);
    }
}
