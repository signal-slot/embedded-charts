//! Comprehensive benchmarking suite for embedded-charts
//!
//! This benchmark suite measures performance across different chart types,
//! data sizes, and rendering scenarios to track optimization progress.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use embedded_charts::prelude::*;
use embedded_graphics::{
    mock_display::MockDisplay,
    pixelcolor::Rgb565,
    prelude::*,
};

/// Generate test data of specified size
fn generate_test_data(size: usize) -> StaticDataSeries<Point2D, 1024> {
    let mut series = StaticDataSeries::new();
    for i in 0..size.min(1024) {
        let x = i as f32;
        let y = (x * 0.1).sin() * 50.0 + 100.0;
        series.push(Point2D::new(x, y)).ok();
    }
    series
}

/// Benchmark line chart rendering with different data sizes
fn bench_line_chart_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("line_chart_rendering");
    
    for size in &[10, 50, 100, 256, 512, 1024] {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let data = generate_test_data(size);
            let chart = LineChart::builder()
                .line_color(Rgb565::BLUE)
                .line_width(2)
                .build()
                .unwrap();
            
            b.iter(|| {
                let mut display = MockDisplay::<Rgb565>::new();
                let viewport = Rectangle::new(Point::zero(), Size::new(320, 240));
                chart.draw(&data, chart.config(), viewport, &mut display).ok();
                black_box(display);
            });
        });
    }
    
    group.finish();
}

/// Benchmark smooth curve rendering with different subdivision levels
fn bench_curve_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("curve_rendering");
    let data = generate_test_data(50);
    
    for subdivisions in &[2, 4, 8, 16] {
        group.bench_with_input(
            BenchmarkId::from_parameter(subdivisions),
            subdivisions,
            |b, &subdivisions| {
                let chart = CurveChart::builder()
                    .line_color(Rgb565::RED)
                    .interpolation(InterpolationType::CubicSpline)
                    .subdivisions(subdivisions)
                    .build()
                    .unwrap();
                
                b.iter(|| {
                    let mut display = MockDisplay::<Rgb565>::new();
                    let viewport = Rectangle::new(Point::zero(), Size::new(320, 240));
                    chart.draw(&data, chart.config(), viewport, &mut display).ok();
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
                let mut display = MockDisplay::<Rgb565>::new();
                let viewport = Rectangle::new(Point::zero(), Size::new(320, 240));
                chart.draw(&data, chart.config(), viewport, &mut display).ok();
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
            let chart = PieChart::builder()
                .radius(100)
                .build()
                .unwrap();
            
            b.iter(|| {
                let mut display = MockDisplay::<Rgb565>::new();
                let viewport = Rectangle::new(Point::zero(), Size::new(320, 240));
                chart.draw(&data, chart.config(), viewport, &mut display).ok();
                black_box(display);
            });
        });
    }
    
    group.finish();
}

/// Benchmark scatter chart rendering
fn bench_scatter_chart_rendering(c: &mut Criterion) {
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
                let mut display = MockDisplay::<Rgb565>::new();
                let viewport = Rectangle::new(Point::zero(), Size::new(320, 240));
                chart.draw(&data, chart.config(), viewport, &mut display).ok();
                black_box(display);
            });
        });
    }
    
    group.finish();
}

/// Benchmark gauge chart rendering
fn bench_gauge_chart_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("gauge_chart_rendering");
    
    let chart = GaugeChart::builder()
        .radius(80)
        .start_angle(225)
        .sweep_angle(270)
        .build()
        .unwrap();
    
    group.bench_function("single_gauge", |b| {
        b.iter(|| {
            let mut display = MockDisplay::<Rgb565>::new();
            let viewport = Rectangle::new(Point::zero(), Size::new(200, 200));
            chart.draw_value(75.0, viewport, &mut display).ok();
            black_box(display);
        });
    });
    
    group.finish();
}

/// Benchmark animation frame updates
fn bench_animation_updates(c: &mut Criterion) {
    let mut group = c.benchmark_group("animation_updates");
    
    let data_from = generate_test_data(50);
    let data_to = generate_test_data(50);
    
    for progress in &[0.0, 0.25, 0.5, 0.75, 1.0] {
        group.bench_with_input(
            BenchmarkId::from_parameter(progress),
            progress,
            |b, &progress| {
                b.iter(|| {
                    let interpolated = data_from.interpolate(&data_to, progress);
                    black_box(interpolated);
                });
            },
        );
    }
    
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
    
    let bounds = DataBounds::new(-100.0, -100.0, 100.0, 100.0);
    let viewport = Rectangle::new(Point::zero(), Size::new(320, 240));
    
    group.bench_function("transform_point", |b| {
        let point = Point2D::new(50.0, 50.0);
        b.iter(|| {
            let transformed = transform_data_point(&point, &bounds, &viewport);
            black_box(transformed);
        });
    });
    
    group.bench_function("transform_batch_100", |b| {
        let points: Vec<Point2D> = (0..100)
            .map(|i| Point2D::new(i as f32 - 50.0, (i as f32).sin() * 50.0))
            .collect();
        
        b.iter(|| {
            let transformed: Vec<Point> = points
                .iter()
                .map(|p| transform_data_point(p, &bounds, &viewport))
                .collect();
            black_box(transformed);
        });
    });
    
    group.finish();
}

// Helper function for coordinate transformation (would be in the actual library)
fn transform_data_point(point: &Point2D, bounds: &DataBounds, viewport: &Rectangle) -> Point {
    let x = ((point.x - bounds.x_min) / (bounds.x_max - bounds.x_min) * viewport.size.width as f32) as i32;
    let y = viewport.size.height as i32
        - ((point.y - bounds.y_min) / (bounds.y_max - bounds.y_min) * viewport.size.height as f32) as i32;
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