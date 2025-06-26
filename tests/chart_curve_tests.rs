//! Comprehensive tests for chart/curve.rs module

use embedded_charts::axes::{AxisOrientation, AxisPosition, LinearAxis};
use embedded_charts::chart::curve::{CurveChart, CurveChartBuilder};
use embedded_charts::chart::line::{LineChartStyle, MarkerStyle};
use embedded_charts::chart::traits::{Chart, ChartConfig, Margins};
use embedded_charts::data::series::StaticDataSeries;
use embedded_charts::data::Point2D;
use embedded_charts::error::ChartError;
use embedded_charts::grid::GridSystem;
use embedded_charts::math::interpolation::{InterpolationConfig, InterpolationType};
use embedded_charts::prelude::*;
use embedded_graphics::{
    mock_display::MockDisplay,
    pixelcolor::Rgb565,
    prelude::{Point, Size},
    primitives::Rectangle,
};

#[test]
fn test_curve_chart_default() {
    let chart: CurveChart<Rgb565> = CurveChart::default();
    assert_eq!(
        chart.interpolation_config().interpolation_type,
        InterpolationType::CubicSpline
    );
    assert_eq!(chart.interpolation_config().subdivisions, 8);
    assert_eq!(chart.interpolation_config().tension, 0.5);
    assert!(!chart.interpolation_config().closed);
}

#[test]
fn test_curve_chart_interpolation_config_setters() {
    let mut chart: CurveChart<Rgb565> = CurveChart::new();

    let config = InterpolationConfig {
        interpolation_type: InterpolationType::CatmullRom,
        subdivisions: 16,
        tension: 0.7,
        closed: true,
    };

    chart.set_interpolation_config(config.clone());

    assert_eq!(
        chart.interpolation_config().interpolation_type,
        InterpolationType::CatmullRom
    );
    assert_eq!(chart.interpolation_config().subdivisions, 16);
    assert_eq!(chart.interpolation_config().tension, 0.7);
    assert!(chart.interpolation_config().closed);
}

#[test]
fn test_curve_chart_style_setters() {
    let mut chart: CurveChart<Rgb565> = CurveChart::new();

    let style = LineChartStyle {
        line_color: Rgb565::GREEN,
        line_width: 5,
        markers: Some(MarkerStyle {
            visible: true,
            color: Rgb565::YELLOW,
            size: 8,
            shape: embedded_charts::chart::line::MarkerShape::Circle,
        }),
        fill_area: true,
        fill_color: Some(Rgb565::BLUE),
        smooth: false,
        smooth_subdivisions: 8,
    };

    chart.set_style(style.clone());

    assert_eq!(chart.style().line_color, Rgb565::GREEN);
    assert_eq!(chart.style().line_width, 5);
    assert!(chart.style().markers.is_some());
    assert!(chart.style().fill_area);
    assert_eq!(chart.style().fill_color, Some(Rgb565::BLUE));
}

#[test]
fn test_curve_chart_config_setters() {
    let mut chart: CurveChart<Rgb565> = CurveChart::new();

    let mut title = heapless::String::<64>::new();
    title.push_str("Test Chart").unwrap();

    let config = ChartConfig {
        title: Some(title),
        background_color: Some(Rgb565::BLACK),
        margins: Margins::new(30, 20, 40, 10),
        show_grid: false,
        grid_color: None,
    };

    chart.set_config(config.clone());

    assert_eq!(
        chart.config().title.as_ref().map(|s| s.as_str()),
        Some("Test Chart")
    );
    assert_eq!(chart.config().background_color, Some(Rgb565::BLACK));
    assert_eq!(chart.config().margins.top, 30);
    assert_eq!(chart.config().margins.right, 20);
    assert_eq!(chart.config().margins.bottom, 40);
    assert_eq!(chart.config().margins.left, 10);
}

#[test]
fn test_curve_chart_grid_setters() {
    let mut chart: CurveChart<Rgb565> = CurveChart::new();

    let grid = GridSystem::new();
    chart.set_grid(Some(grid));

    assert!(chart.grid().is_some());
}

#[test]
fn test_curve_chart_base_chart_access() {
    let mut chart: CurveChart<Rgb565> = CurveChart::new();

    // Test immutable access
    let base = chart.base_chart();
    // Default color is blue
    assert_eq!(base.style().line_color, Rgb565::BLUE);

    // Test mutable access
    let base_mut = chart.base_chart_mut();
    base_mut.set_style(LineChartStyle {
        line_color: Rgb565::RED,
        line_width: 3,
        markers: None,
        fill_area: false,
        fill_color: None,
        smooth: false,
        smooth_subdivisions: 8,
    });

    assert_eq!(chart.style().line_color, Rgb565::RED);
}

#[test]
fn test_curve_chart_builder_all_options() {
    let chart: CurveChart<Rgb565> = CurveChart::builder()
        .line_color(Rgb565::CYAN)
        .line_width(4)
        .interpolation_type(InterpolationType::Linear)
        .subdivisions(20)
        .tension(0.3)
        .closed(true)
        .fill_area(Rgb565::MAGENTA)
        .with_markers(MarkerStyle {
            visible: true,
            color: Rgb565::RED,
            size: 6,
            shape: embedded_charts::chart::line::MarkerShape::Square,
        })
        .with_title("Curve Test")
        .background_color(Rgb565::BLACK)
        .margins(Margins::new(5, 5, 5, 5))
        .build()
        .unwrap();

    assert_eq!(chart.style().line_color, Rgb565::CYAN);
    assert_eq!(chart.style().line_width, 4);
    assert_eq!(
        chart.interpolation_config().interpolation_type,
        InterpolationType::Linear
    );
    assert_eq!(chart.interpolation_config().subdivisions, 20);
    assert_eq!(chart.interpolation_config().tension, 0.3);
    assert!(chart.interpolation_config().closed);
    assert!(chart.style().fill_area);
    assert_eq!(chart.style().fill_color, Some(Rgb565::MAGENTA));
    assert!(chart.style().markers.is_some());
    assert_eq!(
        chart.config().title.as_ref().map(|s| s.as_str()),
        Some("Curve Test")
    );
    assert_eq!(chart.config().background_color, Some(Rgb565::BLACK));
}

#[test]
fn test_curve_chart_builder_with_grid_and_axes() {
    let x_axis = LinearAxis::<f32, Rgb565>::new(
        0.0,
        100.0,
        AxisOrientation::Horizontal,
        AxisPosition::Bottom,
    );

    let y_axis =
        LinearAxis::<f32, Rgb565>::new(0.0, 50.0, AxisOrientation::Vertical, AxisPosition::Left);

    let grid = GridSystem::new();

    let chart: CurveChart<Rgb565> = CurveChart::builder()
        .with_grid(grid)
        .with_x_axis(x_axis)
        .with_y_axis(y_axis)
        .build()
        .unwrap();

    assert!(chart.grid().is_some());
}

#[test]
fn test_curve_chart_builder_default() {
    let builder: CurveChartBuilder<Rgb565> = CurveChartBuilder::default();
    let chart = builder.build().unwrap();

    assert_eq!(
        chart.interpolation_config().interpolation_type,
        InterpolationType::CubicSpline
    );
}

#[test]
fn test_curve_chart_draw_empty_data() {
    let chart: CurveChart<Rgb565> = CurveChart::new();
    let mut display: MockDisplay<Rgb565> = MockDisplay::new();
    display.set_allow_overdraw(true);
    display.set_allow_out_of_bounds_drawing(true);
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(320, 240));
    let config = ChartConfig::default();

    let data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();

    let result = chart.draw(&data, &config, viewport, &mut display);
    assert!(matches!(result, Err(ChartError::InsufficientData)));
}

#[test]
fn test_curve_chart_draw_single_point() {
    let chart: CurveChart<Rgb565> = CurveChart::new();
    let mut display: MockDisplay<Rgb565> = MockDisplay::new();
    display.set_allow_overdraw(true);
    display.set_allow_out_of_bounds_drawing(true);
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(320, 240));
    let config = ChartConfig::default();

    let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
    data.push(Point2D::new(10.0, 20.0)).unwrap();

    // Single point should fall back to line chart
    let result = chart.draw(&data, &config, viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_curve_chart_draw_multiple_points() {
    let chart: CurveChart<Rgb565> = CurveChart::builder()
        .line_color(Rgb565::RED)
        .line_width(2)
        .interpolation_type(InterpolationType::CubicSpline)
        .subdivisions(8)
        .build()
        .unwrap();

    let mut display: MockDisplay<Rgb565> = MockDisplay::new();
    display.set_allow_overdraw(true);
    display.set_allow_out_of_bounds_drawing(true);
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(320, 240));
    let config = ChartConfig::default();

    let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
    data.push(Point2D::new(0.0, 0.0)).unwrap();
    data.push(Point2D::new(10.0, 20.0)).unwrap();
    data.push(Point2D::new(20.0, 15.0)).unwrap();
    data.push(Point2D::new(30.0, 25.0)).unwrap();

    let result = chart.draw(&data, &config, viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_curve_chart_draw_with_markers() {
    let chart: CurveChart<Rgb565> = CurveChart::builder()
        .line_color(Rgb565::BLUE)
        .with_markers(MarkerStyle {
            visible: true,
            color: Rgb565::YELLOW,
            size: 4,
            shape: embedded_charts::chart::line::MarkerShape::Circle,
        })
        .build()
        .unwrap();

    let mut display: MockDisplay<Rgb565> = MockDisplay::new();
    display.set_allow_overdraw(true);
    display.set_allow_out_of_bounds_drawing(true);
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(320, 240));
    let config = ChartConfig::default();

    let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
    data.push(Point2D::new(0.0, 0.0)).unwrap();
    data.push(Point2D::new(10.0, 20.0)).unwrap();
    data.push(Point2D::new(20.0, 15.0)).unwrap();

    let result = chart.draw(&data, &config, viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_curve_chart_draw_with_fill_area() {
    let chart: CurveChart<Rgb565> = CurveChart::builder()
        .line_color(Rgb565::GREEN)
        .fill_area(Rgb565::CYAN)
        .build()
        .unwrap();

    let mut display: MockDisplay<Rgb565> = MockDisplay::new();
    display.set_allow_overdraw(true);
    display.set_allow_out_of_bounds_drawing(true);
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(320, 240));
    let config = ChartConfig::default();

    let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
    data.push(Point2D::new(0.0, 5.0)).unwrap();
    data.push(Point2D::new(10.0, 15.0)).unwrap();
    data.push(Point2D::new(20.0, 10.0)).unwrap();

    let result = chart.draw(&data, &config, viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_curve_chart_different_interpolation_types() {
    let interpolation_types = vec![
        InterpolationType::Linear,
        InterpolationType::CubicSpline,
        InterpolationType::CatmullRom,
        InterpolationType::Bezier,
    ];

    for interp_type in interpolation_types {
        let chart: CurveChart<Rgb565> = CurveChart::builder()
            .interpolation_type(interp_type)
            .build()
            .unwrap();

        let mut display: MockDisplay<Rgb565> = MockDisplay::new();
        display.set_allow_overdraw(true);
        display.set_allow_out_of_bounds_drawing(true);
        let viewport = Rectangle::new(Point::new(0, 0), Size::new(320, 240));
        let config = ChartConfig::default();

        let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
        data.push(Point2D::new(0.0, 0.0)).unwrap();
        data.push(Point2D::new(10.0, 20.0)).unwrap();
        data.push(Point2D::new(20.0, 10.0)).unwrap();

        let result = chart.draw(&data, &config, viewport, &mut display);
        assert!(
            result.is_ok(),
            "Failed for interpolation type: {interp_type:?}"
        );
    }
}

#[test]
fn test_curve_chart_closed_curve() {
    let chart: CurveChart<Rgb565> = CurveChart::builder()
        .interpolation_type(InterpolationType::CubicSpline)
        .closed(true)
        .build()
        .unwrap();

    let mut display: MockDisplay<Rgb565> = MockDisplay::new();
    display.set_allow_overdraw(true);
    display.set_allow_out_of_bounds_drawing(true);
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(320, 240));
    let config = ChartConfig::default();

    let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
    data.push(Point2D::new(10.0, 10.0)).unwrap();
    data.push(Point2D::new(30.0, 10.0)).unwrap();
    data.push(Point2D::new(30.0, 30.0)).unwrap();
    data.push(Point2D::new(10.0, 30.0)).unwrap();

    let result = chart.draw(&data, &config, viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_curve_chart_edge_case_subdivisions() {
    // Test minimum subdivisions
    let chart: CurveChart<Rgb565> = CurveChart::builder()
        .subdivisions(1) // Should be clamped to 2
        .build()
        .unwrap();
    assert_eq!(chart.interpolation_config().subdivisions, 2);

    // Test maximum subdivisions
    let chart: CurveChart<Rgb565> = CurveChart::builder()
        .subdivisions(50) // Should be clamped to 32
        .build()
        .unwrap();
    assert_eq!(chart.interpolation_config().subdivisions, 32);
}

#[test]
fn test_curve_chart_edge_case_tension() {
    // Test minimum tension
    let chart: CurveChart<Rgb565> = CurveChart::builder()
        .tension(-1.0) // Should be clamped to 0.0
        .build()
        .unwrap();
    assert_eq!(chart.interpolation_config().tension, 0.0);

    // Test maximum tension
    let chart: CurveChart<Rgb565> = CurveChart::builder()
        .tension(5.0) // Should be clamped to 1.0
        .build()
        .unwrap();
    assert_eq!(chart.interpolation_config().tension, 1.0);
}

#[test]
fn test_curve_chart_memory_full_scenario() {
    let chart: CurveChart<Rgb565> = CurveChart::builder()
        .subdivisions(32) // Maximum subdivisions
        .build()
        .unwrap();

    let mut display: MockDisplay<Rgb565> = MockDisplay::new();
    display.set_allow_overdraw(true);
    display.set_allow_out_of_bounds_drawing(true);
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(320, 240));
    let config = ChartConfig::default();

    // Create data that would produce maximum interpolated points
    let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();

    // Fill with fewer points to stay within memory limits (512 total points)
    // With 32 subdivisions, we can have at most ~15 points (15 * 32 = 480 < 512)
    for i in 0..8 {
        data.push(Point2D::new(i as f32, (i * 2) as f32)).unwrap();
    }

    let result = chart.draw(&data, &config, viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_curve_chart_transform_edge_cases() {
    let chart: CurveChart<Rgb565> = CurveChart::new();
    let mut display: MockDisplay<Rgb565> = MockDisplay::new();
    display.set_allow_overdraw(true);
    display.set_allow_out_of_bounds_drawing(true);
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(320, 240));
    let config = ChartConfig::default();

    // Test with identical x values
    let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
    data.push(Point2D::new(10.0, 0.0)).unwrap();
    data.push(Point2D::new(10.0, 20.0)).unwrap();
    data.push(Point2D::new(10.0, 40.0)).unwrap();

    let result = chart.draw(&data, &config, viewport, &mut display);
    assert!(result.is_ok());

    // Test with identical y values
    let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
    data.push(Point2D::new(0.0, 15.0)).unwrap();
    data.push(Point2D::new(20.0, 15.0)).unwrap();
    data.push(Point2D::new(40.0, 15.0)).unwrap();

    let result = chart.draw(&data, &config, viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_curve_chart_small_viewport() {
    let chart: CurveChart<Rgb565> = CurveChart::new();
    let mut display: MockDisplay<Rgb565> = MockDisplay::new();
    display.set_allow_overdraw(true);
    display.set_allow_out_of_bounds_drawing(true);
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(10, 10)); // Very small
    let config = ChartConfig::default();

    let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
    data.push(Point2D::new(0.0, 0.0)).unwrap();
    data.push(Point2D::new(10.0, 10.0)).unwrap();

    let result = chart.draw(&data, &config, viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_curve_chart_large_data_values() {
    let chart: CurveChart<Rgb565> = CurveChart::new();
    let mut display: MockDisplay<Rgb565> = MockDisplay::new();
    display.set_allow_overdraw(true);
    display.set_allow_out_of_bounds_drawing(true);
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(320, 240));
    let config = ChartConfig::default();

    let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
    data.push(Point2D::new(0.0, 0.0)).unwrap();
    data.push(Point2D::new(1000.0, 2000.0)).unwrap();
    data.push(Point2D::new(2000.0, 1500.0)).unwrap();

    let result = chart.draw(&data, &config, viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_curve_chart_negative_values() {
    let chart: CurveChart<Rgb565> = CurveChart::new();
    let mut display: MockDisplay<Rgb565> = MockDisplay::new();
    display.set_allow_overdraw(true);
    display.set_allow_out_of_bounds_drawing(true);
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(320, 240));
    let config = ChartConfig::default();

    let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
    data.push(Point2D::new(-10.0, -20.0)).unwrap();
    data.push(Point2D::new(0.0, 0.0)).unwrap();
    data.push(Point2D::new(10.0, -10.0)).unwrap();

    let result = chart.draw(&data, &config, viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_curve_chart_mixed_positive_negative() {
    let chart: CurveChart<Rgb565> = CurveChart::new();
    let mut display: MockDisplay<Rgb565> = MockDisplay::new();
    display.set_allow_overdraw(true);
    display.set_allow_out_of_bounds_drawing(true);
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(320, 240));
    let config = ChartConfig::default();

    let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
    data.push(Point2D::new(-50.0, 100.0)).unwrap();
    data.push(Point2D::new(0.0, -50.0)).unwrap();
    data.push(Point2D::new(50.0, 75.0)).unwrap();
    data.push(Point2D::new(100.0, -25.0)).unwrap();

    let result = chart.draw(&data, &config, viewport, &mut display);
    assert!(result.is_ok());
}
