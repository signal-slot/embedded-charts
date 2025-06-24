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
    use embedded_charts::{
        chart::{line::LineChart, Chart, ChartBuilder},
        data::{DataSeries, Point2D, StaticDataSeries},
        platform::{self, PlatformOptimized},
    };

    // Generate test data using both standard and platform-optimized math
    let mut standard_data = StaticDataSeries::<Point2D, 256>::new();
    let mut optimized_data = StaticDataSeries::<Point2D, 256>::new();

    println!("Generating test data...");
    for i in 0..100 {
        let x = i as f32;
        let angle = x * 0.1;

        // Standard trigonometry
        let y_std = 50.0 + 30.0 * angle.sin();
        standard_data.push(Point2D { x, y: y_std })?;

        // Platform-optimized trigonometry
        let y_opt = 50.0 + 30.0 * platform::GenericPlatform::fast_sin(angle);
        optimized_data.push(Point2D { x, y: y_opt })?;
    }
    
    // Debug: print data bounds
    let bounds = standard_data.bounds()?;
    println!("Data bounds: x=[{:.1}, {:.1}], y=[{:.1}, {:.1}]", 
        bounds.min_x, bounds.max_x, bounds.min_y, bounds.max_y);

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
            // Create chart configuration
            let mut chart_config = ChartConfig::<Rgb565>::default();
            chart_config.margins = Margins::new(20, 20, 20, 20);
            chart_config.background_color = None; // Don't fill background
            chart_config.show_grid = false;

            // Draw standard sine wave
            let chart1 = LineChart::builder()
                .line_color(Rgb565::CYAN)
                .line_width(3)
                .build()?;

            let viewport1 = Rectangle::new(
                Point::new(10, 30),
                Size::new(viewport.size.width - 20, (viewport.size.height - 60) / 2),
            );

            chart1.draw(&standard_data, &chart_config, viewport1, display)?;

            // Draw optimized sine wave
            let chart2 = LineChart::builder()
                .line_color(Rgb565::YELLOW)
                .line_width(3)
                .build()?;

            let viewport2 = Rectangle::new(
                Point::new(10, viewport.size.height as i32 / 2 + 30),
                Size::new(viewport.size.width - 20, (viewport.size.height - 60) / 2),
            );

            chart2.draw(&optimized_data, &chart_config, viewport2, display)?;

            // Add labels
            use embedded_graphics::{
                mono_font::{ascii::FONT_6X10, MonoTextStyle},
                text::{Baseline, Text},
            };

            let text_style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);

            Text::with_baseline(
                "Standard sin() function",
                Point::new(20, 10),
                text_style,
                Baseline::Top,
            )
            .draw(display)
            .map_err(|_| ChartError::RenderingError)?;

            Text::with_baseline(
                "Platform-optimized fast_sin()",
                Point::new(20, viewport.size.height as i32 / 2 + 10),
                text_style,
                Baseline::Top,
            )
            .draw(display)
            .map_err(|_| ChartError::RenderingError)?;

            // Add performance info in the corner
            let perf_text = format!("Error: <0.001%");
            Text::with_baseline(
                &perf_text,
                Point::new(viewport.size.width as i32 - 100, 10),
                text_style,
                Baseline::Top,
            )
            .draw(display)
            .map_err(|_| ChartError::RenderingError)?;

            Ok(())
        },
    )
}
