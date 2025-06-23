//! Platform-specific performance benchmarks
//!
//! This module provides benchmarks that simulate different embedded platforms
//! and their specific constraints (memory, CPU speed, display types).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use embedded_charts::prelude::*;
use embedded_graphics::{
    mock_display::MockDisplay,
    pixelcolor::{BinaryColor, Gray8, Rgb565},
};
use std::vec::Vec;

/// Create a fresh MockDisplay that allows overdrawing and out-of-bounds drawing
/// This prevents "tried to draw pixel twice" and "outside display area" errors
fn create_test_display<C>() -> MockDisplay<C>
where
    C: embedded_graphics::pixelcolor::PixelColor,
{
    let mut display = MockDisplay::new();
    display.set_allow_overdraw(true);
    display.set_allow_out_of_bounds_drawing(true);
    display
}

/// Simulate different display types and their rendering characteristics
mod display_profiles {
    use super::*;

    /// OLED display profile (128x64, monochrome)
    pub fn bench_oled_rendering(c: &mut Criterion) {
        let mut group = c.benchmark_group("oled_display");

        let data = generate_sine_data(32);
        let chart = LineChart::builder()
            .line_color(BinaryColor::On)
            .line_width(1)
            .build()
            .unwrap();

        group.bench_function("128x64_monochrome", |b| {
            b.iter(|| {
                let mut display = create_test_display::<BinaryColor>();
                // Use larger margins to ensure no out-of-bounds drawing
                let viewport = Rectangle::new(Point::new(40, 40), Size::new(48, 24));
                chart
                    .draw(&data, &ChartConfig::default(), viewport, &mut display)
                    .ok();
                black_box(display);
            });
        });

        group.finish();
    }

    /// E-Paper display profile (200x200, grayscale)
    pub fn bench_epaper_rendering(c: &mut Criterion) {
        let mut group = c.benchmark_group("epaper_display");

        let data = generate_sine_data(50);
        let chart = LineChart::builder()
            .line_color(Gray8::new(0))
            .line_width(2)
            .build()
            .unwrap();

        group.bench_function("200x200_grayscale", |b| {
            b.iter(|| {
                let mut display = create_test_display::<Gray8>();
                // Use larger margins to ensure no out-of-bounds drawing
                let viewport = Rectangle::new(Point::new(40, 40), Size::new(120, 120));
                chart
                    .draw(&data, &ChartConfig::default(), viewport, &mut display)
                    .ok();
                black_box(display);
            });
        });

        group.finish();
    }

    /// TFT display profile (320x240, RGB565)
    pub fn bench_tft_rendering(c: &mut Criterion) {
        let mut group = c.benchmark_group("tft_display");

        let data = generate_sine_data(100);
        let chart = LineChart::builder()
            .line_color(Rgb565::BLUE)
            .line_width(2)
            .build()
            .unwrap();

        group.bench_function("320x240_rgb565", |b| {
            b.iter(|| {
                let mut display = create_test_display::<Rgb565>();
                // Use larger margins to ensure no out-of-bounds drawing
                let viewport = Rectangle::new(Point::new(40, 40), Size::new(240, 160));
                chart
                    .draw(&data, &ChartConfig::default(), viewport, &mut display)
                    .ok();
                black_box(display);
            });
        });

        group.finish();
    }
}

/// Simulate different MCU capabilities
mod mcu_profiles {
    use super::*;

    /// Cortex-M0 profile (48MHz, integer math only)
    pub fn bench_cortex_m0_operations(c: &mut Criterion) {
        let mut group = c.benchmark_group("cortex_m0");

        // Simulate integer-only math operations
        group.bench_function("integer_coordinate_transform", |b| {
            let points: Vec<(i32, i32)> = (0..100).map(|i| (i * 10, (i * 10) % 200)).collect();

            b.iter(|| {
                let transformed: Vec<(i32, i32)> = points
                    .iter()
                    .map(|(x, y)| {
                        // Integer-only scaling
                        let scaled_x = (*x * 320) / 1000;
                        let scaled_y = (*y * 240) / 200;
                        (scaled_x, scaled_y)
                    })
                    .collect();
                black_box(transformed);
            });
        });

        group.finish();
    }

    /// Cortex-M4 profile (72MHz, FPU, DSP instructions)
    pub fn bench_cortex_m4_operations(c: &mut Criterion) {
        let mut group = c.benchmark_group("cortex_m4");

        // Floating-point operations with FPU
        group.bench_function("fpu_coordinate_transform", |b| {
            let points: Vec<Point2D> = (0..100)
                .map(|i| Point2D::new(i as f32, (i as f32 * 0.1).sin() * 100.0))
                .collect();

            b.iter(|| {
                let transformed: Vec<(f32, f32)> = points
                    .iter()
                    .map(|p| {
                        let x = p.x * 3.2;
                        let y = p.y * 2.4;
                        (x, y)
                    })
                    .collect();
                black_box(transformed);
            });
        });

        group.finish();
    }
}

/// Memory-constrained scenarios
fn bench_memory_constrained(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_constrained");

    // 1KB memory budget
    group.bench_function("1kb_line_chart", |b| {
        let mut data = StaticDataSeries::<Point2D, 256>::new();
        for i in 0..32 {
            data.push(Point2D::new(i as f32, (i as f32).sin() * 10.0))
                .ok();
        }

        let chart = LineChart::builder()
            .line_color(BinaryColor::On)
            .line_width(1)
            .build()
            .unwrap();

        b.iter(|| {
            let mut display = create_test_display::<BinaryColor>();
            // Use larger margins to ensure no out-of-bounds drawing
            let viewport = Rectangle::new(Point::new(40, 40), Size::new(48, 24));
            chart
                .draw(&data, &ChartConfig::default(), viewport, &mut display)
                .ok();
            black_box(display);
        });
    });

    // 4KB memory budget
    group.bench_function("4kb_multi_series", |b| {
        let mut series1 = StaticDataSeries::<Point2D, 256>::new();
        let mut series2 = StaticDataSeries::<Point2D, 256>::new();

        for i in 0..64 {
            series1
                .push(Point2D::new(i as f32, (i as f32 * 0.1).sin() * 20.0))
                .ok();
            series2
                .push(Point2D::new(i as f32, (i as f32 * 0.1).cos() * 20.0))
                .ok();
        }

        b.iter(|| {
            let chart = LineChart::builder()
                .line_color(Rgb565::BLUE)
                .build()
                .unwrap();

            let mut display = create_test_display::<Rgb565>();
            // Use larger margins to ensure no out-of-bounds drawing
            let viewport = Rectangle::new(Point::new(40, 40), Size::new(100, 60));

            chart
                .draw(&series1, &ChartConfig::default(), viewport, &mut display)
                .ok();
            chart
                .draw(&series2, &ChartConfig::default(), viewport, &mut display)
                .ok();

            black_box(display);
        });
    });

    group.finish();
}

/// Real-time streaming scenarios
fn bench_streaming_scenarios(c: &mut Criterion) {
    let mut group = c.benchmark_group("streaming");

    // Simulate ECG-like data streaming
    group.bench_function("ecg_streaming_update", |b| {
        let mut buffer = StaticDataSeries::<Point2D, 256>::new();

        // Pre-fill with data
        for i in 0..256 {
            let value = generate_ecg_sample(i);
            buffer.push(Point2D::new(i as f32, value)).ok();
        }

        let chart = LineChart::builder()
            .line_color(Rgb565::GREEN)
            .line_width(1)
            .build()
            .unwrap();

        let mut sample_index = 256;

        b.iter(|| {
            // Simulate sliding window by clearing and refilling
            buffer.clear();
            for i in 0..256 {
                let idx = sample_index + i - 255;
                let value = generate_ecg_sample(idx);
                buffer.push(Point2D::new(idx as f32, value)).ok();
            }
            sample_index += 1;

            let mut display = MockDisplay::<Rgb565>::new();
            // Use larger margins to ensure no out-of-bounds drawing
            let viewport = Rectangle::new(Point::new(40, 40), Size::new(240, 160));
            chart
                .draw(&buffer, &ChartConfig::default(), viewport, &mut display)
                .ok();
            black_box(display);
        });
    });

    group.finish();
}

/// Dashboard rendering with multiple charts
fn bench_dashboard_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("dashboard");

    group.bench_function("3_chart_dashboard", |b| {
        // Prepare data for 3 different charts (line, bar, pie)
        let line_data = generate_sine_data(50);
        let bar_data = generate_random_data(10, 0.0, 100.0);
        let pie_data = generate_pie_data(5);

        let line_chart = LineChart::builder()
            .line_color(Rgb565::BLUE)
            .build()
            .unwrap();

        let bar_chart = BarChart::builder()
            .bar_width(BarWidth::Auto)
            .build()
            .unwrap();

        let pie_chart = PieChart::builder().radius(50).build().unwrap();

        b.iter(|| {
            let mut display = create_test_display::<Rgb565>();

            // Render 3 charts in sections with larger margins
            let viewports = [
                Rectangle::new(Point::new(40, 20), Size::new(240, 40)),
                Rectangle::new(Point::new(40, 100), Size::new(240, 40)),
                Rectangle::new(Point::new(40, 180), Size::new(240, 40)),
            ];

            line_chart
                .draw(
                    &line_data,
                    &ChartConfig::default(),
                    viewports[0],
                    &mut display,
                )
                .ok();
            bar_chart
                .draw(
                    &bar_data,
                    &ChartConfig::default(),
                    viewports[1],
                    &mut display,
                )
                .ok();
            pie_chart
                .draw(
                    &pie_data,
                    &ChartConfig::default(),
                    viewports[2],
                    &mut display,
                )
                .ok();

            black_box(display);
        });
    });

    group.finish();
}

// Helper functions
fn generate_sine_data(size: usize) -> StaticDataSeries<Point2D, 256> {
    let mut series = StaticDataSeries::new();
    for i in 0..size.min(256) {
        let x = i as f32;
        let y = (x * 0.1).sin() * 50.0 + 100.0;
        series.push(Point2D::new(x, y)).ok();
    }
    series
}

fn generate_random_data(size: usize, min: f32, max: f32) -> StaticDataSeries<Point2D, 256> {
    let mut series = StaticDataSeries::new();
    for i in 0..size.min(256) {
        let value = min + (max - min) * ((i * 37 + 11) % 100) as f32 / 100.0;
        series.push(Point2D::new(i as f32, value)).ok();
    }
    series
}

fn generate_pie_data(slices: usize) -> StaticDataSeries<Point2D, 256> {
    let mut series = StaticDataSeries::new();
    let total = 100.0;
    let slice_value = total / slices as f32;

    for i in 0..slices.min(256) {
        series.push(Point2D::new(i as f32, slice_value)).ok();
    }
    series
}

fn generate_ecg_sample(index: usize) -> f32 {
    let t = (index % 100) as f32 / 100.0;
    if t < 0.1 {
        0.0
    } else if t < 0.15 {
        40.0 * (t - 0.1) / 0.05
    } else if t < 0.2 {
        40.0 - 80.0 * (t - 0.15) / 0.05
    } else if t < 0.25 {
        -40.0 + 50.0 * (t - 0.2) / 0.05
    } else if t < 0.3 {
        10.0 - 10.0 * (t - 0.25) / 0.05
    } else {
        0.0
    }
}

criterion_group!(
    platform_benches,
    display_profiles::bench_oled_rendering,
    display_profiles::bench_epaper_rendering,
    display_profiles::bench_tft_rendering,
    mcu_profiles::bench_cortex_m0_operations,
    mcu_profiles::bench_cortex_m4_operations,
    bench_memory_constrained,
    bench_streaming_scenarios,
    bench_dashboard_rendering,
);

criterion_main!(platform_benches);
