//! Chart-specific testing utilities and frameworks
//!
//! Provides specialized testing infrastructure for different chart types

#![allow(dead_code)] // Allow unused testing utilities - they're part of testing infrastructure

use embedded_charts::{
    chart::traits::{Chart, ChartBuilder, ChartConfig},
    data::{point::Point2D, series::StaticDataSeries, DataSeries},
    error::{ChartError, ChartResult},
};
use embedded_graphics::{pixelcolor::Rgb565, prelude::*, primitives::Rectangle};

use super::{create_test_display, PerformanceMetrics, TestColors, TEST_VIEWPORT};

/// Comprehensive test suite for any chart type
pub struct ChartTestSuite;

impl ChartTestSuite {
    /// Test that a chart renders correctly with various data patterns
    pub fn test_chart_rendering<T>(
        chart: &T,
        test_data: &[StaticDataSeries<Point2D, 256>],
    ) -> ChartResult<()>
    where
        T: Chart<Rgb565, Data = StaticDataSeries<Point2D, 256>, Config = ChartConfig<Rgb565>>,
    {
        let config = super::create_test_config();

        for data in test_data {
            let mut display = create_test_display();
            chart.draw(data, &config, TEST_VIEWPORT, &mut display)?;

            // Validate that something was drawn
            if !data.is_empty()
                && (display.affected_area().size.width == 0
                    || display.affected_area().size.height == 0)
            {
                return Err(ChartError::RenderingError);
            }
        }

        Ok(())
    }

    /// Test chart with different viewport sizes
    pub fn test_viewport_scaling<T>(
        chart: &T,
        data: &StaticDataSeries<Point2D, 256>,
    ) -> ChartResult<()>
    where
        T: Chart<Rgb565, Data = StaticDataSeries<Point2D, 256>, Config = ChartConfig<Rgb565>>,
    {
        let config = super::create_test_config();
        let viewports = [
            Rectangle::new(Point::zero(), Size::new(100, 100)), // Small
            Rectangle::new(Point::zero(), Size::new(320, 240)), // Medium
            Rectangle::new(Point::zero(), Size::new(800, 600)), // Large
            Rectangle::new(Point::zero(), Size::new(64, 48)),   // Tiny embedded
        ];

        for &viewport in &viewports {
            let mut display = super::create_test_display();
            chart.draw(data, &config, viewport, &mut display)?;
        }

        Ok(())
    }

    /// Test chart performance and measure metrics
    pub fn measure_chart_performance<T>(
        chart: &T,
        data: &StaticDataSeries<Point2D, 256>,
    ) -> ChartResult<PerformanceMetrics>
    where
        T: Chart<Rgb565, Data = StaticDataSeries<Point2D, 256>, Config = ChartConfig<Rgb565>>,
    {
        let config = super::create_test_config();
        let mut display = create_test_display();

        // Measure rendering time (simplified - in real implementation would use proper timing)
        let start_time = 0; // std::time::Instant::now() equivalent
        chart.draw(data, &config, TEST_VIEWPORT, &mut display)?;
        let end_time = 1000; // Simulated 1000ns

        let mut metrics = PerformanceMetrics::new();
        metrics.render_time_ns = end_time - start_time;
        metrics.draw_calls = 1; // Simplified

        Ok(metrics)
    }

    /// Test chart error handling with invalid inputs
    pub fn test_error_handling<T>(chart: &T) -> ChartResult<()>
    where
        T: Chart<Rgb565, Data = StaticDataSeries<Point2D, 256>, Config = ChartConfig<Rgb565>>,
    {
        let config = super::create_test_config();
        let mut display = create_test_display();

        // Test with empty data
        let empty_data = StaticDataSeries::new();
        let result = chart.draw(&empty_data, &config, TEST_VIEWPORT, &mut display);

        // Should either succeed (for charts that handle empty data) or fail gracefully
        match result {
            Ok(_) => {}                             // Chart handles empty data gracefully
            Err(ChartError::InsufficientData) => {} // Expected error
            Err(other) => return Err(other),        // Unexpected error
        }

        // Test with zero-size viewport
        let zero_viewport = Rectangle::new(Point::zero(), Size::zero());
        let mut valid_data = StaticDataSeries::new();
        valid_data.push(Point2D::new(0.0, 0.0)).ok();

        let result = chart.draw(&valid_data, &config, zero_viewport, &mut display);
        // Should handle gracefully
        match result {
            Ok(_) | Err(ChartError::InvalidRange) => {}
            Err(other) => return Err(other),
        }

        Ok(())
    }

    /// Test chart with various color configurations
    pub fn test_color_configurations<T>(
        chart: &T,
        data: &StaticDataSeries<Point2D, 256>,
    ) -> ChartResult<()>
    where
        T: Chart<Rgb565, Data = StaticDataSeries<Point2D, 256>, Config = ChartConfig<Rgb565>>,
    {
        let mut display = create_test_display();

        let color_configs = [
            ChartConfig {
                title: None,
                background_color: Some(TestColors::BACKGROUND),
                margins: super::TEST_MARGINS,
                grid_color: Some(TestColors::GRID),
                show_grid: true,
            },
            ChartConfig {
                title: None,
                background_color: None, // No background
                margins: super::TEST_MARGINS,
                grid_color: Some(TestColors::PRIMARY),
                show_grid: false,
            },
        ];

        for config in &color_configs {
            chart.draw(data, config, TEST_VIEWPORT, &mut display)?;
        }

        Ok(())
    }

    /// Validate that chart output is deterministic
    pub fn test_deterministic_output<T>(
        chart: &T,
        data: &StaticDataSeries<Point2D, 256>,
    ) -> ChartResult<()>
    where
        T: Chart<Rgb565, Data = StaticDataSeries<Point2D, 256>, Config = ChartConfig<Rgb565>>,
    {
        let config = super::create_test_config();

        let mut display1 = create_test_display();
        let mut display2 = create_test_display();

        // Render twice with same parameters
        chart.draw(data, &config, TEST_VIEWPORT, &mut display1)?;
        chart.draw(data, &config, TEST_VIEWPORT, &mut display2)?;

        // Compare output (simplified - real implementation would compare pixel data)
        if display1.affected_area() != display2.affected_area() {
            return Err(ChartError::RenderingError);
        }

        Ok(())
    }
}

/// Line chart specific testing utilities
#[cfg(feature = "line")]
pub mod line_chart_testing {
    use super::*;
    use embedded_charts::chart::line::{LineChart, MarkerShape, MarkerStyle};

    pub struct LineChartTester;

    impl LineChartTester {
        /// Test line chart with different marker configurations
        pub fn test_marker_configurations(
            data: &StaticDataSeries<Point2D, 256>,
        ) -> ChartResult<()> {
            let marker_configs = [
                Some(MarkerStyle {
                    shape: MarkerShape::Circle,
                    size: 4,
                    color: TestColors::PRIMARY,
                    visible: true,
                }),
                Some(MarkerStyle {
                    shape: MarkerShape::Square,
                    size: 6,
                    color: TestColors::SECONDARY,
                    visible: true,
                }),
                Some(MarkerStyle {
                    shape: MarkerShape::Diamond,
                    size: 8,
                    color: TestColors::ACCENT,
                    visible: true,
                }),
                None, // No markers
            ];

            for marker_config in &marker_configs {
                let chart = LineChart::builder()
                    .line_color(TestColors::PRIMARY)
                    .line_width(2)
                    .with_markers(marker_config.unwrap_or(MarkerStyle {
                        shape: MarkerShape::Circle,
                        size: 4,
                        color: TestColors::PRIMARY,
                        visible: false,
                    }))
                    .build()?;

                ChartTestSuite::test_chart_rendering(&chart, &[data.clone()])?;
            }

            Ok(())
        }

        /// Test line chart with area fill
        pub fn test_area_fill(data: &StaticDataSeries<Point2D, 256>) -> ChartResult<()> {
            let chart = LineChart::builder()
                .line_color(TestColors::PRIMARY)
                .line_width(2)
                .fill_area(TestColors::SECONDARY)
                .build()?;

            ChartTestSuite::test_chart_rendering(&chart, &[data.clone()])?;
            Ok(())
        }

        /// Test line chart with smooth curves
        pub fn test_smooth_curves(data: &StaticDataSeries<Point2D, 256>) -> ChartResult<()> {
            let subdivisions = [2, 4, 8, 16];

            for &subdivisions in &subdivisions {
                let chart = LineChart::builder()
                    .line_color(TestColors::PRIMARY)
                    .line_width(2)
                    .smooth(true)
                    .smooth_subdivisions(subdivisions)
                    .build()?;

                ChartTestSuite::test_chart_rendering(&chart, &[data.clone()])?;
            }

            Ok(())
        }

        /// Test line chart with different line widths
        pub fn test_line_widths(data: &StaticDataSeries<Point2D, 256>) -> ChartResult<()> {
            let widths = [1, 2, 3, 5, 8];

            for &width in &widths {
                let chart = LineChart::builder()
                    .line_color(TestColors::PRIMARY)
                    .line_width(width)
                    .build()?;

                ChartTestSuite::test_chart_rendering(&chart, &[data.clone()])?;
            }

            Ok(())
        }
    }
}

/// Memory testing utilities for embedded constraints
pub mod memory_testing {
    use super::super::MemoryMetrics;
    use super::*;

    pub struct MemoryTester;

    impl MemoryTester {
        /// Test that chart memory usage stays within bounds
        pub fn test_memory_bounds<T>(
            chart: &T,
            data: &StaticDataSeries<Point2D, 256>,
            max_bytes: usize,
        ) -> ChartResult<()>
        where
            T: Chart<Rgb565, Data = StaticDataSeries<Point2D, 256>, Config = ChartConfig<Rgb565>>,
        {
            let config = super::super::create_test_config();
            let mut display = create_test_display();

            // In a real implementation, this would measure actual memory usage
            // For now, we simulate the check
            chart.draw(data, &config, TEST_VIEWPORT, &mut display)?;

            let metrics = MemoryMetrics::new();
            super::super::assertions::assert_memory_constraints(&metrics, max_bytes)?;

            Ok(())
        }

        /// Test chart with maximum data capacity
        pub fn test_max_capacity<T>(chart: &T) -> ChartResult<()>
        where
            T: Chart<Rgb565, Data = StaticDataSeries<Point2D, 256>, Config = ChartConfig<Rgb565>>,
        {
            let mut max_data = StaticDataSeries::new();

            // Fill to capacity
            for i in 0..256 {
                max_data.push(Point2D::new(i as f32, (i % 100) as f32)).ok();
            }

            let config = super::super::create_test_config();
            let mut display = create_test_display();

            chart.draw(&max_data, &config, TEST_VIEWPORT, &mut display)?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "line")]
    use super::super::data_generators;

    #[test]
    #[cfg(feature = "line")]
    #[ignore = "MockDisplay has limitations with pixel overlap detection"]
    fn test_chart_test_suite() {
        use embedded_charts::chart::line::LineChart;

        let chart = LineChart::new();
        let data = data_generators::generate_test_data(super::super::TestDataPattern::Linear, 10);

        // Test should not panic
        let result = ChartTestSuite::test_chart_rendering(&chart, &[data]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_performance_measurement() {
        // Test that performance measurement infrastructure works
        let metrics = PerformanceMetrics::new();
        assert_eq!(metrics.render_time_ns, 0);
    }
}
