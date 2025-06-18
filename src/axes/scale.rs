//! Axis scale implementations for different transformation types

use crate::error::{ChartError, ChartResult};
use core::fmt::Debug;

// Import for no_std compatibility
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(feature = "std")]
use std::boxed::Box;

// Import math traits based on feature flags
#[cfg(all(feature = "floating-point", not(feature = "std")))]
use micromath::F32Ext;

/// Trait for axis scale transformations
pub trait ScaleTransform: Debug {
    /// Transform a data value to normalized coordinates [0, 1]
    fn transform(&self, value: f32) -> ChartResult<f32>;

    /// Inverse transform from normalized coordinates [0, 1] to data value
    fn inverse(&self, normalized: f32) -> ChartResult<f32>;

    /// Get nice tick values for this scale
    fn get_ticks(&self, count: usize) -> ChartResult<heapless::Vec<f32, 16>>;

    /// Format a value for display on this scale
    fn format_value(&self, value: f32) -> heapless::String<16>;
}

/// Configuration for axis scales
#[derive(Debug, Clone, Copy)]
pub struct ScaleConfig {
    /// Minimum value of the scale domain
    pub min: f32,
    /// Maximum value of the scale domain
    pub max: f32,
    /// Whether to include zero in the scale
    pub include_zero: bool,
    /// Whether to add padding to the scale bounds
    pub nice_bounds: bool,
}

impl Default for ScaleConfig {
    fn default() -> Self {
        Self {
            min: 0.0,
            max: 100.0,
            include_zero: false,
            nice_bounds: true,
        }
    }
}

/// Linear scale transformation
#[derive(Debug, Clone)]
pub struct LinearScale {
    config: ScaleConfig,
    range: f32,
}

impl LinearScale {
    /// Create a new linear scale
    pub fn new(config: ScaleConfig) -> ChartResult<Self> {
        if config.min >= config.max {
            return Err(ChartError::InvalidRange);
        }

        let range = config.max - config.min;
        Ok(Self { config, range })
    }
}

impl ScaleTransform for LinearScale {
    fn transform(&self, value: f32) -> ChartResult<f32> {
        if value.is_nan() || value.is_infinite() {
            return Err(ChartError::InvalidData);
        }

        let normalized = (value - self.config.min) / self.range;
        Ok(normalized.clamp(0.0, 1.0))
    }

    fn inverse(&self, normalized: f32) -> ChartResult<f32> {
        if !(0.0..=1.0).contains(&normalized) {
            return Err(ChartError::InvalidRange);
        }

        Ok(self.config.min + normalized * self.range)
    }

    fn get_ticks(&self, count: usize) -> ChartResult<heapless::Vec<f32, 16>> {
        let mut ticks = heapless::Vec::new();

        if count == 0 {
            return Ok(ticks);
        }

        if count == 1 {
            let _ = ticks.push((self.config.min + self.config.max) / 2.0);
            return Ok(ticks);
        }

        let step = self.range / (count - 1) as f32;
        for i in 0..count {
            let tick = self.config.min + (i as f32) * step;
            if ticks.push(tick).is_err() {
                break;
            }
        }

        Ok(ticks)
    }

    fn format_value(&self, value: f32) -> heapless::String<16> {
        let mut s = heapless::String::new();

        // Simple formatting logic
        if value == 0.0 {
            let _ = write!(s, "0");
        } else if value.abs() >= 1000.0 {
            let _ = write!(s, "{:.1}k", value / 1000.0);
        } else if value.abs() >= 1.0 {
            let _ = write!(s, "{value:.0}");
        } else if value.abs() >= 0.01 {
            let _ = write!(s, "{value:.2}");
        } else {
            let _ = write!(s, "{value:.1e}");
        }

        s
    }
}

/// Logarithmic scale transformation
#[derive(Debug, Clone)]
pub struct LogarithmicScale {
    config: ScaleConfig,
    base: f32,
    log_min: f32,
    #[allow(dead_code)]
    log_max: f32,
    log_range: f32,
}

impl LogarithmicScale {
    /// Create a new logarithmic scale with specified base
    pub fn new(config: ScaleConfig, base: f32) -> ChartResult<Self> {
        if config.min <= 0.0 || config.max <= 0.0 {
            return Err(ChartError::InvalidRange);
        }

        if config.min >= config.max {
            return Err(ChartError::InvalidRange);
        }

        if base <= 0.0 || base == 1.0 {
            return Err(ChartError::InvalidConfiguration);
        }

        #[cfg(feature = "std")]
        let (log_min, log_max) = (config.min.log(base), config.max.log(base));

        #[cfg(not(feature = "std"))]
        let (log_min, log_max) = (config.min.log(base), config.max.log(base));
        let log_range = log_max - log_min;

        Ok(Self {
            config,
            base,
            log_min,
            log_max,
            log_range,
        })
    }

    /// Create a logarithmic scale with base 10
    pub fn base10(config: ScaleConfig) -> ChartResult<Self> {
        Self::new(config, 10.0)
    }

    /// Create a logarithmic scale with base e (natural logarithm)
    pub fn natural(config: ScaleConfig) -> ChartResult<Self> {
        Self::new(config, core::f32::consts::E)
    }
}

impl ScaleTransform for LogarithmicScale {
    fn transform(&self, value: f32) -> ChartResult<f32> {
        if value <= 0.0 {
            return Err(ChartError::InvalidData);
        }

        if value.is_nan() || value.is_infinite() {
            return Err(ChartError::InvalidData);
        }

        #[cfg(feature = "std")]
        let log_value = value.log(self.base);

        #[cfg(not(feature = "std"))]
        let log_value = value.log(self.base);
        let normalized = (log_value - self.log_min) / self.log_range;
        Ok(normalized.clamp(0.0, 1.0))
    }

    fn inverse(&self, normalized: f32) -> ChartResult<f32> {
        if !(0.0..=1.0).contains(&normalized) {
            return Err(ChartError::InvalidRange);
        }

        let log_value = self.log_min + normalized * self.log_range;
        #[cfg(feature = "std")]
        let result = self.base.powf(log_value);

        #[cfg(not(feature = "std"))]
        let result = self.base.powf(log_value);

        Ok(result)
    }

    fn get_ticks(&self, _count: usize) -> ChartResult<heapless::Vec<f32, 16>> {
        let mut ticks = heapless::Vec::new();

        // Generate ticks at powers of the base
        #[cfg(feature = "std")]
        let start_power = self.config.min.log(self.base).floor();

        #[cfg(not(feature = "std"))]
        let start_power = self.config.min.log(self.base).floor();

        let mut power = start_power;

        // Generate up to 20 powers (to avoid infinite loop)
        for _ in 0..20 {
            #[cfg(feature = "std")]
            let value = self.base.powf(power);

            #[cfg(not(feature = "std"))]
            let value = self.base.powf(power);

            if value > self.config.max * 1.1 {
                // Add small tolerance
                break;
            }

            if value >= self.config.min * 0.9 && value <= self.config.max * 1.1 {
                // Round to avoid floating point precision issues
                let rounded = if self.base == 10.0 {
                    // For base 10, round to nearest power
                    let log_val = value.log10();
                    if (log_val - log_val.round()).abs() < 0.01 {
                        10.0_f32.powf(log_val.round())
                    } else {
                        value
                    }
                } else {
                    value
                };

                if rounded >= self.config.min && rounded <= self.config.max {
                    let _ = ticks.push(rounded);
                }
            }

            // Add intermediate ticks for base 10 only if we have space
            // and the power is small enough to avoid too many ticks
            if self.base == 10.0 && !ticks.is_full() && power < 3.0 {
                for i in 2..10 {
                    if ticks.is_full() {
                        break;
                    }
                    let intermediate = value * (i as f32);
                    if intermediate > self.config.max {
                        break;
                    }
                    if intermediate >= self.config.min {
                        let _ = ticks.push(intermediate);
                    }
                }
            }

            power += 1.0;
        }

        Ok(ticks)
    }

    fn format_value(&self, value: f32) -> heapless::String<16> {
        let mut s = heapless::String::new();

        if self.base == 10.0 {
            // For base 10, use scientific notation for round powers
            #[cfg(feature = "std")]
            let log_value = value.log10();

            #[cfg(not(feature = "std"))]
            let log_value = value.log10();
            if (log_value - log_value.round()).abs() < 0.01 && value < 1000.0 {
                let _ = write!(s, "10^{:.0}", log_value.round());
            } else if value >= 1000.0 {
                let _ = write!(s, "{value:.0}");
            } else {
                let _ = write!(s, "{value:.1}");
            }
        } else {
            // For other bases, use regular formatting
            if value >= 1000.0 {
                let _ = write!(s, "{value:.0}");
            } else if value >= 1.0 {
                let _ = write!(s, "{value:.1}");
            } else {
                let _ = write!(s, "{value:.2}");
            }
        }

        s
    }
}

/// Custom scale with user-defined transformation functions
pub struct CustomScale<F, I>
where
    F: Fn(f32) -> ChartResult<f32>,
    I: Fn(f32) -> ChartResult<f32>,
{
    config: ScaleConfig,
    transform_fn: F,
    inverse_fn: I,
}

impl<F, I> core::fmt::Debug for CustomScale<F, I>
where
    F: Fn(f32) -> ChartResult<f32>,
    I: Fn(f32) -> ChartResult<f32>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CustomScale")
            .field("config", &self.config)
            .field("transform_fn", &"<function>")
            .field("inverse_fn", &"<function>")
            .finish()
    }
}

impl<F, I> CustomScale<F, I>
where
    F: Fn(f32) -> ChartResult<f32>,
    I: Fn(f32) -> ChartResult<f32>,
{
    /// Create a new custom scale with user-defined functions
    pub fn new(config: ScaleConfig, transform_fn: F, inverse_fn: I) -> Self {
        Self {
            config,
            transform_fn,
            inverse_fn,
        }
    }
}

impl<F, I> ScaleTransform for CustomScale<F, I>
where
    F: Fn(f32) -> ChartResult<f32>,
    I: Fn(f32) -> ChartResult<f32>,
{
    fn transform(&self, value: f32) -> ChartResult<f32> {
        (self.transform_fn)(value)
    }

    fn inverse(&self, normalized: f32) -> ChartResult<f32> {
        (self.inverse_fn)(normalized)
    }

    fn get_ticks(&self, count: usize) -> ChartResult<heapless::Vec<f32, 16>> {
        // For custom scales, use linear tick generation in the domain
        LinearScale::new(self.config)?.get_ticks(count)
    }

    fn format_value(&self, value: f32) -> heapless::String<16> {
        // Use default formatting for custom scales
        let mut s = heapless::String::new();
        let _ = write!(s, "{value:.2}");
        s
    }
}

/// Enumeration of available scale types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AxisScaleType {
    /// Linear scale (default)
    Linear,
    /// Logarithmic scale with base 10
    Log10,
    /// Logarithmic scale with base e
    LogE,
    /// Logarithmic scale with custom base
    LogBase(f32),
    /// Custom scale (requires transformation functions)
    Custom,
}

impl Default for AxisScaleType {
    fn default() -> Self {
        Self::Linear
    }
}

/// Main axis scale container
#[derive(Debug)]
pub enum AxisScale {
    /// Linear scale transformation
    Linear(LinearScale),
    /// Logarithmic scale transformation
    Logarithmic(LogarithmicScale),
    /// Custom scale with user-defined transformation
    Custom(Box<dyn ScaleTransform>),
}

impl AxisScale {
    /// Create a new axis scale of the specified type
    pub fn new(scale_type: AxisScaleType, config: ScaleConfig) -> ChartResult<Self> {
        match scale_type {
            AxisScaleType::Linear => Ok(Self::Linear(LinearScale::new(config)?)),
            AxisScaleType::Log10 => Ok(Self::Logarithmic(LogarithmicScale::base10(config)?)),
            AxisScaleType::LogE => Ok(Self::Logarithmic(LogarithmicScale::natural(config)?)),
            AxisScaleType::LogBase(base) => {
                Ok(Self::Logarithmic(LogarithmicScale::new(config, base)?))
            }
            AxisScaleType::Custom => Err(ChartError::InvalidConfiguration),
        }
    }

    /// Transform a value using this scale
    pub fn transform(&self, value: f32) -> ChartResult<f32> {
        match self {
            Self::Linear(scale) => scale.transform(value),
            Self::Logarithmic(scale) => scale.transform(value),
            Self::Custom(scale) => scale.transform(value),
        }
    }

    /// Inverse transform a normalized value
    pub fn inverse(&self, normalized: f32) -> ChartResult<f32> {
        match self {
            Self::Linear(scale) => scale.inverse(normalized),
            Self::Logarithmic(scale) => scale.inverse(normalized),
            Self::Custom(scale) => scale.inverse(normalized),
        }
    }

    /// Get tick values for this scale
    pub fn get_ticks(&self, count: usize) -> ChartResult<heapless::Vec<f32, 16>> {
        match self {
            Self::Linear(scale) => scale.get_ticks(count),
            Self::Logarithmic(scale) => scale.get_ticks(count),
            Self::Custom(scale) => scale.get_ticks(count),
        }
    }

    /// Format a value for display
    pub fn format_value(&self, value: f32) -> heapless::String<16> {
        match self {
            Self::Linear(scale) => scale.format_value(value),
            Self::Logarithmic(scale) => scale.format_value(value),
            Self::Custom(scale) => scale.format_value(value),
        }
    }
}

// Helper for write! macro in no_std
use core::fmt::Write;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_scale() {
        let config = ScaleConfig {
            min: 0.0,
            max: 100.0,
            ..Default::default()
        };

        let scale = LinearScale::new(config).unwrap();

        // Test transform
        assert_eq!(scale.transform(0.0).unwrap(), 0.0);
        assert_eq!(scale.transform(50.0).unwrap(), 0.5);
        assert_eq!(scale.transform(100.0).unwrap(), 1.0);

        // Test inverse
        assert_eq!(scale.inverse(0.0).unwrap(), 0.0);
        assert_eq!(scale.inverse(0.5).unwrap(), 50.0);
        assert_eq!(scale.inverse(1.0).unwrap(), 100.0);

        // Test ticks
        let ticks = scale.get_ticks(5).unwrap();
        assert_eq!(ticks.len(), 5);
        assert_eq!(ticks[0], 0.0);
        assert_eq!(ticks[4], 100.0);
    }

    #[test]
    fn test_logarithmic_scale() {
        let config = ScaleConfig {
            min: 1.0,
            max: 1000.0,
            ..Default::default()
        };

        let scale = LogarithmicScale::base10(config).unwrap();

        // Test transform
        assert!((scale.transform(1.0).unwrap() - 0.0).abs() < 0.001);
        assert!((scale.transform(10.0).unwrap() - 0.333).abs() < 0.01);
        assert!((scale.transform(100.0).unwrap() - 0.667).abs() < 0.01);
        assert!((scale.transform(1000.0).unwrap() - 1.0).abs() < 0.001);

        // Test error for non-positive values
        assert!(scale.transform(0.0).is_err());
        assert!(scale.transform(-1.0).is_err());
    }
}
