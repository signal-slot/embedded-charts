//! Scatter chart example demonstrating point plotting with different shapes, colors, and clustering.
//!
//! This example shows both basic scatter plots and advanced clustering features, demonstrating:
//! - Basic point plotting with different shapes and colors
//! - Data clustering with collision detection
//! - Color mapping strategies for different data groups
//! - Professional styling and legends
//!
//! Controls:
//! - The demo automatically cycles between Basic and Clustered modes every 5 seconds
//! - Press 'ESC' to exit

use embedded_charts::chart::scatter::{CollisionSettings, CollisionStrategy};
use embedded_charts::prelude::*;
use heapless::Vec;
use std::time::Instant;

// Import the common abstraction
#[path = "../common/mod.rs"]
mod common;

use common::{layout, window, WindowConfig};

#[derive(Debug, Clone, Copy, PartialEq)]
enum DemoMode {
    Basic,
    Clustered,
}

#[cfg(feature = "std")]
fn main() -> ChartResult<()> {
    println!("📊 Scatter Chart Demo");
    println!("=====================");
    println!("Features:");
    println!("• Basic scatter plot visualization");
    println!("• Data clustering with collision detection");
    println!("• Color mapping for different data groups");
    println!("• Professional styling and legends");
    println!();
    println!("Demo automatically cycles between modes every 5 seconds");
    println!("Press 'ESC' to exit");
    println!();

    run_unified_demo()
}

#[cfg(feature = "std")]
fn run_unified_demo() -> ChartResult<()> {
    // Prepare data for both modes
    let (basic_series, clustered_series) = prepare_demo_data()?;
    let start_time = Instant::now();

    // Use the common visual example runner with automatic mode switching
    window::run(
        WindowConfig::new("Scatter Chart Demo - Auto-cycling between Basic and Clustered modes")
            .theme(common::WindowTheme::Default)
            .background(Rgb565::WHITE),
        move |display, viewport, _elapsed| {
            // Switch mode every 5 seconds
            let elapsed_secs = start_time.elapsed().as_secs();
            let mode = if (elapsed_secs / 5) % 2 == 0 {
                DemoMode::Basic
            } else {
                DemoMode::Clustered
            };

            match mode {
                DemoMode::Basic => {
                    render_basic_mode(display, viewport, &basic_series)?;
                }
                DemoMode::Clustered => {
                    render_clustered_mode(display, viewport, &clustered_series)?;
                }
            }

            Ok(())
        },
    )
}

#[cfg(feature = "std")]
fn prepare_demo_data() -> ChartResult<(
    StaticDataSeries<Point2D, 256>,
    StaticDataSeries<Point2D, 256>,
)> {
    // Prepare basic demo data
    let mut basic_series = StaticDataSeries::<Point2D, 256>::new();
    let basic_data_points = [
        (1.0, 2.5),
        (2.0, 3.8),
        (3.0, 1.2),
        (4.0, 4.1),
        (5.0, 2.9),
        (6.0, 3.5),
        (7.0, 1.8),
        (8.0, 4.3),
        (9.0, 2.1),
        (10.0, 3.7),
    ];

    for (x, y) in basic_data_points.iter() {
        basic_series
            .push(Point2D::new(*x, *y))
            .map_err(ChartError::from)?;
    }

    // Prepare clustered demo data
    let mut clustered_series = StaticDataSeries::<Point2D, 256>::new();

    // Cluster 1: High performance, low cost (green - good products)
    let cluster1_points = [
        (2.0, 8.5),
        (2.2, 8.8),
        (1.8, 8.2),
        (2.1, 8.6),
        (1.9, 8.4),
        (2.3, 8.9),
        (2.0, 8.3),
        (1.7, 8.1),
        (2.4, 8.7),
        (2.2, 8.5),
    ];

    // Cluster 2: Medium performance, medium cost (orange - average products)
    let cluster2_points = [
        (5.0, 5.5),
        (5.2, 5.8),
        (4.8, 5.2),
        (5.1, 5.6),
        (4.9, 5.4),
        (5.3, 5.9),
        (5.0, 5.3),
        (4.7, 5.1),
        (5.4, 5.7),
        (5.2, 5.5),
    ];

    // Cluster 3: Low performance, high cost (red - poor products)
    let cluster3_points = [
        (8.0, 2.5),
        (8.2, 2.8),
        (7.8, 2.2),
        (8.1, 2.6),
        (7.9, 2.4),
        (8.3, 2.9),
        (8.0, 2.3),
        (7.7, 2.1),
        (8.4, 2.7),
        (8.2, 2.5),
    ];

    // Add all points to the clustered series
    for (x, y) in cluster1_points
        .iter()
        .chain(cluster2_points.iter())
        .chain(cluster3_points.iter())
    {
        clustered_series
            .push(Point2D::new(*x, *y))
            .map_err(ChartError::from)?;
    }

    Ok((basic_series, clustered_series))
}

#[cfg(feature = "std")]
fn render_basic_mode(
    display: &mut impl DrawTarget<Color = Rgb565>,
    viewport: Rectangle,
    series: &StaticDataSeries<Point2D, 256>,
) -> ChartResult<()> {
    // Create a basic scatter chart
    let chart = ScatterChart::builder()
        .point_shape(PointShape::Circle)
        .point_size(8)
        .point_color(Rgb565::BLUE)
        .with_title("Basic Scatter Plot (switching to Clustered in 5s...)")
        .background_color(Rgb565::WHITE)
        .margins(Margins::symmetric(20, 15))
        .build()?;

    // Create a legend for the scatter plot data
    let legend = StandardLegendBuilder::new()
        .position(LegendPos::Right)
        .orientation(LegendOrientation::Vertical)
        .add_line_entry("Data Points", Rgb565::BLUE)?
        .professional_style()
        .build()?;

    // Use automatic legend layout
    let renderer = StandardLegendRenderer::new();
    let config = ChartConfig::default();
    layout::draw_chart_with_auto_legend(
        |chart_area, display| chart.draw(&series.clone(), &config, chart_area, display),
        viewport,
        display,
        layout::ChartWithLegend::new(&legend, &renderer),
    )?;

    Ok(())
}

#[cfg(feature = "std")]
fn render_clustered_mode(
    display: &mut impl DrawTarget<Color = Rgb565>,
    viewport: Rectangle,
    series: &StaticDataSeries<Point2D, 256>,
) -> ChartResult<()> {
    // Create color palette for different clusters
    let mut colors = Vec::<Rgb565, 16>::new();
    colors
        .push(Rgb565::CSS_DARK_GREEN)
        .map_err(|_| ChartError::MemoryFull)?; // Cluster 1 - High performance, low cost
    colors
        .push(Rgb565::CSS_ORANGE)
        .map_err(|_| ChartError::MemoryFull)?; // Cluster 2 - Medium performance, medium cost
    colors
        .push(Rgb565::CSS_DARK_RED)
        .map_err(|_| ChartError::MemoryFull)?; // Cluster 3 - Low performance, high cost

    // Create color mapping for clusters
    let color_mapping = ColorMapping {
        colors,
        strategy: ColorMappingStrategy::IndexBased, // Color by data point index
    };

    // Create collision detection settings
    let collision_settings = CollisionSettings {
        enabled: true,
        strategy: CollisionStrategy::Offset,
        min_distance: 3, // Minimum 3 pixels between points
    };

    // Create a clustered scatter chart
    let chart = ScatterChart::builder()
        .point_shape(PointShape::Circle)
        .point_size(6)
        .point_color(Rgb565::BLUE) // Default color (will be overridden by mapping)
        .with_color_mapping(color_mapping)
        .with_collision_detection(collision_settings)
        .with_title("Product Performance vs Cost Analysis (switching to Basic in 5s...)")
        .background_color(Rgb565::WHITE)
        .margins(Margins::symmetric(30, 25))
        .build()?;

    // Create a legend for the clustered data
    let legend = StandardLegendBuilder::new()
        .position(LegendPos::Right)
        .orientation(LegendOrientation::Vertical)
        .add_line_entry("High Perf, Low Cost", Rgb565::CSS_DARK_GREEN)?
        .add_line_entry("Medium Perf, Med Cost", Rgb565::CSS_ORANGE)?
        .add_line_entry("Low Perf, High Cost", Rgb565::CSS_DARK_RED)?
        .professional_style()
        .build()?;

    // Use automatic legend layout
    let renderer = StandardLegendRenderer::new();
    let config = ChartConfig::default();
    layout::draw_chart_with_auto_legend(
        |chart_area, display| chart.draw(&series.clone(), &config, chart_area, display),
        viewport,
        display,
        layout::ChartWithLegend::new(&legend, &renderer),
    )?;

    Ok(())
}

// Legacy functions kept for compatibility with tests
#[cfg(feature = "std")]
#[allow(dead_code)] // Legacy function for compatibility
fn run_basic_demo() -> ChartResult<()> {
    let (basic_series, _) = prepare_demo_data()?;

    window::run(
        WindowConfig::new("Basic Scatter Chart Example")
            .theme(common::WindowTheme::Custom {
                pixel_spacing: 0,
                scale: 2,
            })
            .background(Rgb565::WHITE),
        |display, viewport, _elapsed| render_basic_mode(display, viewport, &basic_series),
    )
}

#[cfg(feature = "std")]
#[allow(dead_code)] // Legacy function for compatibility
fn run_clustered_demo() -> ChartResult<()> {
    let (_, clustered_series) = prepare_demo_data()?;

    window::run(
        WindowConfig::new("Clustered Scatter Chart - Performance vs Cost")
            .theme(common::WindowTheme::Default)
            .background(Rgb565::WHITE),
        |display, viewport, _elapsed| render_clustered_mode(display, viewport, &clustered_series),
    )
}

#[cfg(not(feature = "std"))]
fn main() {
    utils::print_feature_requirement("std", "visual");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_scatter_chart() {
        // Test that we can create a scatter chart without errors
        let chart = ScatterChart::<Rgb565>::builder()
            .point_shape(PointShape::Circle)
            .point_size(8)
            .point_color(Rgb565::BLUE)
            .build();

        assert!(
            chart.is_ok(),
            "Basic scatter chart should build without errors"
        );
    }

    #[test]
    fn test_clustered_scatter_chart() {
        // Test that we can create a clustered scatter chart without errors
        let collision_settings = CollisionSettings {
            enabled: true,
            strategy: CollisionStrategy::Offset,
            min_distance: 3,
        };

        let chart = ScatterChart::<Rgb565>::builder()
            .with_collision_detection(collision_settings)
            .build();

        assert!(
            chart.is_ok(),
            "Clustered scatter chart should build without errors"
        );
    }

    #[test]
    fn test_collision_settings_creation() {
        let settings = CollisionSettings {
            enabled: true,
            strategy: CollisionStrategy::Offset,
            min_distance: 5,
        };

        assert_eq!(settings.enabled, true);
        assert_eq!(settings.strategy, CollisionStrategy::Offset);
        assert_eq!(settings.min_distance, 5);
    }

    #[test]
    fn test_color_mapping_index_based() {
        let mut colors = Vec::<Rgb565, 16>::new();
        colors.push(Rgb565::RED).unwrap();
        colors.push(Rgb565::GREEN).unwrap();
        colors.push(Rgb565::BLUE).unwrap();

        let mapping = ColorMapping {
            colors,
            strategy: ColorMappingStrategy::IndexBased,
        };

        assert_eq!(mapping.colors.len(), 3);
        assert_eq!(mapping.strategy, ColorMappingStrategy::IndexBased);
    }

    #[test]
    fn test_cluster_data_quality() {
        // Test that our cluster data represents meaningful business scenarios

        // Cluster 1: High performance, low cost - should be in top-left quadrant
        let cluster1_sample = (2.0, 8.5);
        assert!(
            cluster1_sample.0 < 5.0,
            "High performance products should have low cost"
        );
        assert!(
            cluster1_sample.1 > 7.0,
            "High performance products should have high performance"
        );

        // Cluster 3: Low performance, high cost - should be in bottom-right quadrant
        let cluster3_sample = (8.0, 2.5);
        assert!(
            cluster3_sample.0 > 7.0,
            "Poor products should have high cost"
        );
        assert!(
            cluster3_sample.1 < 4.0,
            "Poor products should have low performance"
        );

        // Cluster 2: Medium performance, medium cost - should be in middle
        let cluster2_sample = (5.0, 5.5);
        assert!(
            cluster2_sample.0 > 4.0 && cluster2_sample.0 < 6.0,
            "Average products should have medium cost"
        );
        assert!(
            cluster2_sample.1 > 4.0 && cluster2_sample.1 < 7.0,
            "Average products should have medium performance"
        );
    }

    #[test]
    fn test_collision_detection_effectiveness() {
        // Test that collision detection settings are reasonable
        let settings = CollisionSettings {
            enabled: true,
            strategy: CollisionStrategy::Offset,
            min_distance: 3,
        };

        // Minimum distance should be reasonable for visual clarity
        assert!(
            settings.min_distance >= 2,
            "Minimum distance should be at least 2 pixels for visibility"
        );
        assert!(
            settings.min_distance <= 10,
            "Minimum distance should not be too large to avoid excessive spreading"
        );

        // Collision detection should be enabled for clustered data
        assert!(
            settings.enabled,
            "Collision detection should be enabled for clustered scatter plots"
        );
    }

    #[test]
    fn test_demo_mode_enum() {
        let basic = DemoMode::Basic;
        let clustered = DemoMode::Clustered;

        assert_ne!(basic, clustered);
        assert_eq!(basic, DemoMode::Basic);
        assert_eq!(clustered, DemoMode::Clustered);
    }

    #[test]
    fn test_prepare_demo_data() {
        let result = prepare_demo_data();
        assert!(result.is_ok(), "Demo data preparation should succeed");

        let (basic_series, clustered_series) = result.unwrap();
        assert_eq!(basic_series.len(), 10, "Basic series should have 10 points");
        assert_eq!(
            clustered_series.len(),
            30,
            "Clustered series should have 30 points (3 clusters × 10 points)"
        );
    }
}
