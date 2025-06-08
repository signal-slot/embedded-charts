//! Axis System Demo - Visual Category
//!
//! This example demonstrates the axis system with line charts,
//! showing professional axis styling and grid features.
//!
//! Run with: cargo run --example axis_demo

use embedded_charts::prelude::*;

// Import the common abstraction
#[path = "../common/mod.rs"]
mod common;

use common::{window, WindowConfig, CHART_MARGINS};

#[cfg(feature = "std")]
fn main() -> ChartResult<()> {
    // Create sample data using common utilities

    let mut series = StaticDataSeries::new();
    let data = [
        (0.0, 10.0),
        (1.0, 15.0),
        (2.0, 8.0),
        (3.0, 22.0),
        (4.0, 18.0),
        (5.0, 25.0),
        (6.0, 12.0),
        (7.0, 30.0),
        (8.0, 28.0),
        (9.0, 35.0),
    ];

    for (x, y) in &data {
        series
            .push(Point2D::new(*x, *y))
            .map_err(ChartError::from)?;
    }

    // Get data bounds and calculate nice ranges using the library function
    let bounds = series.bounds()?;
    let ((x_min, x_max), (y_min, y_max)) =
        calculate_nice_ranges_from_bounds(&bounds, RangeCalculationConfig::default());

    // Calculate appropriate tick counts based on range
    let x_range = x_max - x_min;
    let y_range = y_max - y_min;

    // Determine tick count - aim for 5-8 ticks total for readability
    let x_tick_count = if x_range <= 10.0 { 6 } else { 8 };
    let y_tick_count = if y_range <= 20.0 {
        5
    } else if y_range <= 50.0 {
        6
    } else {
        8
    };

    // Create X-axis (horizontal, bottom) with calculated range
    let x_axis = presets::professional_x_axis(x_min, x_max)
        .tick_count(x_tick_count)
        .show_grid(true)
        .build()?;

    // Create Y-axis (vertical, left) with calculated range
    let y_axis = presets::professional_y_axis(y_min, y_max)
        .tick_count(y_tick_count)
        .show_grid(true)
        .build()?;

    // Create line chart with axes and legend
    let mut margins = CHART_MARGINS;
    margins.right += 120; // Space for right-side legend

    let chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .line_width(2)
        .with_title("Temperature Over Time")
        .background_color(Rgb565::WHITE)
        .margins(margins)
        .with_x_axis(x_axis)
        .with_y_axis(y_axis)
        .build()?;

    // Use the common animated example runner
    window::run(
        WindowConfig::new("Axis System Demo")
            .theme(common::WindowTheme::Default)
            .background(Rgb565::WHITE),
        |display, viewport, _elapsed| chart.draw(&series, chart.config(), viewport, display),
    )
}

#[cfg(not(feature = "std"))]
fn main() {
    utils::print_feature_requirement("std", "visual");
}
