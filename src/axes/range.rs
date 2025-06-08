//! Axis range calculation utilities.
//!
//! This module provides functions for calculating intuitive and visually appealing
//! axis ranges based on data bounds. The goal is to create axis ranges that:
//! - Start from meaningful values (often 0 for positive data)
//! - Extend slightly beyond the data to provide visual breathing room
//! - Use "nice" step sizes that are easy to read (1, 2, 5, 10, etc.)
//! - Accommodate proper tick placement

use crate::data::DataBounds;

/// Configuration for axis range calculation
#[derive(Debug, Clone, Copy)]
pub struct RangeCalculationConfig {
    /// Target number of major ticks (default: 5)
    pub target_tick_count: usize,
    /// Threshold for starting from zero (default: 0.3)
    /// If min value is less than max * threshold, start from 0
    pub zero_threshold: f32,
    /// Margin factor for negative data (default: 1.1)
    pub negative_margin: f32,
    /// Margin factor for data far from zero (default: 0.9)
    pub far_from_zero_margin: f32,
}

impl Default for RangeCalculationConfig {
    fn default() -> Self {
        Self {
            target_tick_count: 5,
            zero_threshold: 0.3,
            negative_margin: 1.1,
            far_from_zero_margin: 0.9,
        }
    }
}

/// Calculate a nice axis range for a single dimension
///
/// This function takes a minimum and maximum value and returns a "nice" range
/// that is visually appealing and mathematically convenient.
///
/// # Arguments
///
/// * `min` - Minimum data value
/// * `max` - Maximum data value
/// * `config` - Configuration for range calculation
///
/// # Returns
///
/// A tuple of (nice_min, nice_max) representing the calculated axis range
///
/// # Examples
///
/// ```rust
/// use embedded_charts::axes::range::{calculate_nice_range, RangeCalculationConfig};
///
/// // Data from 8.0 to 35.0 -> Nice range from 0.0 to 40.0
/// let (min, max) = calculate_nice_range(8.0, 35.0, RangeCalculationConfig::default());
/// assert_eq!(min, 0.0);
/// assert_eq!(max, 40.0);
///
/// // Data from 0.0 to 9.0 -> Nice range from 0.0 to 10.0
/// let (min, max) = calculate_nice_range(0.0, 9.0, RangeCalculationConfig::default());
/// assert_eq!(min, 0.0);
/// assert_eq!(max, 10.0);
/// ```
pub fn calculate_nice_range(min: f32, max: f32, config: RangeCalculationConfig) -> (f32, f32) {
    if max <= min {
        // Handle edge case where max <= min
        if min == max {
            if min == 0.0 {
                return (0.0, 1.0);
            } else if min > 0.0 {
                return (0.0, min * 1.2);
            } else {
                return (min * 1.2, 0.0);
            }
        } else {
            return (max, min); // Swap them
        }
    }

    // For positive data, prefer starting from 0 for better context
    let nice_min = if min >= 0.0 && max > 0.0 {
        // If minimum is positive and relatively small compared to max, start from 0
        if min <= max * config.zero_threshold {
            0.0
        } else {
            // Data far from zero - round down to nice value
            #[cfg(feature = "std")]
            let magnitude = 10.0_f32.powf((min * config.far_from_zero_margin).log10().floor());
            #[cfg(all(
                not(feature = "std"),
                any(feature = "floating-point", feature = "libm-math")
            ))]
            let magnitude = {
                use micromath::F32Ext;
                10.0_f32.powf((min * config.far_from_zero_margin).log10().floor())
            };
            #[cfg(not(any(feature = "std", feature = "floating-point", feature = "libm-math")))]
            let magnitude = 1.0; // Simplified for fixed-point and integer math

            #[cfg(feature = "std")]
            let result = (min * config.far_from_zero_margin / magnitude).floor() * magnitude;
            #[cfg(all(
                not(feature = "std"),
                any(feature = "floating-point", feature = "libm-math")
            ))]
            let result = {
                use micromath::F32Ext;
                (min * config.far_from_zero_margin / magnitude).floor() * magnitude
            };
            #[cfg(not(any(feature = "std", feature = "floating-point", feature = "libm-math")))]
            let result = min * config.far_from_zero_margin; // Simplified for fixed-point and integer math
            result
        }
    } else {
        // Negative data - add margin
        min * config.negative_margin
    };

    // Calculate nice maximum that accommodates the next tick beyond data
    let data_range = max - nice_min;
    let rough_step = data_range / config.target_tick_count as f32;

    // Round step to nice values
    #[cfg(feature = "std")]
    let magnitude = 10.0_f32.powf(rough_step.log10().floor());
    #[cfg(all(
        not(feature = "std"),
        any(feature = "floating-point", feature = "libm-math")
    ))]
    let magnitude = {
        use micromath::F32Ext;
        10.0_f32.powf(rough_step.log10().floor())
    };
    #[cfg(not(any(feature = "std", feature = "floating-point", feature = "libm-math")))]
    let magnitude = 1.0; // Simplified for fixed-point and integer math

    let normalized_step = rough_step / magnitude;
    let nice_step = if normalized_step <= 1.0 {
        magnitude
    } else if normalized_step <= 2.0 {
        2.0 * magnitude
    } else if normalized_step <= 5.0 {
        5.0 * magnitude
    } else {
        10.0 * magnitude
    };

    // Find the first tick at or beyond max
    #[cfg(feature = "std")]
    let ticks_from_min = ((max - nice_min) / nice_step).ceil();
    #[cfg(all(
        not(feature = "std"),
        any(feature = "floating-point", feature = "libm-math")
    ))]
    let ticks_from_min = {
        use micromath::F32Ext;
        ((max - nice_min) / nice_step).ceil()
    };
    #[cfg(not(any(feature = "std", feature = "floating-point", feature = "libm-math")))]
    let ticks_from_min = ((max - nice_min) / nice_step + 0.5) as i32 as f32; // Simple ceiling for fixed-point and integer math
    let nice_max = nice_min + (ticks_from_min * nice_step);

    (nice_min, nice_max)
}

/// Calculate nice axis ranges for both X and Y axes from data bounds
///
/// This is a convenience function that applies nice range calculation to both
/// dimensions of a data bounds object.
///
/// # Arguments
///
/// * `bounds` - Data bounds containing min/max for both X and Y
/// * `config` - Configuration for range calculation (applied to both axes)
///
/// # Returns
///
/// A tuple of ((x_min, x_max), (y_min, y_max)) representing nice ranges for both axes
///
/// # Examples
///
/// ```rust
/// use embedded_charts::prelude::*;
/// use embedded_charts::axes::range::{calculate_nice_ranges_from_bounds, RangeCalculationConfig};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut series: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
/// series.push(Point2D::new(0.0, 8.0))?;
/// series.push(Point2D::new(9.0, 35.0))?;
///
/// let bounds = series.bounds()?;
/// let ((x_min, x_max), (y_min, y_max)) = calculate_nice_ranges_from_bounds(
///     &bounds,
///     RangeCalculationConfig::default()
/// );
///
/// assert_eq!((x_min, x_max), (0.0, 10.0));
/// assert_eq!((y_min, y_max), (0.0, 40.0));
/// # Ok(())
/// # }
/// ```
pub fn calculate_nice_ranges_from_bounds<X, Y>(
    bounds: &DataBounds<X, Y>,
    config: RangeCalculationConfig,
) -> ((f32, f32), (f32, f32))
where
    X: Into<f32> + Copy + PartialOrd,
    Y: Into<f32> + Copy + PartialOrd,
{
    let x_range = calculate_nice_range(bounds.min_x.into(), bounds.max_x.into(), config);
    let y_range = calculate_nice_range(bounds.min_y.into(), bounds.max_y.into(), config);
    (x_range, y_range)
}

/// Calculate nice axis ranges with separate configurations for X and Y axes
///
/// This function allows different configurations for X and Y axes, which can be
/// useful when the axes have different characteristics (e.g., time on X-axis,
/// values on Y-axis).
///
/// # Arguments
///
/// * `bounds` - Data bounds containing min/max for both X and Y
/// * `x_config` - Configuration for X-axis range calculation
/// * `y_config` - Configuration for Y-axis range calculation
///
/// # Returns
///
/// A tuple of ((x_min, x_max), (y_min, y_max)) representing nice ranges for both axes
pub fn calculate_nice_ranges_separate_config<X, Y>(
    bounds: &DataBounds<X, Y>,
    x_config: RangeCalculationConfig,
    y_config: RangeCalculationConfig,
) -> ((f32, f32), (f32, f32))
where
    X: Into<f32> + Copy + PartialOrd,
    Y: Into<f32> + Copy + PartialOrd,
{
    let x_range = calculate_nice_range(bounds.min_x.into(), bounds.max_x.into(), x_config);
    let y_range = calculate_nice_range(bounds.min_y.into(), bounds.max_y.into(), y_config);
    (x_range, y_range)
}

/// Preset configurations for common use cases
pub mod presets {
    use super::RangeCalculationConfig;

    /// Standard configuration - good for most charts
    pub fn standard() -> RangeCalculationConfig {
        RangeCalculationConfig::default()
    }

    /// Tight configuration - minimal padding around data
    pub fn tight() -> RangeCalculationConfig {
        RangeCalculationConfig {
            target_tick_count: 4,
            zero_threshold: 0.1,
            negative_margin: 1.05,
            far_from_zero_margin: 0.95,
        }
    }

    /// Loose configuration - more padding around data
    pub fn loose() -> RangeCalculationConfig {
        RangeCalculationConfig {
            target_tick_count: 6,
            zero_threshold: 0.5,
            negative_margin: 1.2,
            far_from_zero_margin: 0.8,
        }
    }

    /// Time series configuration - optimized for time-based data
    pub fn time_series() -> RangeCalculationConfig {
        RangeCalculationConfig {
            target_tick_count: 6,
            zero_threshold: 0.0, // Time rarely starts from 0
            negative_margin: 1.1,
            far_from_zero_margin: 0.9,
        }
    }

    /// Percentage configuration - optimized for percentage data (0-100)
    pub fn percentage() -> RangeCalculationConfig {
        RangeCalculationConfig {
            target_tick_count: 5,
            zero_threshold: 1.0,  // Always start from 0 for percentages
            negative_margin: 1.0, // No negative values expected
            far_from_zero_margin: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_nice_range_positive_data() {
        let config = RangeCalculationConfig::default();

        // Data from 8 to 35 should give 0 to 40
        let (min, max) = calculate_nice_range(8.0, 35.0, config);
        assert_eq!(min, 0.0);
        assert_eq!(max, 40.0);

        // Data from 0 to 9 should give 0 to 10
        let (min, max) = calculate_nice_range(0.0, 9.0, config);
        assert_eq!(min, 0.0);
        assert_eq!(max, 10.0);
    }

    #[test]
    fn test_calculate_nice_range_large_positive_data() {
        let config = RangeCalculationConfig::default();

        // Data from 100 to 150 (min > max * 0.3) should not start from 0
        let (min, max) = calculate_nice_range(100.0, 150.0, config);
        assert!(min > 0.0);
        assert!(min < 100.0);
        assert!(max >= 150.0);
    }

    #[test]
    fn test_calculate_nice_range_negative_data() {
        let config = RangeCalculationConfig::default();

        // Negative data should have appropriate margins
        let (min, max) = calculate_nice_range(-50.0, -10.0, config);
        assert!(min < -50.0);
        assert!(max >= -10.0);
    }

    #[test]
    fn test_calculate_nice_range_edge_cases() {
        let config = RangeCalculationConfig::default();

        // Equal values
        let (min, max) = calculate_nice_range(5.0, 5.0, config);
        assert!(min <= 5.0);
        assert!(max >= 5.0);
        assert!(max > min);

        // Zero values
        let (min, max) = calculate_nice_range(0.0, 0.0, config);
        assert_eq!(min, 0.0);
        assert_eq!(max, 1.0);
    }

    #[test]
    fn test_preset_configurations() {
        // Test that presets create different configurations
        let standard = presets::standard();
        let tight = presets::tight();
        let loose = presets::loose();

        assert_eq!(standard.target_tick_count, 5);
        assert_eq!(tight.target_tick_count, 4);
        assert_eq!(loose.target_tick_count, 6);

        // Test that they produce different results
        let (min1, max1) = calculate_nice_range(8.0, 35.0, tight);
        let (min2, max2) = calculate_nice_range(8.0, 35.0, loose);

        // Loose should generally give larger ranges
        assert!((max2 - min2) >= (max1 - min1));
    }
}
