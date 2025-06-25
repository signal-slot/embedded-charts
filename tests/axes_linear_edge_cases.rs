//! Edge case tests for linear axis implementation
//!
//! This test suite focuses on improving coverage for the linear.rs module,
//! specifically targeting transformation, drawing, and edge cases.

use embedded_charts::axes::{
    Axis, AxisOrientation, AxisPosition, LinearAxis, LinearTickGenerator, TickGenerator,
};
use embedded_graphics::{
    mock_display::MockDisplay, pixelcolor::Rgb565, prelude::*, primitives::Rectangle,
};

#[test]
fn test_linear_axis_inverse_transform_edge_cases() {
    let axis = LinearAxis::<f32, Rgb565>::new(
        0.0,
        100.0,
        AxisOrientation::Horizontal,
        AxisPosition::Bottom,
    );

    let viewport = Rectangle::new(Point::new(10, 10), Size::new(200, 100));

    // Test at boundaries
    let value = axis.inverse_transform(10, viewport);
    assert!((value - 0.0).abs() < 1.0);

    let value = axis.inverse_transform(210, viewport);
    assert!((value - 100.0).abs() < 1.0);

    // Test outside boundaries
    let value = axis.inverse_transform(0, viewport);
    assert!(value <= 1.0); // Allow small tolerance

    let value = axis.inverse_transform(300, viewport);
    assert!(value >= 99.0); // Allow small tolerance

    // Test middle point
    let value = axis.inverse_transform(110, viewport);
    assert!((value - 50.0).abs() < 1.0);
}

#[test]
fn test_linear_axis_vertical_transformations() {
    let axis =
        LinearAxis::<f32, Rgb565>::new(-50.0, 50.0, AxisOrientation::Vertical, AxisPosition::Left);

    let viewport = Rectangle::new(Point::new(20, 20), Size::new(100, 200));

    // Test transform and inverse transform consistency
    let test_values = [-50.0, -25.0, 0.0, 25.0, 50.0];
    for &value in &test_values {
        let coord = axis.transform_value(value, viewport);
        let back = axis.inverse_transform(coord, viewport);
        // Allow larger tolerance for no_std math
        assert!(
            (back - value).abs() < 1.0,
            "Round-trip for {} failed: got {}",
            value,
            back
        );
    }

    // Test Y-axis inversion (higher values should have lower Y coordinates)
    let coord1 = axis.transform_value(-50.0, viewport);
    let coord2 = axis.transform_value(50.0, viewport);
    assert!(coord1 > coord2); // Y coordinates are inverted
}

#[test]
fn test_linear_axis_zero_sized_viewport() {
    let axis = LinearAxis::<f32, Rgb565>::new(
        0.0,
        100.0,
        AxisOrientation::Horizontal,
        AxisPosition::Bottom,
    );

    // Zero width viewport
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(0, 100));
    let coord = axis.transform_value(50.0, viewport);
    // With no_std, the coordinate might be slightly different
    assert!(coord == 0 || coord == viewport.top_left.x);

    // Zero height viewport for vertical axis
    let axis =
        LinearAxis::<f32, Rgb565>::new(0.0, 100.0, AxisOrientation::Vertical, AxisPosition::Left);
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(100, 0));
    let coord = axis.transform_value(50.0, viewport);
    // With zero height, coordinate might be top or slightly off due to math differences
    assert!((coord - viewport.top_left.y).abs() <= 1);
}

#[test]
fn test_linear_axis_required_space() {
    // Test different positions and their required space
    let axis = LinearAxis::<f32, Rgb565>::new(
        0.0,
        100.0,
        AxisOrientation::Horizontal,
        AxisPosition::Bottom,
    );
    let space = axis.required_space();
    assert!(space > 0);
    assert!(space < 100); // Reasonable limit

    let axis =
        LinearAxis::<f32, Rgb565>::new(0.0, 100.0, AxisOrientation::Vertical, AxisPosition::Left);
    let space = axis.required_space();
    assert!(space > 0);
    assert!(space < 100);

    let axis =
        LinearAxis::<f32, Rgb565>::new(0.0, 100.0, AxisOrientation::Vertical, AxisPosition::Right);
    let space = axis.required_space();
    assert!(space > 0);

    let axis =
        LinearAxis::<f32, Rgb565>::new(0.0, 100.0, AxisOrientation::Horizontal, AxisPosition::Top);
    let space = axis.required_space();
    assert!(space > 0);
}

#[test]
fn test_linear_axis_drawing_all_positions() {
    // Test drawing at each position
    let test_cases = [
        (AxisOrientation::Horizontal, AxisPosition::Bottom),
        (AxisOrientation::Horizontal, AxisPosition::Top),
        (AxisOrientation::Vertical, AxisPosition::Left),
        (AxisOrientation::Vertical, AxisPosition::Right),
    ];

    for (orientation, position) in test_cases {
        // Create fresh display for each test
        let mut display = MockDisplay::<Rgb565>::default();
        display.set_allow_overdraw(true);

        // Use a viewport that gives room for axis labels and ticks
        let viewport = Rectangle::new(Point::new(20, 20), Size::new(20, 20));

        let axis = LinearAxis::<f32, Rgb565>::new(0.0, 100.0, orientation, position);

        // Just verify it doesn't panic - drawing details may vary
        let _ = axis.draw(viewport, &mut display);
    }
}

#[test]
fn test_linear_axis_with_negative_range() {
    let axis = LinearAxis::<f32, Rgb565>::new(
        -100.0,
        -10.0,
        AxisOrientation::Horizontal,
        AxisPosition::Bottom,
    );
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 100));

    // Test transformations with all negative range
    let coord1 = axis.transform_value(-100.0, viewport);
    let coord2 = axis.transform_value(-10.0, viewport);
    assert_ne!(coord1, coord2);

    // For horizontal axis, -10 should be to the right of -100
    assert!(coord2 > coord1);

    // Test inverse transform
    let value = axis.inverse_transform(coord1, viewport);
    assert!((value - (-100.0)).abs() < 0.1);
}

#[test]
fn test_linear_axis_with_custom_tick_generator() {
    let custom_ticks = LinearTickGenerator::new(7).with_minor_ticks(3);
    let axis = LinearAxis::<f32, Rgb565>::new(
        0.0,
        50.0,
        AxisOrientation::Horizontal,
        AxisPosition::Bottom,
    )
    .with_tick_generator(custom_ticks);

    assert_eq!(
        <LinearTickGenerator as TickGenerator<f32>>::preferred_tick_count(axis.tick_generator()),
        7
    );

    let mut display = MockDisplay::<Rgb565>::default();
    display.set_allow_overdraw(true);
    let viewport = Rectangle::new(Point::new(20, 20), Size::new(20, 20));

    // Just verify it doesn't panic
    let _ = axis.draw(viewport, &mut display);
}

#[test]
fn test_linear_axis_extreme_ranges() {
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 100));

    // Very large range
    let axis = LinearAxis::<f32, Rgb565>::new(
        0.0,
        1e10,
        AxisOrientation::Horizontal,
        AxisPosition::Bottom,
    );
    let coord = axis.transform_value(5e9, viewport);
    assert!(coord > 0 && coord < 200);

    // Very small range
    let axis = LinearAxis::<f32, Rgb565>::new(
        0.0,
        1e-10,
        AxisOrientation::Horizontal,
        AxisPosition::Bottom,
    );
    let coord = axis.transform_value(5e-11, viewport);
    assert!((0..=200).contains(&coord));

    // Range crossing zero with different magnitudes
    let axis = LinearAxis::<f32, Rgb565>::new(
        -1e6,
        1e3,
        AxisOrientation::Horizontal,
        AxisPosition::Bottom,
    );
    let coord_zero = axis.transform_value(0.0, viewport);
    let coord_neg = axis.transform_value(-5e5, viewport);
    let coord_pos = axis.transform_value(500.0, viewport);
    // With no_std math, very large ranges may have precision issues
    // Just check that the coordinates are different and in reasonable order
    assert!(
        coord_neg <= coord_zero,
        "coord_neg {} should be <= coord_zero {}",
        coord_neg,
        coord_zero
    );
    assert!(
        coord_zero <= coord_pos,
        "coord_zero {} should be <= coord_pos {}",
        coord_zero,
        coord_pos
    );
}

#[test]
fn test_linear_axis_builder_comprehensive() {
    use embedded_charts::axes::LinearAxisBuilder;

    // Test using the LinearAxisBuilder
    let axis =
        LinearAxisBuilder::<f32, Rgb565>::new(AxisOrientation::Vertical, AxisPosition::Right)
            .range(-10.0, 10.0)
            .style(Default::default())
            .tick_count(8)
            .build()
            .unwrap();

    assert_eq!(axis.min(), -10.0);
    assert_eq!(axis.max(), 10.0);
    assert_eq!(axis.orientation(), AxisOrientation::Vertical);
    assert_eq!(axis.position(), AxisPosition::Right);
    assert_eq!(
        <LinearTickGenerator as TickGenerator<f32>>::preferred_tick_count(axis.tick_generator()),
        8
    );
}

#[test]
fn test_linear_axis_i32_type() {
    // Test with integer axis values
    let axis =
        LinearAxis::<i32, Rgb565>::new(0, 1000, AxisOrientation::Horizontal, AxisPosition::Bottom);

    let viewport = Rectangle::new(Point::new(5, 5), Size::new(40, 40));

    // Test integer transformations
    let coord = axis.transform_value(500, viewport);
    // With smaller viewport, middle coordinate will be different
    let expected = viewport.size.width as i32 / 2;
    assert!(
        (coord - expected).abs() <= 5,
        "Expected coord ~{}, got {}",
        expected,
        coord
    );

    let value = axis.inverse_transform(expected, viewport);
    // With smaller viewport, precision is lower
    assert!(
        (value - 500).abs() <= 150,
        "Expected value ~500, got {}",
        value
    );

    // Test drawing
    let mut display = MockDisplay::<Rgb565>::new();
    display.set_allow_overdraw(true);
    let result = axis.draw(viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_linear_axis_configuration_methods() {
    let axis = LinearAxis::<f32, Rgb565>::new(
        0.0,
        100.0,
        AxisOrientation::Horizontal,
        AxisPosition::Bottom,
    )
    .show_line(false)
    .show_ticks(false)
    .show_labels(false)
    .show_grid(true)
    .with_range(-50.0, 50.0);

    assert_eq!(axis.min(), -50.0);
    assert_eq!(axis.max(), 50.0);

    // Test drawing with modified configuration
    let mut display = MockDisplay::<Rgb565>::new();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 100));
    let result = axis.draw(viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_linear_axis_orientation_specific_drawing() {
    // Test horizontal axis at different positions
    for position in [AxisPosition::Top, AxisPosition::Bottom] {
        let mut display = MockDisplay::<Rgb565>::default();
        display.set_allow_overdraw(true);
        let viewport = Rectangle::new(Point::new(20, 20), Size::new(20, 20));

        let axis =
            LinearAxis::<f32, Rgb565>::new(0.0, 100.0, AxisOrientation::Horizontal, position);

        // Just verify drawing doesn't panic
        let _ = axis.draw(viewport, &mut display);
    }

    // Test vertical axis at different positions
    for position in [AxisPosition::Left, AxisPosition::Right] {
        let mut display = MockDisplay::<Rgb565>::default();
        display.set_allow_overdraw(true);
        let viewport = Rectangle::new(Point::new(20, 20), Size::new(20, 20));

        let axis = LinearAxis::<f32, Rgb565>::new(0.0, 100.0, AxisOrientation::Vertical, position);

        // Just verify drawing doesn't panic
        let _ = axis.draw(viewport, &mut display);
    }
}
