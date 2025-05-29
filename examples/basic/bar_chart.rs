//! Visual Bar Chart - Visual Category
//!
//! This example demonstrates how to create a bar chart that users can actually see
//! in a window using the embedded-graphics-simulator.
//!
//! Run with: cargo run --example bar_chart_visual --features std

use embedded_charts::prelude::*;

// Import the common abstraction
#[path = "../common/mod.rs"]
mod common;

use common::{configs, layout, window, WindowConfig, WindowTheme};

#[cfg(feature = "std")]
fn main() -> ChartResult<()> {
    // Create sample sales data
    let mut series = StaticDataSeries::<Point2D, 256>::new();
    let data_points = [
        (1.0, 100.0), // Product 1: 100 units
        (2.0, 150.0), // Product 2: 150 units
        (3.0, 80.0),  // Product 3: 80 units
        (4.0, 200.0), // Product 4: 200 units
        (5.0, 120.0), // Product 5: 120 units
    ];

    for (x, y) in data_points.iter() {
        series
            .push(Point2D::new(*x, *y))
            .map_err(ChartError::from)?;
    }

    // Create a bar chart using common configuration
    let colors = configs::standard_colors();
    let chart = BarChart::builder()
        .colors(&colors[0..5])
        .bar_width(embedded_charts::chart::bar::BarWidth::Auto)
        .spacing(5) // More spacing for better visibility
        .with_title("Product Sales")
        .background_color(Rgb565::WHITE)
        .build()?;

    // Create legend and related objects outside the render loop
    let colors = configs::standard_colors();
    let legend = StandardLegendBuilder::new()
        .position(LegendPos::Right)
        .orientation(LegendOrientation::Vertical)
        .add_bar_entry("Product 1", colors[0])?
        .add_bar_entry("Product 2", colors[1])?
        .add_bar_entry("Product 3", colors[2])?
        .add_bar_entry("Product 4", colors[3])?
        .add_bar_entry("Product 5", colors[4])?
        .professional_style()
        .build()?;

    let renderer = StandardLegendRenderer::new();

    // Use the common animated example runner with auto-legend
    window::run(
        WindowConfig::new("Bar Chart Example")
            .theme(WindowTheme::Default)
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
    println!("Run with: cargo run --example bar_chart --features std");
}
