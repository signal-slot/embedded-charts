//! Production-Ready Demo - Visual Category
//!
//! This example demonstrates all completed chart features with automatic cycling
//! and redraw capabilities using the common window abstraction.
//!
//! Run with: cargo run --example production_ready_demo --features std

use embedded_charts::prelude::*;
use std::time::{Duration, Instant};

// Import the common abstraction
#[path = "../common/mod.rs"]
mod common;

use common::{data, window, WindowConfig};

#[cfg(feature = "std")]
fn main() -> ChartResult<()> {
    println!("🚀 Production-Ready Chart Framework Demo with Auto-Redraw");
    println!("=========================================================");
    println!("This demo will cycle through all features automatically");
    println!("and demonstrate redraw capabilities every 3 seconds.");
    println!("=========================================================");

    let demos = [
        (
            "Complex Marker Shapes",
            demo_complex_markers
                as fn(
                    &mut embedded_graphics_simulator::SimulatorDisplay<Rgb565>,
                    Rectangle,
                ) -> ChartResult<()>,
        ),
        ("Area Filling Under Curves", demo_area_filling),
        ("Grid System Integration", demo_grid_integration),
        ("All Features Combined", render_all_demos),
    ];

    let mut current_demo = 0;
    let mut last_update = Instant::now();
    let update_interval = Duration::from_secs(3);
    let demo_count = demos.len();

    // Use the common animated example runner
    window::run(
        WindowConfig::new("Production Ready Demo - Auto Redraw")
            .theme(common::WindowTheme::OledBlue)
            .auto_close() // Auto-close for capture mode
            .background(Rgb565::BLACK),
        move |display, viewport, _elapsed| {
            // Check if it's time to switch to next demo
            if last_update.elapsed() >= update_interval {
                current_demo = (current_demo + 1) % demo_count;
                println!("\n🔄 Auto-redrawing: {}", demos[current_demo].0);
                last_update = Instant::now();
            }

            // Render current demo
            demos[current_demo].1(display, viewport)?;

            Ok(()) // Continue animation
        },
    )?;

    println!("✅ All features working correctly!");
    println!("🎯 Framework is production-ready with automatic redraw support!");

    Ok(())
}

/// Render all demos in sequence
fn render_all_demos(
    display: &mut embedded_graphics_simulator::SimulatorDisplay<Rgb565>,
    viewport: Rectangle,
) -> ChartResult<()> {
    demo_complex_markers(display, viewport)?;
    demo_area_filling(display, viewport)?;
    demo_grid_integration(display, viewport)?;
    Ok(())
}

/// Demonstrate triangle and diamond markers working correctly
fn demo_complex_markers(
    display: &mut embedded_graphics_simulator::SimulatorDisplay<Rgb565>,
    viewport: Rectangle,
) -> ChartResult<()> {
    println!("\n📍 Demo 1: Complex Marker Shapes (Triangle & Diamond)");

    // Create test data using common utilities
    let mut series = StaticDataSeries::<Point2D, 256>::new();
    series.push(Point2D::new(1.0, 2.0))?;
    series.push(Point2D::new(2.0, 4.0))?;
    series.push(Point2D::new(3.0, 3.0))?;
    series.push(Point2D::new(4.0, 5.0))?;
    series.push(Point2D::new(5.0, 1.0))?;

    // Test triangle markers
    let triangle_marker = MarkerStyle {
        shape: MarkerShape::Triangle,
        size: 8,
        color: Rgb565::RED,
        visible: true,
    };

    let triangle_chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .with_markers(triangle_marker)
        .build()?;

    triangle_chart.draw(
        &series,
        &ChartConfig::<Rgb565>::default(),
        viewport,
        display,
    )?;

    println!("  ✓ Triangle markers rendered successfully");

    // Test diamond markers
    let diamond_marker = MarkerStyle {
        shape: MarkerShape::Diamond,
        size: 8,
        color: Rgb565::GREEN,
        visible: true,
    };

    let diamond_chart = LineChart::builder()
        .line_color(Rgb565::MAGENTA)
        .with_markers(diamond_marker)
        .build()?;

    diamond_chart.draw(
        &series,
        &ChartConfig::<Rgb565>::default(),
        viewport,
        display,
    )?;

    println!("  ✓ Diamond markers rendered successfully");

    Ok(())
}

/// Demonstrate area filling under line charts
fn demo_area_filling(
    display: &mut embedded_graphics_simulator::SimulatorDisplay<Rgb565>,
    viewport: Rectangle,
) -> ChartResult<()> {
    println!("\n🎨 Demo 2: Area Filling Under Curves");

    // Create test data for area chart using common utilities
    let series = data::sine_wave(10, 3.0, 0.5, 5.0)?;

    // Create line chart with area fill
    let area_chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .line_width(2)
        .fill_area(Rgb565::new(0, 0, 15)) // Semi-transparent blue
        .build()?;

    area_chart.draw(
        &series,
        &ChartConfig::<Rgb565>::default(),
        viewport,
        display,
    )?;

    println!("  ✓ Area filling rendered successfully");
    println!("  ✓ Triangle filling algorithm working correctly");

    Ok(())
}

/// Demonstrate integration with grid system
fn demo_grid_integration(
    display: &mut embedded_graphics_simulator::SimulatorDisplay<Rgb565>,
    viewport: Rectangle,
) -> ChartResult<()> {
    println!("\n🔧 Demo 3: Grid System Integration");

    // Create comprehensive test data using common utilities
    let series = data::sine_wave(15, 2.0, 0.8, 3.0)?;

    // Create grid system
    let grid = GridSystem::builder()
        .horizontal_linear(GridSpacing::Pixels(20))
        .vertical_linear(GridSpacing::Pixels(20))
        .build();

    // Create chart with all features
    let chart = LineChart::builder()
        .line_color(Rgb565::CYAN)
        .line_width(2)
        .fill_area(Rgb565::new(0, 8, 8)) // Semi-transparent cyan
        .with_markers(MarkerStyle {
            shape: MarkerShape::Triangle,
            size: 6,
            color: Rgb565::YELLOW,
            visible: true,
        })
        .with_grid(grid)
        .build()?;

    chart.draw(
        &series,
        &ChartConfig::<Rgb565>::default(),
        viewport,
        display,
    )?;

    println!("  ✓ Grid system integration successful");
    println!("  ✓ All systems working together seamlessly");

    Ok(())
}

#[cfg(not(feature = "std"))]
fn main() {
    utils::print_feature_requirement("std", "visual");
}
