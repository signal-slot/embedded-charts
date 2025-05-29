//! Visual Line Chart - Visual Category
//!
//! This example demonstrates how to create a line chart that users can actually see
//! in a window using the embedded-graphics-simulator.
//!
//! Run with: cargo run --example line_chart_visual --features std

use embedded_charts::prelude::*;

// Import the common abstraction
#[path = "../common/mod.rs"]
mod common;

use common::{layout, window, WindowConfig};

#[cfg(feature = "std")]
fn main() -> ChartResult<()> {
    // Create sample temperature data over time
    let mut series = StaticDataSeries::<Point2D, 256>::new();
    let data_points = [
        (0.0, 20.0), // 0 hours: 20°C
        (1.0, 22.0), // 1 hour: 22°C
        (2.0, 25.0), // 2 hours: 25°C
        (3.0, 28.0), // 3 hours: 28°C
        (4.0, 30.0), // 4 hours: 30°C
        (5.0, 27.0), // 5 hours: 27°C
        (6.0, 24.0), // 6 hours: 24°C
        (7.0, 21.0), // 7 hours: 21°C
    ];

    for (x, y) in data_points.iter() {
        series
            .push(Point2D::new(*x, *y))
            .map_err(ChartError::from)?;
    }

    // Create a line chart using common configuration
    let chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .line_width(2)
        .with_title("Temperature Over Time")
        .background_color(Rgb565::WHITE)
        .build()?;

    // Pre-create legend and layout objects outside the loop
    let legend = StandardLegendBuilder::new()
        .position(LegendPos::Right)
        .orientation(LegendOrientation::Vertical)
        .add_line_entry("Temperature", Rgb565::BLUE)?
        .professional_style()
        .build()?;

    let renderer = StandardLegendRenderer::new();

    // Use the common visual example runner with auto-legend
    window::run(
        WindowConfig::new("Line Chart Example")
            .theme(common::WindowTheme::Default)
            .background(Rgb565::WHITE),
        move |display, viewport, _elapsed| {
            // Use automatic legend layout
            layout::draw_chart_with_auto_legend(
                |chart_area, display| chart.draw(&series, chart.config(), chart_area, display),
                viewport,
                display,
                layout::ChartWithLegend::new(&legend, &renderer),
            )
        },
    )
}

#[cfg(not(feature = "std"))]
fn main() {
    println!("This example requires the 'std' feature to be enabled.");
    println!("Run with: cargo run --example line_chart --features std");
}
