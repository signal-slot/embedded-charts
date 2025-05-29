//! Math operation traits for different numeric backends.

use core::fmt::Debug;

/// Core mathematical operations that all numeric types should support
pub trait MathOps<T>: Copy + Clone + Debug {
    /// Calculate the square root
    fn sqrt(self) -> T;

    /// Calculate the absolute value
    fn abs(self) -> T;

    /// Return the minimum of two values
    fn min(self, other: T) -> T;

    /// Return the maximum of two values
    fn max(self, other: T) -> T;

    /// Calculate the floor (largest integer less than or equal to the value)
    fn floor(self) -> T;

    /// Calculate the ceiling (smallest integer greater than or equal to the value)
    fn ceil(self) -> T;

    /// Raise to a power
    fn pow(self, exp: T) -> T;

    /// Natural logarithm
    fn ln(self) -> T;

    /// Base-10 logarithm
    fn log10(self) -> T;
}

/// Trigonometric operations
pub trait TrigOps<T>: Copy + Clone + Debug {
    /// Sine function (input in radians)
    fn sin(self) -> T;

    /// Cosine function (input in radians)
    fn cos(self) -> T;

    /// Tangent function (input in radians)
    fn tan(self) -> T;

    /// Convert degrees to radians
    fn to_radians(self) -> T;

    /// Convert radians to degrees
    fn to_degrees(self) -> T;
}

/// Trait for types that can behave like floating-point numbers
pub trait FloatLike<T>: MathOps<T> + TrigOps<T> + PartialOrd + PartialEq {
    /// Zero value
    fn zero() -> T;

    /// One value
    fn one() -> T;

    /// Pi constant
    fn pi() -> T;

    /// Check if the value is finite
    fn is_finite(self) -> bool;

    /// Check if the value is NaN
    fn is_nan(self) -> bool;

    /// Check if the value is infinite
    fn is_infinite(self) -> bool;
}

/// Backend trait that provides math operations for a specific numeric type
pub trait MathBackend<T> {
    /// Calculate the square root
    fn sqrt(&self, x: T) -> T;

    /// Calculate the absolute value
    fn abs(&self, x: T) -> T;

    /// Return the minimum of two values
    fn min(&self, a: T, b: T) -> T;

    /// Return the maximum of two values
    fn max(&self, a: T, b: T) -> T;

    /// Calculate the floor
    fn floor(&self, x: T) -> T;

    /// Calculate the ceiling
    fn ceil(&self, x: T) -> T;

    /// Raise to a power
    fn pow(&self, x: T, y: T) -> T;

    /// Natural logarithm
    fn ln(&self, x: T) -> T;

    /// Base-10 logarithm
    fn log10(&self, x: T) -> T;

    /// Sine function
    fn sin(&self, x: T) -> T;

    /// Cosine function
    fn cos(&self, x: T) -> T;

    /// Tangent function
    fn tan(&self, x: T) -> T;

    /// Convert degrees to radians
    fn to_radians(&self, degrees: T) -> T;

    /// Convert radians to degrees
    fn to_degrees(&self, radians: T) -> T;

    /// Calculate atan2(y, x) - the angle from the positive x-axis to the point (x, y)
    fn atan2(&self, y: T, x: T) -> T;
}
