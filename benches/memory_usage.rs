//! Memory usage benchmarks
//!
//! Measures memory allocation patterns and usage for different components

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use embedded_charts::{
    chart::{
        bar::BarChart,
        line::LineChart,
        pie::PieChart,
        scatter::ScatterChart,
        traits::{Chart, ChartBuilder, ChartConfig},
    },
    data::{
        point::Point2D,
        series::{MultiSeries, SlidingWindowSeries, StaticDataSeries},
    },
    memory::{ChartMemoryManager, FixedCapacityCollections, LabelStorage, MemoryStats},
};
use embedded_graphics::{pixelcolor::Rgb565, prelude::*, primitives::Rectangle};

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
    group.bench_function("pie_chart", |b| {
        b.iter(|| {
            let chart = PieChart::<Rgb565>::new(Point::new(160, 120), 100);
            let size = type_size(&chart);
            black_box(size);
        });
    });

    // Scatter chart
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
            let manager = ChartMemoryManager::new();
            let size = type_size(&manager);
            black_box(size);
        });
    });

    // MemoryStats tracking
    group.bench_function("memory_stats", |b| {
        b.iter(|| {
            let mut stats = MemoryStats::default();
            stats.used_bytes = 1024;
            stats.total_bytes = 4096;
            stats.peak_bytes = 2048;
            stats.allocation_count = 100;
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
                label.push_str(&format!("Label {}", i)).unwrap();
                storage.store(i, label).unwrap();
            }
            let size = type_size(&storage);
            black_box(size);
        });
    });

    // FixedCapacityCollections
    group.bench_function("fixed_capacity_collections", |b| {
        b.iter(|| {
            let collections = FixedCapacityCollections::<1024>::new();
            let size = type_size(&collections);
            black_box(size);
        });
    });

    group.finish();
}

/// Benchmark rendering memory patterns
fn bench_rendering_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("rendering_memory");
    let config = ChartConfig::<Rgb565>::default();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(320, 240));

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
            let mut display = embedded_graphics::mock_display::MockDisplay::<Rgb565>::new();
            display.set_allow_overdraw(true);

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
                let mut stats = MemoryStats::default();
                stats.used_bytes = size * std::mem::size_of::<Point2D>();
                stats.total_bytes = 256 * std::mem::size_of::<Point2D>();
                stats.allocation_count = size;

                black_box(stats);
            });
        });
    }

    group.finish();
}

/// Benchmark memory pool operations
fn bench_memory_pool(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_pool");

    // Pool allocation
    group.bench_function("pool_allocate", |b| {
        b.iter(|| {
            let mut manager = ChartMemoryManager::new();

            // Simulate allocations
            for i in 0..50 {
                manager.allocate(64).unwrap();
            }

            let stats = manager.get_stats();
            black_box(stats);
        });
    });

    // Pool deallocation and reuse
    group.bench_function("pool_reuse", |b| {
        let mut manager = ChartMemoryManager::new();

        // Pre-allocate some blocks
        let mut handles = Vec::new();
        for _ in 0..10 {
            handles.push(manager.allocate(64).unwrap());
        }

        b.iter(|| {
            // Deallocate
            for handle in &handles {
                manager.deallocate(*handle);
            }

            // Reallocate
            for _ in 0..10 {
                manager.allocate(64).unwrap();
            }

            let stats = manager.get_stats();
            black_box(stats);
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
                grid_visible: true,
                legend_visible: true,
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
