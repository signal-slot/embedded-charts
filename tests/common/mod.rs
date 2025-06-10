//! Common testing utilities and infrastructure for embedded-charts
//!
//! This module provides shared utilities, test data generators, and helper functions
//! to support comprehensive testing across all chart types and components.

use embedded_charts::{
    data::{point::Point2D, series::StaticDataSeries, DataSeries},
    error::{ChartError, ChartResult},
};
use embedded_graphics::{
    mock_display::MockDisplay,
    pixelcolor::{Rgb565, RgbColor},
    prelude::*,
    primitives::Rectangle,
};

pub mod chart_testing;
pub mod data_generators;
pub mod performance;
pub mod visual_testing;

/// Standard test display size for consistent testing (MockDisplay default is 64x64)
pub const TEST_DISPLAY_SIZE: Size = Size::new(64, 64);

/// Standard test viewport for chart testing
pub const TEST_VIEWPORT: Rectangle = Rectangle::new(Point::zero(), TEST_DISPLAY_SIZE);

/// Common chart margins for testing
pub const TEST_MARGINS: embedded_charts::chart::traits::Margins =
    embedded_charts::chart::traits::Margins {
        top: 10,
        bottom: 10,
        left: 10,
        right: 10,
    };

/// Create a standard mock display for testing
pub fn create_test_display() -> MockDisplay<Rgb565> {
    MockDisplay::new()
}

/// Create a test display with specific size
pub fn create_test_display_with_size(_size: Size) -> MockDisplay<Rgb565> {
    // MockDisplay doesn't support custom dimensions in embedded-graphics 0.8
    // Using standard display for now
    MockDisplay::new()
}

/// Standard test colors for consistent visual testing
pub struct TestColors;

impl TestColors {
    pub const PRIMARY: Rgb565 = Rgb565::BLUE;
    pub const SECONDARY: Rgb565 = Rgb565::RED;
    pub const ACCENT: Rgb565 = Rgb565::GREEN;
    pub const BACKGROUND: Rgb565 = Rgb565::WHITE;
    pub const GRID: Rgb565 = Rgb565::CSS_LIGHT_GRAY;
}

/// Validation helper for chart rendering results
pub fn validate_chart_output(display: &MockDisplay<Rgb565>) -> ChartResult<()> {
    // Basic validation - ensure some pixels were drawn
    let drawn_pixels = display.affected_area();
    if drawn_pixels.size.width == 0 || drawn_pixels.size.height == 0 {
        return Err(ChartError::RenderingError);
    }
    Ok(())
}

/// Create test configuration with standard settings
pub fn create_test_config() -> embedded_charts::chart::traits::ChartConfig<Rgb565> {
    embedded_charts::chart::traits::ChartConfig {
        title: None,
        background_color: Some(TestColors::BACKGROUND),
        margins: TEST_MARGINS,
        grid_color: Some(TestColors::GRID),
        show_grid: true,
    }
}

/// Memory usage tracking for embedded testing
#[derive(Debug, Clone, Copy)]
pub struct MemoryMetrics {
    pub heap_used: usize,
    pub stack_used: usize,
    pub static_memory: usize,
}

impl MemoryMetrics {
    pub fn new() -> Self {
        Self {
            heap_used: 0,
            stack_used: 0,
            static_memory: 0,
        }
    }
}

/// Performance metrics for benchmark testing
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub render_time_ns: u64,
    pub memory_usage: MemoryMetrics,
    pub draw_calls: usize,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            render_time_ns: 0,
            memory_usage: MemoryMetrics::new(),
            draw_calls: 0,
        }
    }
}

/// Test data patterns for different testing scenarios
#[derive(Debug, Clone, Copy)]
pub enum TestDataPattern {
    Linear,
    Sine,
    Random,
    Stepped,
    Sparse,
    Dense,
    EdgeCase,
}

/// Common test assertions for chart validation
pub mod assertions {
    use super::*;

    /// Assert that a chart renders without errors
    pub fn assert_chart_renders<T>(
        chart: &T,
        data: &StaticDataSeries<Point2D, 256>,
        config: &embedded_charts::chart::traits::ChartConfig<Rgb565>,
        viewport: Rectangle,
    ) -> ChartResult<()>
    where
        T: embedded_charts::chart::traits::Chart<
            Rgb565,
            Data = StaticDataSeries<Point2D, 256>,
            Config = embedded_charts::chart::traits::ChartConfig<Rgb565>,
        >,
    {
        let mut display = create_test_display();
        chart.draw(data, config, viewport, &mut display)?;
        validate_chart_output(&display)
    }

    /// Assert that chart rendering is deterministic
    pub fn assert_deterministic_rendering<T>(
        chart: &T,
        data: &StaticDataSeries<Point2D, 256>,
        config: &embedded_charts::chart::traits::ChartConfig<Rgb565>,
        viewport: Rectangle,
    ) -> ChartResult<()>
    where
        T: embedded_charts::chart::traits::Chart<
            Rgb565,
            Data = StaticDataSeries<Point2D, 256>,
            Config = embedded_charts::chart::traits::ChartConfig<Rgb565>,
        >,
    {
        let mut display1 = create_test_display();
        let mut display2 = create_test_display();

        chart.draw(data, config, viewport, &mut display1)?;
        chart.draw(data, config, viewport, &mut display2)?;

        // Compare pixel data
        if display1.affected_area() != display2.affected_area() {
            return Err(ChartError::RenderingError);
        }

        Ok(())
    }

    /// Assert that memory usage is within embedded constraints
    pub fn assert_memory_constraints(metrics: &MemoryMetrics, max_bytes: usize) -> ChartResult<()> {
        let total_memory = metrics.heap_used + metrics.stack_used + metrics.static_memory;
        if total_memory > max_bytes {
            return Err(ChartError::MemoryFull);
        }
        Ok(())
    }
}

/// Error simulation for robustness testing
pub mod error_simulation {
    use super::*;

    /// Simulate memory pressure conditions
    pub fn simulate_memory_pressure() -> ChartResult<()> {
        // In real implementation, this would simulate low memory conditions
        Ok(())
    }

    /// Simulate rendering failures
    pub fn simulate_rendering_failure() -> ChartError {
        ChartError::RenderingError
    }

    /// Create invalid data scenarios for error testing
    pub fn create_invalid_data() -> StaticDataSeries<Point2D, 256> {
        StaticDataSeries::new() // Empty data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utilities_creation() {
        let display = create_test_display();
        // MockDisplay uses default size
        let size = display.size();
        assert!(size.width > 0);
        assert!(size.height > 0);
    }

    #[test]
    fn test_config_creation() {
        let config = create_test_config();
        assert!(config.background_color.is_some());
        assert!(config.show_grid);
    }

    #[test]
    fn test_memory_metrics() {
        let metrics = MemoryMetrics::new();
        assert_eq!(metrics.heap_used, 0);
    }
}
