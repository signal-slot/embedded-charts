//! Comprehensive tests for gradient fills and advanced styling

#![cfg(all(test, feature = "color-support"))]

use embedded_charts::{
    render::ChartRenderer,
    style::{
        FillStyle, GradientDirection, LinearGradient, PatternFill, PatternType, RadialGradient,
    },
};
use embedded_graphics::{
    mock_display::MockDisplay, pixelcolor::Rgb565, prelude::*, primitives::Rectangle,
};

#[test]
fn test_linear_gradient_creation() {
    // Simple gradient
    let gradient: LinearGradient<Rgb565, 8> =
        LinearGradient::simple(Rgb565::RED, Rgb565::BLUE, GradientDirection::Horizontal).unwrap();

    assert!(gradient.is_valid());
    assert_eq!(gradient.stop_count(), 2);
    assert_eq!(gradient.direction(), GradientDirection::Horizontal);
}

#[test]
fn test_linear_gradient_multi_stop() {
    let mut gradient: LinearGradient<Rgb565, 8> = LinearGradient::new(GradientDirection::Vertical);

    // Add stops
    gradient.add_stop(0.0, Rgb565::RED).unwrap();
    gradient.add_stop(0.5, Rgb565::GREEN).unwrap();
    gradient.add_stop(1.0, Rgb565::BLUE).unwrap();

    assert_eq!(gradient.stop_count(), 3);
    assert!(gradient.is_valid());

    // Test color interpolation
    assert_eq!(gradient.color_at(0.0), Some(Rgb565::RED));
    assert_eq!(gradient.color_at(0.5), Some(Rgb565::GREEN));
    assert_eq!(gradient.color_at(1.0), Some(Rgb565::BLUE));
}

#[test]
fn test_gradient_color_interpolation() {
    let gradient: LinearGradient<Rgb565, 8> = LinearGradient::simple(
        Rgb565::new(0, 0, 0),    // Black
        Rgb565::new(31, 63, 31), // White in RGB565
        GradientDirection::Horizontal,
    )
    .unwrap();

    // Test intermediate values - with nearest-neighbor interpolation
    let mid_color = gradient.color_at(0.5).unwrap();
    // Should be one of the two colors (not interpolated)
    let storage = mid_color.into_storage();

    // With nearest-neighbor, it should be either black (0,0,0) or white (31,63,31)
    assert!(
        storage == Rgb565::new(0, 0, 0).into_storage()
            || storage == Rgb565::new(31, 63, 31).into_storage()
    );
}

#[test]
fn test_gradient_invalid_position() {
    let mut gradient: LinearGradient<Rgb565, 4> =
        LinearGradient::new(GradientDirection::Horizontal);

    // Invalid positions should fail
    assert!(gradient.add_stop(-0.1, Rgb565::RED).is_err());
    assert!(gradient.add_stop(1.1, Rgb565::RED).is_err());

    // Valid positions should succeed
    assert!(gradient.add_stop(0.0, Rgb565::RED).is_ok());
    assert!(gradient.add_stop(1.0, Rgb565::BLUE).is_ok());
}

#[test]
fn test_radial_gradient() {
    let gradient: RadialGradient<Rgb565, 8> = RadialGradient::simple(
        Rgb565::WHITE,
        Rgb565::BLACK,
        Point::new(50, 50), // Center at 50%, 50%
    )
    .unwrap();

    assert!(gradient.is_valid());
    assert_eq!(gradient.center(), Point::new(50, 50));

    // Test color at distances
    assert_eq!(gradient.color_at_distance(0.0), Some(Rgb565::WHITE));
    assert_eq!(gradient.color_at_distance(1.0), Some(Rgb565::BLACK));
}

#[test]
fn test_pattern_fills() {
    // Horizontal lines pattern
    let pattern = PatternFill::new(
        Rgb565::RED,
        Rgb565::BLUE,
        PatternType::HorizontalLines {
            spacing: 4,
            width: 2,
        },
    );

    // Test pattern at various positions
    assert_eq!(pattern.color_at(0, 0), Rgb565::RED); // On line
    assert_eq!(pattern.color_at(0, 1), Rgb565::RED); // On line
    assert_eq!(pattern.color_at(0, 2), Rgb565::BLUE); // Off line
    assert_eq!(pattern.color_at(0, 3), Rgb565::BLUE); // Off line
    assert_eq!(pattern.color_at(0, 4), Rgb565::RED); // On next line
}

#[test]
fn test_checkerboard_pattern() {
    let pattern = PatternFill::new(
        Rgb565::BLACK,
        Rgb565::WHITE,
        PatternType::Checkerboard { size: 2 },
    );

    // Test checkerboard pattern
    assert_eq!(pattern.color_at(0, 0), Rgb565::BLACK);
    assert_eq!(pattern.color_at(1, 0), Rgb565::BLACK);
    assert_eq!(pattern.color_at(2, 0), Rgb565::WHITE);
    assert_eq!(pattern.color_at(3, 0), Rgb565::WHITE);
    assert_eq!(pattern.color_at(0, 2), Rgb565::WHITE);
    assert_eq!(pattern.color_at(2, 2), Rgb565::BLACK);
}

#[test]
fn test_dot_pattern() {
    let pattern = PatternFill::new(
        Rgb565::RED,
        Rgb565::WHITE,
        PatternType::Dots {
            spacing: 5,
            radius: 1,
        },
    );

    // Center of a dot cell should have the dot
    assert_eq!(pattern.color_at(2, 2), Rgb565::RED);

    // Outside the dot should be background
    assert_eq!(pattern.color_at(0, 0), Rgb565::WHITE);
    assert_eq!(pattern.color_at(4, 4), Rgb565::WHITE);
}

#[test]
fn test_fill_style_variants() {
    // Solid fill
    let solid = FillStyle::solid(Rgb565::RED);
    assert_eq!(solid.solid_color(), Some(Rgb565::RED));

    // Gradient fill
    let gradient: LinearGradient<Rgb565, 8> =
        LinearGradient::simple(Rgb565::RED, Rgb565::BLUE, GradientDirection::Horizontal).unwrap();
    let gradient_fill = FillStyle::linear_gradient(gradient);
    assert_eq!(gradient_fill.solid_color(), None);

    // Pattern fill
    let pattern = PatternFill::new(
        Rgb565::BLACK,
        Rgb565::WHITE,
        PatternType::Checkerboard { size: 10 },
    );
    let pattern_fill = FillStyle::pattern(pattern);
    assert_eq!(pattern_fill.solid_color(), None);
}

#[test]
fn test_gradient_rendering() {
    let mut display = MockDisplay::<Rgb565>::new();
    let rect = Rectangle::new(Point::new(0, 0), Size::new(10, 10));

    // Create a simple gradient
    let gradient: LinearGradient<Rgb565, 8> =
        LinearGradient::simple(Rgb565::RED, Rgb565::BLUE, GradientDirection::Horizontal).unwrap();
    let fill = FillStyle::linear_gradient(gradient);

    // Render the gradient
    ChartRenderer::draw_filled_rectangle(rect, &fill, &mut display).unwrap();

    // Check that pixels were drawn (basic test)
    // First column should be red-ish, last column should be blue-ish
    let first_pixel = display.get_pixel(Point::new(0, 0)).unwrap();
    let last_pixel = display.get_pixel(Point::new(9, 0)).unwrap();

    // Red has more red component than blue
    assert!((first_pixel.into_storage() >> 11) > (first_pixel.into_storage() & 0x1F));
    // Blue has more blue component than red
    assert!((last_pixel.into_storage() & 0x1F) > (last_pixel.into_storage() >> 11));
}

#[test]
fn test_pattern_rendering() {
    let mut display = MockDisplay::<Rgb565>::new();
    let rect = Rectangle::new(Point::new(0, 0), Size::new(4, 4));

    // Create a checkerboard pattern
    let pattern = PatternFill::new(
        Rgb565::BLACK,
        Rgb565::WHITE,
        PatternType::Checkerboard { size: 2 },
    );
    let fill = FillStyle::pattern(pattern);

    // Render the pattern
    ChartRenderer::draw_filled_rectangle(rect, &fill, &mut display).unwrap();

    // Verify checkerboard pattern
    assert_eq!(display.get_pixel(Point::new(0, 0)).unwrap(), Rgb565::BLACK);
    assert_eq!(display.get_pixel(Point::new(1, 0)).unwrap(), Rgb565::BLACK);
    assert_eq!(display.get_pixel(Point::new(2, 0)).unwrap(), Rgb565::WHITE);
    assert_eq!(display.get_pixel(Point::new(3, 0)).unwrap(), Rgb565::WHITE);
    assert_eq!(display.get_pixel(Point::new(0, 2)).unwrap(), Rgb565::WHITE);
    assert_eq!(display.get_pixel(Point::new(2, 2)).unwrap(), Rgb565::BLACK);
}

#[test]
fn test_gradient_memory_bounds() {
    // Test that we can't exceed maximum stops
    let mut gradient: LinearGradient<Rgb565, 4> =
        LinearGradient::new(GradientDirection::Horizontal);

    // Fill up to capacity
    for i in 0..4 {
        gradient.add_stop(i as f32 * 0.25, Rgb565::RED).unwrap();
    }

    // Next one should fail
    assert!(gradient.add_stop(0.9, Rgb565::BLUE).is_err());
}

#[test]
fn test_diagonal_gradients() {
    let gradient: LinearGradient<Rgb565, 8> =
        LinearGradient::simple(Rgb565::RED, Rgb565::BLUE, GradientDirection::Diagonal).unwrap();

    assert_eq!(gradient.direction(), GradientDirection::Diagonal);

    // Reverse diagonal
    let reverse_gradient: LinearGradient<Rgb565, 8> = LinearGradient::simple(
        Rgb565::GREEN,
        Rgb565::YELLOW,
        GradientDirection::ReverseDiagonal,
    )
    .unwrap();

    assert_eq!(
        reverse_gradient.direction(),
        GradientDirection::ReverseDiagonal
    );
}
