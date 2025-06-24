//! Benchmarks for platform-specific optimizations

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use embedded_charts::platform::{self, PlatformOptimized};
use embedded_charts::data::Point2D;

fn benchmark_sqrt(c: &mut Criterion) {
    let values: Vec<f32> = (1..100).map(|i| i as f32).collect();
    
    c.bench_function("platform_fast_sqrt", |b| {
        b.iter(|| {
            for &val in &values {
                black_box(platform::GenericPlatform::fast_sqrt(val));
            }
        })
    });
    
    c.bench_function("std_sqrt", |b| {
        b.iter(|| {
            for &val in &values {
                black_box(val.sqrt());
            }
        })
    });
}

fn benchmark_trig(c: &mut Criterion) {
    let angles: Vec<f32> = (0..360).map(|i| (i as f32).to_radians()).collect();
    
    c.bench_function("platform_fast_sin", |b| {
        b.iter(|| {
            for &angle in &angles {
                black_box(platform::GenericPlatform::fast_sin(angle));
            }
        })
    });
    
    c.bench_function("std_sin", |b| {
        b.iter(|| {
            for &angle in &angles {
                black_box(angle.sin());
            }
        })
    });
}

fn benchmark_line_drawing(c: &mut Criterion) {
    let start = Point2D { x: 0.0, y: 0.0 };
    let end = Point2D { x: 100.0, y: 75.0 };
    let mut pixel_count = 0;
    
    c.bench_function("platform_line_optimized", |b| {
        b.iter(|| {
            pixel_count = 0;
            platform::GenericPlatform::draw_line_optimized(
                start,
                end,
                |_, _| pixel_count += 1
            );
        })
    });
}

fn benchmark_rect_filling(c: &mut Criterion) {
    let top_left = Point2D { x: 0.0, y: 0.0 };
    let mut pixel_count = 0;
    
    c.bench_function("platform_fill_rect_16x16", |b| {
        b.iter(|| {
            pixel_count = 0;
            platform::GenericPlatform::fill_rect_optimized(
                top_left,
                16,
                16,
                |_, _| pixel_count += 1
            );
        })
    });
    
    c.bench_function("platform_fill_rect_64x64", |b| {
        b.iter(|| {
            pixel_count = 0;
            platform::GenericPlatform::fill_rect_optimized(
                top_left,
                64,
                64,
                |_, _| pixel_count += 1
            );
        })
    });
}

criterion_group!(
    benches,
    benchmark_sqrt,
    benchmark_trig,
    benchmark_line_drawing,
    benchmark_rect_filling
);
criterion_main!(benches);