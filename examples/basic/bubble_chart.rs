//! Bubble chart example demonstrating size mapping for data visualization.

use embedded_charts::prelude::*;
use heapless::Vec;

// Import the common abstraction
#[path = "../common/mod.rs"]
mod common;

use common::{window, WindowConfig, WindowTheme};

#[cfg(feature = "std")]
fn main() -> ChartResult<()> {
    // Create sample data points with varying Y values for size mapping
    let mut series = StaticDataSeries::<Point2D, 256>::new();

    // Add data points where Y value represents bubble size
    let data_points = [
        (1.0, 10.0), // Small bubble
        (2.0, 25.0), // Medium bubble
        (3.0, 45.0), // Large bubble
        (4.0, 15.0), // Small bubble
        (5.0, 35.0), // Medium-large bubble
        (6.0, 50.0), // Largest bubble
        (7.0, 20.0), // Small-medium bubble
        (8.0, 40.0), // Large bubble
        (9.0, 30.0), // Medium bubble
        (10.0, 8.0), // Smallest bubble
    ];

    for (x, y) in data_points.iter() {
        series
            .push(Point2D::new(*x, *y))
            .map_err(ChartError::from)?;
    }

    // Create color palette for the bubbles
    let mut colors = Vec::<Rgb565, 16>::new();
    colors
        .push(Rgb565::new(173 >> 3, 216 >> 2, 230 >> 3)) // Light blue
        .map_err(|_| ChartError::MemoryFull)?;
    colors
        .push(Rgb565::new(100 >> 3, 149 >> 2, 237 >> 3)) // Medium blue
        .map_err(|_| ChartError::MemoryFull)?;
    colors
        .push(Rgb565::BLUE)
        .map_err(|_| ChartError::MemoryFull)?;
    colors
        .push(Rgb565::new(0 >> 3, 0 >> 2, 139 >> 3)) // Dark blue
        .map_err(|_| ChartError::MemoryFull)?;
    colors
        .push(Rgb565::CSS_NAVY)
        .map_err(|_| ChartError::MemoryFull)?;

    // Create color mapping
    let color_mapping = ColorMapping {
        colors,
        strategy: ColorMappingStrategy::ValueBased,
    };

    // Create size mapping for bubble effect
    let size_mapping = SizeMapping {
        min_size: 4,
        max_size: 24,
        scaling: SizeScaling::SquareRoot, // Better for area representation
    };

    // Create a bubble chart with size and color mapping
    let chart = ScatterChart::builder()
        .point_shape(PointShape::Circle)
        .point_color(Rgb565::BLUE) // Default color (will be overridden by mapping)
        .with_size_mapping(size_mapping)
        .with_color_mapping(color_mapping)
        .with_title("Bubble Chart Example")
        .background_color(Rgb565::WHITE)
        .margins(Margins::symmetric(25, 20))
        .build()?;

    // Create legend and related objects outside the render loop
    let legend = StandardLegendBuilder::new()
        .position(LegendPos::Right)
        .orientation(LegendOrientation::Vertical)
        .add_line_entry("Bubble Size", Rgb565::BLUE)?
        .add_line_entry("Value Range", Rgb565::CSS_MEDIUM_BLUE)?
        .professional_style()
        .build()?;

    let renderer = StandardLegendRenderer::<Rgb565>::new();

    // Use the common visual example runner
    window::run(
        WindowConfig::new("Bubble Chart Example")
            .theme(WindowTheme::Default)
            .background(Rgb565::WHITE),
        move |display, viewport, _elapsed| chart.draw(&series, chart.config(), viewport, display),
    )
}

#[cfg(not(feature = "std"))]
fn main() {
    println!("This example requires the 'std' feature to be enabled.");
    println!("Run with: cargo run --example bubble_chart --features std");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_chart() {
        // Test that we can create a bubble chart without errors
        let size_mapping = SizeMapping {
            min_size: 4,
            max_size: 24,
            scaling: SizeScaling::SquareRoot,
        };

        let chart = ScatterChart::<Rgb565>::builder()
            .with_size_mapping(size_mapping)
            .build();

        assert!(chart.is_ok(), "Bubble chart should build without errors");
    }

    #[test]
    fn test_size_mapping_creation() {
        let mapping = SizeMapping {
            min_size: 4,
            max_size: 24,
            scaling: SizeScaling::SquareRoot,
        };

        assert_eq!(mapping.min_size, 4);
        assert_eq!(mapping.max_size, 24);
        assert_eq!(mapping.scaling, SizeScaling::SquareRoot);
    }

    #[test]
    fn test_color_mapping_creation() {
        let mut colors = Vec::<Rgb565, 16>::new();
        colors.push(Rgb565::RED).unwrap();
        colors.push(Rgb565::GREEN).unwrap();
        colors.push(Rgb565::BLUE).unwrap();

        let mapping = ColorMapping {
            colors,
            strategy: ColorMappingStrategy::ValueBased,
        };

        assert_eq!(mapping.colors.len(), 3);
        assert_eq!(mapping.strategy, ColorMappingStrategy::ValueBased);
    }
}
