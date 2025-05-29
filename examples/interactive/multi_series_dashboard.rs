//! Multi-Series Dashboard - Interactive Category
//!
//! This example demonstrates advanced features with multiple data series,
//! real-time updates, and interactive elements.
//!
//! Run with: cargo run --example multi_series_dashboard --features std

use embedded_charts::prelude::*;

// Import the common abstraction
#[path = "../common/mod.rs"]
mod common;

use common::{configs, data, utils, WindowConfig, WindowTheme};

#[cfg(feature = "std")]
fn main() -> ChartResult<()> {
    use common::window::run;

    // Create multiple data series for system monitoring
    let cpu_data = data::system_metrics(60, data::SystemMetric::CpuUsage)?;
    let memory_data = data::system_metrics(60, data::SystemMetric::MemoryUsage)?;
    let network_data = data::system_metrics(60, data::SystemMetric::NetworkIO)?;
    let disk_data = data::system_metrics(60, data::SystemMetric::DiskUsage)?;

    // Create charts with different colors
    let colors = configs::professional_colors();
    let cpu_chart = configs::professional_line_chart(colors[0])?;
    let memory_chart = configs::professional_line_chart(colors[1])?;
    let network_chart = configs::professional_line_chart(colors[2])?;
    let disk_chart = configs::professional_line_chart(colors[3])?;

    println!("📊 Dashboard Features:");
    println!("  🔴 CPU Usage: Baseline with periodic spikes");
    println!("  🔵 Memory Usage: Gradual increase over time");
    println!("  🟢 Network I/O: Burst patterns");
    println!("  🟣 Disk Usage: Periodic activity cycles");
    println!();

    // Print data information
    utils::print_series_info(&cpu_data, "CPU Usage");
    utils::print_series_info(&memory_data, "Memory Usage");
    utils::print_series_info(&network_data, "Network I/O");
    utils::print_series_info(&disk_data, "Disk Usage");
    println!();

    // Pre-create legend and layout objects outside the animation loop
    let legend = StandardLegendBuilder::new()
        .position(LegendPos::Right)
        .orientation(LegendOrientation::Vertical)
        .add_line_entry("CPU Usage", colors[0])?
        .add_line_entry("Memory Usage", colors[1])?
        .add_line_entry("Network I/O", colors[2])?
        .add_line_entry("Disk Usage", colors[3])?
        .professional_style()
        .build()?;

    let legend_size = legend.calculate_size();
    let legend_renderer = StandardLegendRenderer::new();

    // Run animated example with real-time updates
    run(
        WindowConfig::new("System Dashboard")
            .theme(WindowTheme::Default)
            .fps(30)
            .background(Rgb565::new(248 >> 3, 248 >> 2, 255 >> 3)),
        move |display, viewport, elapsed_time| {
            // Calculate animation offset based on time
            let offset = (elapsed_time * 10.0) as usize;

            // Create animated data by shifting the original data
            let mut animated_cpu = StaticDataSeries::new();
            let mut animated_memory = StaticDataSeries::new();
            let mut animated_network = StaticDataSeries::new();
            let mut animated_disk = StaticDataSeries::new();

            // Add some points with time-based animation
            for i in 0..40.min(cpu_data.len()) {
                let idx = (i + offset) % cpu_data.len();
                if let (Some(cpu), Some(mem), Some(net), Some(disk)) = (
                    cpu_data.get(idx),
                    memory_data.get(idx),
                    network_data.get(idx),
                    disk_data.get(idx),
                ) {
                    let x = i as f32;
                    let _ = animated_cpu.push(Point2D::new(x, cpu.y));
                    let _ = animated_memory.push(Point2D::new(x, mem.y));
                    let _ = animated_network.push(Point2D::new(x, net.y));
                    let _ = animated_disk.push(Point2D::new(x, disk.y));
                }
            }

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

            // Draw all series on the same chart in adjusted area
            cpu_chart.draw(&animated_cpu, cpu_chart.config(), chart_area, display)?;
            memory_chart.draw(&animated_memory, memory_chart.config(), chart_area, display)?;
            network_chart.draw(
                &animated_network,
                network_chart.config(),
                chart_area,
                display,
            )?;
            disk_chart.draw(&animated_disk, disk_chart.config(), chart_area, display)?;

            // Render the legend using pre-created renderer
            legend_renderer.render(&legend, legend_rect, display)?;

            // Continue animation for 10 seconds
            Ok(())
        },
    )?;

    println!("💡 This example demonstrates:");
    println!("  • Multiple data series on one chart");
    println!("  • Real-time data animation");
    println!("  • Professional color schemes");
    println!("  • Large display optimization");

    Ok(())
}

#[cfg(not(feature = "std"))]
fn main() {
    utils::print_feature_requirement("std", "interactive");
}
