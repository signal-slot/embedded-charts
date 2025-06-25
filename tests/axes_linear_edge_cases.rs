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
    assert!((value - 0.0).abs() < 0.1);

    let value = axis.inverse_transform(210, viewport);
    assert!((value - 100.0).abs() < 0.1);

    // Test outside boundaries
    let value = axis.inverse_transform(0, viewport);
    assert!(value <= 0.0);

    let value = axis.inverse_transform(300, viewport);
    assert!(value >= 100.0);

    // Test middle point
    let value = axis.inverse_transform(110, viewport);
    assert!((value - 50.0).abs() < 0.1);
}

#[test]
fn test_linear_axis_vertical_transformations() {
    let axis = LinearAxis::<f32, Rgb565>::new(
        -50.0,
        50.0,
        AxisOrientation::Vertical,
        AxisPosition::Left,
    );

    let viewport = Rectangle::new(Point::new(20, 20), Size::new(100, 200));

    // Test transform and inverse transform consistency
    let test_values = [-50.0, -25.0, 0.0, 25.0, 50.0];
    for &value in &test_values {
        let coord = axis.transform_value(value, viewport);
        let back = axis.inverse_transform(coord, viewport);
        assert!((back - value).abs() < 0.1);
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
    assert_eq!(coord, 0); // Should handle gracefully

    // Zero height viewport for vertical axis
    let axis = LinearAxis::<f32, Rgb565>::new(
        0.0,
        100.0,
        AxisOrientation::Vertical,
        AxisPosition::Left,
    );
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(100, 0));
    let coord = axis.transform_value(50.0, viewport);
    assert_eq!(coord, 0); // Should handle gracefully
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

    let axis = LinearAxis::<f32, Rgb565>::new(
        0.0,
        100.0,
        AxisOrientation::Vertical,
        AxisPosition::Left,
    );
    let space = axis.required_space();
    assert!(space > 0);
    assert!(space < 100);

    let axis = LinearAxis::<f32, Rgb565>::new(
        0.0,
        100.0,
        AxisOrientation::Vertical,
        AxisPosition::Right,
    );
    let space = axis.required_space();
    assert!(space > 0);

    let axis = LinearAxis::<f32, Rgb565>::new(
        0.0,
        100.0,
        AxisOrientation::Horizontal,
        AxisPosition::Top,
    );
    let space = axis.required_space();
    assert!(space > 0);
}

#[test]
fn test_linear_axis_drawing_all_positions() {
    let mut display = MockDisplay::<Rgb565>::new();
    let viewport = Rectangle::new(Point::new(50, 50), Size::new(200, 150));

    // Test drawing at each position
    let test_cases = [
        (AxisOrientation::Horizontal, AxisPosition::Bottom),
        (AxisOrientation::Horizontal, AxisPosition::Top),
        (AxisOrientation::Vertical, AxisPosition::Left),
        (AxisOrientation::Vertical, AxisPosition::Right),
    ];

    for (orientation, position) in test_cases {
        display.clear(Rgb565::BLACK).unwrap();

        let axis = LinearAxis::<f32, Rgb565>::new(0.0, 100.0, orientation, position);

        let result = axis.draw(viewport, &mut display);
        assert!(result.is_ok());

        // Verify that something was drawn
        let pixels: Vec<_> = display
            .affected_area()
            .points()
            .filter(|p| display.get_pixel(*p).unwrap() != Rgb565::BLACK)
            .collect();
        assert!(
            !pixels.is_empty(),
            "No pixels drawn for {:?} {:?}",
            orientation,
            position
        );
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

    assert_eq!(axis.tick_generator().preferred_tick_count(), 7);

    let mut display = MockDisplay::<Rgb565>::new();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(300, 200));

    let result = axis.draw(viewport, &mut display);
    assert!(result.is_ok());
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
    assert!(coord >= 0 && coord <= 200);

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
    assert!(coord_neg < coord_zero);
    assert!(coord_zero < coord_pos);
}

#[test]
fn test_linear_axis_builder_comprehensive() {
    use embedded_charts::axes::LinearAxisBuilder;
    
    // Test using the LinearAxisBuilder
    let axis = LinearAxisBuilder::<f32, Rgb565>::new(
        AxisOrientation::Vertical,
        AxisPosition::Right,
    )
    .range(-10.0, 10.0)
    .style(Default::default())
    .tick_count(8)
    .build()
    .unwrap();

    assert_eq!(axis.min(), -10.0);
    assert_eq!(axis.max(), 10.0);
    assert_eq!(axis.orientation(), AxisOrientation::Vertical);
    assert_eq!(axis.position(), AxisPosition::Right);
    assert_eq!(axis.tick_generator().preferred_tick_count(), 8);
}

#[test]
fn test_linear_axis_i32_type() {
    // Test with integer axis values
    let axis = LinearAxis::<i32, Rgb565>::new(
        0,
        1000,
        AxisOrientation::Horizontal,
        AxisPosition::Bottom,
    );

    let viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 100));

    // Test integer transformations
    let coord = axis.transform_value(500, viewport);
    assert_eq!(coord, 100); // Exactly middle

    let value = axis.inverse_transform(100, viewport);
    assert_eq!(value, 500);

    // Test drawing
    let mut display = MockDisplay::<Rgb565>::new();
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
    let mut display = MockDisplay::<Rgb565>::new();
    let viewport = Rectangle::new(Point::new(10, 10), Size::new(200, 150));

    // Test horizontal axis at different positions
    for position in [AxisPosition::Top, AxisPosition::Bottom] {
        display.clear(Rgb565::BLACK).unwrap();

        let axis = LinearAxis::<f32, Rgb565>::new(0.0, 100.0, AxisOrientation::Horizontal, position);

        axis.draw(viewport, &mut display).unwrap();

        // Check that axis line is horizontal
        let affected = display.affected_area();
        let width = affected.size.width;
        assert!(width > 100); // Should span most of viewport
    }

    // Test vertical axis at different positions
    for position in [AxisPosition::Left, AxisPosition::Right] {
        display.clear(Rgb565::BLACK).unwrap();

        let axis = LinearAxis::<f32, Rgb565>::new(0.0, 100.0, AxisOrientation::Vertical, position);

        axis.draw(viewport, &mut display).unwrap();

        // Check that axis line is vertical
        let affected = display.affected_area();
        let height = affected.size.height;
        assert!(height > 100); // Should span most of viewport
    }
}