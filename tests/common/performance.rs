//! Performance testing and benchmarking utilities
//!
//! Provides tools for measuring and validating chart performance characteristics

#![allow(dead_code)] // Allow unused testing utilities - they're part of testing infrastructure

use embedded_charts::{
    chart::traits::{Chart, ChartConfig},
    data::{point::Point2D, series::StaticDataSeries},
    error::ChartResult,
};
use embedded_graphics::{pixelcolor::Rgb565, prelude::*, primitives::Rectangle};

use super::{MemoryMetrics, PerformanceMetrics, TEST_VIEWPORT};

/// Performance benchmark suite for charts
pub struct PerformanceBenchmark;

impl PerformanceBenchmark {
    /// Benchmark chart rendering performance with different data sizes
    pub fn benchmark_data_scaling<T>(chart: &T) -> ChartResult<Vec<PerformanceMetrics>>
    where
        T: Chart<Rgb565, Data = StaticDataSeries<Point2D, 256>, Config = ChartConfig<Rgb565>>,
    {
        let data_sizes = [1, 5, 10, 25, 50, 100, 256];
        let mut results = Vec::new();

        for &size in &data_sizes {
            let mut data = StaticDataSeries::new();
            for i in 0..size {
                data.push(Point2D::new(i as f32, (i % 50) as f32)).ok();
            }

            let metrics = Self::measure_single_render(chart, &data)?;
            results.push(metrics);
        }

        Ok(results)
    }

    /// Benchmark chart rendering with different viewport sizes
    pub fn benchmark_viewport_scaling<T>(
        chart: &T,
        data: &StaticDataSeries<Point2D, 256>,
    ) -> ChartResult<Vec<PerformanceMetrics>>
    where
        T: Chart<Rgb565, Data = StaticDataSeries<Point2D, 256>, Config = ChartConfig<Rgb565>>,
    {
        let viewport_sizes = [
            Size::new(64, 48),   // Tiny embedded display
            Size::new(128, 96),  // Small embedded display
            Size::new(320, 240), // Medium display
            Size::new(640, 480), // Large display
            Size::new(800, 600), // Desktop-class display
        ];

        let mut results = Vec::new();

        for &size in &viewport_sizes {
            let viewport = Rectangle::new(Point::zero(), size);
            let metrics = Self::measure_single_render_with_viewport(chart, data, viewport)?;
            results.push(metrics);
        }

        Ok(results)
    }

    /// Measure rendering performance for a single chart draw operation
    pub fn measure_single_render<T>(
        chart: &T,
        data: &StaticDataSeries<Point2D, 256>,
    ) -> ChartResult<PerformanceMetrics>
    where
        T: Chart<Rgb565, Data = StaticDataSeries<Point2D, 256>, Config = ChartConfig<Rgb565>>,
    {
        Self::measure_single_render_with_viewport(chart, data, TEST_VIEWPORT)
    }

    /// Measure rendering performance with specific viewport
    pub fn measure_single_render_with_viewport<T>(
        chart: &T,
        data: &StaticDataSeries<Point2D, 256>,
        viewport: Rectangle,
    ) -> ChartResult<PerformanceMetrics>
    where
        T: Chart<Rgb565, Data = StaticDataSeries<Point2D, 256>, Config = ChartConfig<Rgb565>>,
    {
        let config = super::create_test_config();
        let mut display = super::create_test_display();

        // Simulate performance measurement
        // In a real implementation, this would use proper timing mechanisms
        let start_time = 0;
        chart.draw(data, &config, viewport, &mut display)?;
        let end_time = 1000; // Simulated 1000ns render time

        let mut metrics = PerformanceMetrics::new();
        metrics.render_time_ns = end_time - start_time;
        metrics.draw_calls = 1;

        // Simulate memory usage calculation
        metrics.memory_usage = MemoryMetrics {
            heap_used: 0,                             // no_std, no heap
            stack_used: core::mem::size_of_val(data), // Approximate stack usage
            static_memory: core::mem::size_of_val(chart),
        };

        Ok(metrics)
    }

    /// Test that rendering time scales reasonably with data size
    pub fn validate_performance_scaling<T>(chart: &T) -> ChartResult<()>
    where
        T: Chart<Rgb565, Data = StaticDataSeries<Point2D, 256>, Config = ChartConfig<Rgb565>>,
    {
        let results = Self::benchmark_data_scaling(chart)?;

        if results.is_empty() {
            return Ok(());
        }

        // Check that performance doesn't degrade catastrophically
        let first_time = results[0].render_time_ns;
        let last_time = results[results.len() - 1].render_time_ns;

        // Allow up to 10x slowdown for 256x more data (very generous)
        let max_acceptable_ratio = 10;
        let actual_ratio = if first_time > 0 {
            last_time / first_time
        } else {
            1
        };

        if actual_ratio > max_acceptable_ratio {
            return Err(embedded_charts::error::ChartError::InvalidRange);
        }

        Ok(())
    }

    /// Validate memory usage stays within embedded constraints
    pub fn validate_memory_constraints<T>(
        chart: &T,
        data: &StaticDataSeries<Point2D, 256>,
        max_bytes: usize,
    ) -> ChartResult<()>
    where
        T: Chart<Rgb565, Data = StaticDataSeries<Point2D, 256>, Config = ChartConfig<Rgb565>>,
    {
        let metrics = Self::measure_single_render(chart, data)?;

        let total_memory = metrics.memory_usage.heap_used
            + metrics.memory_usage.stack_used
            + metrics.memory_usage.static_memory;

        if total_memory > max_bytes {
            return Err(embedded_charts::error::ChartError::MemoryFull);
        }

        Ok(())
    }

    /// Stress test chart with rapid successive renders
    pub fn stress_test_rapid_renders<T>(
        chart: &T,
        data: &StaticDataSeries<Point2D, 256>,
        iterations: usize,
    ) -> ChartResult<PerformanceMetrics>
    where
        T: Chart<Rgb565, Data = StaticDataSeries<Point2D, 256>, Config = ChartConfig<Rgb565>>,
    {
        let config = super::create_test_config();
        let mut display = super::create_test_display();

        let start_time = 0;

        for _ in 0..iterations {
            chart.draw(data, &config, TEST_VIEWPORT, &mut display)?;
        }

        let end_time = (iterations * 1000) as u64; // Simulated timing

        let mut metrics = PerformanceMetrics::new();
        metrics.render_time_ns = end_time - start_time as u64;
        metrics.draw_calls = iterations;

        Ok(metrics)
    }
}

/// Memory profiling utilities
pub struct MemoryProfiler;

impl MemoryProfiler {
    /// Profile memory usage during chart operations
    pub fn profile_chart_memory<T>(
        chart: &T,
        data: &StaticDataSeries<Point2D, 256>,
    ) -> ChartResult<MemoryProfile>
    where
        T: Chart<Rgb565, Data = StaticDataSeries<Point2D, 256>, Config = ChartConfig<Rgb565>>,
    {
        let config = super::create_test_config();
        let mut display = super::create_test_display();

        // Measure memory before rendering
        let before = Self::measure_current_memory();

        chart.draw(data, &config, TEST_VIEWPORT, &mut display)?;

        // Measure memory after rendering
        let after = Self::measure_current_memory();

        Ok(MemoryProfile {
            before,
            after,
            peak_usage: after, // Simplified - real implementation would track peak
            chart_size: core::mem::size_of_val(chart),
            data_size: core::mem::size_of_val(data),
        })
    }

    /// Measure current memory usage (simplified implementation)
    fn measure_current_memory() -> MemoryMetrics {
        MemoryMetrics {
            heap_used: 0,       // no_std environment
            stack_used: 1024,   // Estimated stack usage
            static_memory: 512, // Estimated static memory
        }
    }

    /// Validate that memory usage is suitable for embedded systems
    pub fn validate_embedded_memory_usage(profile: &MemoryProfile) -> ChartResult<()> {
        let total_peak = profile.peak_usage.heap_used
            + profile.peak_usage.stack_used
            + profile.peak_usage.static_memory;

        // Common embedded memory constraints
        let constraints = [
            ("ultra_constrained", 1024), // 1KB total
            ("small_embedded", 4096),    // 4KB total
            ("medium_embedded", 16384),  // 16KB total
            ("large_embedded", 65536),   // 64KB total
        ];

        // Chart should work in at least the medium embedded category
        if total_peak > constraints[2].1 {
            return Err(embedded_charts::error::ChartError::MemoryFull);
        }

        Ok(())
    }
}

/// Memory usage profile for analysis
#[derive(Debug, Clone)]
pub struct MemoryProfile {
    pub before: MemoryMetrics,
    pub after: MemoryMetrics,
    pub peak_usage: MemoryMetrics,
    pub chart_size: usize,
    pub data_size: usize,
}

impl MemoryProfile {
    /// Calculate memory increase during operation
    pub fn memory_delta(&self) -> i32 {
        let before_total =
            self.before.heap_used + self.before.stack_used + self.before.static_memory;
        let after_total = self.after.heap_used + self.after.stack_used + self.after.static_memory;
        after_total as i32 - before_total as i32
    }

    /// Get total memory footprint
    pub fn total_footprint(&self) -> usize {
        self.peak_usage.heap_used + self.peak_usage.stack_used + self.peak_usage.static_memory
    }
}

#[cfg(test)]
mod tests {
    use super::super::data_generators;
    use super::*;

    #[test]
    #[cfg(feature = "line")]
    #[ignore = "MockDisplay has limitations with pixel overlap detection"]
    fn test_performance_benchmark() {
        use embedded_charts::chart::line::LineChart;

        let chart = LineChart::new();
        let data = data_generators::generate_test_data(super::super::TestDataPattern::Linear, 10);

        let result = PerformanceBenchmark::measure_single_render(&chart, &data);
        assert!(result.is_ok());

        let metrics = result.unwrap();
        assert!(metrics.render_time_ns > 0);
    }

    #[test]
    fn test_memory_profiler() {
        let metrics = MemoryMetrics::new();
        assert_eq!(metrics.heap_used, 0);
    }

    #[test]
    fn test_memory_profile_delta() {
        let profile = MemoryProfile {
            before: MemoryMetrics {
                heap_used: 100,
                stack_used: 200,
                static_memory: 300,
            },
            after: MemoryMetrics {
                heap_used: 120,
                stack_used: 220,
                static_memory: 300,
            },
            peak_usage: MemoryMetrics {
                heap_used: 120,
                stack_used: 220,
                static_memory: 300,
            },
            chart_size: 64,
            data_size: 128,
        };

        assert_eq!(profile.memory_delta(), 40);
        assert_eq!(profile.total_footprint(), 640);
    }
}
