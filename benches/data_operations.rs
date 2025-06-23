//! Data operations performance benchmarks
//!
//! Measures performance of data series operations, bounds calculations, and data management

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use embedded_charts::data::{
    bounds::DataBounds,
    point::Point2D,
    series::{MultiSeries, StaticDataSeries},
    DataPoint, DataSeries,
};
use std::hint::black_box;

#[cfg(feature = "animations")]
use embedded_charts::data::series::SlidingWindowSeries;

/// Benchmark StaticDataSeries operations
fn bench_static_data_series(c: &mut Criterion) {
    let mut group = c.benchmark_group("static_data_series");

    // Push operations
    group.bench_function("push_single", |b| {
        b.iter(|| {
            let mut series = StaticDataSeries::<Point2D, 256>::new();
            for i in 0..100 {
                series.push(Point2D::new(i as f32, (i * 2) as f32)).unwrap();
            }
            black_box(series);
        });
    });

    // Extend operations
    group.bench_function("extend_from_slice", |b| {
        let points: Vec<Point2D> = (0..100)
            .map(|i| Point2D::new(i as f32, (i * 2) as f32))
            .collect();

        b.iter(|| {
            let mut series = StaticDataSeries::<Point2D, 256>::new();
            for point in &points {
                series.push(*point).unwrap();
            }
            black_box(series);
        });
    });

    // Iteration performance
    let mut series = StaticDataSeries::<Point2D, 256>::new();
    for i in 0..256 {
        series.push(Point2D::new(i as f32, (i * 2) as f32)).unwrap();
    }

    group.bench_function("iterate_256_points", |b| {
        b.iter(|| {
            let sum: f32 = series.iter().map(|p| p.x() + p.y()).sum();
            black_box(sum);
        });
    });

    // Note: Sorting benchmarks commented out because f32 doesn't implement Ord
    // The library would need to provide sort_by_x_float() and sort_by_y_float() methods
    // that handle NaN values appropriately for f32 coordinates.

    group.finish();
}

/// Benchmark data bounds calculations
fn bench_data_bounds(c: &mut Criterion) {
    let mut group = c.benchmark_group("data_bounds");

    for size in [10, 50, 100, 256] {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut series = StaticDataSeries::<Point2D, 256>::new();
            for i in 0..size {
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

    // Edge cases
    group.bench_function("bounds_with_negatives", |b| {
        let mut series = StaticDataSeries::<Point2D, 256>::new();
        for i in -50..50 {
            series.push(Point2D::new(i as f32, -i as f32)).unwrap();
        }

        b.iter(|| {
            let bounds = series.bounds().unwrap();
            black_box(bounds);
        });
    });

    group.bench_function("bounds_expansion", |b| {
        let initial = DataBounds::new(0.0, 0.0, 10.0, 10.0).unwrap();

        b.iter(|| {
            let mut bounds = initial;
            for i in 0..50 {
                bounds.expand_to_include(&Point2D::new((i * 2) as f32, (i * 3) as f32));
            }
            black_box(bounds);
        });
    });

    group.finish();
}

/// Benchmark MultiSeries operations
fn bench_multi_series(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi_series");

    // Adding series
    group.bench_function("add_series", |b| {
        b.iter(|| {
            let mut multi = MultiSeries::<Point2D, 8, 256>::new();
            for series_idx in 0..4 {
                let mut series = StaticDataSeries::<Point2D, 256>::new();
                for i in 0..50 {
                    series
                        .push(Point2D::new(i as f32, (i + series_idx * 10) as f32))
                        .unwrap();
                }
                multi.add_series(series).unwrap();
            }
            black_box(multi);
        });
    });

    // Combined bounds calculation
    group.bench_function("combined_bounds", |b| {
        let mut multi = MultiSeries::<Point2D, 8, 256>::new();
        for series_idx in 0..4 {
            let mut series = StaticDataSeries::<Point2D, 256>::new();
            for i in 0..100 {
                series
                    .push(Point2D::new(i as f32, (i + series_idx * 20) as f32))
                    .unwrap();
            }
            multi.add_series(series).unwrap();
        }

        b.iter(|| {
            let bounds = multi.combined_bounds().unwrap();
            black_box(bounds);
        });
    });

    // Iteration over all series
    group.bench_function("iterate_all_series", |b| {
        let mut multi = MultiSeries::<Point2D, 8, 256>::new();
        for series_idx in 0..4 {
            let mut series = StaticDataSeries::<Point2D, 256>::new();
            for i in 0..50 {
                series
                    .push(Point2D::new(i as f32, (i + series_idx * 10) as f32))
                    .unwrap();
            }
            multi.add_series(series).unwrap();
        }

        b.iter(|| {
            let sum: f32 = multi
                .iter_series()
                .flat_map(|series| series.iter_ref())
                .map(|p| p.x() + p.y())
                .sum();
            black_box(sum);
        });
    });

    group.finish();
}

/// Benchmark SlidingWindowSeries operations
#[cfg(feature = "animations")]
fn bench_sliding_window(c: &mut Criterion) {
    let mut group = c.benchmark_group("sliding_window");

    // Push and slide operations
    group.bench_function("push_with_slide", |b| {
        let mut window = SlidingWindowSeries::<Point2D, 100>::new();
        for i in 0..100 {
            window.push(Point2D::new(i as f32, i as f32));
        }

        b.iter(|| {
            // Push new point, sliding out the oldest
            window.push(Point2D::new(101.0, 101.0));
            black_box(&window);
        });
    });

    // Window iteration
    group.bench_function("iterate_window", |b| {
        let mut window = SlidingWindowSeries::<Point2D, 100>::new();
        for i in 0..100 {
            window.push(Point2D::new(i as f32, i as f32));
        }

        b.iter(|| {
            let sum: f32 = window.iter().map(|p| p.x() + p.y()).sum();
            black_box(sum);
        });
    });

    // Note: SlidingWindowSeries doesn't have a bounds() method
    // This would need to be calculated manually by iterating through the window

    group.finish();
}

/// Benchmark label operations
fn bench_label_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("label_operations");

    // Label assignment
    group.bench_function("set_labels", |b| {
        b.iter(|| {
            let mut series = StaticDataSeries::<Point2D, 256>::new();
            let labels = ["Data 1", "Data 2", "Data 3", "Data 4", "Data 5"];

            for (i, _label) in labels.iter().enumerate() {
                series.push(Point2D::new(i as f32, i as f32)).unwrap();
            }
            // Series can only have one label for the whole series
            series.set_label("Test Series");
            black_box(series);
        });
    });

    // Label retrieval
    group.bench_function("get_labels", |b| {
        let mut series = StaticDataSeries::<Point2D, 256>::new();
        for i in 0..10 {
            series.push(Point2D::new(i as f32, i as f32)).unwrap();
        }
        series.set_label("Test Series");

        b.iter(|| {
            let label = series.label();
            black_box(label);
        });
    });

    group.finish();
}

/// Benchmark data transformation operations
fn bench_data_transformation(c: &mut Criterion) {
    let mut group = c.benchmark_group("data_transformation");

    // Normalization
    group.bench_function("normalize_values", |b| {
        let mut series = StaticDataSeries::<Point2D, 256>::new();
        for i in 0..100 {
            series
                .push(Point2D::new(i as f32, (i * 10) as f32))
                .unwrap();
        }

        b.iter(|| {
            let bounds = series.bounds().unwrap();
            let normalized: Vec<Point2D> = series
                .iter()
                .map(|p| {
                    let norm_x = (p.x() - bounds.min_x) / (bounds.max_x - bounds.min_x);
                    let norm_y = (p.y() - bounds.min_y) / (bounds.max_y - bounds.min_y);
                    Point2D::new(norm_x, norm_y)
                })
                .collect();
            black_box(normalized);
        });
    });

    // Filtering
    group.bench_function("filter_range", |b| {
        let mut series = StaticDataSeries::<Point2D, 256>::new();
        for i in 0..200 {
            series
                .push(Point2D::new(i as f32, (i % 100) as f32))
                .unwrap();
        }

        b.iter(|| {
            let filtered: Vec<Point2D> = series
                .iter_ref()
                .filter(|p| p.x() >= 50.0 && p.x() <= 150.0)
                .copied()
                .collect();
            black_box(filtered);
        });
    });

    group.finish();
}

// Group benchmarks based on available features
#[cfg(not(feature = "animations"))]
criterion_group!(
    data_benches,
    bench_static_data_series,
    bench_data_bounds,
    bench_multi_series,
    bench_label_operations,
    bench_data_transformation
);

#[cfg(feature = "animations")]
criterion_group!(
    data_benches,
    bench_static_data_series,
    bench_data_bounds,
    bench_multi_series,
    bench_sliding_window,
    bench_label_operations,
    bench_data_transformation
);

criterion_main!(data_benches);
