//! Visual Pie Chart - Visual Category
//!
//! This example demonstrates how to create a pie chart that users can actually see
//! in a window using the embedded-graphics-simulator.
//!
//! Run with: cargo run --example pie_chart_visual --features std

use embedded_charts::prelude::*;

// Import the common abstraction
#[path = "../common/mod.rs"]
mod common;

use common::{configs, layout, window, WindowConfig};

#[cfg(feature = "std")]
fn main() -> ChartResult<()> {
    // Create sample market share data
    let mut series = StaticDataSeries::<Point2D, 256>::new();
    let data_points = [
        (1.0, 35.0), // Company A: 35%
        (2.0, 25.0), // Company B: 25%
        (3.0, 20.0), // Company C: 20%
        (4.0, 15.0), // Company D: 15%
        (5.0, 5.0),  // Others: 5%
    ];

    for (x, y) in data_points.iter() {
        series
            .push(Point2D::new(*x, *y))
            .map_err(ChartError::from)?;
    }

    // Create a pie chart with appropriate sizing for large display
    let colors = configs::standard_colors();

    // Debug: Print the colors being used (only once)
    println!("=== PIE CHART COLOR DEBUG ===");
    println!("Standard colors from configs:");
    for (i, color) in colors[0..5].iter().enumerate() {
        println!(
            "  [{}]: RGB({}, {}, {})",
            i,
            color.r(),
            color.g(),
            color.b()
        );
    }

    let chart = PieChart::builder()
        .radius(100) // Larger radius for better visibility
        .colors(&colors[0..5])
        .with_title("Market Share")
        .build()?;

    // Debug: Print the chart's actual colors
    println!("Chart colors after building:");
    for (i, color) in chart.style().colors.iter().enumerate() {
        println!(
            "  [{}]: RGB({}, {}, {})",
            i,
            color.r(),
            color.g(),
            color.b()
        );
    }

    // Create legend and related objects outside the render loop
    let legend = StandardLegendBuilder::new()
        .position(LegendPos::Right)
        .orientation(LegendOrientation::Vertical)
        .add_pie_entry("Company A", colors[0])?
        .add_pie_entry("Company B", colors[1])?
        .add_pie_entry("Company C", colors[2])?
        .add_pie_entry("Company D", colors[3])?
        .add_pie_entry("Others", colors[4])?
        .professional_style()
        .build()?;

    let renderer = StandardLegendRenderer::new();

    window::run_static(
        WindowConfig::new("Pie Chart Example")
            .theme(common::WindowTheme::Default)
            .background(Rgb565::WHITE),
        move |display, viewport| {
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
    utils::print_feature_requirement("std", "visual");
}
