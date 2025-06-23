//! Simple performance benchmarks for embedded-charts
//!
//! Basic benchmarks to establish performance baseline

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use embedded_charts::{
    chart::{
        bar::{BarChart, BarOrientation, BarWidth},
        line::LineChart,
        traits::{Chart, ChartBuilder, ChartConfig},
    },
    data::{point::Point2D, series::StaticDataSeries, DataPoint, DataSeries},
};
use embedded_graphics::{
    mock_display::MockDisplay, pixelcolor::Rgb565, prelude::*, primitives::Rectangle,
};

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

/// Create test data
fn create_test_data(size: usize) -> StaticDataSeries<Point2D, 256> {
    let mut data = StaticDataSeries::new();
    for i in 0..size.min(256) {
        let x = (i as f32 / size as f32) * 50.0; // Scale x to fit in 60x60
        let y = (x * 0.1).sin() * 20.0 + 30.0; // Scale y to fit in 60x60
        data.push(Point2D::new(x, y)).unwrap();
    }
    data
}

/// Benchmark line chart rendering
fn bench_line_chart(c: &mut Criterion) {
    c.bench_function("line_chart_render", |b| {
        let data = create_test_data(100);
        let config = ChartConfig::<Rgb565>::default();
        let viewport = Rectangle::new(Point::new(20, 20), Size::new(20, 20));

        let chart = LineChart::builder()
            .line_color(Rgb565::BLUE)
            .line_width(2)
            .build()
            .unwrap();

        b.iter(|| {
            let mut display = create_test_display::<Rgb565>();
            display.set_allow_overdraw(true);
            display.set_allow_out_of_bounds_drawing(true);

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

/// Benchmark bar chart rendering
fn bench_bar_chart(c: &mut Criterion) {
    c.bench_function("bar_chart_render", |b| {
        let data = create_test_data(20);
        let config = ChartConfig::<Rgb565>::default();
        let viewport = Rectangle::new(Point::new(20, 20), Size::new(20, 20));

        let chart = BarChart::builder()
            .orientation(BarOrientation::Vertical)
            .bar_width(BarWidth::Fixed(10))
            .colors(&[Rgb565::BLUE])
            .build()
            .unwrap();

        b.iter(|| {
            let mut display = create_test_display::<Rgb565>();
            display.set_allow_overdraw(true);
            display.set_allow_out_of_bounds_drawing(true);

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

/// Benchmark data series operations
fn bench_data_operations(c: &mut Criterion) {
    c.bench_function("data_series_push", |b| {
        b.iter(|| {
            let mut series = StaticDataSeries::<Point2D, 256>::new();
            for i in 0..100 {
                series.push(Point2D::new(i as f32, (i * 2) as f32)).unwrap();
            }
            black_box(series);
        });
    });

    c.bench_function("data_series_iteration", |b| {
        let mut series = StaticDataSeries::<Point2D, 256>::new();
        for i in 0..100 {
            series.push(Point2D::new(i as f32, (i * 2) as f32)).unwrap();
        }

        b.iter(|| {
            let sum: f32 = series.iter().map(|p| p.x() + p.y()).sum();
            black_box(sum);
        });
    });

    c.bench_function("data_series_bounds", |b| {
        let mut series = StaticDataSeries::<Point2D, 256>::new();
        for i in 0..100 {
            let x = i as f32;
            let y = (x * 0.1).sin() * 50.0 + 50.0;
            series.push(Point2D::new(x, y)).unwrap();
        }

        b.iter(|| {
            let bounds = series.bounds().unwrap();
            black_box(bounds);
        });
    });
}

/// Benchmark interpolation
#[cfg(feature = "smooth-curves")]
fn bench_interpolation(c: &mut Criterion) {
    use embedded_charts::math::interpolation::{
        CurveInterpolator, InterpolationConfig, InterpolationType,
    };

    c.bench_function("cubic_spline_interpolation", |b| {
        let points = vec![
            Point2D::new(0.0, 10.0),
            Point2D::new(1.0, 20.0),
            Point2D::new(2.0, 5.0),
            Point2D::new(3.0, 25.0),
            Point2D::new(4.0, 15.0),
        ];

        let config = InterpolationConfig {
            interpolation_type: InterpolationType::CubicSpline,
            subdivisions: 10,
            tension: 0.5,
            closed: false,
        };

        b.iter(|| {
            let interpolated = CurveInterpolator::interpolate(&points, &config).unwrap();
            black_box(interpolated);
        });
    });
}

#[cfg(feature = "smooth-curves")]
criterion_group!(
    benches,
    bench_line_chart,
    bench_bar_chart,
    bench_data_operations,
    bench_interpolation
);

#[cfg(not(feature = "smooth-curves"))]
criterion_group!(
    benches,
    bench_line_chart,
    bench_bar_chart,
    bench_data_operations
);

criterion_main!(benches);
