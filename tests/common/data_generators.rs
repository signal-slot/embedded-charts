//! Test data generators for comprehensive chart testing
//!
//! Provides various data patterns and edge cases for testing different chart scenarios

use embedded_charts::data::{point::Point2D, series::StaticDataSeries, DataSeries};
use heapless::Vec;

use super::TestDataPattern;

/// Generate test data series with specified pattern and size
pub fn generate_test_data(pattern: TestDataPattern, size: usize) -> StaticDataSeries<Point2D, 256> {
    let mut series = StaticDataSeries::new();
    let size = size.min(256); // Respect capacity limits

    match pattern {
        TestDataPattern::Linear => {
            for i in 0..size {
                let x = i as f32;
                let y = x * 2.0 + 10.0;
                series.push(Point2D::new(x, y)).ok();
            }
        }
        TestDataPattern::Sine => {
            for i in 0..size {
                let x = i as f32 * 0.1;
                let y = (x * 2.0 * core::f32::consts::PI).sin() * 10.0 + 20.0;
                series.push(Point2D::new(x, y)).ok();
            }
        }
        TestDataPattern::Random => {
            // Pseudo-random for deterministic testing
            let mut seed = 12345u32;
            for i in 0..size {
                seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
                let x = i as f32;
                let y = (seed % 100) as f32;
                series.push(Point2D::new(x, y)).ok();
            }
        }
        TestDataPattern::Stepped => {
            for i in 0..size {
                let x = i as f32;
                let y = ((i / 5) * 10) as f32;
                series.push(Point2D::new(x, y)).ok();
            }
        }
        TestDataPattern::Sparse => {
            for i in (0..size).step_by(5) {
                let x = i as f32;
                let y = x + 10.0;
                series.push(Point2D::new(x, y)).ok();
            }
        }
        TestDataPattern::Dense => {
            for i in 0..size {
                let x = i as f32 * 0.1;
                let y = x + 10.0;
                series.push(Point2D::new(x, y)).ok();
            }
        }
        TestDataPattern::EdgeCase => {
            // Include edge cases like zero, negative, large values
            let test_points = [
                (0.0, 0.0),
                (-10.0, -5.0),
                (1000.0, 999.0),
                (f32::MIN, f32::MAX),
                (0.001, 0.001),
            ];
            for (x, y) in test_points.iter().take(size) {
                series.push(Point2D::new(*x, *y)).ok();
            }
        }
    }

    series
}

/// Generate temperature sensor data for realistic testing
pub fn generate_temperature_data(hours: usize) -> StaticDataSeries<Point2D, 256> {
    let mut series = StaticDataSeries::new();
    let hours = hours.min(256);

    for i in 0..hours {
        let hour = i as f32;
        // Simulate daily temperature variation
        let base_temp = 20.0;
        let daily_variation = 10.0 * (hour * 2.0 * core::f32::consts::PI / 24.0).sin();
        let random_noise = ((i * 17) % 5) as f32 - 2.0; // Small random variation
        let temperature = base_temp + daily_variation + random_noise;

        series.push(Point2D::new(hour, temperature)).ok();
    }

    series
}

/// Generate financial data (OHLC style) as price points
pub fn generate_stock_data(days: usize) -> StaticDataSeries<Point2D, 256> {
    let mut series = StaticDataSeries::new();
    let days = days.min(256);
    let mut price = 100.0;

    for i in 0..days {
        let day = i as f32;
        // Simple price walk with some volatility
        let change = ((i * 23) % 21) as f32 - 10.0; // -10 to +10
        price += change * 0.1;
        price = price.max(1.0); // Don't go negative

        series.push(Point2D::new(day, price)).ok();
    }

    series
}

/// Generate sensor data with periodic spikes
pub fn generate_sensor_data_with_spikes(readings: usize) -> StaticDataSeries<Point2D, 256> {
    let mut series = StaticDataSeries::new();
    let readings = readings.min(256);

    for i in 0..readings {
        let time = i as f32;
        let base_value = 50.0;

        // Add periodic spikes every 20 readings
        let spike = if i % 20 == 0 { 30.0 } else { 0.0 };

        // Add some noise
        let noise = ((i * 13) % 11) as f32 - 5.0;

        let value = base_value + spike + noise;
        series.push(Point2D::new(time, value)).ok();
    }

    series
}

/// Generate memory usage data (always increasing with plateaus)
pub fn generate_memory_usage_data(samples: usize) -> StaticDataSeries<Point2D, 256> {
    let mut series = StaticDataSeries::new();
    let samples = samples.min(256);
    let mut memory = 0.0;

    for i in 0..samples {
        let time = i as f32;

        // Memory generally increases with occasional garbage collection drops
        if i % 30 == 29 {
            memory *= 0.7; // GC event
        } else {
            memory += ((i % 5) + 1) as f32 * 0.5; // Variable allocation
        }

        series.push(Point2D::new(time, memory)).ok();
    }

    series
}

/// Generate edge case data for stress testing
pub fn generate_edge_case_data() -> Vec<StaticDataSeries<Point2D, 256>, 10> {
    let mut edge_cases = Vec::new();

    // Case 1: Empty data
    edge_cases.push(StaticDataSeries::new()).ok();

    // Case 2: Single point
    let mut single = StaticDataSeries::new();
    single.push(Point2D::new(0.0, 0.0)).ok();
    edge_cases.push(single).ok();

    // Case 3: Two identical points
    let mut identical = StaticDataSeries::new();
    identical.push(Point2D::new(5.0, 10.0)).ok();
    identical.push(Point2D::new(5.0, 10.0)).ok();
    edge_cases.push(identical).ok();

    // Case 4: Very large values
    let mut large = StaticDataSeries::new();
    large.push(Point2D::new(1e6, 1e6)).ok();
    large.push(Point2D::new(2e6, 2e6)).ok();
    edge_cases.push(large).ok();

    // Case 5: Very small values
    let mut small = StaticDataSeries::new();
    small.push(Point2D::new(1e-6, 1e-6)).ok();
    small.push(Point2D::new(2e-6, 2e-6)).ok();
    edge_cases.push(small).ok();

    // Case 6: Mixed positive/negative
    let mut mixed = StaticDataSeries::new();
    mixed.push(Point2D::new(-10.0, -5.0)).ok();
    mixed.push(Point2D::new(0.0, 0.0)).ok();
    mixed.push(Point2D::new(10.0, 5.0)).ok();
    edge_cases.push(mixed).ok();

    // Case 7: Flat line (all same Y)
    let mut flat = StaticDataSeries::new();
    for i in 0..5 {
        flat.push(Point2D::new(i as f32, 42.0)).ok();
    }
    edge_cases.push(flat).ok();

    // Case 8: Vertical line (all same X)
    let mut vertical = StaticDataSeries::new();
    for i in 0..5 {
        vertical.push(Point2D::new(10.0, i as f32)).ok();
    }
    edge_cases.push(vertical).ok();

    edge_cases
}

/// Generate multi-series test data
pub fn generate_multi_series_data(
    series_count: usize,
    points_per_series: usize,
) -> Vec<StaticDataSeries<Point2D, 256>, 8> {
    let mut multi_series = Vec::new();
    let series_count = series_count.min(8);

    for series_idx in 0..series_count {
        let mut series = StaticDataSeries::new();
        let points = points_per_series.min(256);

        for i in 0..points {
            let x = i as f32;
            // Each series has different characteristics
            let y = match series_idx {
                0 => x * 2.0,                        // Linear
                1 => (x * 0.1).sin() * 10.0 + 20.0,  // Sine
                2 => x.powi(2) * 0.1,                // Quadratic
                3 => 50.0 - x * 0.5,                 // Declining
                _ => x + (series_idx as f32 * 10.0), // Offset linear
            };
            series.push(Point2D::new(x, y)).ok();
        }

        multi_series.push(series).ok();
    }

    multi_series
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_test_data() {
        let data = generate_test_data(TestDataPattern::Linear, 10);
        assert_eq!(data.len(), 10);
    }

    #[test]
    fn test_temperature_data_generation() {
        let data = generate_temperature_data(24);
        assert_eq!(data.len(), 24);

        // Check that we have reasonable temperature values
        for point in data.iter() {
            assert!(point.y >= -20.0 && point.y <= 60.0);
        }
    }

    #[test]
    fn test_edge_case_data_generation() {
        let edge_cases = generate_edge_case_data();
        assert!(!edge_cases.is_empty());

        // First case should be empty
        assert_eq!(edge_cases[0].len(), 0);

        // Second case should have one point
        assert_eq!(edge_cases[1].len(), 1);
    }

    #[test]
    fn test_multi_series_generation() {
        let multi = generate_multi_series_data(3, 10);
        assert_eq!(multi.len(), 3);

        for series in multi.iter() {
            assert_eq!(series.len(), 10);
        }
    }
}
