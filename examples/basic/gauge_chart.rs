//! Basic Gauge Chart - Visual Category
//!
//! This example demonstrates how to create a basic gauge chart that users can actually see
//! in a window using the embedded-graphics-simulator.

use embedded_charts::prelude::*;

// Import the common abstraction
#[path = "../common/mod.rs"]
mod common;

use common::{window, WindowConfig, WindowTheme};

#[cfg(feature = "std")]
fn main() -> ChartResult<()> {
    println!("Basic gauge chart created successfully!");
    println!("Gauge type: Semicircle");
    println!("Value range: 0-100");
    println!("Current value: 75.0 (in red zone)");
    println!("Threshold zones: Green (0-30), Yellow (30-70), Red (70-100)");

    // Create a basic gauge chart
    let gauge = GaugeChart::builder()
        .gauge_type(GaugeType::Semicircle)
        .value_range(0.0, 100.0)
        .radius(80)
        .needle_style(NeedleShape::Arrow, Rgb565::RED, 0.85, 3)
        // Clear existing zones and add custom ones
        .add_threshold_zone(0.0, 30.0, Rgb565::GREEN)
        .add_threshold_zone(30.0, 70.0, Rgb565::YELLOW)
        .add_threshold_zone(70.0, 100.0, Rgb565::RED)
        .with_title("Basic Gauge")
        .build()?;

    // Create data with a single value (75% - in the red zone)
    let mut data: StaticDataSeries<Point2D, 1> = StaticDataSeries::new();
    data.push(Point2D::new(0.0, 75.0))?;

    // Create legend and related objects outside the render loop
    let legend = StandardLegendBuilder::new()
        .position(LegendPos::Right)
        .orientation(LegendOrientation::Vertical)
        .add_line_entry("Safe Zone", Rgb565::GREEN)?
        .add_line_entry("Warning Zone", Rgb565::YELLOW)?
        .add_line_entry("Danger Zone", Rgb565::RED)?
        .professional_style()
        .build()?;

    let renderer = StandardLegendRenderer::<Rgb565>::new();

    // Use the common animated example runner
    window::run(
        WindowConfig::new("Basic Gauge Chart")
            .theme(WindowTheme::Default)
            .background(Rgb565::WHITE),
        move |display, viewport, _elapsed| gauge.draw(&data, gauge.config(), viewport, display),
    )
}

#[cfg(not(feature = "std"))]
fn main() {
    println!("This example requires the 'std' feature to be enabled.");
    println!("Run with: cargo run --example gauge_chart --features std");
}
