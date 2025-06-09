//! Smooth Curve Chart - Visual Category
//!
//! This example demonstrates smooth curve interpolation using different algorithms
//! to create flowing curves from discrete data points.

use embedded_charts::chart::CurveChart;
use embedded_charts::math::interpolation::InterpolationType;
use embedded_charts::prelude::*;

// Import the common abstraction
#[path = "../common/mod.rs"]
mod common;

use common::{window, WindowConfig};

#[cfg(feature = "std")]
fn main() -> ChartResult<()> {
    println!("🎯 Smooth Curve Chart Demo");

    // Sample temperature data (time in hours, temperature in °C)
    let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
    let readings = [
        (0.0, 20.0),
        (3.0, 15.0),
        (6.0, 25.0),
        (9.0, 35.0),
        (12.0, 40.0),
        (15.0, 30.0),
        (18.0, 22.0),
        (21.0, 18.0),
        (24.0, 20.0),
    ];

    for (hour, temp) in readings.iter() {
        data.push(Point2D::new(*hour, *temp))?;
    }

    // Create a smooth curve chart with Catmull-Rom interpolation (more natural)
    let curve_chart = CurveChart::builder()
        .line_color(Rgb565::BLUE)
        .line_width(3)
        .interpolation_type(InterpolationType::CatmullRom) // More natural for temperature data
        .subdivisions(4) // Reduced subdivisions for cleaner, less wiggly curves
        .tension(0.5) // Moderate tension for balanced smoothness
        .fill_area(Rgb565::CSS_LIGHT_BLUE)
        .with_markers(MarkerStyle {
            shape: MarkerShape::Circle,
            size: 12,
            color: Rgb565::RED,
            visible: true,
        })
        .build()?;

    // Configure window
    let window_config = WindowConfig::new("Temperature Over Time - Smooth Curve")
        .size(Size::new(800, 600))
        .background(Rgb565::WHITE);

    // Run the window with chart rendering
    window::run_static(window_config, move |display, viewport| {
        let config = ChartConfig {
            title: Some(heapless::String::try_from("Temperature Over Time").unwrap_or_default()),
            background_color: None, // Window handles background
            margins: common::CHART_MARGINS,
            grid_color: Some(Rgb565::CSS_LIGHT_GRAY),
            show_grid: true,
        };

        curve_chart.draw(&data, &config, viewport, display)
    })?;

    println!("✅ Demo completed!");
    println!(
        "💡 The red dots show original data points, the smooth blue curve shows interpolation"
    );

    Ok(())
}

#[cfg(not(feature = "std"))]
fn main() {
    println!("This example requires the 'std' feature to run the simulator.");
    println!("Run with: cargo run --example smooth_curve_demo --features std,line");
}
