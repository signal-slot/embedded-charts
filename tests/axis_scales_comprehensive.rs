//! Comprehensive tests for axis scale transformations

#![cfg(feature = "floating-point")]

use embedded_charts::{
    axes::scale::{
        AxisScale, AxisScaleType, LinearScale, LogarithmicScale, ScaleConfig, ScaleTransform,
    },
    error::{ChartError, ChartResult},
};

// Helper to check approximate equality
fn approx_eq(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() < epsilon
}

mod linear_scale_tests {
    use super::*;

    #[test]
    fn test_linear_scale_creation() {
        let config = ScaleConfig {
            min: 0.0,
            max: 100.0,
            ..Default::default()
        };

        let scale = LinearScale::new(config).unwrap();
        assert!(scale.transform(0.0).is_ok());
        assert!(scale.transform(100.0).is_ok());
    }

    #[test]
    fn test_linear_scale_invalid_range() {
        let config = ScaleConfig {
            min: 100.0,
            max: 0.0,
            ..Default::default()
        };

        assert!(matches!(
            LinearScale::new(config),
            Err(ChartError::InvalidRange)
        ));
    }

    #[test]
    fn test_linear_scale_transform() {
        let config = ScaleConfig {
            min: 0.0,
            max: 100.0,
            ..Default::default()
        };

        let scale = LinearScale::new(config).unwrap();

        // Test exact values
        assert_eq!(scale.transform(0.0).unwrap(), 0.0);
        assert_eq!(scale.transform(50.0).unwrap(), 0.5);
        assert_eq!(scale.transform(100.0).unwrap(), 1.0);

        // Test out of range values (should be clamped)
        assert_eq!(scale.transform(-10.0).unwrap(), 0.0);
        assert_eq!(scale.transform(110.0).unwrap(), 1.0);
    }

    #[test]
    fn test_linear_scale_inverse() {
        let config = ScaleConfig {
            min: 0.0,
            max: 100.0,
            ..Default::default()
        };

        let scale = LinearScale::new(config).unwrap();

        assert_eq!(scale.inverse(0.0).unwrap(), 0.0);
        assert_eq!(scale.inverse(0.5).unwrap(), 50.0);
        assert_eq!(scale.inverse(1.0).unwrap(), 100.0);

        // Test invalid normalized values
        assert!(scale.inverse(-0.1).is_err());
        assert!(scale.inverse(1.1).is_err());
    }

    #[test]
    fn test_linear_scale_negative_range() {
        let config = ScaleConfig {
            min: -50.0,
            max: 50.0,
            ..Default::default()
        };

        let scale = LinearScale::new(config).unwrap();

        assert_eq!(scale.transform(-50.0).unwrap(), 0.0);
        assert_eq!(scale.transform(0.0).unwrap(), 0.5);
        assert_eq!(scale.transform(50.0).unwrap(), 1.0);
    }

    #[test]
    fn test_linear_scale_ticks() {
        let config = ScaleConfig {
            min: 0.0,
            max: 100.0,
            ..Default::default()
        };

        let scale = LinearScale::new(config).unwrap();

        // Test various tick counts
        let ticks = scale.get_ticks(0).unwrap();
        assert_eq!(ticks.len(), 0);

        let ticks = scale.get_ticks(1).unwrap();
        assert_eq!(ticks.len(), 1);
        assert_eq!(ticks[0], 50.0);

        let ticks = scale.get_ticks(5).unwrap();
        assert_eq!(ticks.len(), 5);
        assert_eq!(ticks[0], 0.0);
        assert_eq!(ticks[1], 25.0);
        assert_eq!(ticks[2], 50.0);
        assert_eq!(ticks[3], 75.0);
        assert_eq!(ticks[4], 100.0);
    }

    #[test]
    fn test_linear_scale_format_value() {
        let config = ScaleConfig::default();
        let scale = LinearScale::new(config).unwrap();

        assert_eq!(scale.format_value(0.0).as_str(), "0");
        assert_eq!(scale.format_value(1234.0).as_str(), "1.2k");
        assert_eq!(scale.format_value(0.123).as_str(), "0.12");
        assert_eq!(scale.format_value(0.001).as_str(), "1.0e-3");
    }

    #[test]
    fn test_linear_scale_nan_and_inf() {
        let config = ScaleConfig {
            min: 0.0,
            max: 100.0,
            ..Default::default()
        };

        let scale = LinearScale::new(config).unwrap();

        assert!(matches!(
            scale.transform(f32::NAN),
            Err(ChartError::InvalidData)
        ));
        assert!(matches!(
            scale.transform(f32::INFINITY),
            Err(ChartError::InvalidData)
        ));
        assert!(matches!(
            scale.transform(f32::NEG_INFINITY),
            Err(ChartError::InvalidData)
        ));
    }
}

#[cfg(feature = "std")]
mod logarithmic_scale_tests {
    use super::*;

    #[test]
    fn test_log_scale_creation() {
        let config = ScaleConfig {
            min: 1.0,
            max: 1000.0,
            ..Default::default()
        };

        // Test base 10
        let scale = LogarithmicScale::base10(config).unwrap();
        assert!(scale.transform(1.0).is_ok());
        assert!(scale.transform(1000.0).is_ok());

        // Test natural log
        let scale = LogarithmicScale::natural(config).unwrap();
        assert!(scale.transform(1.0).is_ok());
        assert!(scale.transform(1000.0).is_ok());

        // Test custom base
        let scale = LogarithmicScale::new(config, 2.0).unwrap();
        assert!(scale.transform(1.0).is_ok());
        assert!(scale.transform(1000.0).is_ok());
    }

    #[test]
    fn test_log_scale_invalid_range() {
        // Test negative values
        let config = ScaleConfig {
            min: -1.0,
            max: 100.0,
            ..Default::default()
        };
        assert!(LogarithmicScale::base10(config).is_err());

        // Test zero min
        let config = ScaleConfig {
            min: 0.0,
            max: 100.0,
            ..Default::default()
        };
        assert!(LogarithmicScale::base10(config).is_err());

        // Test invalid base
        let config = ScaleConfig {
            min: 1.0,
            max: 100.0,
            ..Default::default()
        };
        assert!(LogarithmicScale::new(config, 1.0).is_err());
        assert!(LogarithmicScale::new(config, 0.0).is_err());
        assert!(LogarithmicScale::new(config, -2.0).is_err());
    }

    #[test]
    fn test_log10_scale_transform() {
        let config = ScaleConfig {
            min: 1.0,
            max: 1000.0,
            ..Default::default()
        };

        let scale = LogarithmicScale::base10(config).unwrap();

        // Test exact powers of 10
        assert!(approx_eq(scale.transform(1.0).unwrap(), 0.0, 0.001));
        assert!(approx_eq(scale.transform(10.0).unwrap(), 0.333, 0.01));
        assert!(approx_eq(scale.transform(100.0).unwrap(), 0.667, 0.01));
        assert!(approx_eq(scale.transform(1000.0).unwrap(), 1.0, 0.001));

        // Test intermediate values
        assert!(scale.transform(5.0).unwrap() > 0.0);
        assert!(scale.transform(5.0).unwrap() < 0.333);
        assert!(scale.transform(500.0).unwrap() > 0.667);
        assert!(scale.transform(500.0).unwrap() < 1.0);
    }

    #[test]
    fn test_log_scale_invalid_values() {
        let config = ScaleConfig {
            min: 1.0,
            max: 1000.0,
            ..Default::default()
        };

        let scale = LogarithmicScale::base10(config).unwrap();

        // Test negative and zero values
        assert!(scale.transform(0.0).is_err());
        assert!(scale.transform(-1.0).is_err());

        // Test NaN and infinity
        assert!(scale.transform(f32::NAN).is_err());
        assert!(scale.transform(f32::INFINITY).is_err());
    }

    #[test]
    fn test_log_scale_inverse() {
        let config = ScaleConfig {
            min: 1.0,
            max: 1000.0,
            ..Default::default()
        };

        let scale = LogarithmicScale::base10(config).unwrap();

        // Test round-trip conversion
        for value in [1.0, 10.0, 100.0, 1000.0] {
            let normalized = scale.transform(value).unwrap();
            let restored = scale.inverse(normalized).unwrap();
            assert!(approx_eq(value, restored, 0.001));
        }

        // Test invalid normalized values
        assert!(scale.inverse(-0.1).is_err());
        assert!(scale.inverse(1.1).is_err());
    }

    #[test]
    fn test_log_scale_ticks() {
        let config = ScaleConfig {
            min: 1.0,
            max: 1000.0,
            ..Default::default()
        };

        let scale = LogarithmicScale::base10(config).unwrap();
        let ticks = scale.get_ticks(10).unwrap();

        // Should include at least the main powers of 10 within range
        assert!(ticks.contains(&1.0));
        assert!(ticks.contains(&10.0));

        // For a range 1-1000, we should get multiple ticks including intermediate values
        assert!(ticks.len() >= 4);

        // Check that ticks are in ascending order
        for i in 1..ticks.len() {
            assert!(ticks[i] > ticks[i - 1]);
        }

        // Check that all ticks are within range
        for &tick in &ticks {
            assert!((1.0..=1000.0).contains(&tick));
        }
    }

    #[test]
    fn test_log_scale_format_value() {
        let config = ScaleConfig {
            min: 0.001,
            max: 1000000.0,
            ..Default::default()
        };

        let scale = LogarithmicScale::base10(config).unwrap();

        // Test power of 10 formatting
        assert_eq!(scale.format_value(0.001).as_str(), "10^-3");
        assert_eq!(scale.format_value(1.0).as_str(), "10^0");
        assert_eq!(scale.format_value(10.0).as_str(), "10^1");
        assert_eq!(scale.format_value(100.0).as_str(), "10^2");
        assert_eq!(scale.format_value(1000.0).as_str(), "1000");

        // Test non-power of 10 formatting
        assert_eq!(scale.format_value(50.0).as_str(), "50.0");
        assert_eq!(scale.format_value(1500.0).as_str(), "1500");
    }

    #[test]
    fn test_natural_log_scale() {
        let config = ScaleConfig {
            min: 1.0,
            max: core::f32::consts::E.powi(3), // e^3
            ..Default::default()
        };

        let scale = LogarithmicScale::natural(config).unwrap();

        // Test exact powers of e
        assert!(approx_eq(scale.transform(1.0).unwrap(), 0.0, 0.001));
        assert!(approx_eq(
            scale.transform(core::f32::consts::E).unwrap(),
            0.333,
            0.01
        ));
        assert!(approx_eq(
            scale.transform(core::f32::consts::E.powi(2)).unwrap(),
            0.667,
            0.01
        ));
        assert!(approx_eq(
            scale.transform(core::f32::consts::E.powi(3)).unwrap(),
            1.0,
            0.001
        ));
    }
}

mod axis_scale_enum_tests {
    use super::*;

    #[test]
    fn test_axis_scale_creation() {
        let config = ScaleConfig {
            min: 0.0,
            max: 100.0,
            ..Default::default()
        };

        // Test linear scale
        let scale = AxisScale::new(AxisScaleType::Linear, config).unwrap();
        assert!(matches!(scale, AxisScale::Linear(_)));

        // Test log scales (only if std feature is enabled)
        #[cfg(feature = "std")]
        {
            let config = ScaleConfig {
                min: 1.0,
                max: 1000.0,
                ..Default::default()
            };

            let scale = AxisScale::new(AxisScaleType::Log10, config).unwrap();
            assert!(matches!(scale, AxisScale::Logarithmic(_)));

            let scale = AxisScale::new(AxisScaleType::LogE, config).unwrap();
            assert!(matches!(scale, AxisScale::Logarithmic(_)));

            let scale = AxisScale::new(AxisScaleType::LogBase(2.0), config).unwrap();
            assert!(matches!(scale, AxisScale::Logarithmic(_)));
        }

        // Test custom scale requires functions
        assert!(matches!(
            AxisScale::new(AxisScaleType::Custom, config),
            Err(ChartError::InvalidConfiguration)
        ));
    }

    #[test]
    fn test_axis_scale_transform() {
        let config = ScaleConfig {
            min: 0.0,
            max: 100.0,
            ..Default::default()
        };

        let scale = AxisScale::new(AxisScaleType::Linear, config).unwrap();

        assert_eq!(scale.transform(0.0).unwrap(), 0.0);
        assert_eq!(scale.transform(50.0).unwrap(), 0.5);
        assert_eq!(scale.transform(100.0).unwrap(), 1.0);
    }

    #[test]
    fn test_axis_scale_inverse() {
        let config = ScaleConfig {
            min: 0.0,
            max: 100.0,
            ..Default::default()
        };

        let scale = AxisScale::new(AxisScaleType::Linear, config).unwrap();

        assert_eq!(scale.inverse(0.0).unwrap(), 0.0);
        assert_eq!(scale.inverse(0.5).unwrap(), 50.0);
        assert_eq!(scale.inverse(1.0).unwrap(), 100.0);
    }

    #[test]
    fn test_axis_scale_ticks() {
        let config = ScaleConfig {
            min: 0.0,
            max: 100.0,
            ..Default::default()
        };

        let scale = AxisScale::new(AxisScaleType::Linear, config).unwrap();
        let ticks = scale.get_ticks(5).unwrap();

        assert_eq!(ticks.len(), 5);
        assert_eq!(ticks[0], 0.0);
        assert_eq!(ticks[4], 100.0);
    }

    #[test]
    fn test_axis_scale_format() {
        let config = ScaleConfig {
            min: 0.0,
            max: 10000.0,
            ..Default::default()
        };

        let scale = AxisScale::new(AxisScaleType::Linear, config).unwrap();

        assert_eq!(scale.format_value(0.0).as_str(), "0");
        assert_eq!(scale.format_value(1234.0).as_str(), "1.2k");
        assert_eq!(scale.format_value(10000.0).as_str(), "10.0k");
    }
}

mod scale_config_tests {
    use super::*;

    #[test]
    fn test_scale_config_default() {
        let config = ScaleConfig::default();

        assert_eq!(config.min, 0.0);
        assert_eq!(config.max, 100.0);
        assert!(!config.include_zero);
        assert!(config.nice_bounds);
    }

    #[test]
    fn test_scale_config_custom() {
        let config = ScaleConfig {
            min: -50.0,
            max: 50.0,
            include_zero: true,
            nice_bounds: false,
        };

        assert_eq!(config.min, -50.0);
        assert_eq!(config.max, 50.0);
        assert!(config.include_zero);
        assert!(!config.nice_bounds);
    }
}

#[cfg(feature = "std")]
mod custom_scale_tests {
    use super::*;
    use embedded_charts::axes::scale::CustomScale;

    #[test]
    fn test_custom_scale_sqrt() {
        let config = ScaleConfig {
            min: 0.0,
            max: 100.0,
            ..Default::default()
        };

        // Create a square root scale
        let scale = CustomScale::new(
            config,
            |value: f32| -> ChartResult<f32> {
                if value < 0.0 {
                    return Err(ChartError::InvalidData);
                }
                let normalized = value.sqrt() / 10.0; // sqrt(100) = 10
                Ok(normalized.clamp(0.0, 1.0))
            },
            |normalized: f32| -> ChartResult<f32> {
                if !(0.0..=1.0).contains(&normalized) {
                    return Err(ChartError::InvalidRange);
                }
                let value = (normalized * 10.0).powi(2);
                Ok(value)
            },
        );

        // Test transform
        assert_eq!(scale.transform(0.0).unwrap(), 0.0);
        assert_eq!(scale.transform(25.0).unwrap(), 0.5);
        assert_eq!(scale.transform(100.0).unwrap(), 1.0);

        // Test inverse
        assert_eq!(scale.inverse(0.0).unwrap(), 0.0);
        assert_eq!(scale.inverse(0.5).unwrap(), 25.0);
        assert_eq!(scale.inverse(1.0).unwrap(), 100.0);
    }

    #[test]
    fn test_custom_scale_ticks() {
        let config = ScaleConfig {
            min: 0.0,
            max: 100.0,
            ..Default::default()
        };

        let scale = CustomScale::new(
            config,
            |value: f32| Ok(value / 100.0),
            |normalized: f32| Ok(normalized * 100.0),
        );

        // Custom scales use linear tick generation
        let ticks = scale.get_ticks(5).unwrap();
        assert_eq!(ticks.len(), 5);
        assert_eq!(ticks[0], 0.0);
        assert_eq!(ticks[2], 50.0);
        assert_eq!(ticks[4], 100.0);
    }
}

mod axis_scale_type_tests {
    use super::*;

    #[test]
    fn test_axis_scale_type_default() {
        assert_eq!(AxisScaleType::default(), AxisScaleType::Linear);
    }

    #[test]
    fn test_axis_scale_type_equality() {
        assert_eq!(AxisScaleType::Linear, AxisScaleType::Linear);
        assert_eq!(AxisScaleType::Log10, AxisScaleType::Log10);
        assert_eq!(AxisScaleType::LogBase(2.0), AxisScaleType::LogBase(2.0));
        assert_ne!(AxisScaleType::Linear, AxisScaleType::Log10);
        assert_ne!(AxisScaleType::LogBase(2.0), AxisScaleType::LogBase(3.0));
    }
}
