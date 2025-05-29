//! Legend System Demo - Four-Sided Legend Positioning
//!
//! This example showcases comprehensive legend functionality with legends positioned
//! on all four sides of the chart (top, bottom, left, right). Each legend demonstrates
//! different entry types, orientations, and styling options.
//!
//! Features demonstrated:
//! - Top legend: Horizontal orientation with line entries
//! - Bottom legend: Horizontal orientation with bar entries  
//! - Left legend: Vertical orientation with pie entries
//! - Right legend: Vertical orientation with custom entries
//! - Proper chart area adjustment for all legends
//! - Different styling for each legend position
//!
//! Run with: cargo run --example legend_demo --features std,fonts

use embedded_charts::prelude::*;

// Import the common abstraction
#[path = "../common/mod.rs"]
mod common;

use common::{configs, data, utils, window, WindowConfig};

#[cfg(feature = "std")]
fn main() -> ChartResult<()> {
    // Use the common animated example runner for legend demonstration
    window::run(
        WindowConfig::new("Four-Sided Legend Demo")
            .theme(common::WindowTheme::Default)
            .background(Rgb565::WHITE),
        |display, viewport, _elapsed| {
            // Demo comprehensive four-sided legend layout
            demo_four_sided_legends(display, viewport)
        },
    )
}

fn demo_four_sided_legends<D>(display: &mut D, viewport: Rectangle) -> ChartResult<()>
where
    D: DrawTarget<Color = Rgb565>,
{
    // Create sample data for multiple series
    let data1 = data::sine_wave(30, 12.0, 0.15, 0.0)?;
    let data2 = data::cosine_wave(30, 10.0, 0.15, 0.5)?;
    let data3 = data::linear_data(30, 0.8, 2.0, 8.0)?;
    let data4 = data::exponential_data(20, 1.1, 5.0)?;

    // Get professional color palette
    let colors = configs::professional_colors();

    // Calculate chart area with margins for all four legends
    let legend_margins = 80u32; // Space for legends on all sides
    let chart_area = Rectangle::new(
        Point::new(
            viewport.top_left.x + legend_margins as i32,
            viewport.top_left.y + legend_margins as i32,
        ),
        Size::new(
            viewport.size.width.saturating_sub(legend_margins * 2),
            viewport.size.height.saturating_sub(legend_margins * 2),
        ),
    );

    // Create charts with different colors
    let chart1 = LineChart::builder()
        .line_color(colors[0]) // Steel Blue
        .line_width(2)
        .build()?;

    let chart2 = LineChart::builder()
        .line_color(colors[1]) // Crimson
        .line_width(2)
        .build()?;

    let chart3 = LineChart::builder()
        .line_color(colors[2]) // Forest Green
        .line_width(2)
        .build()?;

    let chart4 = LineChart::builder()
        .line_color(colors[3]) // Orange
        .line_width(2)
        .build()?;

    // Draw the charts in the adjusted chart area
    chart1.draw(&data1, chart1.config(), chart_area, display)?;
    chart2.draw(&data2, chart2.config(), chart_area, display)?;
    chart3.draw(&data3, chart3.config(), chart_area, display)?;
    chart4.draw(&data4, chart4.config(), chart_area, display)?;

    // Create and render legends for all four positions
    create_and_render_top_legend(display, viewport, chart_area, &colors)?;
    create_and_render_bottom_legend(display, viewport, chart_area, &colors)?;
    create_and_render_left_legend(display, viewport, chart_area, &colors)?;
    create_and_render_right_legend(display, viewport, chart_area, &colors)?;

    Ok(())
}

/// Create and render the top legend (horizontal orientation, line entries)
fn create_and_render_top_legend<D>(
    display: &mut D,
    viewport: Rectangle,
    chart_area: Rectangle,
    colors: &[Rgb565],
) -> ChartResult<()>
where
    D: DrawTarget<Color = Rgb565>,
{
    // Create top legend with line entries (horizontal orientation)
    let top_legend = StandardLegendBuilder::new()
        .position(LegendPos::Top)
        .orientation(LegendOrientation::Horizontal)
        .add_line_entry("Temperature", colors[0])?
        .add_line_entry("Humidity", colors[1])?
        .professional_style()
        .build()?;

    // Calculate position for top legend
    let calculator = PositionCalculator::new(viewport, chart_area)
        .with_margins(LegendMargins::new(10, 10, 10, 10))
        .with_alignment(LegendAlignment::Center);

    let legend_size = top_legend.calculate_size();
    let legend_rect = calculator.calculate_legend_rect(LegendPos::Top, legend_size)?;

    // Render the top legend
    let renderer = StandardLegendRenderer::new();
    renderer.render(&top_legend, legend_rect, display)?;

    Ok(())
}

/// Create and render the bottom legend (horizontal orientation, bar entries)
fn create_and_render_bottom_legend<D>(
    display: &mut D,
    viewport: Rectangle,
    chart_area: Rectangle,
    colors: &[Rgb565],
) -> ChartResult<()>
where
    D: DrawTarget<Color = Rgb565>,
{
    // Create bottom legend with bar entries (horizontal orientation)
    let bottom_legend = StandardLegendBuilder::new()
        .position(LegendPos::Bottom)
        .orientation(LegendOrientation::Horizontal)
        .add_bar_entry("Pressure", colors[2])?
        .add_bar_entry("Wind Speed", colors[3])?
        .professional_style()
        .build()?;

    // Calculate position for bottom legend
    let calculator = PositionCalculator::new(viewport, chart_area)
        .with_margins(LegendMargins::new(10, 10, 10, 10))
        .with_alignment(LegendAlignment::Center);

    let legend_size = bottom_legend.calculate_size();
    let legend_rect = calculator.calculate_legend_rect(LegendPos::Bottom, legend_size)?;

    // Render the bottom legend
    let renderer = StandardLegendRenderer::new();
    renderer.render(&bottom_legend, legend_rect, display)?;

    Ok(())
}

/// Create and render the left legend (vertical orientation, pie entries)
fn create_and_render_left_legend<D>(
    display: &mut D,
    viewport: Rectangle,
    chart_area: Rectangle,
    colors: &[Rgb565],
) -> ChartResult<()>
where
    D: DrawTarget<Color = Rgb565>,
{
    // Create left legend with pie entries (vertical orientation)
    let left_legend = StandardLegendBuilder::new()
        .position(LegendPos::Left)
        .orientation(LegendOrientation::Vertical)
        .add_pie_entry("Solar", colors[4])?
        .add_pie_entry("Wind", colors[5])?
        .add_pie_entry("Hydro", colors[0])?
        .compact_style()
        .build()?;

    // Calculate position for left legend
    let calculator = PositionCalculator::new(viewport, chart_area)
        .with_margins(LegendMargins::new(10, 10, 10, 10))
        .with_alignment(LegendAlignment::Center);

    let legend_size = left_legend.calculate_size();
    let legend_rect = calculator.calculate_legend_rect(LegendPos::Left, legend_size)?;

    // Render the left legend
    let renderer = StandardLegendRenderer::new();
    renderer.render(&left_legend, legend_rect, display)?;

    Ok(())
}

/// Create and render the right legend (vertical orientation, custom entries)
fn create_and_render_right_legend<D>(
    display: &mut D,
    viewport: Rectangle,
    chart_area: Rectangle,
    colors: &[Rgb565],
) -> ChartResult<()>
where
    D: DrawTarget<Color = Rgb565>,
{
    // Create right legend with custom entries (vertical orientation)
    let right_legend = StandardLegendBuilder::new()
        .position(LegendPos::Right)
        .orientation(LegendOrientation::Vertical)
        .add_custom_entry("CPU", colors[0], SymbolShape::Circle, 12)?
        .add_custom_entry("Memory", colors[1], SymbolShape::Square, 12)?
        .add_custom_entry("Disk", colors[2], SymbolShape::Triangle, 12)?
        .add_custom_entry("Network", colors[3], SymbolShape::Diamond, 12)?
        .professional_style()
        .build()?;

    // Calculate position for right legend
    let calculator = PositionCalculator::new(viewport, chart_area)
        .with_margins(LegendMargins::new(10, 10, 10, 10))
        .with_alignment(LegendAlignment::Center);

    let legend_size = right_legend.calculate_size();
    let legend_rect = calculator.calculate_legend_rect(LegendPos::Right, legend_size)?;

    // Render the right legend
    let renderer = StandardLegendRenderer::new();
    renderer.render(&right_legend, legend_rect, display)?;

    Ok(())
}

/// Simple chart demo for when fonts feature is not available

#[cfg(not(feature = "std"))]
fn main() {
    utils::print_feature_requirement("std", "visual");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four_sided_legend_layout() {
        // Test that all four legend positions can be calculated
        let viewport = Rectangle::new(Point::zero(), Size::new(400, 300));
        let chart_area = Rectangle::new(Point::new(80, 80), Size::new(240, 140));
        let calculator = PositionCalculator::new(viewport, chart_area);

        let legend_size = Size::new(80, 60);

        // Test all four positions
        for position in &[
            LegendPos::Top,
            LegendPos::Bottom,
            LegendPos::Left,
            LegendPos::Right,
        ] {
            let rect = calculator
                .calculate_legend_rect(*position, legend_size)
                .unwrap();
            assert!(rect.size == legend_size);

            // Verify legend fits within viewport
            assert!(calculator
                .validate_legend_fit(*position, legend_size)
                .unwrap());
        }
    }

    #[test]
    fn test_legend_orientations() {
        // Test that horizontal orientation is used for top/bottom
        // and vertical orientation is used for left/right
        let top_legend = StandardLegendBuilder::new()
            .position(LegendPos::Top)
            .orientation(LegendOrientation::Horizontal)
            .add_line_entry("Test", Rgb565::RED)
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(top_legend.orientation(), LegendOrientation::Horizontal);

        let right_legend = StandardLegendBuilder::new()
            .position(LegendPos::Right)
            .orientation(LegendOrientation::Vertical)
            .add_line_entry("Test", Rgb565::BLUE)
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(right_legend.orientation(), LegendOrientation::Vertical);
    }

    #[test]
    fn test_legend_entry_types() {
        // Test different entry types for each legend using builder pattern
        let legend = StandardLegendBuilder::new()
            .add_line_entry("Line", Rgb565::RED)
            .unwrap()
            .add_bar_entry("Bar", Rgb565::GREEN)
            .unwrap()
            .add_pie_entry("Pie", Rgb565::BLUE)
            .unwrap()
            .add_custom_entry("Custom", Rgb565::YELLOW, SymbolShape::Circle, 12)
            .unwrap();

        let built_legend = legend.build().unwrap();
        assert_eq!(built_legend.entries().len(), 4);
    }

    #[test]
    fn test_chart_area_adjustment() {
        let viewport = Rectangle::new(Point::zero(), Size::new(400, 300));
        let legend_margins = 80u32;

        let adjusted_area = Rectangle::new(
            Point::new(legend_margins as i32, legend_margins as i32),
            Size::new(
                400_u32.saturating_sub(legend_margins * 2),
                300_u32.saturating_sub(legend_margins * 2),
            ),
        );

        // Verify the chart area is properly reduced
        assert_eq!(adjusted_area.size.width, 240);
        assert_eq!(adjusted_area.size.height, 140);
        assert_eq!(adjusted_area.top_left.x, 80);
        assert_eq!(adjusted_area.top_left.y, 80);
    }

    #[test]
    fn test_simple_chart_compatibility() {
        // This should work regardless of features
        let data = data::linear_data(5, 1.0, 0.0, 0.0).unwrap();
        assert_eq!(data.len(), 5);
    }
}
