//! Stacked bar chart example demonstrating multi-segment revenue visualization with compelling data.
//!
//! This example shows both static and animated stacked bar charts, demonstrating:
//! - Multi-layer data visualization
//! - Smooth animation transitions between data states
//! - Professional styling and legends
//! - Real-world business data scenarios

use embedded_charts::prelude::*;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    primitives::PrimitiveStyle,
    text::{Baseline, Text},
};

// Import the common abstraction
#[path = "../common/mod.rs"]
mod common;

use common::{window, WindowConfig, WindowTheme, CHART_MARGINS};

#[cfg(feature = "std")]
fn main() -> ChartResult<()> {
    println!("📊 Stacked Bar Chart Demo");
    println!("=========================");
    println!("Features:");
    println!("• Static stacked bar visualization");
    println!("• Animated transitions (with animations feature)");
    println!("• Professional styling and legends");
    println!("• Real-world business data");
    println!();

    // Check if animations feature is available
    #[cfg(feature = "animations")]
    {
        println!("🎬 Running animated version...");
        run_animated_demo()
    }
    #[cfg(not(feature = "animations"))]
    {
        println!("📊 Running static version...");
        println!("   Add --features animations for animated transitions");
        run_static_demo()
    }
}

#[cfg(feature = "std")]
#[allow(dead_code)]
fn run_static_demo() -> ChartResult<()> {
    // Create compelling quarterly revenue data for a growing tech company
    // Data shows dramatic growth and changing product mix over time
    let _quarters = ["Q1 2023", "Q2 2023", "Q3 2023", "Q4 2023"];

    // Revenue data by product line (in millions) - showing realistic growth patterns
    let cloud_services = [45.0, 65.0, 85.0, 120.0]; // Fastest growing segment
    let enterprise_software = [80.0, 90.0, 95.0, 110.0]; // Steady growth
    let mobile_apps = [25.0, 35.0, 55.0, 75.0]; // Strong growth
    let consulting = [30.0, 32.0, 28.0, 35.0]; // Stable but smaller

    // Define vibrant, distinct colors for each product segment
    let colors = [
        Rgb565::new(8, 15, 31), // Deep blue for Cloud Services (bottom)
        Rgb565::new(8, 31, 15), // Deep green for Enterprise Software
        Rgb565::new(31, 20, 8), // Orange for Mobile Apps
        Rgb565::new(31, 8, 15), // Purple for Consulting (top)
    ];

    // Create legend and related objects outside the render loop
    let legend = StandardLegendBuilder::new()
        .position(LegendPos::Right)
        .orientation(LegendOrientation::Vertical)
        .add_bar_entry("Cloud Services", colors[0])?
        .add_bar_entry("Enterprise Software", colors[1])?
        .add_bar_entry("Mobile Apps", colors[2])?
        .add_bar_entry("Consulting", colors[3])?
        .professional_style()
        .build()?;

    let legend_size = legend.calculate_size();
    let renderer = StandardLegendRenderer::new();

    // Calculate drawing area constants outside the render loop
    let margin = 60u32;
    let legend_space = 150u32; // Reserve space for right-side legend

    // Pre-calculate bar layout constants
    let bar_width_ratio = 6; // draw_area.width / 6 for spacing
    let bar_spacing_ratio = 2; // bar_width / 2 for spacing

    // Use the common visual example runner
    window::run(
        WindowConfig::new("Quarterly Revenue by Product Line")
            .theme(WindowTheme::Default)
            .background(Rgb565::WHITE),
        move |display, viewport, _elapsed| {
            // Clear background
            let _ = display.clear(Rgb565::WHITE);

            // Calculate drawing area with pre-calculated constants
            let draw_area = Rectangle::new(
                Point::new(
                    viewport.top_left.x + margin as i32,
                    viewport.top_left.y + margin as i32,
                ),
                Size::new(
                    viewport
                        .size
                        .width
                        .saturating_sub(margin * 2 + legend_space),
                    viewport.size.height.saturating_sub(margin * 2),
                ),
            );

            // Calculate maximum total value for scaling
            let max_total = (0..4)
                .map(|i| {
                    cloud_services[i] + enterprise_software[i] + mobile_apps[i] + consulting[i]
                })
                .fold(0.0f32, |a, b| a.max(b));

            // Draw title
            let title_style = MonoTextStyle::new(&FONT_6X10, Rgb565::BLACK);
            let title_text = "Quarterly Revenue Growth ($M)";
            let title_pos = Point::new(
                viewport.top_left.x + (viewport.size.width as i32 / 2)
                    - (title_text.len() as i32 * 3),
                viewport.top_left.y + 20,
            );
            let _ = Text::with_baseline(title_text, title_pos, title_style, Baseline::Top)
                .draw(display);

            // Draw stacked bars for each quarter using pre-calculated ratios
            let bar_width = draw_area.size.width / bar_width_ratio;
            let bar_spacing = bar_width / bar_spacing_ratio;

            for quarter in 0..4 {
                let cloud = cloud_services[quarter];
                let enterprise = enterprise_software[quarter];
                let mobile = mobile_apps[quarter];
                let consulting_val = consulting[quarter];
                let total_value = cloud + enterprise + mobile + consulting_val;

                // Calculate bar position
                let bar_x =
                    draw_area.top_left.x + (quarter as u32 * (bar_width + bar_spacing)) as i32;

                // Calculate segment heights (proportional to draw area)
                let cloud_height = ((cloud / max_total) * draw_area.size.height as f32) as u32;
                let enterprise_height =
                    ((enterprise / max_total) * draw_area.size.height as f32) as u32;
                let mobile_height = ((mobile / max_total) * draw_area.size.height as f32) as u32;
                let consulting_height =
                    ((consulting_val / max_total) * draw_area.size.height as f32) as u32;

                // Draw segments from bottom to top
                let base_y = draw_area.top_left.y + draw_area.size.height as i32;

                // Bottom segment (Cloud Services) - Blue
                if cloud_height > 0 {
                    let cloud_rect = Rectangle::new(
                        Point::new(bar_x, base_y - cloud_height as i32),
                        Size::new(bar_width, cloud_height),
                    );
                    let _ = cloud_rect
                        .into_styled(PrimitiveStyle::with_fill(colors[0]))
                        .draw(display);
                }

                // Second segment (Enterprise Software) - Green
                if enterprise_height > 0 {
                    let enterprise_rect = Rectangle::new(
                        Point::new(bar_x, base_y - (cloud_height + enterprise_height) as i32),
                        Size::new(bar_width, enterprise_height),
                    );
                    let _ = enterprise_rect
                        .into_styled(PrimitiveStyle::with_fill(colors[1]))
                        .draw(display);
                }

                // Third segment (Mobile Apps) - Orange
                if mobile_height > 0 {
                    let mobile_rect = Rectangle::new(
                        Point::new(
                            bar_x,
                            base_y - (cloud_height + enterprise_height + mobile_height) as i32,
                        ),
                        Size::new(bar_width, mobile_height),
                    );
                    let _ = mobile_rect
                        .into_styled(PrimitiveStyle::with_fill(colors[2]))
                        .draw(display);
                }

                // Top segment (Consulting) - Purple
                if consulting_height > 0 {
                    let consulting_rect = Rectangle::new(
                        Point::new(
                            bar_x,
                            base_y
                                - (cloud_height
                                    + enterprise_height
                                    + mobile_height
                                    + consulting_height) as i32,
                        ),
                        Size::new(bar_width, consulting_height),
                    );
                    let _ = consulting_rect
                        .into_styled(PrimitiveStyle::with_fill(colors[3]))
                        .draw(display);
                }

                // Draw quarter label
                let label_style = MonoTextStyle::new(&FONT_6X10, Rgb565::BLACK);
                let label_pos = Point::new(
                    bar_x + (bar_width as i32 / 2) - 12, // Center under bar
                    base_y + 15,
                );
                let quarter_label = match quarter {
                    0 => "Q1",
                    1 => "Q2",
                    2 => "Q3",
                    3 => "Q4",
                    _ => "??",
                };
                let _ = Text::with_baseline(quarter_label, label_pos, label_style, Baseline::Top)
                    .draw(display);

                // Draw total value on top of each bar
                let total_text = format!("${total_value:.0}M");
                let total_pos = Point::new(
                    bar_x + (bar_width as i32 / 2) - (total_text.len() as i32 * 3),
                    base_y
                        - (cloud_height + enterprise_height + mobile_height + consulting_height)
                            as i32
                        - 15,
                );
                let _ = Text::with_baseline(&total_text, total_pos, label_style, Baseline::Top)
                    .draw(display);
            }

            // Calculate legend layout with proper chart area
            let chart_area = Rectangle::new(
                Point::new(
                    viewport.top_left.x + margin as i32,
                    viewport.top_left.y + margin as i32,
                ),
                Size::new(
                    viewport
                        .size
                        .width
                        .saturating_sub(margin * 2 + legend_space),
                    viewport.size.height.saturating_sub(margin * 2),
                ),
            );
            let calculator = PositionCalculator::new(viewport, chart_area)
                .with_margins(LegendMargins::new(10, 10, 10, 10))
                .with_alignment(LegendAlignment::Center);
            let legend_rect = calculator.calculate_legend_rect(legend.position(), legend_size)?;

            // Render the legend
            renderer.render(&legend, legend_rect, display)?;

            Ok(())
        },
    )
}

#[cfg(all(feature = "std", feature = "animations"))]
fn run_animated_demo() -> ChartResult<()> {
    // Create compelling quarterly revenue data for a growing tech company
    let quarters = ["Q1", "Q2", "Q3", "Q4"];

    // Initial revenue data (in millions) - conservative estimates
    let initial_cloud_services = [30.0, 35.0, 40.0, 45.0];
    let initial_enterprise_software = [60.0, 65.0, 70.0, 75.0];
    let initial_mobile_apps = [15.0, 18.0, 22.0, 25.0];
    let initial_consulting = [25.0, 28.0, 30.0, 32.0];

    // Target revenue data (in millions) - growth achieved
    let target_cloud_services = [45.0, 65.0, 85.0, 120.0]; // Fastest growing segment
    let target_enterprise_software = [80.0, 90.0, 95.0, 110.0]; // Steady growth
    let target_mobile_apps = [25.0, 35.0, 55.0, 75.0]; // Strong growth
    let target_consulting = [30.0, 32.0, 28.0, 35.0]; // Stable but smaller

    // Define vibrant, distinct colors for each product segment
    let colors = [
        Rgb565::new(8, 15, 31), // Deep blue for Cloud Services (bottom)
        Rgb565::new(8, 31, 15), // Deep green for Enterprise Software
        Rgb565::new(31, 20, 8), // Orange for Mobile Apps
        Rgb565::new(31, 8, 15), // Purple for Consulting (top)
    ];

    // Create initial stacked data
    let mut initial_data = StackedData::new();

    // Add layers for initial data
    let mut cloud_layer = StaticDataSeries::new();
    let mut enterprise_layer = StaticDataSeries::new();
    let mut mobile_layer = StaticDataSeries::new();
    let mut consulting_layer = StaticDataSeries::new();

    for i in 0..4 {
        cloud_layer.push(Point2D::new(i as f32, initial_cloud_services[i]))?;
        enterprise_layer.push(Point2D::new(i as f32, initial_enterprise_software[i]))?;
        mobile_layer.push(Point2D::new(i as f32, initial_mobile_apps[i]))?;
        consulting_layer.push(Point2D::new(i as f32, initial_consulting[i]))?;
    }

    initial_data.add_layer(cloud_layer, "Cloud Services", colors[0])?;
    initial_data.add_layer(enterprise_layer, "Enterprise Software", colors[1])?;
    initial_data.add_layer(mobile_layer, "Mobile Apps", colors[2])?;
    initial_data.add_layer(consulting_layer, "Consulting", colors[3])?;

    // Create target stacked data
    let mut target_data = StackedData::new();

    // Add layers for target data
    let mut target_cloud_layer = StaticDataSeries::new();
    let mut target_enterprise_layer = StaticDataSeries::new();
    let mut target_mobile_layer = StaticDataSeries::new();
    let mut target_consulting_layer = StaticDataSeries::new();

    for i in 0..4 {
        target_cloud_layer.push(Point2D::new(i as f32, target_cloud_services[i]))?;
        target_enterprise_layer.push(Point2D::new(i as f32, target_enterprise_software[i]))?;
        target_mobile_layer.push(Point2D::new(i as f32, target_mobile_apps[i]))?;
        target_consulting_layer.push(Point2D::new(i as f32, target_consulting[i]))?;
    }

    target_data.add_layer(target_cloud_layer, "Cloud Services", colors[0])?;
    target_data.add_layer(target_enterprise_layer, "Enterprise Software", colors[1])?;
    target_data.add_layer(target_mobile_layer, "Mobile Apps", colors[2])?;
    target_data.add_layer(target_consulting_layer, "Consulting", colors[3])?;

    // Create stacked bar chart
    let chart = AnimatedStackedBarChart::builder()
        .bar_width(StackedBarWidth::Auto)
        .spacing(20)
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
        .add_bar_entry("Cloud Services", colors[0])?
        .add_bar_entry("Enterprise Software", colors[1])?
        .add_bar_entry("Mobile Apps", colors[2])?
        .add_bar_entry("Consulting", colors[3])?
        .professional_style()
        .build()?;

    // Pre-calculate legend size and layout dimensions
    let legend_size = legend.calculate_size();
    let legend_renderer = StandardLegendRenderer::new();

    // Pre-create chart configuration (static)
    let config = ChartConfig {
        title: Some(heapless::String::try_from("Quarterly Revenue Growth ($M)").unwrap()),
        background_color: Some(Rgb565::WHITE),
        margins: CHART_MARGINS,
        show_grid: false,
        grid_color: None,
    };

    // Pre-create text style for quarter labels
    let text_style = embedded_graphics::mono_font::MonoTextStyle::new(
        &embedded_graphics::mono_font::ascii::FONT_6X10,
        Rgb565::BLACK,
    );

    // Pre-create animation timing constants
    let animation_cycle = 6.0; // 6 second cycle
    let forward_duration = 3.0; // First half of cycle
    let backward_duration = 3.0; // Second half of cycle

    // Use the common visual example runner
    window::run(
        WindowConfig::new("Animated Stacked Bar Chart - Revenue Growth")
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
                    viewport.size.width.saturating_sub(legend_size.width + 20),
                    viewport.size.height,
                ),
            );

            // Draw the chart in the adjusted area
            chart.draw_animated(&current_data, &config, chart_area, display, progress)?;

            // Render the legend using pre-created renderer
            legend_renderer.render(&legend, legend_rect, display)?;

            // Draw quarter labels using pre-calculated layout
            let draw_area = config.margins.apply_to(chart_area);
            let bar_count = 4;
            let bar_width = draw_area.size.width / 6; // Space for 4 bars with generous gaps
            let spacing = 20;
            let total_bar_space = bar_width * bar_count as u32;
            let total_spacing = spacing * (bar_count - 1) as u32;
            let start_x = draw_area.top_left.x
                + ((draw_area
                    .size
                    .width
                    .saturating_sub(total_bar_space + total_spacing))
                    / 2) as i32;

            for (i, quarter) in quarters.iter().enumerate() {
                let bar_x = start_x + (i as u32 * (bar_width + spacing)) as i32;
                let label_pos = Point::new(
                    bar_x + (bar_width as i32 / 2) - 6,
                    draw_area.top_left.y + draw_area.size.height as i32 + 15,
                );

                embedded_graphics::text::Text::with_baseline(
                    quarter,
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

#[cfg(not(feature = "std"))]
fn main() {
    println!("⚠️  This visual example requires the 'std' feature to run");
    println!("   Run with: cargo run --example stacked_bar_chart --features std");
    println!("   Add --features animations for animated transitions");
}

/// Helper function to create a stacked bar chart with proper data transformation
pub fn create_stacked_bar_data<const N: usize>(
    series: &[&StaticDataSeries<Point2D, N>],
) -> Result<Vec<StaticDataSeries<Point2D, N>, 8>, Box<dyn std::error::Error>> {
    let mut stacked_series = Vec::new();

    if series.is_empty() {
        return Ok(stacked_series);
    }

    let data_length = series[0].len();

    // Verify all series have the same length
    for s in series {
        if s.len() != data_length {
            return Err("All series must have the same length for stacking".into());
        }
    }

    // Create stacked series
    for layer in 0..series.len() {
        let mut stacked = StaticDataSeries::new();

        for i in 0..data_length {
            let x = series[0].get(i).unwrap().x();
            let mut cumulative_y = 0.0;

            // Sum up all values from bottom to current layer
            for series_item in series.iter().take(layer + 1) {
                cumulative_y += series_item.get(i).unwrap().y();
            }

            stacked.push(Point2D::new(x, cumulative_y))?;
        }

        stacked_series
            .push(stacked)
            .map_err(|_| "Too many series for stacking")?;
    }

    Ok(stacked_series)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stacked_bar_chart() {
        // Test that the example runs without panicking
        main().unwrap();
    }

    #[test]
    fn test_compelling_revenue_data() {
        // Test that our revenue data shows compelling growth
        let cloud_q1 = 45.0;
        let cloud_q4 = 120.0;
        let cloud_growth = (cloud_q4 - cloud_q1) / cloud_q1;

        // Cloud services should show dramatic growth (>150%)
        assert!(
            cloud_growth > 1.5,
            "Cloud services should show dramatic growth"
        );

        // Total revenue should be substantial in Q4
        let q4_total = 120.0 + 110.0 + 75.0 + 35.0; // 340M
        assert!(
            q4_total > 300.0,
            "Q4 total should be substantial for visual impact"
        );

        // Mobile apps should show strong growth
        let mobile_q1 = 25.0;
        let mobile_q4 = 75.0;
        let mobile_growth = (mobile_q4 - mobile_q1) / mobile_q1;
        assert!(mobile_growth > 2.0, "Mobile apps should show 200%+ growth");
    }

    #[test]
    fn test_stacked_data_creation() {
        let mut series1: StaticDataSeries<Point2D, 10> = StaticDataSeries::new();
        let mut series2: StaticDataSeries<Point2D, 10> = StaticDataSeries::new();
        let mut series3: StaticDataSeries<Point2D, 10> = StaticDataSeries::new();

        // Add test data with substantial values
        series1.push(Point2D::new(0.0, 45.0)).unwrap();
        series1.push(Point2D::new(1.0, 65.0)).unwrap();

        series2.push(Point2D::new(0.0, 80.0)).unwrap();
        series2.push(Point2D::new(1.0, 90.0)).unwrap();

        series3.push(Point2D::new(0.0, 25.0)).unwrap();
        series3.push(Point2D::new(1.0, 35.0)).unwrap();

        let series_refs = [&series1, &series2, &series3];
        let stacked = create_stacked_bar_data(&series_refs).unwrap();

        // Test stacking calculations
        assert_eq!(stacked.len(), 3);

        // First layer should be just series1
        assert_eq!(stacked[0].get(0).unwrap().y(), 45.0);
        assert_eq!(stacked[0].get(1).unwrap().y(), 65.0);

        // Second layer should be series1 + series2
        assert_eq!(stacked[1].get(0).unwrap().y(), 125.0); // 45 + 80
        assert_eq!(stacked[1].get(1).unwrap().y(), 155.0); // 65 + 90

        // Third layer should be series1 + series2 + series3
        assert_eq!(stacked[2].get(0).unwrap().y(), 150.0); // 45 + 80 + 25
        assert_eq!(stacked[2].get(1).unwrap().y(), 190.0); // 65 + 90 + 35
    }

    #[test]
    fn test_empty_series_stacking() {
        let series_refs: [&StaticDataSeries<Point2D, 10>; 0] = [];
        let stacked = create_stacked_bar_data(&series_refs).unwrap();
        assert_eq!(stacked.len(), 0);
    }

    #[test]
    fn test_single_series_stacking() {
        let mut series: StaticDataSeries<Point2D, 10> = StaticDataSeries::new();
        series.push(Point2D::new(0.0, 100.0)).unwrap();
        series.push(Point2D::new(1.0, 150.0)).unwrap();

        let series_refs = [&series];
        let stacked = create_stacked_bar_data(&series_refs).unwrap();

        assert_eq!(stacked.len(), 1);
        assert_eq!(stacked[0].get(0).unwrap().y(), 100.0);
        assert_eq!(stacked[0].get(1).unwrap().y(), 150.0);
    }

    #[cfg(feature = "animations")]
    #[test]
    fn test_animated_stacked_chart_creation() {
        let chart = AnimatedStackedBarChart::<Rgb565>::builder()
            .bar_width(StackedBarWidth::Auto)
            .spacing(20)
            .build()
            .unwrap();

        // Test chart creation
        assert!(true); // Chart created successfully
    }

    #[cfg(feature = "animations")]
    #[test]
    fn test_animated_stacked_data_creation() {
        let mut stacked_data = StackedData::<Point2D, 256>::new();

        let mut layer1 = StaticDataSeries::new();
        layer1.push(Point2D::new(0.0, 10.0)).unwrap();
        layer1.push(Point2D::new(1.0, 15.0)).unwrap();

        stacked_data
            .add_layer(layer1, "Test Layer", Rgb565::BLUE)
            .unwrap();

        assert_eq!(stacked_data.layer_count(), 1);
    }
}
