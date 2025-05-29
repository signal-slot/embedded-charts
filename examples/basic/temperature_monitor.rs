//! Temperature Monitor - Visual Category
//!
//! This example demonstrates the common abstraction for visual examples
//! that use SimulatorDisplay with windows.
//!
//! Run with: cargo run --example temperature_monitor --features std

use embedded_charts::prelude::*;

// Import the common abstraction
#[path = "../common/mod.rs"]
mod common;

use common::{configs, data, layout, utils, WindowConfig};

#[cfg(feature = "std")]
fn main() -> ChartResult<()> {
    // Generate temperature data for 24 hours
    let temperature_data = data::temperature_data(24)?;

    // Create a professional chart
    let chart = configs::professional_line_chart(Rgb565::new(220 >> 3, 20 >> 2, 60 >> 3))?; // Crimson

    // Create legend and related objects outside the render loop
    let legend = StandardLegendBuilder::new()
        .position(LegendPos::Right)
        .orientation(LegendOrientation::Vertical)
        .add_line_entry("Temperature", Rgb565::new(220 >> 3, 20 >> 2, 60 >> 3))?
        .professional_style()
        .build()?;

    let renderer = StandardLegendRenderer::new();

    // Use the common animated example runner with auto-legend
    common::window::run(
        WindowConfig::new("Temperature Monitor")
            .theme(common::WindowTheme::Default)
            .background(Rgb565::new(248 >> 3, 248 >> 2, 255 >> 3)), // Very light blue
        move |display, viewport, _time| {
            // Use automatic legend layout
            layout::draw_chart_with_auto_legend(
                |chart_area, display| {
                    chart.draw(&temperature_data, chart.config(), chart_area, display)
                },
                viewport,
                display,
                layout::ChartWithLegend::new(&legend, &renderer),
            )?;

            // Print data information
            utils::print_series_info(&temperature_data, "Temperature");

            println!("🌡️  Temperature range: 15°C to 30°C over 24 hours");
            println!("📊 Chart shows realistic daily temperature cycle");

            Ok(())
        },
    )
}

#[cfg(not(feature = "std"))]
fn main() {
    utils::print_feature_requirement("std", "visual");
}
