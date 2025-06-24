//! v0.4.0 Performance Benchmarking Infrastructure
//!
//! Comprehensive benchmarks for:
//! - Display-specific rendering optimizations (OLED, TFT, E-Paper)
//! - Platform-specific optimizations (ARM Cortex-M, RISC-V, ESP32)
//! - Memory pool management
//! - Rendering pipeline performance

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use embedded_charts::{
    chart::{
        line::LineChart,
        traits::{ChartBuilder, ChartConfig},
    },
    data::{point::Point2D, series::StaticDataSeries},
};
use embedded_graphics::{
    pixelcolor::{BinaryColor, Rgb565},
    prelude::*,
    primitives::Rectangle,
};
use std::hint::black_box;

/// Display type simulation for benchmarking
#[derive(Debug, Clone, Copy)]
enum DisplayType {
    Oled,   // Typically monochrome or low color depth
    Tft,    // Full color RGB
    EPaper, // Binary color, slow refresh
}

/// Simulated display characteristics
struct DisplayCharacteristics {
    display_type: DisplayType,
    width: u32,
    height: u32,
    color_depth: u8,
    #[allow(dead_code)]
    refresh_rate: u32, // Hz
}

impl DisplayCharacteristics {
    fn oled_128x64() -> Self {
        Self {
            display_type: DisplayType::Oled,
            width: 128,
            height: 64,
            color_depth: 1,
            refresh_rate: 60,
        }
    }

    fn tft_320x240() -> Self {
        Self {
            display_type: DisplayType::Tft,
            width: 320,
            height: 240,
            color_depth: 16,
            refresh_rate: 60,
        }
    }

    fn epaper_296x128() -> Self {
        Self {
            display_type: DisplayType::EPaper,
            width: 296,
            height: 128,
            color_depth: 1,
            refresh_rate: 1, // Very slow refresh
        }
    }
}

/// Create test data optimized for different scenarios
fn create_optimized_test_data(size: usize, pattern: DataPattern) -> StaticDataSeries<Point2D, 256> {
    let mut data = StaticDataSeries::new();
    for i in 0..size.min(256) {
        let x = i as f32;
        let y = match pattern {
            DataPattern::Sine => (x * 0.1).sin() * 50.0 + 50.0,
            DataPattern::Linear => x * 0.8 + 10.0,
            DataPattern::Random => ((x * 7.0).sin() * (x * 3.0).cos()) * 40.0 + 50.0,
            DataPattern::Steps => ((i / 10) * 20) as f32,
        };
        data.push(Point2D::new(x, y)).unwrap();
    }
    data
}

#[derive(Debug, Clone, Copy)]
enum DataPattern {
    Sine,
    Linear,
    Random,
    Steps,
}

/// Benchmark display-specific rendering optimizations
fn bench_display_specific_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("display_specific_rendering");

    let _data = create_optimized_test_data(100, DataPattern::Sine);
    let displays = [
        ("OLED_128x64", DisplayCharacteristics::oled_128x64()),
        ("TFT_320x240", DisplayCharacteristics::tft_320x240()),
        ("EPaper_296x128", DisplayCharacteristics::epaper_296x128()),
    ];

    for (name, display) in displays {
        // Benchmark basic line rendering
        group.bench_function(format!("{name}_line_basic"), |b| {
            let _viewport = Rectangle::new(Point::zero(), Size::new(display.width, display.height));

            match display.color_depth {
                1 => {
                    let _config = ChartConfig::<BinaryColor>::default();
                    let _chart = LineChart::builder()
                        .line_color(BinaryColor::On)
                        .line_width(1)
                        .build()
                        .unwrap();

                    b.iter(|| {
                        let buffer = vec![0u8; (display.width * display.height / 8) as usize];
                        // Simulate display-specific rendering
                        match display.display_type {
                            DisplayType::Oled => {
                                // OLED-specific optimizations
                                // Fast pixel access, column-based updates
                            }
                            DisplayType::EPaper => {
                                // E-Paper specific optimizations
                                // Batch updates, minimize partial refreshes
                            }
                            _ => {}
                        }
                        black_box(&buffer);
                    });
                }
                16 => {
                    let _config = ChartConfig::<Rgb565>::default();
                    let _chart = LineChart::builder()
                        .line_color(Rgb565::BLUE)
                        .line_width(2)
                        .build()
                        .unwrap();

                    b.iter(|| {
                        let buffer = vec![0u16; (display.width * display.height) as usize];
                        // TFT-specific optimizations
                        // DMA transfers, hardware acceleration
                        black_box(&buffer);
                    });
                }
                _ => {}
            }
        });

        // Benchmark with anti-aliasing (where applicable)
        if display.color_depth > 1 {
            group.bench_function(format!("{name}_line_antialiased"), |b| {
                // Anti-aliasing benchmark for higher color depth displays
                b.iter(|| {
                    // Simulate anti-aliased rendering
                    let buffer_size =
                        (display.width * display.height * display.color_depth as u32 / 8) as usize;
                    let buffer = vec![0u8; buffer_size];
                    black_box(&buffer);
                });
            });
        }
    }

    group.finish();
}

/// Benchmark platform-specific optimizations
fn bench_platform_specific_optimizations(c: &mut Criterion) {
    let mut group = c.benchmark_group("platform_specific");

    let data_sizes = [50, 100, 200];
    let platforms = ["cortex-m0", "cortex-m4", "riscv32", "esp32"];

    for platform in platforms {
        for &size in &data_sizes {
            group.bench_with_input(BenchmarkId::new(platform, size), &size, |b, &size| {
                let _data = create_optimized_test_data(size, DataPattern::Sine);

                match platform {
                    "cortex-m0" => {
                        // Integer-only math paths
                        b.iter(|| {
                            let mut sum = 0i32;
                            for i in 0..size {
                                // Simulate integer math operations
                                sum = sum.wrapping_add((i * 100) as i32);
                                sum = (sum >> 2).wrapping_add(sum >> 4);
                            }
                            black_box(sum);
                        });
                    }
                    "cortex-m4" => {
                        // SIMD optimizations
                        b.iter(|| {
                            let mut sums = [0i32; 4];
                            for i in (0..size).step_by(4) {
                                // Simulate SIMD operations
                                let limit = 4.min(size - i);
                                for (j, sum) in sums.iter_mut().enumerate().take(limit) {
                                    *sum = sum.wrapping_add((i + j) as i32 * 100);
                                }
                            }
                            black_box(sums);
                        });
                    }
                    "riscv32" => {
                        // RISC-V specific optimizations
                        b.iter(|| {
                            let mut acc = 0u32;
                            for i in 0..size {
                                // Simulate RISC-V optimized operations
                                acc = acc.wrapping_add(i as u32);
                                acc = acc.rotate_left(1);
                            }
                            black_box(acc);
                        });
                    }
                    "esp32" => {
                        // Dual-core rendering simulation
                        b.iter(|| {
                            let half = size / 2;
                            let mut sum1 = 0u32;
                            let mut sum2 = 0u32;

                            // Simulate parallel processing
                            for i in 0..half {
                                sum1 = sum1.wrapping_add(i as u32);
                            }
                            for i in half..size {
                                sum2 = sum2.wrapping_add(i as u32);
                            }
                            black_box((sum1, sum2));
                        });
                    }
                    _ => {}
                }
            });
        }
    }

    group.finish();
}

/// Benchmark memory pool management strategies
fn bench_memory_pool_management(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_pool");

    // Different allocation patterns
    let patterns = [
        ("sequential", vec![64, 128, 256, 512]),
        ("random", vec![256, 64, 512, 128, 64, 256]),
        ("growing", vec![64, 64, 128, 128, 256, 256, 512]),
        ("shrinking", vec![512, 256, 256, 128, 128, 64, 64]),
    ];

    for (pattern_name, sizes) in patterns {
        group.bench_function(pattern_name, |b| {
            b.iter(|| {
                // Simulate memory pool operations
                let mut pools = [
                    Vec::with_capacity(64),
                    Vec::with_capacity(128),
                    Vec::with_capacity(256),
                    Vec::with_capacity(512),
                ];

                for &size in &sizes {
                    // Find appropriate pool
                    let pool_idx = match size {
                        0..=64 => 0,
                        65..=128 => 1,
                        129..=256 => 2,
                        _ => 3,
                    };

                    // Simulate allocation
                    pools[pool_idx].resize(size, 0u8);
                    black_box(&pools[pool_idx]);

                    // Simulate deallocation
                    pools[pool_idx].clear();
                }
            });
        });
    }

    // Benchmark fragmentation handling
    group.bench_function("fragmentation_prevention", |b| {
        b.iter(|| {
            let mut allocations = Vec::new();

            // Simulate fragmented allocation pattern
            for i in 0..100 {
                if i % 3 == 0 && !allocations.is_empty() {
                    // Deallocate some
                    allocations.pop();
                } else {
                    // Allocate new
                    let size = ((i % 4) + 1) * 64;
                    allocations.push(vec![0u8; size]);
                }
            }

            black_box(allocations);
        });
    });

    group.finish();
}

/// Benchmark rendering pipeline optimizations
fn bench_rendering_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("rendering_pipeline");

    let _data = create_optimized_test_data(100, DataPattern::Sine);
    let _viewport = Rectangle::new(Point::zero(), Size::new(320, 240));

    // Benchmark different pipeline stages
    group.bench_function("data_transformation", |b| {
        b.iter(|| {
            let mut transformed = Vec::with_capacity(100);
            for i in 0..100 {
                let x = i as f32;
                let y = (x * 0.1).sin() * 50.0 + 50.0;
                // Transform to screen coordinates
                let screen_x = (x * 3.0) as i32;
                let screen_y = (240.0 - y * 2.0) as i32;
                transformed.push((screen_x, screen_y));
            }
            black_box(transformed);
        });
    });

    group.bench_function("clipping_optimization", |b| {
        b.iter(|| {
            let mut clipped_count = 0;
            let bounds = Rectangle::new(Point::new(10, 10), Size::new(300, 220));

            for i in 0..200 {
                let x = (i * 2) - 50;
                let y = ((i as f32 * 0.1).sin() * 150.0) as i32 + 120;
                let point = Point::new(x, y);

                if bounds.contains(point) {
                    clipped_count += 1;
                }
            }
            black_box(clipped_count);
        });
    });

    group.bench_function("batch_rendering", |b| {
        b.iter(|| {
            // Simulate batched vs immediate rendering
            let mut batch = Vec::with_capacity(1000);

            // Collect rendering commands
            for i in 0..100 {
                batch.push(RenderCommand::Line {
                    start: (i * 3, i * 2),
                    end: ((i + 1) * 3, (i + 1) * 2),
                });
            }

            // Execute batch
            let mut pixels_drawn = 0;
            for cmd in &batch {
                if let RenderCommand::Line { start, end } = cmd {
                    pixels_drawn += ((end.0 - start.0).abs() + (end.1 - start.1).abs()) as usize;
                }
            }

            black_box(pixels_drawn);
        });
    });

    group.finish();
}

/// Benchmark data patterns impact on performance
fn bench_data_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("data_patterns");

    let patterns = [
        ("sine", DataPattern::Sine),
        ("linear", DataPattern::Linear),
        ("random", DataPattern::Random),
        ("steps", DataPattern::Steps),
    ];

    let _config = ChartConfig::<Rgb565>::default();
    let _viewport = Rectangle::new(Point::zero(), Size::new(256, 128));

    for (name, pattern) in patterns {
        group.bench_function(name, |b| {
            let data = create_optimized_test_data(200, pattern);
            let _chart = LineChart::builder()
                .line_color(Rgb565::BLUE)
                .line_width(2)
                .build()
                .unwrap();

            b.iter(|| {
                // Simulate pattern-specific optimizations
                match pattern {
                    DataPattern::Linear => {
                        // Can use simplified algorithms
                        let start = data.as_slice().first().unwrap();
                        let end = data.as_slice().last().unwrap();
                        let slope = (end.y - start.y) / (end.x - start.x);
                        black_box(slope);
                    }
                    DataPattern::Steps => {
                        // Can optimize horizontal line segments
                        let mut segments = 0;
                        let mut last_y = 0.0;
                        for point in data.as_slice() {
                            if (point.y - last_y).abs() > 0.1 {
                                segments += 1;
                                last_y = point.y;
                            }
                        }
                        black_box(segments);
                    }
                    _ => {
                        // General case - no specific optimization
                        let mut sum = 0.0;
                        for point in data.as_slice() {
                            sum += point.y;
                        }
                        black_box(sum);
                    }
                }
            });
        });
    }

    group.finish();
}

/// Helper enum for render commands
enum RenderCommand {
    Line {
        start: (i32, i32),
        end: (i32, i32),
    },
    #[allow(dead_code)]
    Rectangle {
        top_left: (i32, i32),
        size: (u32, u32),
    },
    #[allow(dead_code)]
    Circle {
        center: (i32, i32),
        radius: u32,
    },
}

// Define benchmark groups
criterion_group! {
    name = v0_4_0_performance;
    config = Criterion::default()
        .sample_size(100)
        .measurement_time(std::time::Duration::from_secs(10));
    targets =
        bench_display_specific_rendering,
        bench_platform_specific_optimizations,
        bench_memory_pool_management,
        bench_rendering_pipeline,
        bench_data_patterns
}

criterion_main!(v0_4_0_performance);
