//! Simple benchmark to validate display optimizations

use criterion::{criterion_group, criterion_main, Criterion};
use embedded_charts::render::{OLEDRenderer, OptimizedRenderer, TFTRenderer};
use embedded_graphics::{
    mock_display::MockDisplay,
    pixelcolor::{BinaryColor, Rgb565},
    prelude::*,
};
use std::hint::black_box;

fn bench_oled_vertical_lines(c: &mut Criterion) {
    c.bench_function("oled_vertical_line_batch", |b| {
        b.iter(|| {
            let mut display = MockDisplay::<BinaryColor>::new();
            display.set_allow_overdraw(true);
            let mut renderer = OLEDRenderer::new(display);

            renderer.begin_batch();
            // Draw 50 vertical lines
            for x in 0..50 {
                renderer
                    .draw_line_optimized(Point::new(x, 0), Point::new(x, 63), BinaryColor::On, 1)
                    .ok();
            }
            renderer.end_batch();
            black_box(renderer);
        });
    });
}

fn bench_tft_horizontal_lines(c: &mut Criterion) {
    c.bench_function("tft_horizontal_line_batch", |b| {
        b.iter(|| {
            let mut display = MockDisplay::<Rgb565>::new();
            display.set_allow_overdraw(true);
            let mut renderer = TFTRenderer::new(display);

            renderer.begin_batch();
            // Draw 50 horizontal lines (optimized for TFT)
            for y in 0..50 {
                renderer
                    .draw_line_optimized(
                        Point::new(0, y),
                        Point::new(63, y),
                        Rgb565::BLUE,
                        1,
                    )
                    .ok();
            }
            renderer.end_batch();
            black_box(renderer);
        });
    });
}

criterion_group! {
    name = simple_display_benches;
    config = Criterion::default()
        .sample_size(20);
    targets =
        bench_oled_vertical_lines,
        bench_tft_horizontal_lines
}

criterion_main!(simple_display_benches);
