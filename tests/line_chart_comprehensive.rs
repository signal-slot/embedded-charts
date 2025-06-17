//! Comprehensive test suite for chart/line.rs
//! Target: Increase coverage from 42.25% to 70%
//!
//! This test suite covers:
//! - Grid system integration
//! - Axis integration and AxisChart trait
//! - Transform point method with various scenarios
//! - All marker shapes (Circle, Square, Diamond, Triangle)
//! - Area fill rendering
//! - Smooth curve rendering with error cases
//! - Builder pattern edge cases
//! - Complex multi-feature scenarios
//! - Animation support (when feature enabled)

#![cfg(feature = "line")]

use embedded_charts::{
    axes::{AxisOrientation, AxisPosition, LinearAxis},
    chart::{
        line::{LineChart, LineChartStyle, MarkerShape, MarkerStyle},
        traits::{AxisChart, Chart, ChartBuilder, ChartConfig, Margins},
    },
    data::{point::Point2D, series::StaticDataSeries},
    grid::GridSystem,
};
use embedded_graphics::{
    mock_display::MockDisplay,
    pixelcolor::{Rgb565, RgbColor},
    prelude::*,
    primitives::Rectangle,
};

/// Helper to create test data series
fn create_test_series(values: &[(f32, f32)]) -> StaticDataSeries<Point2D, 256> {
    let mut series = StaticDataSeries::new();
    for &(x, y) in values {
        series.push(Point2D::new(x, y)).unwrap();
    }
    series
}

/// Helper to create a test display
fn create_test_display() -> MockDisplay<Rgb565> {
    let mut display = MockDisplay::<Rgb565>::new();
    display.set_allow_overdraw(true);
    display.set_allow_out_of_bounds_drawing(true);
    display
}

#[test]
fn test_line_chart_with_grid_system() {
    let mut chart = LineChart::<Rgb565>::new();

    // Create a grid system
    let mut grid = GridSystem::<Rgb565>::new();
    grid.set_enabled(true);

    // Set the grid
    chart.set_grid(Some(grid));

    // Verify grid was set
    assert!(chart.grid().is_some());

    // Test rendering with grid
    let mut display = create_test_display();
    let data = create_test_series(&[(0.0, 10.0), (1.0, 20.0), (2.0, 15.0)]);
    let config = ChartConfig::default();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(64, 64));

    let result = chart.draw(&data, &config, viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_line_chart_with_axes() {
    let mut chart = LineChart::<Rgb565>::new();

    // Create axes
    let x_axis = LinearAxis::<f32, Rgb565>::new(
        0.0,
        100.0,
        AxisOrientation::Horizontal,
        AxisPosition::Bottom,
    )
    .with_range(0.0, 100.0);

    let y_axis =
        LinearAxis::<f32, Rgb565>::new(0.0, 50.0, AxisOrientation::Vertical, AxisPosition::Left)
            .with_range(0.0, 50.0);

    // Set axes using AxisChart trait
    chart.set_x_axis(x_axis);
    chart.set_y_axis(y_axis);

    // Verify axes were set
    assert!(chart.x_axis().is_ok());
    assert!(chart.y_axis().is_ok());

    // Test rendering with axes
    let mut display = create_test_display();
    let data = create_test_series(&[(10.0, 20.0), (50.0, 35.0), (90.0, 25.0)]);
    let config = ChartConfig::default();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(64, 64));

    let result = chart.draw(&data, &config, viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_all_marker_shapes() {
    let data = create_test_series(&[(0.0, 10.0), (1.0, 20.0), (2.0, 15.0), (3.0, 25.0)]);
    let config = ChartConfig::default();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(64, 64));

    // Test Circle markers
    {
        let mut display = create_test_display();
        let chart = LineChart::builder()
            .line_color(Rgb565::BLUE)
            .with_markers(MarkerStyle {
                shape: MarkerShape::Circle,
                size: 6,
                color: Rgb565::RED,
                visible: true,
            })
            .build()
            .unwrap();

        let result = chart.draw(&data, &config, viewport, &mut display);
        assert!(result.is_ok());
    }

    // Test Square markers
    {
        let mut display = create_test_display();
        let chart = LineChart::builder()
            .line_color(Rgb565::BLUE)
            .with_markers(MarkerStyle {
                shape: MarkerShape::Square,
                size: 6,
                color: Rgb565::GREEN,
                visible: true,
            })
            .build()
            .unwrap();

        let result = chart.draw(&data, &config, viewport, &mut display);
        assert!(result.is_ok());
    }

    // Test Diamond markers
    {
        let mut display = create_test_display();
        let chart = LineChart::builder()
            .line_color(Rgb565::BLUE)
            .with_markers(MarkerStyle {
                shape: MarkerShape::Diamond,
                size: 8,
                color: Rgb565::YELLOW,
                visible: true,
            })
            .build()
            .unwrap();

        let result = chart.draw(&data, &config, viewport, &mut display);
        assert!(result.is_ok());
    }

    // Test Triangle markers
    {
        let mut display = create_test_display();
        let chart = LineChart::builder()
            .line_color(Rgb565::BLUE)
            .with_markers(MarkerStyle {
                shape: MarkerShape::Triangle,
                size: 8,
                color: Rgb565::MAGENTA,
                visible: true,
            })
            .build()
            .unwrap();

        let result = chart.draw(&data, &config, viewport, &mut display);
        assert!(result.is_ok());
    }
}

#[test]
fn test_invisible_markers() {
    let mut display = create_test_display();
    let data = create_test_series(&[(0.0, 10.0), (1.0, 20.0)]);
    let config = ChartConfig::default();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(64, 64));

    let chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .with_markers(MarkerStyle {
            shape: MarkerShape::Circle,
            size: 6,
            color: Rgb565::RED,
            visible: false, // Markers not visible
        })
        .build()
        .unwrap();

    let result = chart.draw(&data, &config, viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_area_fill_rendering() {
    let mut display = create_test_display();
    let data = create_test_series(&[
        (0.0, 10.0),
        (1.0, 20.0),
        (2.0, 15.0),
        (3.0, 25.0),
        (4.0, 5.0),
    ]);
    let config = ChartConfig::default();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(64, 64));

    let chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .line_width(2)
        .fill_area(Rgb565::CSS_LIGHT_BLUE)
        .build()
        .unwrap();

    let result = chart.draw(&data, &config, viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_area_fill_with_margins() {
    let mut display = create_test_display();
    let data = create_test_series(&[(0.0, 10.0), (1.0, 20.0), (2.0, 15.0)]);
    let config = ChartConfig::<Rgb565> {
        margins: Margins {
            top: 10,
            right: 10,
            bottom: 10,
            left: 10,
        },
        ..Default::default()
    };
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(64, 64));

    let chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .fill_area(Rgb565::CSS_LIGHT_BLUE)
        .build()
        .unwrap();

    let result = chart.draw(&data, &config, viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
#[cfg(feature = "smooth-curves")]
fn test_smooth_curve_rendering() {
    let mut display = create_test_display();
    let data = create_test_series(&[
        (0.0, 10.0),
        (1.0, 20.0),
        (2.0, 5.0),
        (3.0, 25.0),
        (4.0, 15.0),
    ]);
    let config = ChartConfig::default();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(64, 64));

    let chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .smooth(true)
        .build()
        .unwrap();

    let result = chart.draw(&data, &config, viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
#[cfg(feature = "smooth-curves")]
fn test_smooth_curve_with_fill() {
    let mut display = create_test_display();
    let data = create_test_series(&[(0.0, 10.0), (1.0, 20.0), (2.0, 15.0), (3.0, 25.0)]);
    let config = ChartConfig::default();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(64, 64));

    let chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .smooth(true)
        .fill_area(Rgb565::CSS_LIGHT_BLUE)
        .build()
        .unwrap();

    let result = chart.draw(&data, &config, viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_transform_point_with_axes() {
    let mut chart = LineChart::<Rgb565>::new();

    // Set up axes with specific ranges
    let x_axis = LinearAxis::<f32, Rgb565>::new(
        0.0,
        100.0,
        AxisOrientation::Horizontal,
        AxisPosition::Bottom,
    )
    .with_range(0.0, 100.0);
    let y_axis =
        LinearAxis::<f32, Rgb565>::new(0.0, 50.0, AxisOrientation::Vertical, AxisPosition::Left)
            .with_range(0.0, 50.0);

    chart.set_x_axis(x_axis);
    chart.set_y_axis(y_axis);

    // Test rendering with axes ranges
    let mut display = create_test_display();
    let data = create_test_series(&[(25.0, 25.0), (50.0, 40.0), (75.0, 10.0)]);
    let config = ChartConfig::default();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(100, 100));

    let result = chart.draw(&data, &config, viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_empty_data_handling() {
    let mut display = create_test_display();
    let data = create_test_series(&[]); // Empty data
    let config = ChartConfig::default();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(64, 64));

    let chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .build()
        .unwrap();

    let result = chart.draw(&data, &config, viewport, &mut display);
    // Chart returns InsufficientData error for empty data
    assert!(result.is_err());
    assert!(matches!(
        result,
        Err(embedded_charts::error::ChartError::InsufficientData)
    ));
}

#[test]
fn test_single_point_data() {
    let mut display = create_test_display();
    let data = create_test_series(&[(1.0, 10.0)]); // Single point
    let config = ChartConfig::default();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(64, 64));

    let chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .with_markers(MarkerStyle {
            shape: MarkerShape::Circle,
            size: 6,
            color: Rgb565::RED,
            visible: true,
        })
        .build()
        .unwrap();

    let result = chart.draw(&data, &config, viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_builder_edge_cases() {
    // Test with very long title that might exceed heapless capacity
    let long_title = "This is a very long title that might exceed the capacity of the heapless string used in the chart configuration";
    let chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .with_title(long_title)
        .build();

    // Should truncate or handle gracefully
    assert!(chart.is_ok());

    // Test builder with all options
    let chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .line_width(3)
        .fill_area(Rgb565::CSS_LIGHT_BLUE)
        .with_markers(MarkerStyle {
            shape: MarkerShape::Diamond,
            size: 8,
            color: Rgb565::RED,
            visible: true,
        })
        .smooth(true)
        .with_title("Complex Chart")
        .background_color(Rgb565::BLACK)
        .margins(Margins {
            top: 10,
            right: 10,
            bottom: 10,
            left: 10,
        })
        .build();

    assert!(chart.is_ok());
}

#[test]
fn test_complex_multi_feature_scenario() {
    let mut chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .line_width(2)
        .fill_area(Rgb565::CSS_LIGHT_BLUE)
        .with_markers(MarkerStyle {
            shape: MarkerShape::Square,
            size: 6,
            color: Rgb565::RED,
            visible: true,
        })
        .with_title("Multi-Feature Chart")
        .background_color(Rgb565::WHITE)
        .margins(Margins {
            top: 5,
            right: 5,
            bottom: 5,
            left: 5,
        })
        .build()
        .unwrap();

    // Add grid system
    let mut grid = GridSystem::<Rgb565>::new();
    grid.set_enabled(true);
    chart.set_grid(Some(grid));

    // Add axes
    let x_axis = LinearAxis::<f32, Rgb565>::new(
        0.0,
        10.0,
        AxisOrientation::Horizontal,
        AxisPosition::Bottom,
    )
    .with_range(0.0, 10.0);
    let y_axis =
        LinearAxis::<f32, Rgb565>::new(0.0, 100.0, AxisOrientation::Vertical, AxisPosition::Left)
            .with_range(0.0, 100.0);

    chart.set_x_axis(x_axis);
    chart.set_y_axis(y_axis);

    // Test rendering
    let mut display = create_test_display();
    let data = create_test_series(&[
        (0.0, 10.0),
        (2.5, 50.0),
        (5.0, 75.0),
        (7.5, 40.0),
        (10.0, 90.0),
    ]);
    let config = chart.config().clone();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(64, 64));

    let result = chart.draw(&data, &config, viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_zero_width_viewport() {
    let mut display = create_test_display();
    let data = create_test_series(&[(0.0, 10.0), (1.0, 20.0)]);
    let config = ChartConfig::default();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(0, 64));

    let chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .build()
        .unwrap();

    let result = chart.draw(&data, &config, viewport, &mut display);
    // Should handle gracefully
    assert!(result.is_ok());
}

#[test]
fn test_zero_height_viewport() {
    let mut display = create_test_display();
    let data = create_test_series(&[(0.0, 10.0), (1.0, 20.0)]);
    let config = ChartConfig::default();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(64, 0));

    let chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .build()
        .unwrap();

    let result = chart.draw(&data, &config, viewport, &mut display);
    // Should handle gracefully
    assert!(result.is_ok());
}

#[test]
fn test_data_at_numeric_limits() {
    let mut display = create_test_display();
    let data = create_test_series(&[(f32::MIN, f32::MIN), (0.0, 0.0), (f32::MAX, f32::MAX)]);
    let config = ChartConfig::default();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(64, 64));

    let chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .build()
        .unwrap();

    let result = chart.draw(&data, &config, viewport, &mut display);
    // Should handle extreme values
    assert!(result.is_ok());
}

#[test]
#[cfg(feature = "animations")]
fn test_animated_line_chart() {
    use embedded_charts::chart::line::AnimatedLineChart;

    let _chart = AnimatedLineChart::<Rgb565>::new();
    // Just verify it can be created
}

#[test]
#[cfg(feature = "animations")]
fn test_animated_line_chart_builder() {
    use embedded_charts::chart::line::AnimatedLineChart;

    let chart = AnimatedLineChart::<Rgb565>::builder()
        .line_color(Rgb565::BLUE)
        .line_width(2)
        .with_title("Animated Chart")
        .build();

    assert!(chart.is_ok());
}

#[test]
fn test_default_implementations() {
    let chart = LineChart::<Rgb565>::default();
    assert_eq!(chart.style().line_color, Rgb565::BLUE);
    assert_eq!(chart.style().line_width, 1); // Default is 1, not 2
    assert!(!chart.style().fill_area);
    assert!(chart.style().markers.is_none());
    assert!(!chart.style().smooth);
}

#[test]
fn test_marker_style_default() {
    let marker = MarkerStyle::<Rgb565>::default();
    assert_eq!(marker.shape, MarkerShape::Circle);
    assert_eq!(marker.size, 4);
    assert_eq!(marker.color, Rgb565::RED);
    assert!(marker.visible);
}

#[test]
fn test_line_chart_style_accessors() {
    let mut style = LineChartStyle::<Rgb565> {
        line_color: Rgb565::BLUE,
        line_width: 2,
        fill_area: false,
        fill_color: None,
        markers: None,
        smooth: false,
        smooth_subdivisions: 8,
    };

    // Test with fill color
    style.fill_area = true;
    style.fill_color = Some(Rgb565::CSS_LIGHT_BLUE);
    assert!(style.fill_color.is_some());

    // Test with markers
    style.markers = Some(MarkerStyle::default());
    assert!(style.markers.is_some());
}

#[test]
fn test_large_dataset() {
    let mut display = create_test_display();
    let mut data = StaticDataSeries::<Point2D, 256>::new();

    // Fill with maximum points
    for i in 0..256 {
        let x = i as f32;
        let y = (x * 0.1).sin() * 50.0 + 50.0;
        data.push(Point2D::new(x, y)).unwrap();
    }

    let config = ChartConfig::default();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(64, 64));

    let chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .line_width(1)
        .build()
        .unwrap();

    let result = chart.draw(&data, &config, viewport, &mut display);
    assert!(result.is_ok());
}
