//! Baseline performance measurements for v0.4.0
//!
//! Quick baseline measurements to establish current performance

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use embedded_charts::{
    chart::{
        line::LineChart,
        traits::{Chart, ChartBuilder, ChartConfig},
    },
    data::{point::Point2D, series::StaticDataSeries},
};
use embedded_graphics::{
    mock_display::MockDisplay, pixelcolor::Rgb565, prelude::*, primitives::Rectangle,
};
use std::hint::black_box;

/// Create test data
fn create_test_data(size: usize) -> StaticDataSeries<Point2D, 256> {
    let mut data = StaticDataSeries::new();
    for i in 0..size.min(256) {
        let x = i as f32;
        let y = (x * 0.1).sin() * 50.0 + 50.0;
        data.push(Point2D::new(x, y)).unwrap();
    }
    data
}

/// Baseline: Line chart rendering performance
fn bench_baseline_line_chart(c: &mut Criterion) {
    let mut group = c.benchmark_group("baseline_line_chart");

    for size in [50, 100, 200] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let data = create_test_data(size);
            let config = ChartConfig::<Rgb565>::default();
            let viewport = Rectangle::new(Point::new(40, 40), Size::new(240, 160));
            let chart = LineChart::builder()
                .line_color(Rgb565::BLUE)
                .line_width(2)
                .build()
                .unwrap();

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
    }

    group.finish();
}

/// Baseline: Memory allocation patterns
fn bench_baseline_memory_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("baseline_memory");

    group.bench_function("data_series_allocation", |b| {
        b.iter(|| {
            let mut data = StaticDataSeries::<Point2D, 256>::new();
            for i in 0..200 {
                let x = i as f32;
                let y = (x * 0.1).sin() * 50.0;
                data.push(Point2D::new(x, y)).ok();
            }
            black_box(data);
        });
    });

    group.bench_function("display_buffer_allocation", |b| {
        b.iter(|| {
            let display = MockDisplay::<Rgb565>::new();
            black_box(display);
        });
    });

    group.finish();
}

/// Baseline: Coordinate transformation performance
fn bench_baseline_transformations(c: &mut Criterion) {
    let mut group = c.benchmark_group("baseline_transform");

    let data = create_test_data(100);
    let _viewport = Rectangle::new(Point::new(0, 0), Size::new(320, 240));

    group.bench_function("point_transformation", |b| {
        b.iter(|| {
            let mut sum = 0i32;
            for point in data.as_slice() {
                // Simulate coordinate transformation
                let screen_x = ((point.x / 100.0) * 320.0) as i32;
                let screen_y = (240.0 - (point.y / 100.0) * 240.0) as i32;
                sum = sum.wrapping_add(screen_x).wrapping_add(screen_y);
            }
            black_box(sum);
        });
    });

    group.finish();
}

criterion_group! {
    name = baseline_benches;
    config = Criterion::default()
        .sample_size(50)
        .measurement_time(std::time::Duration::from_secs(5));
    targets =
        bench_baseline_line_chart,
        bench_baseline_memory_allocation,
        bench_baseline_transformations
}

criterion_main!(baseline_benches);
