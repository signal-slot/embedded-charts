// Common Utilities for Examples
//
// This module provides essential utilities for examples to eliminate boilerplate code duplication.

pub mod window;

#[cfg(all(feature = "std", feature = "capture"))]
pub mod capture;

use embedded_charts::prelude::*;
use embedded_graphics::pixelcolor::Rgb565;

/// Re-export commonly used types
#[allow(unused_imports)] // May not be used by all examples
pub use window::WindowConfig;
#[allow(unused_imports)] // May not be used by all examples
pub use window::WindowTheme;

/// Chart margins optimized for examples with axes and legends
#[allow(dead_code)] // Standard margins for examples
pub const CHART_MARGINS: Margins = Margins {
    top: 20,
    right: 20,
    bottom: 40,
    left: 60,
};

/// Data generation utilities
#[allow(dead_code)] // Utility functions for examples data generation
pub mod data {
    use super::*;
    use core::f32::consts::PI;

    /// Generate sine wave data
    pub fn sine_wave(
        points: usize,
        amplitude: f32,
        frequency: f32,
        phase: f32,
    ) -> ChartResult<StaticDataSeries<Point2D, 256>> {
        let mut series = StaticDataSeries::new();

        for i in 0..points {
            let x = i as f32;
            let y = amplitude * (frequency * x + phase).sin();
            series.push(Point2D::new(x, y)).map_err(ChartError::from)?;
        }

        Ok(series)
    }

    /// Generate cosine wave data
    pub fn cosine_wave(
        points: usize,
        amplitude: f32,
        frequency: f32,
        phase: f32,
    ) -> ChartResult<StaticDataSeries<Point2D, 256>> {
        let mut series = StaticDataSeries::new();

        for i in 0..points {
            let x = i as f32;
            let y = amplitude * (frequency * x + phase).cos();
            series.push(Point2D::new(x, y)).map_err(ChartError::from)?;
        }

        Ok(series)
    }

    /// Generate linear data with optional noise
    pub fn linear_data(
        points: usize,
        slope: f32,
        intercept: f32,
        noise: f32,
    ) -> ChartResult<StaticDataSeries<Point2D, 256>> {
        let mut series = StaticDataSeries::new();

        for i in 0..points {
            let x = i as f32;
            let noise_factor = if noise > 0.0 {
                // Simple pseudo-random noise using sine
                noise * (x * 12.9898).sin() * (x * 78.233).cos()
            } else {
                0.0
            };
            let y = slope * x + intercept + noise_factor;
            series.push(Point2D::new(x, y)).map_err(ChartError::from)?;
        }

        Ok(series)
    }

    /// Generate exponential data
    pub fn exponential_data(
        points: usize,
        base: f32,
        scale: f32,
    ) -> ChartResult<StaticDataSeries<Point2D, 256>> {
        let mut series = StaticDataSeries::new();

        for i in 0..points {
            let x = i as f32;
            let y = scale * base.powf(x / 10.0); // Scale down x to prevent overflow
            series.push(Point2D::new(x, y)).map_err(ChartError::from)?;
        }

        Ok(series)
    }

    /// Generate temperature sensor data simulation
    pub fn temperature_data(hours: usize) -> ChartResult<StaticDataSeries<Point2D, 256>> {
        let mut series = StaticDataSeries::new();

        for i in 0..hours {
            let hour = i as f32;
            // Daily temperature cycle: cooler at night, warmer during day
            let daily_cycle = 5.0 * (2.0 * PI * hour / 24.0 - PI / 2.0).sin();
            // Base temperature
            let base_temp = 20.0;
            // Small random variations
            let variation =
                2.0 * (hour * core::f32::consts::PI).sin() * (hour * core::f32::consts::E).cos();

            let temperature = base_temp + daily_cycle + variation;
            series
                .push(Point2D::new(hour, temperature))
                .map_err(ChartError::from)?;
        }

        Ok(series)
    }

    /// Generate system monitoring data
    pub fn system_metrics(
        points: usize,
        metric_type: SystemMetric,
    ) -> ChartResult<StaticDataSeries<Point2D, 256>> {
        let mut series = StaticDataSeries::new();

        for i in 0..points {
            let time = i as f32;
            let value = match metric_type {
                SystemMetric::CpuUsage => {
                    let base = 25.0;
                    let variation = 15.0 * (time * 0.1).sin();
                    let spikes = if (time as usize) % 20 == 0 { 30.0 } else { 0.0 };
                    (base + variation + spikes).max(0.0).min(100.0)
                }
                SystemMetric::MemoryUsage => {
                    let base = 40.0;
                    let trend = time * 0.5;
                    let variation = 5.0 * (time * 0.15).cos();
                    (base + trend + variation).max(0.0).min(100.0)
                }
                SystemMetric::NetworkIO => {
                    let base = 10.0;
                    let bursts = if (time as usize) % 15 == 0 { 40.0 } else { 0.0 };
                    let variation = 8.0 * (time * 0.2).sin();
                    (base + bursts + variation).max(0.0).min(100.0)
                }
                SystemMetric::DiskUsage => {
                    let base = 5.0;
                    let activity = 20.0 * (time * 0.05).sin().abs();
                    let variation = 3.0 * (time * 0.3).cos();
                    (base + activity + variation).max(0.0).min(100.0)
                }
            };

            series
                .push(Point2D::new(time, value))
                .map_err(ChartError::from)?;
        }

        Ok(series)
    }

    /// System metric types for monitoring data
    #[derive(Debug, Clone, Copy)]
    pub enum SystemMetric {
        CpuUsage,
        MemoryUsage,
        NetworkIO,
        DiskUsage,
    }
}

/// Standard chart configurations
#[allow(dead_code)] // Chart configuration utilities for examples
pub mod configs {
    use super::*;

    /// Create a professional line chart configuration
    pub fn professional_line_chart(color: Rgb565) -> ChartResult<LineChart<Rgb565>> {
        LineChart::builder().line_color(color).line_width(2).build()
    }

    /// Standard color palette for multi-series charts
    pub fn standard_colors() -> [Rgb565; 8] {
        [
            Rgb565::new(31, 8, 8),  // Red
            Rgb565::new(8, 16, 31), // Blue
            Rgb565::new(8, 31, 8),  // Green
            Rgb565::new(20, 8, 31), // Purple
            Rgb565::new(31, 20, 8), // Orange
            Rgb565::new(8, 31, 31), // Cyan
            Rgb565::new(31, 8, 20), // Magenta
            Rgb565::new(20, 20, 8), // Yellow
        ]
    }

    /// Professional color palette
    pub fn professional_colors() -> [Rgb565; 6] {
        [
            Rgb565::new(15, 45, 31), // Steel Blue - brighter
            Rgb565::new(31, 8, 15),  // Crimson - brighter
            Rgb565::new(15, 63, 15), // Lime Green - brighter
            Rgb565::new(31, 45, 8),  // Orange - brighter
            Rgb565::new(25, 15, 31), // Blue Violet - brighter
            Rgb565::new(8, 55, 31),  // Deep Sky Blue - brighter
        ]
    }
}

/// Auto-legend layout functionality
#[allow(dead_code)] // Layout utilities for charts
pub mod layout {
    use super::*;
    use embedded_graphics::draw_target::DrawTarget;

    /// Chart with automatic legend layout
    #[allow(dead_code)] // Utility struct for charts with legends
    pub struct ChartWithLegend<'a, C: PixelColor> {
        pub legend: Option<&'a StandardLegend<C>>,
        pub renderer: Option<&'a StandardLegendRenderer<C>>,
    }

    #[allow(dead_code)] // Utility methods for chart with legend
    impl<'a, C: PixelColor> ChartWithLegend<'a, C> {
        pub fn new(legend: &'a StandardLegend<C>, renderer: &'a StandardLegendRenderer<C>) -> Self {
            Self {
                legend: Some(legend),
                renderer: Some(renderer),
            }
        }

        pub fn none() -> Self {
            Self {
                legend: None,
                renderer: None,
            }
        }
    }

    /// Draw any chart with automatic legend layout
    pub fn draw_chart_with_auto_legend<D>(
        chart_drawer: impl FnOnce(Rectangle, &mut D) -> ChartResult<()>,
        viewport: Rectangle,
        display: &mut D,
        legend_setup: ChartWithLegend<Rgb565>,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        if let (Some(legend), Some(renderer)) = (legend_setup.legend, legend_setup.renderer) {
            // Simple layout: legend on right, chart on left
            let legend_size = legend.calculate_size();
            let legend_width = legend_size.width + 20; // Add padding

            let chart_area = Rectangle::new(
                viewport.top_left,
                Size::new(
                    viewport.size.width.saturating_sub(legend_width),
                    viewport.size.height,
                ),
            );

            let legend_rect = Rectangle::new(
                Point::new(
                    viewport.top_left.x + chart_area.size.width as i32 + 10,
                    viewport.top_left.y + 20,
                ),
                legend_size,
            );

            // Draw chart in adjusted area
            chart_drawer(chart_area, display)?;

            // Render legend
            renderer.render(legend, legend_rect, display)?;
        } else {
            // No legend, use full viewport
            chart_drawer(viewport, display)?;
        }

        Ok(())
    }
}

/// Utility functions for common example patterns
#[allow(dead_code)] // Utility functions for examples
pub mod utils {
    use super::*;

    /// Print data series information
    pub fn print_series_info(series: &StaticDataSeries<Point2D, 256>, name: &str) {
        if let (Some(first), Some(last)) =
            (series.get(0), series.get(series.len().saturating_sub(1)))
        {
            println!(
                "📈 {}: {} points ({}→{})",
                name,
                series.len(),
                format_point(first),
                format_point(last)
            );
        } else {
            println!("📈 {}: {} points", name, series.len());
        }
    }

    /// Format a point for display
    pub fn format_point(point: Point2D) -> heapless::String<32> {
        let mut s = heapless::String::new();
        let _ = core::fmt::write(&mut s, format_args!("({:.1}, {:.1})", point.x, point.y));
        s
    }

    /// Print feature requirement message
    pub fn print_feature_requirement(feature: &str, example_type: &str) {
        println!(
            "⚠️  This {} example requires the '{}' feature to run",
            example_type, feature
        );
        println!(
            "   Run with: cargo run --example <example_name> --features {}",
            feature
        );
    }
}
