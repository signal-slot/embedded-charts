//! Donut Chart Example - Visual Category
//!
//! This example demonstrates how to create donut charts with various configurations
//! using the embedded-graphics-simulator. Donut charts are pie charts with a hollow
//! center, providing space for additional information or improved visual clarity.
//!
//! Features demonstrated:
//! - Basic donut chart creation
//! - Different inner radius configurations
//! - Comparison with regular pie charts
//! - Best practices for embedded displays
//! - Memory usage considerations
//!
//! Run with: cargo run --example donut_chart --features "std,pie"

use embedded_charts::prelude::*;

// Import the common abstraction
#[path = "../common/mod.rs"]
mod common;

use common::{configs, window, WindowConfig};

#[cfg(feature = "std")]
fn main() -> ChartResult<()> {
    // Create sample data representing storage usage
    let mut series = StaticDataSeries::<Point2D, 256>::new();
    let data_points = [
        (1.0, 45.0), // Documents: 45%
        (2.0, 25.0), // Photos: 25%
        (3.0, 15.0), // Videos: 15%
        (4.0, 10.0), // Apps: 10%
        (5.0, 5.0),  // Other: 5%
    ];

    for (x, y) in data_points.iter() {
        series
            .push(Point2D::new(*x, *y))
            .map_err(ChartError::from)?;
    }

    let colors = configs::professional_colors();

    // Create different donut charts demonstrating new helper methods
    
    // 1. Thin donut using convenience method
    let thin_donut = PieChart::builder()
        .center(Point::new(120, 120))
        .radius(80)
        .thin_donut() // 25% inner radius using helper method
        .colors(&colors[0..5])
        .with_title("Thin Donut (25%)")
        .background_color(Rgb565::WHITE)
        .build()?;

    // 2. Balanced donut using convenience method
    let balanced_donut = PieChart::builder()
        .center(Point::new(360, 120))
        .radius(80)
        .balanced_donut() // 50% inner radius using helper method
        .colors(&colors[0..5])
        .with_title("Balanced Donut (50%)")
        .background_color(Rgb565::WHITE)
        .build()?;

    // 3. Thick donut using convenience method
    let thick_donut = PieChart::builder()
        .center(Point::new(600, 120))
        .radius(80)
        .thick_donut() // 75% inner radius using helper method
        .colors(&colors[0..5])
        .with_title("Thick Donut (75%)")
        .background_color(Rgb565::WHITE)
        .build()?;

    // 4. Regular pie chart for comparison
    let pie_chart = PieChart::builder()
        .center(Point::new(240, 320))
        .radius(80)
        // No .donut() call = regular pie chart
        .colors(&colors[0..5])
        .with_title("Regular Pie Chart")
        .build()?;

    // 5. Donut chart optimized for small embedded displays using percentage method
    let embedded_donut = PieChart::builder()
        .center(Point::new(480, 320))
        .radius(60) // Smaller radius for constrained displays
        .donut_percentage(50) // 50% inner radius using percentage method
        .colors(&colors[0..5])
        .with_title("Embedded (50% method)")
        .background_color(Rgb565::WHITE)
        .build()?;

    // Create a shared legend for all charts
    let legend = StandardLegendBuilder::new()
        .position(LegendPos::Bottom)
        .orientation(LegendOrientation::Horizontal)
        .add_pie_entry("Documents", colors[0])?
        .add_pie_entry("Photos", colors[1])?
        .add_pie_entry("Videos", colors[2])?
        .add_pie_entry("Apps", colors[3])?
        .add_pie_entry("Other", colors[4])?
        .professional_style()
        .build()?;

    let renderer = StandardLegendRenderer::new();

    // Print donut chart guidance for developers
    println!("=== DONUT CHART CONFIGURATION GUIDE ===");
    println!("New helper methods for easier donut creation:");
    println!("  • .thin_donut()     -> 25% inner radius (emphasizes segments)");
    println!("  • .balanced_donut() -> 50% inner radius (best readability)");
    println!("  • .thick_donut()    -> 75% inner radius (maximizes center space)");
    println!("  • .donut_percentage(n) -> n% inner radius (custom percentage)");
    println!("  • .donut(pixels)    -> exact pixel inner radius (manual control)");
    println!();
    println!("Embedded system recommendations:");
    println!("  • Small displays (≤128px): Use .balanced_donut() with 60px radius");
    println!("  • Medium displays (240px): Use .donut_percentage(40-60) with 80px radius");
    println!("  • Large displays (≥480px): Any method with 100px+ radius");
    println!();
    println!("Memory considerations:");
    println!("  • Donut charts use same memory footprint as pie charts");
    println!("  • Center area available for totals, status, or icons");
    println!("  • Smaller outer radius improves performance on constrained systems");

    window::run_static(
        WindowConfig::new("Donut Chart Examples - Storage Usage")
            .theme(common::WindowTheme::Default)
            .background(Rgb565::WHITE)
            .size(Size::new(800, 480)), // Larger window to show multiple charts
        move |display, viewport| {
            // Draw all charts in the same viewport
            thin_donut.draw(&series, thin_donut.config(), viewport, display)?;
            balanced_donut.draw(&series, balanced_donut.config(), viewport, display)?;
            thick_donut.draw(&series, thick_donut.config(), viewport, display)?;
            pie_chart.draw(&series, pie_chart.config(), viewport, display)?;
            embedded_donut.draw(&series, embedded_donut.config(), viewport, display)?;

            // Draw legend at the bottom
            let legend_area = Rectangle::new(
                Point::new(50, 400),
                Size::new(700, 60)
            );
            renderer.render(&legend, legend_area, display)?;

            // Add informational text
            // Note: This would require text rendering capabilities
            // For now, we'll rely on the console output for guidance

            Ok(())
        },
    )
}

#[cfg(not(feature = "std"))]
fn main() {
    println!("This example requires the 'std' and 'pie' features.");
    println!("Run with: cargo run --example donut_chart --features \"std,pie\"");
}