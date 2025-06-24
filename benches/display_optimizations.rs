//! Benchmarks for display-specific optimizations
//!
//! Compares performance between generic and optimized renderers

use criterion::{criterion_group, criterion_main, Criterion};
use embedded_charts::{
    chart::{
        line::LineChart,
        traits::{Chart, ChartBuilder, ChartConfig},
    },
    data::{point::Point2D, series::StaticDataSeries, DataSeries},
    render::{EPaperRenderer, OLEDRenderer, OptimizedRenderer, TFTRenderer},
};
use embedded_graphics::{
    mock_display::MockDisplay,
    pixelcolor::{BinaryColor, Rgb565},
    prelude::*,
    primitives::Rectangle,
};
use std::hint::black_box;

/// Create test data
fn create_test_data(size: usize, vertical_bias: bool) -> StaticDataSeries<Point2D, 256> {
    let mut data = StaticDataSeries::new();
    for i in 0..size.min(256) {
        let x = if vertical_bias {
            30.0 + (i % 5) as f32 // Mostly vertical lines, within bounds
        } else {
            (i as f32 * 0.5).min(60.0) // Keep within MockDisplay bounds
        };
        let y = ((i as f32 * 0.1).sin() * 20.0 + 30.0).clamp(0.0, 60.0);
        data.push(Point2D::new(x, y)).unwrap();
    }
    data
}

/// Benchmark OLED optimizations
fn bench_oled_optimizations(c: &mut Criterion) {
    let mut group = c.benchmark_group("oled_optimizations");

    // Test vertical line optimization
    let _vertical_data = create_test_data(100, true);
    let _horizontal_data = create_test_data(100, false);

    group.bench_function("generic_vertical", |b| {
        b.iter(|| {
            let mut display = MockDisplay::<BinaryColor>::new();
            display.set_allow_out_of_bounds_drawing(true);
            display.set_allow_overdraw(true);

            // Simulate drawing without optimization
            for x in 10..20 {
                for y in 10..50 {
                    display.draw_pixel(Point::new(x * 3, y), BinaryColor::On);
                }
            }
            black_box(display);
        });
    });

    group.bench_function("optimized_vertical", |b| {
        b.iter(|| {
            let mut display = MockDisplay::<BinaryColor>::new();
            display.set_allow_overdraw(true);
            let mut renderer = OLEDRenderer::new(display);

            renderer.begin_batch();
            // Draw simple vertical lines to test optimization
            for x in 10..20 {
                renderer
                    .draw_line_optimized(
                        Point::new(x * 3, 10),
                        Point::new(x * 3, 50),
                        BinaryColor::On,
                        1,
                    )
                    .ok();
            }
            renderer.end_batch();
            black_box(renderer);
        });
    });

    group.finish();
}

/// Benchmark TFT optimizations
fn bench_tft_optimizations(c: &mut Criterion) {
    let mut group = c.benchmark_group("tft_optimizations");

    let data = create_test_data(100, false);

    group.bench_function("generic_rendering", |b| {
        b.iter(|| {
            let mut display = MockDisplay::<Rgb565>::new();
            display.set_allow_out_of_bounds_drawing(true);
            display.set_allow_overdraw(true);

            // Simulate standard line drawing
            for i in 0..data.len() - 1 {
                let p1 = data.as_slice()[i];
                let p2 = data.as_slice()[i + 1];

                // Bresenham's line algorithm simulation
                let dx = (p2.x - p1.x).abs();
                let dy = (p2.y - p1.y).abs();
                let steps = dx.max(dy) as i32;

                for step in 0..=steps {
                    let t = step as f32 / steps as f32;
                    let x = p1.x + (p2.x - p1.x) * t;
                    let y = p1.y + (p2.y - p1.y) * t;
                    display.draw_pixel(Point::new(x as i32, y as i32), Rgb565::BLUE);
                }
            }
            black_box(display);
        });
    });

    group.bench_function("optimized_rendering", |b| {
        b.iter(|| {
            let mut display = MockDisplay::<Rgb565>::new();
            display.set_allow_overdraw(true);
            let mut renderer = TFTRenderer::new(display);

            renderer.begin_batch();
            for i in 0..data.len() - 1 {
                let p1 = data.as_slice()[i];
                let p2 = data.as_slice()[i + 1];
                renderer
                    .draw_line_optimized(
                        Point::new(p1.x as i32, p1.y as i32),
                        Point::new(p2.x as i32, p2.y as i32),
                        Rgb565::BLUE,
                        2,
                    )
                    .ok();
            }
            renderer.end_batch();
            black_box(renderer);
        });
    });

    // Benchmark horizontal line optimization (common in bar charts)
    group.bench_function("horizontal_lines", |b| {
        b.iter(|| {
            let mut display = MockDisplay::<Rgb565>::new();
            display.set_allow_overdraw(true);
            let mut renderer = TFTRenderer::new(display);

            renderer.begin_batch();
            for y in (0..60).step_by(5) {
                renderer
                    .draw_line_optimized(Point::new(5, y), Point::new(60, y), Rgb565::RED, 1)
                    .ok();
            }
            renderer.end_batch();
            black_box(renderer);
        });
    });

    group.finish();
}

/// Benchmark E-Paper optimizations
fn bench_epaper_optimizations(c: &mut Criterion) {
    let mut group = c.benchmark_group("epaper_optimizations");

    // E-Paper benefits from minimal update regions
    group.bench_function("full_refresh", |b| {
        b.iter(|| {
            let mut display = MockDisplay::<BinaryColor>::new();
            display.set_allow_out_of_bounds_drawing(true);
            display.set_allow_overdraw(true);

            // Simulate full screen update
            for y in 0..64 {
                for x in 0..64 {
                    let color = if (x + y) % 2 == 0 {
                        BinaryColor::On
                    } else {
                        BinaryColor::Off
                    };
                    display.draw_pixel(Point::new(x, y), color);
                }
            }
            black_box(display);
        });
    });

    group.bench_function("partial_refresh", |b| {
        b.iter(|| {
            let mut display = MockDisplay::<BinaryColor>::new();
            display.set_allow_overdraw(true);
            let mut renderer = EPaperRenderer::new(display);

            renderer.begin_batch();
            // Only update specific regions
            renderer
                .draw_filled_rect_optimized(
                    Rectangle::new(Point::new(10, 10), Size::new(20, 20)),
                    BinaryColor::On,
                )
                .ok();
            renderer
                .draw_filled_rect_optimized(
                    Rectangle::new(Point::new(35, 35), Size::new(20, 20)),
                    BinaryColor::Off,
                )
                .ok();
            renderer.end_batch();

            black_box(renderer);
        });
    });

    group.finish();
}

/// Benchmark real chart rendering with optimizations
fn bench_chart_with_optimizations(c: &mut Criterion) {
    let mut group = c.benchmark_group("chart_optimizations");

    let data = create_test_data(200, false);
    let config = ChartConfig::<Rgb565>::default();
    let viewport = Rectangle::new(Point::new(5, 5), Size::new(54, 54));
    let chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .line_width(2)
        .build()
        .unwrap();

    group.bench_function("standard_chart", |b| {
        b.iter(|| {
            let mut display = MockDisplay::<Rgb565>::new();
            display.set_allow_out_of_bounds_drawing(true);
            display.set_allow_overdraw(true);
            chart
                .draw(
                    black_box(&data),
                    black_box(&config),
                    black_box(viewport),
                    &mut display,
                )
                .unwrap();
        });
    });

    // In practice, charts would use the optimized renderer internally
    // This demonstrates the potential integration
    group.bench_function("optimized_chart_simulation", |b| {
        b.iter(|| {
            let mut display = MockDisplay::<Rgb565>::new();
            display.set_allow_overdraw(true);
            let mut renderer = TFTRenderer::new(display);

            renderer.begin_batch();
            // Simulate optimized chart rendering
            for i in 0..data.len() - 1 {
                let p1 = data.as_slice()[i];
                let p2 = data.as_slice()[i + 1];

                // Transform to viewport coordinates
                let x1 = viewport.top_left.x + ((p1.x / 60.0) * viewport.size.width as f32) as i32;
                let y1 = viewport.top_left.y + viewport.size.height as i32
                    - ((p1.y / 60.0) * viewport.size.height as f32) as i32;
                let x2 = viewport.top_left.x + ((p2.x / 60.0) * viewport.size.width as f32) as i32;
                let y2 = viewport.top_left.y + viewport.size.height as i32
                    - ((p2.y / 60.0) * viewport.size.height as f32) as i32;

                renderer
                    .draw_line_optimized(Point::new(x1, y1), Point::new(x2, y2), Rgb565::BLUE, 2)
                    .ok();
            }
            renderer.end_batch();
            black_box(renderer);
        });
    });

    group.finish();
}

criterion_group! {
    name = display_optimization_benches;
    config = Criterion::default()
        .sample_size(50)
        .measurement_time(std::time::Duration::from_secs(5));
    targets =
        bench_oled_optimizations,
        bench_tft_optimizations,
        bench_epaper_optimizations,
        bench_chart_with_optimizations
}

criterion_main!(display_optimization_benches);
