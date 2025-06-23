//! Memory usage benchmarks
//!
//! Measures memory allocation patterns and usage for different components

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use embedded_charts::{
    chart::{
        bar::BarChart,
        line::LineChart,
        traits::{Chart, ChartBuilder, ChartConfig},
    },
    data::{
        point::Point2D,
        series::{MultiSeries, StaticDataSeries},
    },
    memory::{ChartMemoryManager, FixedCapacityCollections, LabelStorage, MemoryStats},
};

#[cfg(feature = "animations")]
use embedded_charts::data::series::SlidingWindowSeries;

#[cfg(feature = "scatter")]
use embedded_charts::chart::scatter::ScatterChart;

#[cfg(feature = "pie")]
use embedded_charts::chart::pie::PieChart;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*, primitives::Rectangle};

/// Create a fresh MockDisplay that allows overdrawing and out-of-bounds drawing
/// This prevents "tried to draw pixel twice" and "outside display area" errors
fn create_test_display<C>() -> embedded_graphics::mock_display::MockDisplay<C>
where
    C: embedded_graphics::pixelcolor::PixelColor,
{
    let mut display = embedded_graphics::mock_display::MockDisplay::new();
    display.set_allow_overdraw(true);
    display.set_allow_out_of_bounds_drawing(true);
    display
}

/// Helper to estimate memory usage of a type
fn type_size<T>(_: &T) -> usize {
    std::mem::size_of::<T>()
}

/// Benchmark memory usage of data structures
fn bench_data_structure_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("data_structure_memory");

    // StaticDataSeries memory usage
    group.bench_function("static_data_series", |b| {
        b.iter(|| {
            let series = StaticDataSeries::<Point2D, 256>::new();
            let size = type_size(&series);
            black_box(size);
        });
    });

    // Filled StaticDataSeries
    group.bench_function("static_data_series_filled", |b| {
        b.iter(|| {
            let mut series = StaticDataSeries::<Point2D, 256>::new();
            for i in 0..256 {
                series.push(Point2D::new(i as f32, i as f32)).unwrap();
            }
            let size = type_size(&series);
            black_box(size);
        });
    });

    // MultiSeries memory usage
    group.bench_function("multi_series", |b| {
        b.iter(|| {
            let multi = MultiSeries::<Point2D, 8, 256>::new();
            let size = type_size(&multi);
            black_box(size);
        });
    });

    // SlidingWindowSeries memory usage
    #[cfg(feature = "animations")]
    group.bench_function("sliding_window", |b| {
        b.iter(|| {
            let window = SlidingWindowSeries::<Point2D, 100>::new();
            let size = type_size(&window);
            black_box(size);
        });
    });

    group.finish();
}

/// Benchmark chart instance memory usage
fn bench_chart_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("chart_memory");

    // Line chart
    group.bench_function("line_chart", |b| {
        b.iter(|| {
            let chart = LineChart::<Rgb565>::new();
            let size = type_size(&chart);
            black_box(size);
        });
    });

    // Bar chart
    group.bench_function("bar_chart", |b| {
        b.iter(|| {
            let chart = BarChart::<Rgb565>::new();
            let size = type_size(&chart);
            black_box(size);
        });
    });

    // Pie chart
    #[cfg(feature = "pie")]
    group.bench_function("pie_chart", |b| {
        b.iter(|| {
            let chart = PieChart::<Rgb565>::new(Point::new(160, 120), 100);
            let size = type_size(&chart);
            black_box(size);
        });
    });

    // Scatter chart
    #[cfg(feature = "scatter")]
    group.bench_function("scatter_chart", |b| {
        b.iter(|| {
            let chart = ScatterChart::<Rgb565>::new();
            let size = type_size(&chart);
            black_box(size);
        });
    });

    // Chart with configuration
    group.bench_function("configured_line_chart", |b| {
        b.iter(|| {
            let chart = LineChart::builder()
                .line_color(Rgb565::BLUE)
                .line_width(2)
                .with_title("Memory Test")
                .build()
                .unwrap();
            let size = type_size(&chart);
            black_box(size);
        });
    });

    group.finish();
}

/// Benchmark memory management components
fn bench_memory_management(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_management");

    // ChartMemoryManager
    group.bench_function("memory_manager", |b| {
        b.iter(|| {
            let manager = ChartMemoryManager::<4096>::new();
            let size = type_size(&manager);
            black_box(size);
        });
    });

    // MemoryStats tracking
    group.bench_function("memory_stats", |b| {
        b.iter(|| {
            let mut stats = MemoryStats::new(4096);
            stats.update_usage(1024);
            stats.peak_usage = 2048;
            let size = type_size(&stats);
            black_box(size);
        });
    });

    // LabelStorage
    group.bench_function("label_storage", |b| {
        b.iter(|| {
            let mut storage = LabelStorage::<32, 16>::new();
            for i in 0..10 {
                let mut label = heapless::String::<32>::new();
                label.push_str(&format!("Label {i}")).unwrap();
                storage.add_label(&label).unwrap();
            }
            let size = type_size(&storage);
            black_box(size);
        });
    });

    // FixedCapacityCollections
    group.bench_function("fixed_capacity_collections", |b| {
        b.iter(|| {
            let data_vec = FixedCapacityCollections::data_vec::<Point2D, 256>();
            let string_vec = FixedCapacityCollections::string_vec::<16, 32>();
            let color_vec = FixedCapacityCollections::color_vec::<Rgb565, 16>();
            let size = type_size(&data_vec) + type_size(&string_vec) + type_size(&color_vec);
            black_box(size);
        });
    });

    group.finish();
}

/// Benchmark rendering memory patterns
fn bench_rendering_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("rendering_memory");
    let config = ChartConfig::<Rgb565>::default();
    let viewport = Rectangle::new(Point::new(40, 40), Size::new(240, 160));

    // Memory allocation pattern during rendering
    group.bench_function("render_allocation_pattern", |b| {
        let mut data = StaticDataSeries::<Point2D, 256>::new();
        for i in 0..100 {
            data.push(Point2D::new(i as f32, (i * 2) as f32)).unwrap();
        }

        let chart = LineChart::builder()
            .line_color(Rgb565::BLUE)
            .line_width(2)
            .build()
            .unwrap();

        b.iter(|| {
            let mut display = create_test_display::<Rgb565>();

            // Measure before rendering
            let before_size = type_size(&display);

            chart.draw(&data, &config, viewport, &mut display).unwrap();

            // Measure after rendering
            let after_size = type_size(&display);

            black_box((before_size, after_size));
        });
    });

    group.finish();
}

/// Benchmark memory usage scaling with data size
fn bench_memory_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_scaling");

    for size in [10, 50, 100, 200, 256] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                let mut series = StaticDataSeries::<Point2D, 256>::new();
                for i in 0..size {
                    series.push(Point2D::new(i as f32, i as f32)).unwrap();
                }

                // Simulate memory tracking
                let mut stats = MemoryStats::new(256 * std::mem::size_of::<Point2D>());
                stats.update_usage(size * std::mem::size_of::<Point2D>());

                black_box(stats);
            });
        });
    }

    group.finish();
}

/// Benchmark memory pool operations
fn bench_memory_pool(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_pool");

    // Memory usage tracking
    group.bench_function("memory_usage_tracking", |b| {
        b.iter(|| {
            let mut manager = ChartMemoryManager::<4096>::new();

            // Simulate memory usage updates
            for i in 0..50 {
                manager.update_usage(i * 64);
            }

            let stats = manager.stats();
            black_box(stats);
        });
    });

    // High water mark tracking
    group.bench_function("high_water_mark", |b| {
        let mut manager = ChartMemoryManager::<8192>::new();

        b.iter(|| {
            // Update with varying usage
            for i in 0..10 {
                manager.update_usage(i * 256);
            }

            let high_water = manager.high_water_mark();
            black_box(high_water);

            manager.reset_stats();
        });
    });

    group.finish();
}

/// Benchmark configuration memory overhead
fn bench_config_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("config_memory");

    // Default config
    group.bench_function("default_config", |b| {
        b.iter(|| {
            let config = ChartConfig::<Rgb565>::default();
            let size = type_size(&config);
            black_box(size);
        });
    });

    // Full config with all options
    group.bench_function("full_config", |b| {
        b.iter(|| {
            let mut title = heapless::String::new();
            title.push_str("Performance Test Chart").unwrap();

            let config = ChartConfig::<Rgb565> {
                title: Some(title),
                background_color: Some(Rgb565::BLACK),
                margins: embedded_charts::chart::traits::Margins {
                    top: 20,
                    right: 20,
                    bottom: 30,
                    left: 30,
                },
                show_grid: true,
                grid_color: Some(Rgb565::new(10, 10, 10)),
            };

            let size = type_size(&config);
            black_box(size);
        });
    });

    group.finish();
}

criterion_group!(
    memory_benches,
    bench_data_structure_memory,
    bench_chart_memory,
    bench_memory_management,
    bench_rendering_memory,
    bench_memory_scaling,
    bench_memory_pool,
    bench_config_memory
);

criterion_main!(memory_benches);
