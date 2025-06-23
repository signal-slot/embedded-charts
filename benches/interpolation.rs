//! Interpolation performance benchmarks
//!
//! Measures performance of different interpolation algorithms

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use embedded_charts::{
    data::{point::Point2D, series::StaticDataSeries},
    math::interpolation::{CurveInterpolator, InterpolationConfig, InterpolationType},
};
use std::hint::black_box;

/// Create test points for interpolation
fn create_test_points(size: usize) -> Vec<Point2D> {
    (0..size)
        .map(|i| {
            let x = i as f32;
            let y = (x * 0.2).sin() * 30.0 + 50.0 + (x * 0.05).cos() * 10.0;
            Point2D::new(x, y)
        })
        .collect()
}

/// Benchmark different interpolation types
fn bench_interpolation_types(c: &mut Criterion) {
    let mut group = c.benchmark_group("interpolation_types");
    let points = create_test_points(10);

    // Linear interpolation
    group.bench_function("linear", |b| {
        let config = InterpolationConfig {
            interpolation_type: InterpolationType::Linear,
            subdivisions: 10,
            tension: 0.5,
            closed: false,
        };

        b.iter(|| {
            let interpolated = CurveInterpolator::interpolate(&points, &config).unwrap();
            black_box(interpolated);
        });
    });

    // Cubic spline interpolation
    group.bench_function("cubic_spline", |b| {
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

    // Catmull-Rom interpolation
    group.bench_function("catmull_rom", |b| {
        let config = InterpolationConfig {
            interpolation_type: InterpolationType::CatmullRom,
            subdivisions: 10,
            tension: 0.5,
            closed: false,
        };

        b.iter(|| {
            let interpolated = CurveInterpolator::interpolate(&points, &config).unwrap();
            black_box(interpolated);
        });
    });

    // Bezier interpolation
    group.bench_function("bezier", |b| {
        let config = InterpolationConfig {
            interpolation_type: InterpolationType::Bezier,
            subdivisions: 10,
            tension: 0.5,
            closed: false,
        };

        b.iter(|| {
            let interpolated = CurveInterpolator::interpolate(&points, &config).unwrap();
            black_box(interpolated);
        });
    });

    group.finish();
}

/// Benchmark interpolation with different subdivision counts
fn bench_subdivision_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("subdivision_scaling");
    let points = create_test_points(10);

    for subdivisions in [5, 10, 20, 40] {
        group.throughput(Throughput::Elements(subdivisions as u64));
        group.bench_with_input(
            BenchmarkId::new("cubic_spline", subdivisions),
            &subdivisions,
            |b, &subdivisions| {
                let config = InterpolationConfig {
                    interpolation_type: InterpolationType::CubicSpline,
                    subdivisions,
                    tension: 0.5,
                    closed: false,
                };

                b.iter(|| {
                    let interpolated = CurveInterpolator::interpolate(&points, &config).unwrap();
                    black_box(interpolated);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark interpolation with different input sizes
fn bench_input_size_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("input_size_scaling");
    let config = InterpolationConfig {
        interpolation_type: InterpolationType::CubicSpline,
        subdivisions: 10,
        tension: 0.5,
        closed: false,
    };

    for size in [5, 10, 20, 50] {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let points = create_test_points(size);

            b.iter(|| {
                let interpolated = CurveInterpolator::interpolate(&points, &config).unwrap();
                black_box(interpolated);
            });
        });
    }

    group.finish();
}

/// Benchmark series interpolation
fn bench_series_interpolation(c: &mut Criterion) {
    let mut group = c.benchmark_group("series_interpolation");

    // Prepare series with different sizes
    let sizes = [10, 25, 50, 100];

    for size in sizes {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut series = StaticDataSeries::<Point2D, 256>::new();
            for i in 0..size {
                let x = i as f32;
                let y = (x * 0.1).sin() * 50.0 + 50.0;
                series.push(Point2D::new(x, y)).unwrap();
            }

            let config = InterpolationConfig {
                interpolation_type: InterpolationType::CatmullRom,
                subdivisions: 8,
                tension: 0.5,
                closed: false,
            };

            b.iter(|| {
                // Convert series to points for interpolation
                let points: Vec<Point2D> = series.iter_ref().copied().collect();
                let interpolated = CurveInterpolator::interpolate(&points, &config).unwrap();
                black_box(interpolated);
            });
        });
    }

    group.finish();
}

/// Benchmark tension parameter effects
fn bench_tension_effects(c: &mut Criterion) {
    let mut group = c.benchmark_group("tension_effects");
    let points = create_test_points(20);

    for tension in [0.0, 0.25, 0.5, 0.75, 1.0] {
        group.bench_with_input(
            BenchmarkId::new("catmull_rom", format!("{tension:.2}")),
            &tension,
            |b, &tension| {
                let config = InterpolationConfig {
                    interpolation_type: InterpolationType::CatmullRom,
                    subdivisions: 10,
                    tension,
                    closed: false,
                };

                b.iter(|| {
                    let interpolated = CurveInterpolator::interpolate(&points, &config).unwrap();
                    black_box(interpolated);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark edge cases and special scenarios
fn bench_edge_cases(c: &mut Criterion) {
    let mut group = c.benchmark_group("edge_cases");

    // Very few points (minimum for spline)
    group.bench_function("minimal_points", |b| {
        let points = vec![
            Point2D::new(0.0, 10.0),
            Point2D::new(1.0, 20.0),
            Point2D::new(2.0, 15.0),
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

    // Straight line (no curvature)
    group.bench_function("straight_line", |b| {
        let points: Vec<Point2D> = (0..20).map(|i| Point2D::new(i as f32, i as f32)).collect();

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

    // Sharp angles
    group.bench_function("sharp_angles", |b| {
        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(10.0, 50.0),
            Point2D::new(20.0, 0.0),
            Point2D::new(30.0, 50.0),
            Point2D::new(40.0, 0.0),
        ];

        let config = InterpolationConfig {
            interpolation_type: InterpolationType::Bezier,
            subdivisions: 15,
            tension: 0.7,
            closed: false,
        };

        b.iter(|| {
            let interpolated = CurveInterpolator::interpolate(&points, &config).unwrap();
            black_box(interpolated);
        });
    });

    group.finish();
}

/// Benchmark smoothing operations
fn bench_smoothing(c: &mut Criterion) {
    let mut group = c.benchmark_group("smoothing");

    // Noisy data smoothing
    let noisy_points: Vec<Point2D> = (0..50)
        .map(|i| {
            let x = i as f32;
            let base_y = (x * 0.1).sin() * 30.0 + 50.0;
            let noise = ((i * 17) % 10) as f32 - 5.0; // Pseudo-random noise
            Point2D::new(x, base_y + noise)
        })
        .collect();

    for factor in [0.0, 0.3, 0.5, 0.7, 1.0] {
        group.bench_with_input(
            BenchmarkId::new("smooth_points", format!("{factor:.1}")),
            &factor,
            |b, &factor| {
                b.iter(|| {
                    // Create a series from points for smoothing
                    let mut series = StaticDataSeries::<Point2D, 256>::new();
                    for point in &noisy_points {
                        series.push(*point).unwrap();
                    }
                    // Convert series to points for smoothing
                    let points: Vec<Point2D> = series.iter_ref().copied().collect();
                    let smoothed = CurveInterpolator::smooth_series(&points, factor, 1).unwrap();
                    black_box(smoothed);
                });
            },
        );
    }

    // Series smoothing
    group.bench_function("smooth_series", |b| {
        let mut series = StaticDataSeries::<Point2D, 256>::new();
        for point in &noisy_points {
            series.push(*point).unwrap();
        }

        b.iter(|| {
            // Convert series to points for smoothing
            let points: Vec<Point2D> = series.iter_ref().copied().collect();
            let smoothed = CurveInterpolator::smooth_series(&points, 0.5, 1).unwrap();
            black_box(smoothed);
        });
    });

    group.finish();
}

criterion_group!(
    interpolation_benches,
    bench_interpolation_types,
    bench_subdivision_scaling,
    bench_input_size_scaling,
    bench_series_interpolation,
    bench_tension_effects,
    bench_edge_cases,
    bench_smoothing
);

criterion_main!(interpolation_benches);
