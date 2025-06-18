//! Chart rendering performance benchmarks
//!
//! Measures rendering performance for different chart types under various conditions

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use embedded_charts::{
    chart::{
        bar::{BarChart, BarOrientation, BarWidth},
        line::{LineChart, MarkerShape, MarkerStyle},
        traits::{Chart, ChartBuilder, ChartConfig},
    },
    data::{point::Point2D, series::StaticDataSeries},
};

#[cfg(feature = "scatter")]
use embedded_charts::chart::scatter::ScatterChart;

#[cfg(feature = "pie")]
use embedded_charts::chart::pie::PieChart;
use embedded_graphics::{
    mock_display::MockDisplay, pixelcolor::Rgb565, prelude::*, primitives::Rectangle,
};

/// Create test data with specified number of points
fn create_test_data(size: usize) -> StaticDataSeries<Point2D, 256> {
    let mut data = StaticDataSeries::new();
    for i in 0..size.min(256) {
        let x = i as f32;
        let y = (x * 0.1).sin() * 50.0 + 50.0;
        data.push(Point2D::new(x, y)).unwrap();
    }
    data
}

/// Create a mock display for benchmarking
fn create_display() -> MockDisplay<Rgb565> {
    let mut display = MockDisplay::<Rgb565>::new();
    display.set_allow_overdraw(true);
    display.set_allow_out_of_bounds_drawing(true);
    display
}

/// Benchmark line chart rendering with different data sizes
fn bench_line_chart_data_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("line_chart_data_scaling");
    let config = ChartConfig::<Rgb565>::default();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(320, 240));

    for size in [10, 50, 100, 200, 256] {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let data = create_test_data(size);
            let chart = LineChart::builder()
                .line_color(Rgb565::BLUE)
                .line_width(2)
                .build()
                .unwrap();

            b.iter(|| {
                let mut display = create_display();
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

/// Benchmark line chart with different features enabled
fn bench_line_chart_features(c: &mut Criterion) {
    let mut group = c.benchmark_group("line_chart_features");
    let data = create_test_data(100);
    let config = ChartConfig::<Rgb565>::default();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(320, 240));

    // Basic line
    group.bench_function("basic", |b| {
        let chart = LineChart::builder()
            .line_color(Rgb565::BLUE)
            .line_width(2)
            .build()
            .unwrap();

        b.iter(|| {
            let mut display = create_display();
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

    // With markers
    group.bench_function("with_markers", |b| {
        let chart = LineChart::builder()
            .line_color(Rgb565::BLUE)
            .line_width(2)
            .with_markers(MarkerStyle {
                shape: MarkerShape::Circle,
                size: 6,
                color: Rgb565::RED,
                visible: true,
            })
            .build()
            .unwrap();

        b.iter(|| {
            let mut display = create_display();
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

    // With area fill
    group.bench_function("with_area_fill", |b| {
        let chart = LineChart::builder()
            .line_color(Rgb565::BLUE)
            .line_width(2)
            .fill_area(Rgb565::CSS_LIGHT_BLUE)
            .build()
            .unwrap();

        b.iter(|| {
            let mut display = create_display();
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

    #[cfg(feature = "smooth-curves")]
    group.bench_function("with_smooth_curves", |b| {
        let chart = LineChart::builder()
            .line_color(Rgb565::BLUE)
            .line_width(2)
            .smooth(true)
            .build()
            .unwrap();

        b.iter(|| {
            let mut display = create_display();
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

    group.finish();
}

/// Benchmark bar chart rendering
fn bench_bar_chart(c: &mut Criterion) {
    let mut group = c.benchmark_group("bar_chart");
    let data = create_test_data(20);
    let config = ChartConfig::<Rgb565>::default();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(320, 240));

    // Vertical bars
    group.bench_function("vertical", |b| {
        let chart = BarChart::builder()
            .orientation(BarOrientation::Vertical)
            .bar_width(BarWidth::Fixed(10))
            .colors(&[Rgb565::BLUE])
            .build()
            .unwrap();

        b.iter(|| {
            let mut display = create_display();
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

    // Horizontal bars
    group.bench_function("horizontal", |b| {
        let chart = BarChart::builder()
            .orientation(BarOrientation::Horizontal)
            .bar_width(BarWidth::Fixed(10))
            .colors(&[Rgb565::RED])
            .build()
            .unwrap();

        b.iter(|| {
            let mut display = create_display();
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

    // Multi-color bars
    group.bench_function("multi_color", |b| {
        let chart = BarChart::builder()
            .bar_width(BarWidth::Auto)
            .colors(&[Rgb565::BLUE, Rgb565::RED, Rgb565::GREEN, Rgb565::YELLOW])
            .build()
            .unwrap();

        b.iter(|| {
            let mut display = create_display();
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

    group.finish();
}

/// Benchmark pie chart rendering
#[cfg(feature = "pie")]
fn bench_pie_chart(c: &mut Criterion) {
    let mut group = c.benchmark_group("pie_chart");
    let config = ChartConfig::<Rgb565>::default();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(240, 240));

    for slice_count in [4, 8, 12, 16] {
        group.bench_with_input(
            BenchmarkId::from_parameter(slice_count),
            &slice_count,
            |b, &slice_count| {
                let data = create_test_data(slice_count);
                let chart = PieChart::builder()
                    .colors(&[
                        Rgb565::BLUE,
                        Rgb565::RED,
                        Rgb565::GREEN,
                        Rgb565::YELLOW,
                        Rgb565::CYAN,
                        Rgb565::MAGENTA,
                    ])
                    .build()
                    .unwrap();

                b.iter(|| {
                    let mut display = create_display();
                    chart
                        .draw(
                            black_box(&data),
                            black_box(&config),
                            black_box(viewport),
                            &mut display,
                        )
                        .unwrap();
                });
            },
        );
    }

    group.finish();
}

/// Benchmark scatter chart rendering
#[cfg(feature = "scatter")]
fn bench_scatter_chart(c: &mut Criterion) {
    let mut group = c.benchmark_group("scatter_chart");
    let data = create_test_data(50);
    let config = ChartConfig::<Rgb565>::default();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(320, 240));

    // Basic scatter
    group.bench_function("basic", |b| {
        let chart = ScatterChart::builder()
            .point_color(Rgb565::BLUE)
            .point_size(4)
            .build()
            .unwrap();

        b.iter(|| {
            let mut display = create_display();
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

    // Note: Connection style benchmark commented out as the API has changed
    // The scatter chart builder may not have a connection_style method anymore

    group.finish();
}

/// Benchmark viewport scaling performance
fn bench_viewport_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("viewport_scaling");
    let data = create_test_data(100);
    let config = ChartConfig::<Rgb565>::default();
    let chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .line_width(2)
        .build()
        .unwrap();

    for size in [64, 128, 256, 512] {
        group.throughput(Throughput::Elements((size * size) as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let viewport = Rectangle::new(Point::new(0, 0), Size::new(size, size));

            b.iter(|| {
                let mut display = create_display();
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

/// Benchmark rendering with margins and configuration
fn bench_chart_configuration(c: &mut Criterion) {
    let mut group = c.benchmark_group("chart_configuration");
    let data = create_test_data(100);
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(320, 240));
    let chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .line_width(2)
        .build()
        .unwrap();

    // Default config
    group.bench_function("default", |b| {
        let config = ChartConfig::<Rgb565>::default();

        b.iter(|| {
            let mut display = create_display();
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

    // With margins
    group.bench_function("with_margins", |b| {
        let config = ChartConfig::<Rgb565> {
            margins: embedded_charts::chart::traits::Margins {
                top: 20,
                right: 20,
                bottom: 30,
                left: 30,
            },
            ..Default::default()
        };

        b.iter(|| {
            let mut display = create_display();
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

    // With title and background
    group.bench_function("with_title_and_bg", |b| {
        let mut title = heapless::String::new();
        title.push_str("Performance Test").unwrap();
        let config = ChartConfig::<Rgb565> {
            title: Some(title),
            background_color: Some(Rgb565::BLACK),
            ..Default::default()
        };

        b.iter(|| {
            let mut display = create_display();
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

    group.finish();
}

// Group benchmarks based on available features
criterion_group! {
    name = rendering_benches;
    config = Criterion::default();
    targets =
        bench_line_chart_data_scaling,
        bench_line_chart_features,
        bench_bar_chart,
        bench_viewport_scaling,
        bench_chart_configuration
}

#[cfg(feature = "pie")]
criterion_group! {
    name = pie_benches;
    config = Criterion::default();
    targets = bench_pie_chart
}

#[cfg(feature = "scatter")]
criterion_group! {
    name = scatter_benches;
    config = Criterion::default();
    targets = bench_scatter_chart
}

// Main function that includes all available benchmark groups
#[cfg(all(not(feature = "pie"), not(feature = "scatter")))]
criterion_main!(rendering_benches);

#[cfg(all(feature = "pie", not(feature = "scatter")))]
criterion_main!(rendering_benches, pie_benches);

#[cfg(all(not(feature = "pie"), feature = "scatter"))]
criterion_main!(rendering_benches, scatter_benches);

#[cfg(all(feature = "pie", feature = "scatter"))]
criterion_main!(rendering_benches, pie_benches, scatter_benches);
