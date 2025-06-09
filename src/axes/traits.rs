//! Core traits for axis implementations.

use crate::axes::{AxisOrientation, AxisPosition};
use crate::error::ChartResult;
use crate::math::{Math, NumericConversion};
use embedded_graphics::{prelude::*, primitives::Rectangle};

/// Core trait for all axis types
pub trait Axis<T, C: PixelColor> {
    /// The tick generator type for this axis
    type TickGenerator: TickGenerator<T>;
    /// The style type for this axis
    type Style;

    /// Get the minimum value of the axis
    fn min(&self) -> T;

    /// Get the maximum value of the axis
    fn max(&self) -> T;

    /// Get the axis orientation
    fn orientation(&self) -> AxisOrientation;

    /// Get the axis position
    fn position(&self) -> AxisPosition;

    /// Transform a data value to screen coordinate
    ///
    /// # Arguments
    /// * `value` - The data value to transform
    /// * `viewport` - The available drawing area
    fn transform_value(&self, value: T, viewport: Rectangle) -> i32;

    /// Transform a screen coordinate back to data value
    ///
    /// # Arguments
    /// * `coordinate` - The screen coordinate
    /// * `viewport` - The available drawing area
    fn inverse_transform(&self, coordinate: i32, viewport: Rectangle) -> T;

    /// Get the tick generator for this axis
    fn tick_generator(&self) -> &Self::TickGenerator;

    /// Get the style configuration
    fn style(&self) -> &Self::Style;

    /// Draw the axis to the target
    ///
    /// # Arguments
    /// * `viewport` - The area to draw the axis in
    /// * `target` - The display target to draw to
    fn draw<D>(&self, viewport: Rectangle, target: &mut D) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>;

    /// Calculate the space required for this axis (labels, ticks, etc.)
    fn required_space(&self) -> u32;
}

/// Trait for generating tick marks and labels
pub trait TickGenerator<T> {
    /// Generate tick positions for the given range
    ///
    /// # Arguments
    /// * `min` - Minimum value of the range
    /// * `max` - Maximum value of the range
    /// * `max_ticks` - Maximum number of ticks to generate
    fn generate_ticks(&self, min: T, max: T, max_ticks: usize) -> heapless::Vec<Tick<T>, 32>;

    /// Get the preferred number of ticks
    fn preferred_tick_count(&self) -> usize;

    /// Set the preferred number of ticks
    fn set_preferred_tick_count(&mut self, count: usize);
}

/// Trait for rendering axis components
pub trait AxisRenderer<C: PixelColor> {
    /// Draw the main axis line
    ///
    /// # Arguments
    /// * `start` - Start point of the axis line
    /// * `end` - End point of the axis line
    /// * `style` - Line style to use
    /// * `target` - The display target to draw to
    fn draw_axis_line<D>(
        &self,
        start: Point,
        end: Point,
        style: &crate::style::LineStyle<C>,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>;

    /// Draw a tick mark
    ///
    /// # Arguments
    /// * `position` - Position of the tick mark
    /// * `length` - Length of the tick mark
    /// * `orientation` - Orientation of the axis
    /// * `style` - Line style to use
    /// * `target` - The display target to draw to
    fn draw_tick<D>(
        &self,
        position: Point,
        length: u32,
        orientation: AxisOrientation,
        style: &crate::style::LineStyle<C>,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>;

    /// Draw a grid line
    ///
    /// # Arguments
    /// * `start` - Start point of the grid line
    /// * `end` - End point of the grid line
    /// * `style` - Line style to use
    /// * `target` - The display target to draw to
    fn draw_grid_line<D>(
        &self,
        start: Point,
        end: Point,
        style: &crate::style::LineStyle<C>,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>;

    /// Draw a label
    ///
    /// # Arguments
    /// * `text` - The text to draw
    /// * `position` - Position to draw the label
    /// * `target` - The display target to draw to
    fn draw_label<D>(&self, text: &str, position: Point, target: &mut D) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>;
}

/// Represents a single tick mark on an axis
#[derive(Debug, Clone, PartialEq)]
pub struct Tick<T> {
    /// The value at this tick position
    pub value: T,
    /// Whether this is a major tick (with label) or minor tick
    pub is_major: bool,
    /// Optional label for this tick
    pub label: Option<heapless::String<16>>,
}

impl<T> Tick<T> {
    /// Create a new major tick with a label
    pub fn major(value: T, label: &str) -> Self {
        Self {
            value,
            is_major: true,
            label: heapless::String::try_from(label).ok(),
        }
    }

    /// Create a new minor tick without a label
    pub fn minor(value: T) -> Self {
        Self {
            value,
            is_major: false,
            label: None,
        }
    }

    /// Create a new major tick without a label
    pub fn major_unlabeled(value: T) -> Self {
        Self {
            value,
            is_major: true,
            label: None,
        }
    }
}

/// Trait for types that can be used as axis values
pub trait AxisValue: Copy + PartialOrd + core::fmt::Display {
    /// Convert to f32 for calculations
    fn to_f32(self) -> f32;

    /// Create from f32
    fn from_f32(value: f32) -> Self;

    /// Get a nice step size for this value type
    fn nice_step(range: Self) -> Self;

    /// Format this value for display
    fn format(&self) -> heapless::String<16>;
}

impl AxisValue for f32 {
    fn to_f32(self) -> f32 {
        self
    }

    fn from_f32(value: f32) -> Self {
        value
    }

    fn nice_step(range: Self) -> Self {
        let range_num = range.to_number();
        let abs_range = Math::abs(range_num);
        let magnitude = Math::floor(Math::log10(abs_range));
        let ten = 10.0f32.to_number();
        let normalized = range_num / Math::pow(ten, magnitude);

        let one = 1.0f32.to_number();
        let two = 2.0f32.to_number();
        let five = 5.0f32.to_number();
        let ten_norm = 10.0f32.to_number();

        let nice_normalized = if normalized <= one {
            one
        } else if normalized <= two {
            two
        } else if normalized <= five {
            five
        } else {
            ten_norm
        };

        let result = if magnitude >= 0.0.to_number() && magnitude <= 10.0.to_number() {
            nice_normalized * Math::pow(ten, magnitude)
        } else {
            // Fallback for extreme magnitudes to prevent overflow
            nice_normalized
        };
        f32::from_number(result)
    }

    fn format(&self) -> heapless::String<16> {
        // Simple formatting for no_std
        let self_num = self.to_number();
        let fract_part = self_num - Math::floor(self_num);
        let zero = 0.0f32.to_number();

        if fract_part == zero {
            // Integer formatting
            let int_val = *self as i32;
            let mut result = heapless::String::new();
            if int_val == 0 {
                let _ = result.push('0');
            } else {
                let mut val = int_val.abs();
                let mut digits = heapless::Vec::<u8, 16>::new();
                while val > 0 {
                    let _ = digits.push((val % 10) as u8 + b'0');
                    val /= 10;
                }
                if int_val < 0 {
                    let _ = result.push('-');
                }
                for &digit in digits.iter().rev() {
                    let _ = result.push(digit as char);
                }
            }
            result
        } else {
            // For floating point, just show as integer for simplicity in no_std
            let int_val = *self as i32;
            let mut result = heapless::String::new();
            if int_val == 0 {
                let _ = result.push('0');
            } else {
                let mut val = int_val.abs();
                let mut digits = heapless::Vec::<u8, 16>::new();
                while val > 0 {
                    let _ = digits.push((val % 10) as u8 + b'0');
                    val /= 10;
                }
                if int_val < 0 {
                    let _ = result.push('-');
                }
                for &digit in digits.iter().rev() {
                    let _ = result.push(digit as char);
                }
            }
            result
        }
    }
}

impl AxisValue for i32 {
    fn to_f32(self) -> f32 {
        self as f32
    }

    fn from_f32(value: f32) -> Self {
        let value_num = value.to_number();
        let rounded = Math::floor(value_num + 0.5f32.to_number());
        f32::from_number(rounded) as i32
    }

    fn nice_step(range: Self) -> Self {
        let range_f32 = range.abs() as f32;
        let range_num = range_f32.to_number();
        let magnitude = Math::floor(Math::log10(range_num));
        let ten = 10.0f32.to_number();
        let normalized = range_num / Math::pow(ten, magnitude);

        let one = 1.0f32.to_number();
        let two = 2.0f32.to_number();
        let five = 5.0f32.to_number();
        let ten_norm = 10.0f32.to_number();

        let nice_normalized = if normalized <= one {
            one
        } else if normalized <= two {
            two
        } else if normalized <= five {
            five
        } else {
            ten_norm
        };

        let result = if magnitude >= 0.0.to_number() && magnitude <= 10.0.to_number() {
            nice_normalized * Math::pow(ten, magnitude)
        } else {
            // Fallback for extreme magnitudes to prevent overflow
            nice_normalized
        };
        let rounded = Math::floor(result + 0.5f32.to_number());
        f32::from_number(rounded) as i32
    }

    fn format(&self) -> heapless::String<16> {
        let mut result = heapless::String::new();
        if *self == 0 {
            let _ = result.push('0');
        } else {
            let mut val = self.abs();
            let mut digits = heapless::Vec::<u8, 16>::new();
            while val > 0 {
                let _ = digits.push((val % 10) as u8 + b'0');
                val /= 10;
            }
            if *self < 0 {
                let _ = result.push('-');
            }
            for &digit in digits.iter().rev() {
                let _ = result.push(digit as char);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick_creation() {
        let major_tick = Tick::major(5.0, "5.0");
        assert!(major_tick.is_major);
        assert_eq!(major_tick.value, 5.0);
        assert!(major_tick.label.is_some());

        let minor_tick = Tick::minor(2.5);
        assert!(!minor_tick.is_major);
        assert_eq!(minor_tick.value, 2.5);
        assert!(minor_tick.label.is_none());
    }

    #[test]
    #[cfg(not(any(feature = "fixed-point", feature = "integer-math")))] // Skip for fixed-point and integer-math to avoid overflow
    fn test_axis_value_f32() {
        let value = core::f32::consts::PI;
        assert_eq!(value.to_f32(), core::f32::consts::PI);
        assert_eq!(f32::from_f32(core::f32::consts::PI), core::f32::consts::PI);

        let step = f32::nice_step(7.3);
        assert!(step > 0.0);
    }

    #[test]
    #[cfg(not(any(feature = "fixed-point", feature = "integer-math")))] // Skip for fixed-point and integer-math to avoid overflow
    fn test_axis_value_i32() {
        let value = 42i32;
        assert_eq!(value.to_f32(), 42.0);
        assert_eq!(i32::from_f32(42.7), 43);

        let step = i32::nice_step(73);
        assert!(step > 0);
    }
}
