//! Comprehensive benchmarking suite for embedded-charts
//!
//! This benchmark suite measures performance across different chart types,
//! data sizes, and rendering scenarios to track optimization progress.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
#[cfg(feature = "gauge")]
use embedded_charts::chart::{GaugeChart, GaugeType};
#[cfg(feature = "scatter")]
use embedded_charts::chart::{PointShape, ScatterChart};
use embedded_charts::prelude::*;
use embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb565};
use std::hint::black_box;

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

/// Generate test data of specified size for line charts
fn generate_line_data(size: usize) -> StaticDataSeries<Point2D, 256> {
    let mut series = StaticDataSeries::new();
    for i in 0..size.min(256) {
        let x = i as f32;
        let y = (x * 0.1).sin() * 50.0 + 100.0;
        series.push(Point2D::new(x, y)).ok();
    }
    series
}

/// Generate test data of specified size for other charts
fn generate_test_data(size: usize) -> StaticDataSeries<Point2D, 256> {
    let mut series = StaticDataSeries::new();
    for i in 0..size.min(256) {
        let x = i as f32;
        let y = (x * 0.1).sin() * 50.0 + 100.0;
        series.push(Point2D::new(x, y)).ok();
    }
    series
}

/// Benchmark line chart rendering with different data sizes
fn bench_line_chart_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("line_chart_rendering");

    for size in &[10, 50, 100, 256] {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let data = generate_line_data(size);
            let chart = LineChart::builder()
                .line_color(Rgb565::BLUE)
                .line_width(2)
                .build()
                .unwrap();

            b.iter(|| {
                let mut display = create_test_display::<Rgb565>();
                // Use larger margins to ensure no out-of-bounds drawing
                let viewport = Rectangle::new(Point::new(80, 80), Size::new(160, 80));
                chart
                    .draw(&data, &ChartConfig::default(), viewport, &mut display)
                    .ok();
                black_box(display);
            });
        });
    }

    group.finish();
}

/// Benchmark smooth curve rendering with different subdivision levels
fn bench_curve_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("curve_rendering");
    let data = generate_line_data(50);

    for subdivisions in &[2, 4, 8, 16] {
        group.bench_with_input(
            BenchmarkId::from_parameter(subdivisions),
            subdivisions,
            |b, &subdivisions| {
                let chart = CurveChart::builder()
                    .line_color(Rgb565::RED)
                    .interpolation_type(InterpolationType::CubicSpline)
                    .subdivisions(subdivisions)
                    .build()
                    .unwrap();

                b.iter(|| {
                    let mut display = create_test_display::<Rgb565>();
                    // Use larger margins to ensure no out-of-bounds drawing
                    let viewport = Rectangle::new(Point::new(80, 80), Size::new(160, 80));
                    chart
                        .draw(&data, &ChartConfig::default(), viewport, &mut display)
                        .ok();
                    black_box(display);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark bar chart rendering
fn bench_bar_chart_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("bar_chart_rendering");

    for size in &[5, 10, 20, 50, 100] {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let data = generate_test_data(size);
            let chart = BarChart::builder()
                .bar_width(BarWidth::Auto)
                .build()
                .unwrap();

            b.iter(|| {
                let mut display = create_test_display::<Rgb565>();
                // Use larger margins to ensure no out-of-bounds drawing
                let viewport = Rectangle::new(Point::new(80, 80), Size::new(160, 80));
                chart
                    .draw(&data, &ChartConfig::default(), viewport, &mut display)
                    .ok();
                black_box(display);
            });
        });
    }

    group.finish();
}

/// Benchmark pie chart rendering
fn bench_pie_chart_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("pie_chart_rendering");

    for slices in &[3, 5, 8, 12, 16] {
        group.bench_with_input(BenchmarkId::from_parameter(slices), slices, |b, &slices| {
            let data = generate_test_data(slices);
            let chart = PieChart::builder().radius(100).build().unwrap();

            b.iter(|| {
                let mut display = create_test_display::<Rgb565>();
                // Use larger margins to ensure no out-of-bounds drawing
                let viewport = Rectangle::new(Point::new(80, 80), Size::new(160, 80));
                chart
                    .draw(&data, &ChartConfig::default(), viewport, &mut display)
                    .ok();
                black_box(display);
            });
        });
    }

    group.finish();
}

/// Benchmark scatter chart rendering
fn bench_scatter_chart_rendering(c: &mut Criterion) {
    #[cfg(feature = "scatter")]
    {
        let mut group = c.benchmark_group("scatter_chart_rendering");

        for size in &[20, 50, 100, 200, 500] {
            group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
                let data = generate_test_data(size);
                let chart = ScatterChart::builder()
                    .point_size(4)
                    .point_shape(PointShape::Circle)
                    .build()
                    .unwrap();

                b.iter(|| {
                    let mut display = create_test_display::<Rgb565>();
                    // Use larger margins to ensure no out-of-bounds drawing
                    let viewport = Rectangle::new(Point::new(80, 80), Size::new(160, 80));
                    chart
                        .draw(&data, &ChartConfig::default(), viewport, &mut display)
                        .ok();
                    black_box(display);
                });
            });
        }

        group.finish();
    }
    #[cfg(not(feature = "scatter"))]
    {
        let _ = c; // Suppress unused warning
    }
}

/// Benchmark gauge chart rendering
fn bench_gauge_chart_rendering(c: &mut Criterion) {
    #[cfg(feature = "gauge")]
    {
        let mut group = c.benchmark_group("gauge_chart_rendering");

        let chart = GaugeChart::builder()
            .radius(80)
            .gauge_type(GaugeType::ThreeQuarter)
            .build()
            .unwrap();

        group.bench_function("single_gauge", |b| {
            let mut gauge_data = StaticDataSeries::<Point2D, 1>::new();
            gauge_data.push(Point2D::new(0.0, 75.0)).ok();

            b.iter(|| {
                let mut display = create_test_display::<Rgb565>();
                // Use larger margins to ensure no out-of-bounds drawing
                let viewport = Rectangle::new(Point::new(80, 80), Size::new(120, 120));
                chart
                    .draw(&gauge_data, &ChartConfig::default(), viewport, &mut display)
                    .ok();
                black_box(display);
            });
        });

        group.finish();
    }
    #[cfg(not(feature = "gauge"))]
    {
        let _ = c; // Suppress unused warning
    }
}

/// Benchmark animation frame updates
fn bench_animation_updates(c: &mut Criterion) {
    let mut group = c.benchmark_group("animation_updates");

    // TODO: Implement interpolation for StaticDataSeries when animations feature is added
    // For now, just benchmark data cloning as a placeholder
    let data = generate_test_data(50);

    group.bench_function("data_clone", |b| {
        b.iter(|| {
            let cloned = data.clone();
            black_box(cloned);
        });
    });

    group.finish();
}

/// Benchmark memory allocation patterns
fn bench_memory_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_patterns");

    // Benchmark data series operations
    group.bench_function("series_push_1024", |b| {
        b.iter(|| {
            let mut series = StaticDataSeries::<Point2D, 1024>::new();
            for i in 0..1024 {
                series.push(Point2D::new(i as f32, i as f32)).ok();
            }
            black_box(series);
        });
    });

    group.bench_function("series_clear_refill", |b| {
        let mut series = generate_test_data(256);
        b.iter(|| {
            series.clear();
            for i in 0..256 {
                series.push(Point2D::new(i as f32, i as f32)).ok();
            }
            black_box(&series);
        });
    });

    group.finish();
}

/// Benchmark coordinate transformation
fn bench_coordinate_transform(c: &mut Criterion) {
    let mut group = c.benchmark_group("coordinate_transform");

    let bounds = DataBounds::<f32, f32>::new(-100.0, 100.0, -100.0, 100.0).unwrap();
    // Use larger margins to ensure no out-of-bounds drawing
    let viewport = Rectangle::new(Point::new(80, 80), Size::new(160, 80));

    group.bench_function("transform_point", |b| {
        let point = Point2D::new(50.0, 50.0);
        b.iter(|| {
            let transformed = transform_data_point(&point, &bounds, &viewport);
            black_box(transformed);
        });
    });

    group.bench_function("transform_batch_100", |b| {
        let points: std::vec::Vec<Point2D> = (0..100)
            .map(|i| Point2D::new(i as f32 - 50.0, (i as f32).sin() * 50.0))
            .collect();

        b.iter(|| {
            let transformed: std::vec::Vec<Point> = points
                .iter()
                .map(|p| transform_data_point(p, &bounds, &viewport))
                .collect();
            black_box(transformed);
        });
    });

    group.finish();
}

// Helper function for coordinate transformation (would be in the actual library)
fn transform_data_point(
    point: &Point2D,
    bounds: &DataBounds<f32, f32>,
    viewport: &Rectangle,
) -> Point {
    let x = ((point.x - bounds.min_x) / (bounds.max_x - bounds.min_x) * viewport.size.width as f32)
        as i32;
    let y = viewport.size.height as i32
        - ((point.y - bounds.min_y) / (bounds.max_y - bounds.min_y) * viewport.size.height as f32)
            as i32;
    Point::new(viewport.top_left.x + x, viewport.top_left.y + y)
}

criterion_group!(
    benches,
    bench_line_chart_rendering,
    bench_curve_rendering,
    bench_bar_chart_rendering,
    bench_pie_chart_rendering,
    bench_scatter_chart_rendering,
    bench_gauge_chart_rendering,
    bench_animation_updates,
    bench_memory_patterns,
    bench_coordinate_transform,
);

criterion_main!(benches);
