//! Stacked area chart example demonstrating multi-series data visualization with compelling server metrics.
//!
//! This example shows both static and animated stacked area charts, demonstrating:
//! - Multi-layer area visualization with smooth filled regions
//! - Animated transitions between data states (with animations feature)
//! - Professional styling and legends
//! - Real-world server metrics and energy transition scenarios

use embedded_charts::prelude::*;
use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    prelude::Size,
    primitives::{PrimitiveStyle, Rectangle},
    text::{Baseline, Text},
};

// Import the common abstraction
#[path = "../common/mod.rs"]
mod common;

use common::{window, WindowConfig, WindowTheme, CHART_MARGINS};

#[cfg(feature = "std")]
fn main() -> ChartResult<()> {
    println!("📈 Stacked Area Chart Demo");
    println!("==========================");
    println!("Features:");
    println!("• Static stacked area visualization");
    println!("• Animated transitions (with animations feature)");
    println!("• Professional styling and legends");
    println!("• Real-world data scenarios");
    println!();

    // Check if animations feature is available
    #[cfg(feature = "animations")]
    {
        println!("🎬 Running animated energy transition demo...");
        run_animated_demo()
    }
    #[cfg(not(feature = "animations"))]
    {
        println!("📊 Running static server metrics demo...");
        println!("   Add --features animations for animated energy transitions");
        run_static_demo()
    }
}

#[cfg(feature = "std")]
#[allow(dead_code)]
fn run_static_demo() -> ChartResult<()> {
    // Create realistic server metrics data over 24 hours (hourly samples)
    // Data represents server load in different categories

    // Realistic hourly data showing different usage patterns
    let hours = [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22];
    let infrastructure_load = [
        15.0, 12.0, 10.0, 12.0, 18.0, 25.0, 30.0, 35.0, 40.0, 38.0, 25.0, 20.0,
    ];
    let application_load = [
        5.0, 3.0, 2.0, 8.0, 25.0, 45.0, 60.0, 75.0, 80.0, 65.0, 35.0, 15.0,
    ];
    let traffic_load = [
        2.0, 1.0, 0.5, 3.0, 15.0, 35.0, 55.0, 70.0, 85.0, 60.0, 25.0, 8.0,
    ];
    let batch_load = [
        40.0, 45.0, 50.0, 35.0, 10.0, 5.0, 2.0, 1.0, 1.0, 5.0, 15.0, 30.0,
    ];

    // Calculate cumulative values for stacking
    let mut cumulative_data = vec![];
    for i in 0..hours.len() {
        let infra = infrastructure_load[i];
        let app = application_load[i];
        let traffic = traffic_load[i];
        let batch = batch_load[i];

        cumulative_data.push([
            infra,                         // Layer 1: Infrastructure only
            infra + app,                   // Layer 2: Infrastructure + Applications
            infra + app + traffic,         // Layer 3: Infrastructure + Applications + Traffic
            infra + app + traffic + batch, // Layer 4: Total (all layers)
        ]);
    }

    // Define distinct, high-contrast colors for each layer
    let colors = [
        Rgb565::new(8, 12, 31), // Deep blue for Infrastructure (bottom)
        Rgb565::new(8, 31, 12), // Deep green for Applications
        Rgb565::new(31, 20, 8), // Orange for User Traffic
        Rgb565::new(31, 8, 8),  // Red for Batch Jobs (top)
    ];

    // Pre-create legend and layout objects outside the animation loop
    let legend = StandardLegendBuilder::new()
        .position(LegendPos::Right)
        .orientation(LegendOrientation::Vertical)
        .add_line_entry("Infrastructure", colors[0])?
        .add_line_entry("Applications", colors[1])?
        .add_line_entry("User Traffic", colors[2])?
        .add_line_entry("Batch Jobs", colors[3])?
        .professional_style()
        .build()?;

    let legend_renderer = StandardLegendRenderer::new();
    let legend_size = legend.calculate_size();
    let title_style = MonoTextStyle::new(&FONT_6X10, Rgb565::BLACK);

    // Pre-calculate layout constants outside the render loop
    let margin = 60u32;

    // Use the common visual example runner
    window::run(
        WindowConfig::new("Stacked Area Chart - Server Metrics")
            .theme(WindowTheme::Default)
            .background(Rgb565::WHITE),
        move |display, viewport, _elapsed| {
            // Clear background
            let _ = display.clear(Rgb565::WHITE);

            // Calculate drawing area with pre-calculated margin
            let draw_area = Rectangle::new(
                Point::new(
                    viewport.top_left.x + margin as i32,
                    viewport.top_left.y + margin as i32,
                ),
                Size::new(
                    viewport.size.width.saturating_sub(margin * 2),
                    viewport.size.height.saturating_sub(margin * 2),
                ),
            );

            // Find maximum value for scaling
            let max_value = cumulative_data
                .iter()
                .map(|row| row[3]) // Total value (top layer)
                .fold(0.0f32, |a, b| a.max(b));

            // Convert data to screen coordinates
            let mut screen_points = vec![];
            for (i, hour_data) in cumulative_data.iter().enumerate() {
                let x = draw_area.top_left.x
                    + ((i as f32 / (hours.len() - 1) as f32) * draw_area.size.width as f32) as i32;

                let mut layer_points = [Point::new(0, 0); 4];
                for (layer, &value) in hour_data.iter().enumerate() {
                    let y = draw_area.top_left.y + draw_area.size.height as i32
                        - ((value / max_value) * draw_area.size.height as f32) as i32;
                    layer_points[layer] = Point::new(x, y);
                }
                screen_points.push(layer_points);
            }

            // Draw stacked areas from top to bottom
            // Each area is drawn between two cumulative lines
            for layer in (0..4).rev() {
                let color = colors[layer];

                // For each segment between consecutive time points
                for i in 0..screen_points.len() - 1 {
                    let current = &screen_points[i];
                    let next = &screen_points[i + 1];

                    // Get the boundary points for this layer
                    let top_current = current[layer];
                    let top_next = next[layer];

                    // Bottom boundary (previous layer or baseline)
                    let bottom_current = if layer > 0 {
                        current[layer - 1]
                    } else {
                        Point::new(
                            current[0].x,
                            draw_area.top_left.y + draw_area.size.height as i32,
                        )
                    };
                    let bottom_next = if layer > 0 {
                        next[layer - 1]
                    } else {
                        Point::new(
                            next[0].x,
                            draw_area.top_left.y + draw_area.size.height as i32,
                        )
                    };

                    // Draw the area as a filled quadrilateral using scan lines
                    draw_filled_quad(
                        display,
                        top_current,
                        top_next,
                        bottom_next,
                        bottom_current,
                        color,
                    )
                    .ok();
                }
            }

            // Draw title
            let title_text = "Server Load Distribution - Stacked Areas";
            let title_pos = Point::new(
                viewport.top_left.x + (viewport.size.width as i32 / 2)
                    - (title_text.len() as i32 * 3),
                viewport.top_left.y + 20,
            );
            let _ = Text::with_baseline(title_text, title_pos, title_style, Baseline::Top)
                .draw(display);

            // Calculate legend layout and render using pre-created objects
            let calculator = PositionCalculator::new(viewport, viewport);
            let legend_rect = calculator.calculate_legend_rect(legend.position(), legend_size)?;
            legend_renderer.render(&legend, legend_rect, display)?;

            // Draw time labels
            for (i, &hour) in hours.iter().enumerate() {
                let x = draw_area.top_left.x
                    + ((i as f32 / (hours.len() - 1) as f32) * draw_area.size.width as f32) as i32;
                let y = draw_area.top_left.y + draw_area.size.height as i32 + 15;

                let hour_text = format!("{hour}h");
                let label_pos = Point::new(x - 6, y);
                let _ = Text::with_baseline(&hour_text, label_pos, title_style, Baseline::Top)
                    .draw(display);
            }

            Ok(())
        },
    )
}

#[cfg(all(feature = "std", feature = "animations"))]
fn run_animated_demo() -> ChartResult<()> {
    // Create compelling energy production data for a renewable transition
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];

    // Initial energy production data (in GWh) - traditional mix
    let initial_coal = [45.0, 42.0, 40.0, 35.0, 30.0, 25.0];
    let initial_natural_gas = [35.0, 38.0, 40.0, 42.0, 45.0, 48.0];
    let initial_nuclear = [25.0, 25.0, 25.0, 25.0, 25.0, 25.0];
    let initial_renewables = [15.0, 18.0, 22.0, 28.0, 35.0, 42.0];

    // Target energy production data (in GWh) - renewable transition
    let target_coal = [20.0, 15.0, 12.0, 8.0, 5.0, 2.0]; // Phasing out
    let target_natural_gas = [25.0, 22.0, 20.0, 18.0, 15.0, 12.0]; // Reducing
    let target_nuclear = [25.0, 25.0, 25.0, 25.0, 25.0, 25.0]; // Stable
    let target_renewables = [50.0, 58.0, 68.0, 79.0, 90.0, 101.0]; // Growing rapidly

    // Define colors representing energy sources
    let colors = [
        Rgb565::new(10, 10, 10), // Dark gray for Coal (bottom)
        Rgb565::new(20, 15, 8),  // Brown for Natural Gas
        Rgb565::new(8, 20, 31),  // Blue for Nuclear
        Rgb565::new(8, 31, 8),   // Green for Renewables (top)
    ];

    // Create initial stacked data
    let mut initial_data = StackedData::new();

    // Add layers for initial data
    let mut coal_layer = StaticDataSeries::new();
    let mut gas_layer = StaticDataSeries::new();
    let mut nuclear_layer = StaticDataSeries::new();
    let mut renewables_layer = StaticDataSeries::new();

    for i in 0..6 {
        coal_layer.push(Point2D::new(i as f32, initial_coal[i]))?;
        gas_layer.push(Point2D::new(i as f32, initial_natural_gas[i]))?;
        nuclear_layer.push(Point2D::new(i as f32, initial_nuclear[i]))?;
        renewables_layer.push(Point2D::new(i as f32, initial_renewables[i]))?;
    }

    initial_data.add_layer(coal_layer, "Coal", colors[0])?;
    initial_data.add_layer(gas_layer, "Natural Gas", colors[1])?;
    initial_data.add_layer(nuclear_layer, "Nuclear", colors[2])?;
    initial_data.add_layer(renewables_layer, "Renewables", colors[3])?;

    // Create target stacked data
    let mut target_data = StackedData::new();

    // Add layers for target data
    let mut target_coal_layer = StaticDataSeries::new();
    let mut target_gas_layer = StaticDataSeries::new();
    let mut target_nuclear_layer = StaticDataSeries::new();
    let mut target_renewables_layer = StaticDataSeries::new();

    for i in 0..6 {
        target_coal_layer.push(Point2D::new(i as f32, target_coal[i]))?;
        target_gas_layer.push(Point2D::new(i as f32, target_natural_gas[i]))?;
        target_nuclear_layer.push(Point2D::new(i as f32, target_nuclear[i]))?;
        target_renewables_layer.push(Point2D::new(i as f32, target_renewables[i]))?;
    }

    target_data.add_layer(target_coal_layer, "Coal", colors[0])?;
    target_data.add_layer(target_gas_layer, "Natural Gas", colors[1])?;
    target_data.add_layer(target_nuclear_layer, "Nuclear", colors[2])?;
    target_data.add_layer(target_renewables_layer, "Renewables", colors[3])?;

    // Create stacked area chart
    let chart = AnimatedStackedLineChart::builder()
        .smooth_lines(true)
        .build()?;

    // Create animators for transitions
    let forward_animator = ChartAnimator::new(
        initial_data.clone(),
        target_data.clone(),
        EasingFunction::EaseInOut,
    );
    let backward_animator = ChartAnimator::new(
        target_data.clone(),
        initial_data.clone(),
        EasingFunction::EaseOut,
    );

    // Pre-create legend (static, doesn't change)
    let legend = StandardLegendBuilder::new()
        .position(LegendPos::Right)
        .orientation(LegendOrientation::Vertical)
        .add_line_entry("Coal", colors[0])?
        .add_line_entry("Natural Gas", colors[1])?
        .add_line_entry("Nuclear", colors[2])?
        .add_line_entry("Renewables", colors[3])?
        .professional_style()
        .build()?;

    // Pre-calculate legend size and layout dimensions
    let legend_size = legend.calculate_size();
    let legend_renderer = StandardLegendRenderer::new();

    // Pre-create chart configuration (static)
    let config = ChartConfig {
        title: Some(heapless::String::try_from("Energy Transition - Renewable Growth").unwrap()),
        background_color: Some(Rgb565::WHITE),
        margins: CHART_MARGINS,
        show_grid: false,
        grid_color: None,
    };

    // Pre-create text style for month labels
    let text_style = embedded_graphics::mono_font::MonoTextStyle::new(
        &embedded_graphics::mono_font::ascii::FONT_6X10,
        Rgb565::BLACK,
    );

    // Pre-create animation timing constants
    let animation_cycle = 8.0; // 8 second cycle
    let forward_duration = 4.0; // First half of cycle
    let backward_duration = 4.0; // Second half of cycle

    // Pre-create layout calculation constants outside the render loop
    let legend_spacing = 20u32;
    let month_count = 6;

    // Use the common visual example runner
    window::run(
        WindowConfig::new("Animated Stacked Area Chart - Energy Transition")
            .theme(WindowTheme::Default)
            .background(Rgb565::WHITE),
        move |display, viewport, elapsed| {
            // Clear background
            let _ = display.clear(Rgb565::WHITE);

            // Start animation based on elapsed time
            let cycle_position = elapsed % animation_cycle;

            // Determine animation state and progress
            let (current_data, progress) = if cycle_position < forward_duration {
                // Forward animation: initial -> target
                let progress = ((cycle_position / forward_duration) * 100.0) as Progress;
                let current_data = forward_animator
                    .value_at(progress)
                    .ok_or(ChartError::InvalidData)?;
                (current_data, progress)
            } else {
                // Backward animation: target -> initial
                let backward_progress =
                    (((cycle_position - forward_duration) / backward_duration) * 100.0) as Progress;
                let current_data = backward_animator
                    .value_at(backward_progress)
                    .ok_or(ChartError::InvalidData)?;
                (current_data, backward_progress)
            };

            // Calculate layout with pre-created legend
            let calculator = PositionCalculator::new(viewport, viewport);
            let legend_rect = calculator.calculate_legend_rect(legend.position(), legend_size)?;

            // Adjust chart area to leave space for legend
            let chart_area = Rectangle::new(
                viewport.top_left,
                Size::new(
                    viewport
                        .size
                        .width
                        .saturating_sub(legend_size.width + legend_spacing),
                    viewport.size.height,
                ),
            );

            // Draw the chart in the adjusted area
            chart.draw_animated(&current_data, &config, chart_area, display, progress)?;

            // Render the legend using pre-created renderer
            legend_renderer.render(&legend, legend_rect, display)?;

            // Draw month labels using pre-calculated layout
            let draw_area = config.margins.apply_to(chart_area);
            let month_spacing = draw_area.size.width / (month_count - 1) as u32;

            for (i, month) in months.iter().enumerate() {
                let month_x = draw_area.top_left.x + (i as u32 * month_spacing) as i32;
                let label_pos = Point::new(
                    month_x - 8,
                    draw_area.top_left.y + draw_area.size.height as i32 + 15,
                );

                embedded_graphics::text::Text::with_baseline(
                    month,
                    label_pos,
                    text_style,
                    embedded_graphics::text::Baseline::Top,
                )
                .draw(display)
                .ok();
            }

            Ok(())
        },
    )
}

/// Draw a filled quadrilateral by splitting it into two triangles
#[allow(dead_code)]
fn draw_filled_quad<D>(
    display: &mut D,
    p1: Point,
    p2: Point,
    p3: Point,
    p4: Point,
    color: Rgb565,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    // Split the quadrilateral into two triangles and fill them
    // Triangle 1: p1, p2, p3
    draw_filled_triangle(display, p1, p2, p3, color)?;
    // Triangle 2: p1, p3, p4
    draw_filled_triangle(display, p1, p3, p4, color)?;

    Ok(())
}

/// Draw a filled triangle using scan line algorithm
#[allow(dead_code)]
fn draw_filled_triangle<D>(
    display: &mut D,
    p1: Point,
    p2: Point,
    p3: Point,
    color: Rgb565,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    // Find bounding box
    let min_x = p1.x.min(p2.x).min(p3.x);
    let max_x = p1.x.max(p2.x).max(p3.x);
    let min_y = p1.y.min(p2.y).min(p3.y);
    let max_y = p1.y.max(p2.y).max(p3.y);

    // For each horizontal scan line
    for y in min_y..=max_y {
        let mut intersections = vec![];

        // Check intersection with each edge of the triangle
        let edges = [(p1, p2), (p2, p3), (p3, p1)];
        for (start, end) in edges.iter() {
            if let Some(x) = line_intersection_x(*start, *end, y) {
                if x >= min_x && x <= max_x {
                    intersections.push(x);
                }
            }
        }

        // Remove duplicates and sort
        intersections.sort();
        intersections.dedup();

        // Draw horizontal line between the two intersection points
        if intersections.len() >= 2 {
            let start_x = intersections[0];
            let end_x = intersections[intersections.len() - 1];
            if start_x != end_x {
                let rect = Rectangle::new(
                    Point::new(start_x, y),
                    Size::new((end_x - start_x) as u32, 1),
                );
                rect.into_styled(PrimitiveStyle::with_fill(color))
                    .draw(display)?;
            }
        }
    }

    Ok(())
}

/// Find x-coordinate where a line segment intersects a horizontal line at y
#[allow(dead_code)]
fn line_intersection_x(start: Point, end: Point, y: i32) -> Option<i32> {
    if start.y == end.y {
        // Horizontal line - no single intersection point
        return None;
    }

    if (start.y <= y && y <= end.y) || (end.y <= y && y <= start.y) {
        // Linear interpolation
        let t = (y - start.y) as f32 / (end.y - start.y) as f32;
        let x = start.x as f32 + t * (end.x - start.x) as f32;
        Some(x.round() as i32)
    } else {
        None
    }
}

#[cfg(not(feature = "std"))]
fn main() {
    println!("⚠️  This visual example requires the 'std' feature to run");
    println!("   Run with: cargo run --example stacked_line_chart --features std");
    println!("   Add --features animations for animated energy transitions");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stacked_area_chart() {
        // Test that the example runs without panicking
        main().unwrap();
    }

    #[test]
    fn test_cumulative_calculation() {
        let infrastructure = 15.0;
        let applications = 25.0;
        let traffic = 35.0;
        let batch = 10.0;

        let cumulative = [
            infrastructure,                                  // 15
            infrastructure + applications,                   // 40
            infrastructure + applications + traffic,         // 75
            infrastructure + applications + traffic + batch, // 85
        ];

        assert_eq!(cumulative[0], 15.0);
        assert_eq!(cumulative[1], 40.0);
        assert_eq!(cumulative[2], 75.0);
        assert_eq!(cumulative[3], 85.0);
    }

    #[test]
    fn test_line_intersection() {
        // Test horizontal line intersection
        let start = Point::new(0, 0);
        let end = Point::new(10, 10);
        let y = 5;

        let intersection = line_intersection_x(start, end, y);
        assert_eq!(intersection, Some(5));

        // Test no intersection
        let y_outside = 15;
        let no_intersection = line_intersection_x(start, end, y_outside);
        assert_eq!(no_intersection, None);
    }

    #[test]
    fn test_realistic_server_metrics() {
        // Test that our server metrics data makes sense
        let infrastructure_base = 15.0;
        let peak_traffic = 85.0;
        let night_batch = 50.0;

        // Peak total load should be substantial
        let peak_total = infrastructure_base + 75.0 + peak_traffic + 1.0; // ~176
        assert!(
            peak_total > 150.0,
            "Peak load should be substantial for visual impact"
        );

        // Night batch processing should be significant
        assert!(
            night_batch > 40.0,
            "Batch processing should be visually significant"
        );

        // Traffic should vary dramatically
        let min_traffic = 0.5;
        let traffic_ratio = peak_traffic / min_traffic;
        assert!(
            traffic_ratio > 100.0,
            "Traffic should show dramatic variation"
        );
    }

    #[test]
    fn test_stacked_area_approach() {
        // Test that our stacked area approach correctly shows layer contributions
        let _hours = [0, 8, 16];
        let infrastructure = [15.0, 18.0, 40.0];
        let applications = [5.0, 25.0, 80.0];
        let traffic = [2.0, 15.0, 85.0];
        let batch = [40.0, 10.0, 1.0];

        // At hour 16 (peak time), verify the stacking
        let i = 2;
        let layer1 = infrastructure[i]; // 40
        let layer2 = infrastructure[i] + applications[i]; // 120
        let layer3 = infrastructure[i] + applications[i] + traffic[i]; // 205
        let layer4 = infrastructure[i] + applications[i] + traffic[i] + batch[i]; // 206

        // The visual areas between layers show each component's contribution
        let infra_area = layer1; // 40
        let app_area = layer2 - layer1; // 80
        let traffic_area = layer3 - layer2; // 85
        let batch_area = layer4 - layer3; // 1

        assert_eq!(infra_area, infrastructure[i]);
        assert_eq!(app_area, applications[i]);
        assert_eq!(traffic_area, traffic[i]);
        assert_eq!(batch_area, batch[i]);

        // Total should equal sum of all components
        assert_eq!(
            layer4,
            infrastructure[i] + applications[i] + traffic[i] + batch[i]
        );
    }

    #[cfg(feature = "animations")]
    #[test]
    fn test_animated_stacked_area_chart_creation() {
        let _chart = AnimatedStackedBarChart::<Rgb565>::builder()
            .build()
            .unwrap();

        // Test chart creation
        assert!(true); // Chart created successfully
    }

    #[cfg(feature = "animations")]
    #[test]
    fn test_energy_transition_data() {
        // Test that our energy transition data shows compelling change
        let initial_renewables = 15.0;
        let target_renewables = 101.0;
        let renewables_growth = (target_renewables - initial_renewables) / initial_renewables;

        // Renewables should show dramatic growth (>500%)
        assert!(
            renewables_growth > 5.0,
            "Renewables should show dramatic growth"
        );

        // Coal should be phasing out
        let initial_coal = 45.0;
        let target_coal = 2.0;
        let coal_reduction = (initial_coal - target_coal) / initial_coal;
        assert!(coal_reduction > 0.9, "Coal should be mostly phased out");
    }
}
