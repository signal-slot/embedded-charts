//! Memory usage benchmarks and profiling
//!
//! This module measures memory usage patterns and allocation behavior
//! to ensure the library stays within embedded system constraints.

use criterion::{criterion_group, criterion_main, Criterion};
#[cfg(feature = "gauge")]
use embedded_charts::chart::GaugeChart;
#[cfg(feature = "scatter")]
use embedded_charts::chart::ScatterChart;
use embedded_charts::prelude::*;
use std::hint::black_box;
use std::mem;

/// Measure static memory usage of different chart types
fn bench_chart_memory_footprint(c: &mut Criterion) {
    let mut group = c.benchmark_group("chart_memory_footprint");

    group.bench_function("line_chart_size", |b| {
        b.iter(|| {
            let chart = LineChart::builder()
                .line_color(embedded_graphics::pixelcolor::Rgb565::BLUE)
                .build()
                .unwrap();
            black_box(mem::size_of_val(&chart));
        });
    });

    group.bench_function("bar_chart_size", |b| {
        b.iter(|| {
            let chart: BarChart<embedded_graphics::pixelcolor::Rgb565> = BarChart::builder()
                .bar_width(BarWidth::Auto)
                .build()
                .unwrap();
            black_box(mem::size_of_val(&chart));
        });
    });

    group.bench_function("pie_chart_size", |b| {
        b.iter(|| {
            let chart: PieChart<embedded_graphics::pixelcolor::Rgb565> =
                PieChart::builder().radius(100).build().unwrap();
            black_box(mem::size_of_val(&chart));
        });
    });

    #[cfg(feature = "gauge")]
    group.bench_function("gauge_chart_size", |b| {
        b.iter(|| {
            let chart: GaugeChart<embedded_graphics::pixelcolor::Rgb565> =
                GaugeChart::builder().radius(80).build().unwrap();
            black_box(mem::size_of_val(&chart));
        });
    });

    #[cfg(feature = "scatter")]
    group.bench_function("scatter_chart_size", |b| {
        b.iter(|| {
            let chart: ScatterChart<embedded_graphics::pixelcolor::Rgb565> =
                ScatterChart::builder().point_size(4).build().unwrap();
            black_box(mem::size_of_val(&chart));
        });
    });

    group.finish();
}

/// Measure data series memory usage with different capacities
fn bench_data_series_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("data_series_memory");

    // Measure empty series
    group.bench_function("series_empty_32", |b| {
        b.iter(|| {
            let series = StaticDataSeries::<Point2D, 32>::new();
            black_box(mem::size_of_val(&series));
        });
    });

    group.bench_function("series_empty_256", |b| {
        b.iter(|| {
            let series = StaticDataSeries::<Point2D, 256>::new();
            black_box(mem::size_of_val(&series));
        });
    });

    group.bench_function("series_empty_1024", |b| {
        b.iter(|| {
            let series = StaticDataSeries::<Point2D, 1024>::new();
            black_box(mem::size_of_val(&series));
        });
    });

    // Measure full series
    group.bench_function("series_full_32", |b| {
        b.iter(|| {
            let mut series = StaticDataSeries::<Point2D, 32>::new();
            for i in 0..32 {
                series.push(Point2D::new(i as f32, i as f32)).ok();
            }
            black_box(mem::size_of_val(&series));
        });
    });

    group.bench_function("series_full_256", |b| {
        b.iter(|| {
            let mut series = StaticDataSeries::<Point2D, 256>::new();
            for i in 0..256 {
                series.push(Point2D::new(i as f32, i as f32)).ok();
            }
            black_box(mem::size_of_val(&series));
        });
    });

    group.finish();
}

/// Measure memory usage of complete chart configurations
fn bench_complete_chart_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("complete_chart_memory");

    // Small embedded system scenario (1KB budget)
    group.bench_function("minimal_1kb_setup", |b| {
        b.iter(|| {
            let mut data = StaticDataSeries::<Point2D, 32>::new();
            for i in 0..32 {
                data.push(Point2D::new(i as f32, (i as f32).sin() * 10.0))
                    .ok();
            }

            let chart = LineChart::builder()
                .line_color(embedded_graphics::pixelcolor::BinaryColor::On)
                .line_width(1)
                .build()
                .unwrap();

            let total_size = mem::size_of_val(&data) + mem::size_of_val(&chart);
            black_box(total_size);
        });
    });

    // Medium embedded system scenario (4KB budget)
    group.bench_function("standard_4kb_setup", |b| {
        b.iter(|| {
            let mut data = StaticDataSeries::<Point2D, 128>::new();
            for i in 0..128 {
                data.push(Point2D::new(i as f32, (i as f32 * 0.1).sin() * 50.0))
                    .ok();
            }

            let chart = LineChart::builder()
                .line_color(embedded_graphics::pixelcolor::Rgb565::BLUE)
                .line_width(2)
                .with_markers(MarkerStyle {
                    shape: MarkerShape::Circle,
                    size: 3,
                    color: embedded_graphics::pixelcolor::Rgb565::RED,
                    visible: true,
                })
                .build()
                .unwrap();

            let total_size = mem::size_of_val(&data) + mem::size_of_val(&chart);
            black_box(total_size);
        });
    });

    // Large embedded system scenario (16KB budget)
    group.bench_function("advanced_16kb_setup", |b| {
        b.iter(|| {
            let mut data1 = StaticDataSeries::<Point2D, 256>::new();
            let mut data2 = StaticDataSeries::<Point2D, 256>::new();
            let mut data3 = StaticDataSeries::<Point2D, 256>::new();

            for i in 0..256 {
                data1
                    .push(Point2D::new(i as f32, (i as f32 * 0.1).sin() * 50.0))
                    .ok();
                data2
                    .push(Point2D::new(i as f32, (i as f32 * 0.1).cos() * 50.0))
                    .ok();
                data3
                    .push(Point2D::new(i as f32, (i as f32 * 0.05).sin() * 30.0))
                    .ok();
            }

            let chart = CurveChart::builder()
                .line_color(embedded_graphics::pixelcolor::Rgb565::RED)
                .interpolation_type(InterpolationType::CubicSpline)
                .subdivisions(8)
                .build()
                .unwrap();

            let total_size = mem::size_of_val(&data1)
                + mem::size_of_val(&data2)
                + mem::size_of_val(&data3)
                + mem::size_of_val(&chart);
            black_box(total_size);
        });
    });

    group.finish();
}

/// Measure memory allocation patterns during operations
fn bench_allocation_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("allocation_patterns");

    // Measure allocations during data updates
    group.bench_function("streaming_update_pattern", |b| {
        let mut buffer = StaticDataSeries::<Point2D, 256>::new();

        // Pre-fill
        for i in 0..256 {
            buffer.push(Point2D::new(i as f32, i as f32)).ok();
        }

        let mut index = 256;

        b.iter(|| {
            // Sliding window update - simulate by clearing and refilling
            if buffer.len() >= 256 {
                // Copy all but first element
                let mut temp = StaticDataSeries::<Point2D, 256>::new();
                for i in 1..buffer.len() {
                    if let Some(point) = buffer.get(i) {
                        temp.push(point).ok();
                    }
                }
                buffer = temp;
            }
            buffer
                .push(Point2D::new(index as f32, (index as f32).sin()))
                .ok();
            index += 1;
            black_box(&buffer);
        });
    });

    // Measure allocations during aggregation
    group.bench_function("aggregation_pattern", |b| {
        let mut source = StaticDataSeries::<Point2D, 1024>::new();
        for i in 0..1024 {
            source
                .push(Point2D::new(i as f32, (i as f32 * 0.01).sin() * 100.0))
                .ok();
        }

        b.iter(|| {
            let mut aggregated = StaticDataSeries::<Point2D, 128>::new();

            // Simple downsampling by averaging
            for chunk in 0..128 {
                let start = chunk * 8;
                let end = (chunk + 1) * 8;

                let mut sum_x = 0.0;
                let mut sum_y = 0.0;
                let mut count = 0;

                for i in start..end.min(source.len()) {
                    if let Some(point) = source.get(i) {
                        sum_x += point.x;
                        sum_y += point.y;
                        count += 1;
                    }
                }

                if count > 0 {
                    aggregated
                        .push(Point2D::new(sum_x / count as f32, sum_y / count as f32))
                        .ok();
                }
            }

            black_box(aggregated);
        });
    });

    group.finish();
}

/// Measure memory overhead of different feature combinations
fn bench_feature_memory_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("feature_memory_overhead");

    // Base chart
    group.bench_function("base_line_chart", |b| {
        b.iter(|| {
            let chart = LineChart::builder()
                .line_color(embedded_graphics::pixelcolor::Rgb565::BLUE)
                .build()
                .unwrap();
            black_box(mem::size_of_val(&chart));
        });
    });

    // Chart with markers
    group.bench_function("line_chart_with_markers", |b| {
        b.iter(|| {
            let chart = LineChart::builder()
                .line_color(embedded_graphics::pixelcolor::Rgb565::BLUE)
                .with_markers(MarkerStyle {
                    shape: MarkerShape::Circle,
                    size: 4,
                    color: embedded_graphics::pixelcolor::Rgb565::RED,
                    visible: true,
                })
                .build()
                .unwrap();
            black_box(mem::size_of_val(&chart));
        });
    });

    // Chart with area fill
    group.bench_function("line_chart_with_area", |b| {
        b.iter(|| {
            let chart = LineChart::builder()
                .line_color(embedded_graphics::pixelcolor::Rgb565::BLUE)
                .fill_area(embedded_graphics::pixelcolor::Rgb565::new(0, 0, 255))
                .build()
                .unwrap();
            black_box(mem::size_of_val(&chart));
        });
    });

    // Chart with all features
    group.bench_function("line_chart_all_features", |b| {
        b.iter(|| {
            let chart = LineChart::builder()
                .line_color(embedded_graphics::pixelcolor::Rgb565::BLUE)
                .line_width(3)
                .with_markers(MarkerStyle {
                    shape: MarkerShape::Square,
                    size: 5,
                    color: embedded_graphics::pixelcolor::Rgb565::RED,
                    visible: true,
                })
                .fill_area(embedded_graphics::pixelcolor::Rgb565::new(0, 0, 255))
                .with_title("Test Chart")
                .build()
                .unwrap();
            black_box(mem::size_of_val(&chart));
        });
    });

    group.finish();
}

/// Memory usage summary reporting
#[allow(dead_code)]
fn generate_memory_report() {
    println!("\n=== Memory Usage Report ===\n");

    // Chart sizes
    println!("Chart Type Memory Footprint:");
    // Note: Chart types require color type parameter
    use embedded_graphics::pixelcolor::Rgb565;
    println!(
        "  LineChart<Rgb565>: {} bytes",
        mem::size_of::<LineChart<Rgb565>>()
    );
    println!(
        "  BarChart<Rgb565>: {} bytes",
        mem::size_of::<BarChart<Rgb565>>()
    );
    println!(
        "  PieChart<Rgb565>: {} bytes",
        mem::size_of::<PieChart<Rgb565>>()
    );
    #[cfg(feature = "gauge")]
    println!(
        "  GaugeChart<Rgb565>: {} bytes",
        mem::size_of::<GaugeChart<Rgb565>>()
    );
    #[cfg(feature = "scatter")]
    println!(
        "  ScatterChart<Rgb565>: {} bytes",
        mem::size_of::<ScatterChart<Rgb565>>()
    );

    // Data structure sizes
    println!("\nData Structure Sizes:");
    println!("  Point2D: {} bytes", mem::size_of::<Point2D>());
    println!(
        "  DataBounds<f32, f32>: {} bytes",
        mem::size_of::<DataBounds<f32, f32>>()
    );
    println!(
        "  StaticDataSeries<Point2D, 32>: {} bytes",
        mem::size_of::<StaticDataSeries<Point2D, 32>>()
    );
    println!(
        "  StaticDataSeries<Point2D, 256>: {} bytes",
        mem::size_of::<StaticDataSeries<Point2D, 256>>()
    );
    println!(
        "  StaticDataSeries<Point2D, 1024>: {} bytes",
        mem::size_of::<StaticDataSeries<Point2D, 1024>>()
    );

    // Configuration sizes
    println!("\nConfiguration Sizes:");
    println!(
        "  ChartConfig<Rgb565>: {} bytes",
        mem::size_of::<ChartConfig<Rgb565>>()
    );
    println!("  Margins: {} bytes", mem::size_of::<Margins>());

    println!("\n=== End Memory Report ===\n");
}

criterion_group!(
    memory_benches,
    bench_chart_memory_footprint,
    bench_data_series_memory,
    bench_complete_chart_memory,
    bench_allocation_patterns,
    bench_feature_memory_overhead,
);

criterion_main!(memory_benches);
