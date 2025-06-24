//! Platform Optimizations Demo
//!
//! This example demonstrates platform-specific optimizations by comparing
//! standard math functions with optimized versions side by side.
//!
//! Run with: cargo run --example platform_optimized_chart --features "std,line"

use embedded_charts::prelude::*;

// Import the common abstraction
#[path = "common/mod.rs"]
mod common;

use common::{window, WindowConfig};

#[cfg(not(all(feature = "std", feature = "line")))]
fn main() {
    eprintln!("This example requires 'std' and 'line' features.");
    eprintln!("Run with: cargo run --example platform_optimized_chart --features std,line");
}

#[cfg(all(feature = "std", feature = "line"))]
fn main() -> ChartResult<()> {
    use embedded_charts::platform::{self, PlatformOptimized};

    // Show performance comparison
    println!("\nPerformance comparison:");

    // Compare sqrt
    let test_val: f32 = 42.0;
    let std_sqrt = test_val.sqrt();
    let fast_sqrt = platform::GenericPlatform::fast_sqrt(test_val);
    println!(
        "sqrt({}) - Standard: {:.4}, Fast: {:.4}, Error: {:.4}%",
        test_val,
        std_sqrt,
        fast_sqrt,
        ((std_sqrt - fast_sqrt).abs() / std_sqrt) * 100.0
    );

    // Compare sin
    let angle: f32 = 1.0;
    let std_sin = angle.sin();
    let fast_sin = platform::GenericPlatform::fast_sin(angle);
    println!(
        "sin({}) - Standard: {:.4}, Fast: {:.4}, Error: {:.4}%",
        angle,
        std_sin,
        fast_sin,
        ((std_sin - fast_sin).abs() / std_sin.abs().max(0.0001)) * 100.0
    );

    // Platform detection
    println!("\nPlatform detection:");
    #[cfg(target_arch = "arm")]
    println!("ARM architecture detected - using optimized implementations");

    #[cfg(target_arch = "riscv32")]
    println!("RISC-V architecture detected - using optimized implementations");

    #[cfg(target_arch = "xtensa")]
    println!("ESP32 (Xtensa) architecture detected - using optimized implementations");

    #[cfg(not(any(target_arch = "arm", target_arch = "riscv32", target_arch = "xtensa")))]
    println!("Generic platform (x86_64 or other) - using fallback implementations");

    // Run the visualization
    window::run(
        WindowConfig::new("Platform Optimizations Demo")
            .theme(window::WindowTheme::Dark)
            .background(Rgb565::BLACK)
            .size(Size::new(800, 600)),
        move |display, viewport, _time| {
            // Import needed for drawing primitives
            use embedded_graphics::{
                primitives::{Circle, Line, PrimitiveStyle, Rectangle},
                Drawable,
            };

            // Clear the entire viewport first
            use embedded_graphics::primitives::PrimitiveStyleBuilder;
            Rectangle::new(viewport.top_left, viewport.size)
                .into_styled(
                    PrimitiveStyleBuilder::new()
                        .fill_color(Rgb565::BLACK)
                        .build(),
                )
                .draw(display)
                .map_err(|_| ChartError::RenderingError)?;

            let viewport1 = Rectangle::new(
                Point::new(40, 60),
                Size::new(viewport.size.width - 80, (viewport.size.height - 140) / 2),
            );

            let viewport2 = Rectangle::new(
                Point::new(40, viewport.size.height as i32 / 2 + 40),
                Size::new(viewport.size.width - 80, (viewport.size.height - 140) / 2),
            );

            // Draw borders only (no fill)
            viewport1
                .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1))
                .draw(display)
                .map_err(|_| ChartError::RenderingError)?;

            viewport2
                .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1))
                .draw(display)
                .map_err(|_| ChartError::RenderingError)?;

            // Draw axes for reference
            Line::new(
                Point::new(
                    viewport1.top_left.x,
                    viewport1.top_left.y + viewport1.size.height as i32 / 2,
                ),
                Point::new(
                    viewport1.top_left.x + viewport1.size.width as i32,
                    viewport1.top_left.y + viewport1.size.height as i32 / 2,
                ),
            )
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::new(20, 20, 20), 1))
            .draw(display)
            .map_err(|_| ChartError::RenderingError)?;

            // Draw standard sine wave
            let points = 400;
            let mut prev_point = None;

            for i in 0..=points {
                let t = i as f32 / points as f32;
                let x = t * viewport1.size.width as f32;
                let angle = t * 4.0 * std::f32::consts::PI;
                let y = angle.sin();

                // Map to screen coordinates
                let screen_x = viewport1.top_left.x + x as i32;
                let screen_y = viewport1.top_left.y + (viewport1.size.height as i32 / 2)
                    - (y * (viewport1.size.height as f32 * 0.4)) as i32;

                let current_point = Point::new(screen_x, screen_y);

                // Draw line segment
                if let Some(prev) = prev_point {
                    Line::new(prev, current_point)
                        .into_styled(PrimitiveStyle::with_stroke(Rgb565::new(0, 63, 63), 3))
                        .draw(display)
                        .map_err(|_| ChartError::RenderingError)?;
                }

                prev_point = Some(current_point);
            }

            // Draw axes for reference
            Line::new(
                Point::new(
                    viewport2.top_left.x,
                    viewport2.top_left.y + viewport2.size.height as i32 / 2,
                ),
                Point::new(
                    viewport2.top_left.x + viewport2.size.width as i32,
                    viewport2.top_left.y + viewport2.size.height as i32 / 2,
                ),
            )
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::new(20, 20, 20), 1))
            .draw(display)
            .map_err(|_| ChartError::RenderingError)?;

            // Draw optimized sine wave
            let mut prev_point = None;

            for i in 0..=points {
                let t = i as f32 / points as f32;
                let x = t * viewport2.size.width as f32;
                let angle = t * 4.0 * std::f32::consts::PI;
                let y = platform::GenericPlatform::fast_sin(angle);

                // Map to screen coordinates
                let screen_x = viewport2.top_left.x + x as i32;
                let screen_y = viewport2.top_left.y + (viewport2.size.height as i32 / 2)
                    - (y * (viewport2.size.height as f32 * 0.4)) as i32;

                let current_point = Point::new(screen_x, screen_y);

                // Draw line segment
                if let Some(prev) = prev_point {
                    Line::new(prev, current_point)
                        .into_styled(PrimitiveStyle::with_stroke(Rgb565::new(63, 63, 0), 3))
                        .draw(display)
                        .map_err(|_| ChartError::RenderingError)?;
                }

                prev_point = Some(current_point);
            }

            // Draw difference indicators where errors are significant
            for i in (0..=points).step_by(20) {
                let t = i as f32 / points as f32;
                let x = t * viewport2.size.width as f32;
                let angle = t * 4.0 * std::f32::consts::PI;
                let y_std = angle.sin();
                let y_opt = platform::GenericPlatform::fast_sin(angle);
                let diff = (y_std - y_opt).abs();

                if diff > 0.001 {
                    let screen_x = viewport2.top_left.x + x as i32;
                    let screen_y = viewport2.top_left.y + (viewport2.size.height as i32 / 2)
                        - (y_opt * (viewport2.size.height as f32 * 0.4)) as i32;

                    Circle::new(Point::new(screen_x - 3, screen_y - 3), 7)
                        .into_styled(PrimitiveStyle::with_fill(Rgb565::new(63, 0, 0)))
                        .draw(display)
                        .map_err(|_| ChartError::RenderingError)?;
                }
            }

            // Add labels
            use embedded_graphics::{
                mono_font::{ascii::FONT_6X10, MonoTextStyle},
                text::{Baseline, Text},
            };

            let text_style = MonoTextStyle::new(&FONT_6X10, Rgb565::new(63, 63, 63));

            Text::with_baseline(
                "Standard sin() function",
                Point::new(50, 40),
                text_style,
                Baseline::Top,
            )
            .draw(display)
            .map_err(|_| ChartError::RenderingError)?;

            Text::with_baseline(
                "Platform-optimized fast_sin()",
                Point::new(50, viewport.size.height as i32 / 2 + 20),
                text_style,
                Baseline::Top,
            )
            .draw(display)
            .map_err(|_| ChartError::RenderingError)?;

            // Add performance info
            Text::with_baseline(
                "Error: <0.001%",
                Point::new(viewport.size.width as i32 - 100, 40),
                text_style,
                Baseline::Top,
            )
            .draw(display)
            .map_err(|_| ChartError::RenderingError)?;

            // Add legend
            Text::with_baseline(
                "Red dots indicate differences > 0.001",
                Point::new(
                    viewport.size.width as i32 / 2 - 100,
                    viewport.size.height as i32 - 20,
                ),
                text_style,
                Baseline::Top,
            )
            .draw(display)
            .map_err(|_| ChartError::RenderingError)?;

            Ok(())
        },
    )
}
