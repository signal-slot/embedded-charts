//! Multi-Series Chart - Interactive Category
//!
//! This example demonstrates how to create a professional multi-series line chart
//! that displays multiple data sets on the same chart with different colors and styles.
//! Perfect for comparing multiple metrics or datasets side by side.
//!
//! Run with: cargo run --example multi_series_chart --features std

use embedded_charts::prelude::*;
use embedded_graphics::primitives::PrimitiveStyle;

// Import the common abstraction
#[path = "../common/mod.rs"]
mod common;

use common::{data, WindowConfig};

#[cfg(feature = "std")]
fn main() -> ChartResult<()> {
    // Generate sample system monitoring data using common utilities
    let cpu_data = data::system_metrics(60, data::SystemMetric::CpuUsage)?;
    let memory_data = data::system_metrics(60, data::SystemMetric::MemoryUsage)?;
    let network_data = data::system_metrics(60, data::SystemMetric::NetworkIO)?;
    let disk_data = data::system_metrics(60, data::SystemMetric::DiskUsage)?;

    // Define colors for each series outside the render loop
    let colors = [
        Rgb565::new(0, 15, 31), // Blue for CPU
        Rgb565::new(31, 0, 0),  // Red for Memory
        Rgb565::new(0, 31, 0),  // Green for Network
        Rgb565::new(31, 0, 31), // Magenta for Disk
    ];

    // Calculate unified data bounds for all series outside the render loop
    let mut all_points = StaticDataSeries::<Point2D, 256>::new();
    for point in cpu_data.iter() {
        all_points.push(point).map_err(|_| ChartError::MemoryFull)?;
    }
    for point in memory_data.iter() {
        all_points.push(point).map_err(|_| ChartError::MemoryFull)?;
    }
    for point in network_data.iter() {
        all_points.push(point).map_err(|_| ChartError::MemoryFull)?;
    }
    for point in disk_data.iter() {
        all_points.push(point).map_err(|_| ChartError::MemoryFull)?;
    }
    let unified_bounds = all_points.bounds()?;

    // Create chart configuration outside the render loop
    let chart_config = ChartConfig {
        title: None,
        background_color: None,
        margins: Margins::new(60, 40, 60, 80),
        show_grid: false,
        grid_color: None,
    };

    // Pre-create series data array and series names
    let series_data = [&cpu_data, &memory_data, &network_data, &disk_data];
    let series_names = ["CPU", "Memory", "Network", "Disk"];

    // Create text style outside the render loop
    use embedded_graphics::mono_font::{ascii::FONT_6X10, MonoTextStyle};
    let text_style = MonoTextStyle::new(&FONT_6X10, Rgb565::BLACK);

    // Pre-create all charts outside the render loop
    let charts: [LineChart<Rgb565>; 4] = [
        LineChart::builder()
            .line_color(colors[0])
            .line_width(3)
            .with_markers(MarkerStyle {
                shape: MarkerShape::Circle,
                size: 4,
                color: colors[0],
                visible: true,
            })
            .build()?,
        LineChart::builder()
            .line_color(colors[1])
            .line_width(3)
            .with_markers(MarkerStyle {
                shape: MarkerShape::Circle,
                size: 4,
                color: colors[1],
                visible: true,
            })
            .build()?,
        LineChart::builder()
            .line_color(colors[2])
            .line_width(3)
            .with_markers(MarkerStyle {
                shape: MarkerShape::Circle,
                size: 4,
                color: colors[2],
                visible: true,
            })
            .build()?,
        LineChart::builder()
            .line_color(colors[3])
            .line_width(3)
            .with_markers(MarkerStyle {
                shape: MarkerShape::Circle,
                size: 4,
                color: colors[3],
                visible: true,
            })
            .build()?,
    ];

    // Create legend and related objects outside the render loop
    let legend = StandardLegendBuilder::new()
        .position(LegendPos::Right)
        .orientation(LegendOrientation::Vertical)
        .add_line_entry("CPU", colors[0])?
        .add_line_entry("Memory", colors[1])?
        .add_line_entry("Network", colors[2])?
        .add_line_entry("Disk", colors[3])?
        .professional_style()
        .build()?;

    let legend_size = legend.calculate_size();
    let renderer = StandardLegendRenderer::new();
    let calculator = PositionCalculator::new(
        Rectangle::new(Point::zero(), Size::new(800, 600)),
        Rectangle::new(Point::zero(), Size::new(800, 600)),
    );

    // Use the common animated example runner with proper size
    common::window::run(
        WindowConfig::new("Multi-Series System Monitor")
            .theme(common::WindowTheme::Default)
            .background(Rgb565::WHITE),
        move |display, viewport, _time| {
            use embedded_graphics::prelude::*;
            use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
            use embedded_graphics::text::{Baseline, Text};

            // Clear the display with white background
            Rectangle::new(viewport.top_left, viewport.size)
                .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
                .draw(display)
                .map_err(|_| ChartError::RenderingError)?;

            // Draw each series using the same coordinate system and pre-created charts
            for (i, &series) in series_data.iter().enumerate() {
                // Draw the series using unified bounds for consistent scaling
                draw_series_with_bounds(
                    &charts[i],
                    series,
                    &unified_bounds,
                    &chart_config,
                    viewport,
                    display,
                )?;

                println!(
                    "Drawing {} series with {} points",
                    series_names[i],
                    series.len()
                );
            }

            // Draw title
            Text::with_baseline(
                "Multi-Series System Monitor",
                Point::new(viewport.top_left.x + 40, viewport.top_left.y + 20),
                text_style,
                Baseline::Top,
            )
            .draw(display)
            .map_err(|_| ChartError::RenderingError)?;

            // Calculate legend layout
            let legend_rect = calculator.calculate_legend_rect(legend.position(), legend_size)?;

            // Render the legend
            renderer.render(&legend, legend_rect, display)?;

            // Draw simple axes
            draw_simple_axes(viewport, &chart_config, &unified_bounds, display)?;

            // Print information about the series
            println!("📈 Generated data for 4 series:");
            for (i, &name) in series_names.iter().enumerate() {
                println!("  {} series: {} points", name, series_data[i].len());
            }

            println!("\n🎯 Chart Features Demonstrated:");
            println!("  • Multiple line series with unified scaling");
            println!("  • Consistent coordinate transformation");
            println!("  • Professional color scheme");
            println!("  • System monitoring simulation");

            println!("\n📊 Data Patterns:");
            println!("  🔵 CPU Usage: Baseline with periodic spikes");
            println!("  🔴 Memory Usage: Gradual increase over time");
            println!("  🟢 Network I/O: Burst patterns");
            println!("  🟣 Disk Usage: Periodic activity cycles");

            Ok(())
        },
    )
}

/// Draw a series with specific bounds for consistent scaling
fn draw_series_with_bounds<D>(
    chart: &LineChart<Rgb565>,
    data: &StaticDataSeries<Point2D, 256>,
    bounds: &DataBounds<f32, f32>,
    config: &ChartConfig<Rgb565>,
    viewport: Rectangle,
    target: &mut D,
) -> ChartResult<()>
where
    D: DrawTarget<Color = Rgb565>,
{
    if data.is_empty() {
        return Ok(());
    }

    // Apply margins to get the actual drawing area
    let draw_area = config.margins.apply_to(viewport);

    // Transform all points to screen coordinates using unified bounds
    let mut screen_points = heapless::Vec::<Point, 256>::new();
    for point in data.iter() {
        let screen_point = transform_point_with_bounds(&point, bounds, draw_area);
        screen_points
            .push(screen_point)
            .map_err(|_| ChartError::MemoryFull)?;
    }

    // Draw lines between consecutive points
    let line_style =
        PrimitiveStyle::with_stroke(chart.style().line_color, chart.style().line_width);
    for window in screen_points.windows(2) {
        if let [p1, p2] = window {
            Line::new(*p1, *p2)
                .into_styled(line_style)
                .draw(target)
                .map_err(|_| ChartError::RenderingError)?;
        }
    }

    // Draw markers if enabled
    if let Some(marker_style) = &chart.style().markers {
        if marker_style.visible {
            for &screen_point in screen_points.iter() {
                draw_marker(screen_point, marker_style, target)?;
            }
        }
    }

    Ok(())
}

/// Transform a data point to screen coordinates using specific bounds
fn transform_point_with_bounds(
    point: &Point2D,
    bounds: &DataBounds<f32, f32>,
    draw_area: Rectangle,
) -> Point {
    let data_x = point.x;
    let data_y = point.y;
    let min_x = bounds.min_x;
    let max_x = bounds.max_x;
    let min_y = bounds.min_y;
    let max_y = bounds.max_y;

    // Normalize to 0-1 range
    let norm_x = if max_x > min_x {
        (data_x - min_x) / (max_x - min_x)
    } else {
        0.5
    };

    let norm_y = if max_y > min_y {
        (data_y - min_y) / (max_y - min_y)
    } else {
        0.5
    };

    // Transform to screen coordinates (Y is flipped)
    let screen_x = draw_area.top_left.x + (norm_x * draw_area.size.width as f32) as i32;
    let screen_y = draw_area.top_left.y + draw_area.size.height as i32
        - (norm_y * draw_area.size.height as f32) as i32;

    Point::new(screen_x, screen_y)
}

/// Draw a marker at the specified point
fn draw_marker<D>(
    center: Point,
    marker_style: &MarkerStyle<Rgb565>,
    target: &mut D,
) -> ChartResult<()>
where
    D: DrawTarget<Color = Rgb565>,
{
    let style = PrimitiveStyle::with_fill(marker_style.color);
    let radius = marker_style.size / 2;

    match marker_style.shape {
        MarkerShape::Circle => {
            Circle::new(
                Point::new(center.x - radius as i32, center.y - radius as i32),
                marker_style.size,
            )
            .into_styled(style)
            .draw(target)
            .map_err(|_| ChartError::RenderingError)?;
        }
        MarkerShape::Square => {
            Rectangle::new(
                Point::new(center.x - radius as i32, center.y - radius as i32),
                Size::new(marker_style.size, marker_style.size),
            )
            .into_styled(style)
            .draw(target)
            .map_err(|_| ChartError::RenderingError)?;
        }
        _ => {
            // For other shapes, just draw a circle for simplicity
            Circle::new(
                Point::new(center.x - radius as i32, center.y - radius as i32),
                marker_style.size,
            )
            .into_styled(style)
            .draw(target)
            .map_err(|_| ChartError::RenderingError)?;
        }
    }

    Ok(())
}

/// Draw simple axes for the chart
fn draw_simple_axes<D>(
    viewport: Rectangle,
    config: &ChartConfig<Rgb565>,
    bounds: &DataBounds<f32, f32>,
    target: &mut D,
) -> ChartResult<()>
where
    D: DrawTarget<Color = Rgb565>,
{
    let draw_area = config.margins.apply_to(viewport);
    let axis_color = Rgb565::BLACK;
    let axis_style = PrimitiveStyle::with_stroke(axis_color, 1);

    // Draw X-axis (bottom)
    Line::new(
        Point::new(
            draw_area.top_left.x,
            draw_area.top_left.y + draw_area.size.height as i32,
        ),
        Point::new(
            draw_area.top_left.x + draw_area.size.width as i32,
            draw_area.top_left.y + draw_area.size.height as i32,
        ),
    )
    .into_styled(axis_style)
    .draw(target)
    .map_err(|_| ChartError::RenderingError)?;

    // Draw Y-axis (left)
    Line::new(
        draw_area.top_left,
        Point::new(
            draw_area.top_left.x,
            draw_area.top_left.y + draw_area.size.height as i32,
        ),
    )
    .into_styled(axis_style)
    .draw(target)
    .map_err(|_| ChartError::RenderingError)?;

    // Add some tick marks and labels
    use embedded_graphics::mono_font::{ascii::FONT_5X8, MonoTextStyle};
    use embedded_graphics::text::{Baseline, Text};

    let text_style = MonoTextStyle::new(&FONT_5X8, Rgb565::BLACK);

    // Y-axis labels
    let y_steps = 5;
    for i in 0..=y_steps {
        let y_pos = draw_area.top_left.y + (i * draw_area.size.height as i32 / y_steps);
        let value = bounds.max_y - (i as f32 * (bounds.max_y - bounds.min_y) / y_steps as f32);

        // Draw tick mark
        Line::new(
            Point::new(draw_area.top_left.x - 3, y_pos),
            Point::new(draw_area.top_left.x, y_pos),
        )
        .into_styled(axis_style)
        .draw(target)
        .map_err(|_| ChartError::RenderingError)?;

        // Draw label
        let label = if value < 10.0 {
            heapless::String::<8>::try_from(format!("{value:.1}").as_str()).unwrap_or_default()
        } else {
            heapless::String::<8>::try_from(format!("{value:.0}").as_str()).unwrap_or_default()
        };

        Text::with_baseline(
            &label,
            Point::new(draw_area.top_left.x - 25, y_pos),
            text_style,
            Baseline::Middle,
        )
        .draw(target)
        .map_err(|_| ChartError::RenderingError)?;
    }

    Ok(())
}

#[cfg(not(feature = "std"))]
fn main() {
    utils::print_feature_requirement("std", "interactive");
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::mock_display::MockDisplay;

    #[test]
    fn test_chart_creation() {
        let chart = LineChart::builder()
            .line_color(Rgb565::RED)
            .line_width(2)
            .build()
            .unwrap();

        assert_eq!(chart.style().line_width, 2);
    }

    #[test]
    fn test_data_generation() {
        let cpu_data = data::system_metrics(10, data::SystemMetric::CpuUsage).unwrap();
        let memory_data = data::system_metrics(10, data::SystemMetric::MemoryUsage).unwrap();

        assert_eq!(cpu_data.len(), 10);
        assert_eq!(memory_data.len(), 10);
    }

    #[test]
    fn test_multi_series_rendering() {
        let mut display = MockDisplay::<Rgb565>::new();
        let chart = LineChart::builder()
            .line_color(Rgb565::BLUE)
            .build()
            .unwrap();

        let data = data::system_metrics(5, data::SystemMetric::CpuUsage).unwrap();
        let viewport = Rectangle::new(Point::new(0, 0), Size::new(100, 100));

        // Should render without errors
        chart
            .draw(&data, chart.config(), viewport, &mut display)
            .unwrap();
    }
}
