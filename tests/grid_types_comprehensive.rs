//! Comprehensive tests for grid type implementations

use embedded_charts::{
    axes::{linear::LinearAxis, ticks::LinearTickGenerator, AxisOrientation, AxisPosition},
    grid::{
        traits::{Grid, GridOrientation, TickAlignedGrid},
        types::{CustomGrid, GridSpacing, GridType, LinearGrid, TickBasedGrid},
        GridStyle,
    },
};
use embedded_graphics::{
    mock_display::MockDisplay,
    pixelcolor::{BinaryColor, Gray8, Rgb565},
    prelude::*,
    primitives::Rectangle,
};

/// Helper function to create a test display
fn create_test_display<C: PixelColor>() -> MockDisplay<C> {
    MockDisplay::new()
}

#[test]
fn test_grid_spacing_variants() {
    // Test equality and inequality
    assert_eq!(GridSpacing::Pixels(20), GridSpacing::Pixels(20));
    assert_ne!(GridSpacing::Pixels(20), GridSpacing::Pixels(30));
    assert_ne!(GridSpacing::Pixels(20), GridSpacing::DataUnits(20.0));
    assert_ne!(GridSpacing::DataUnits(10.0), GridSpacing::Auto);

    // Test copy trait
    let spacing = GridSpacing::Pixels(15);
    let spacing_copy = spacing;
    assert_eq!(spacing, spacing_copy);
}

#[test]
fn test_grid_type_enum() {
    assert_eq!(GridType::Linear, GridType::Linear);
    assert_ne!(GridType::Linear, GridType::TickBased);
    assert_ne!(GridType::TickBased, GridType::Custom);

    // Test exhaustiveness
    match GridType::Linear {
        GridType::Linear => {}
        GridType::TickBased => {}
        GridType::Custom => {}
    }
}

#[test]
fn test_linear_grid_creation() {
    let grid: LinearGrid<Rgb565> =
        LinearGrid::new(GridOrientation::Horizontal, GridSpacing::Pixels(20));
    assert_eq!(grid.orientation(), GridOrientation::Horizontal);
    assert!(grid.is_visible());
    assert_eq!(grid.spacing(), 20.0);

    // Test convenience constructors
    let h_grid: LinearGrid<Rgb565> = LinearGrid::horizontal(GridSpacing::Pixels(25));
    assert_eq!(h_grid.orientation(), GridOrientation::Horizontal);

    let v_grid: LinearGrid<Rgb565> = LinearGrid::vertical(GridSpacing::Auto);
    assert_eq!(v_grid.orientation(), GridOrientation::Vertical);
}

#[test]
fn test_linear_grid_style_configuration() {
    let custom_style = GridStyle::<Rgb565>::new();
    let grid = LinearGrid::new(GridOrientation::Vertical, GridSpacing::Pixels(30))
        .with_style(custom_style.clone());

    assert_eq!(grid.style().major.enabled, custom_style.major.enabled);
}

#[test]
fn test_linear_grid_visibility() {
    let mut grid: LinearGrid<Rgb565> =
        LinearGrid::horizontal(GridSpacing::Pixels(20)).with_visibility(false);

    assert!(!grid.is_visible());

    grid.set_visible(true);
    assert!(grid.is_visible());
}

#[test]
fn test_linear_grid_spacing_calculation() {
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 100));

    // Test fixed pixel spacing
    let grid_pixels: LinearGrid<Rgb565> = LinearGrid::horizontal(GridSpacing::Pixels(25));
    let positions = grid_pixels.calculate_positions(viewport);

    // Should start at 25 and increment by 25
    assert!(!positions.is_empty());
    if positions.len() >= 2 {
        assert_eq!(positions[1] - positions[0], 25);
    }

    // Test auto spacing
    let grid_auto: LinearGrid<Rgb565> = LinearGrid::vertical(GridSpacing::Auto);
    let positions_auto = grid_auto.calculate_positions(viewport);
    assert!(!positions_auto.is_empty());
}

#[test]
fn test_linear_grid_drawing() {
    let mut display = create_test_display::<Rgb565>();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(60, 60));

    let grid = LinearGrid::horizontal(GridSpacing::Pixels(20));
    let result = grid.draw(viewport, &mut display);
    assert!(result.is_ok());

    // Test with invisible grid
    let invisible_grid = LinearGrid::vertical(GridSpacing::Pixels(20)).with_visibility(false);
    let result = invisible_grid.draw(viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_linear_grid_spacing_setter() {
    let mut grid: LinearGrid<Rgb565> = LinearGrid::horizontal(GridSpacing::Pixels(20));
    assert_eq!(grid.spacing(), 20.0);

    grid.set_spacing(30.0);
    assert_eq!(grid.spacing(), 30.0);
}

#[test]
fn test_tick_based_grid_creation() {
    let grid: TickBasedGrid<f32, Rgb565> = TickBasedGrid::new(GridOrientation::Horizontal);
    assert_eq!(grid.orientation(), GridOrientation::Horizontal);
    assert!(grid.is_visible());
    assert!(!grid.is_major_ticks_only());

    // Test convenience constructors
    let h_grid: TickBasedGrid<f32, Rgb565> = TickBasedGrid::horizontal();
    assert_eq!(h_grid.orientation(), GridOrientation::Horizontal);

    let v_grid: TickBasedGrid<i32, Rgb565> = TickBasedGrid::vertical();
    assert_eq!(v_grid.orientation(), GridOrientation::Vertical);
}

#[test]
fn test_tick_based_grid_major_ticks_only() {
    let mut grid: TickBasedGrid<f32, Rgb565> =
        TickBasedGrid::horizontal().with_major_ticks_only(true);

    assert!(grid.is_major_ticks_only());

    grid.set_major_ticks_only(false);
    assert!(!grid.is_major_ticks_only());
}

#[test]
fn test_tick_based_grid_with_axis() {
    let mut display = create_test_display::<Rgb565>();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(64, 50));

    let axis = LinearAxis::new(
        0.0f32,
        100.0,
        AxisOrientation::Horizontal,
        AxisPosition::Bottom,
    )
    .with_tick_generator(LinearTickGenerator::new(5));

    let grid: TickBasedGrid<f32, Rgb565> = TickBasedGrid::vertical();

    // Test drawing with axis
    let result = grid.draw_with_axis(viewport, &axis, &mut display);
    assert!(result.is_ok());

    // Test tick position calculation
    let positions = grid.calculate_tick_positions(viewport, &axis);
    assert!(!positions.is_empty());
}

#[test]
fn test_tick_based_grid_spacing() {
    let grid: TickBasedGrid<f32, Rgb565> = TickBasedGrid::horizontal();
    assert_eq!(grid.spacing(), 1.0); // Default spacing

    let mut grid_mut = grid;
    grid_mut.set_spacing(2.0); // Should be no-op for tick-based grids
    assert_eq!(grid_mut.spacing(), 1.0); // Unchanged
}

#[test]
fn test_custom_grid_creation() {
    let grid: CustomGrid<Rgb565> = CustomGrid::new(GridOrientation::Horizontal);
    assert_eq!(grid.orientation(), GridOrientation::Horizontal);
    assert!(grid.is_visible());

    // Test convenience constructors
    let h_grid: CustomGrid<Rgb565> = CustomGrid::horizontal();
    assert_eq!(h_grid.orientation(), GridOrientation::Horizontal);

    let v_grid: CustomGrid<Rgb565> = CustomGrid::vertical();
    assert_eq!(v_grid.orientation(), GridOrientation::Vertical);
}

#[test]
fn test_custom_grid_line_management() {
    let mut grid: CustomGrid<Rgb565> = CustomGrid::horizontal();

    // Add single line
    assert!(grid.add_line(50).is_ok());
    assert!(grid.add_line(100).is_ok());

    let positions = grid.calculate_positions(Rectangle::new(Point::zero(), Size::new(200, 200)));
    assert_eq!(positions.len(), 2);
    assert_eq!(positions[0], 50);
    assert_eq!(positions[1], 100);

    // Clear lines
    grid.clear_lines();
    let positions_after_clear =
        grid.calculate_positions(Rectangle::new(Point::zero(), Size::new(200, 200)));
    assert!(positions_after_clear.is_empty());
}

#[test]
fn test_custom_grid_builder_pattern() {
    let grid: CustomGrid<Rgb565> = CustomGrid::horizontal().with_lines(&[25, 50, 75, 100]);

    let positions = grid.calculate_positions(Rectangle::new(Point::zero(), Size::new(200, 200)));
    assert_eq!(positions.len(), 4);
}

#[test]
fn test_custom_grid_capacity_limit() {
    let mut grid: CustomGrid<Rgb565> = CustomGrid::vertical();

    // Try to add more than capacity
    for i in 0..70 {
        let _ = grid.add_line(i * 10);
    }

    let positions = grid.calculate_positions(Rectangle::new(Point::zero(), Size::new(1000, 1000)));
    assert!(positions.len() <= 64); // Should be limited by heapless::Vec capacity
}

#[test]
fn test_custom_grid_spacing_calculation() {
    let grid: CustomGrid<Rgb565> = CustomGrid::horizontal().with_lines(&[10, 30, 60, 100]);

    // Average spacing should be (20 + 30 + 40) / 3 = 30
    assert_eq!(grid.spacing(), 30.0);

    // Test with single line
    let single_line_grid: CustomGrid<Rgb565> = CustomGrid::vertical().with_lines(&[50]);
    assert_eq!(single_line_grid.spacing(), 1.0); // Default for insufficient data
}

#[test]
fn test_grid_drawing_with_different_viewports() {
    let viewports = [
        Rectangle::new(Point::new(0, 0), Size::new(50, 50)),
        Rectangle::new(Point::new(5, 5), Size::new(40, 40)),
        Rectangle::new(Point::new(0, 0), Size::new(60, 60)),
    ];

    let grid = LinearGrid::horizontal(GridSpacing::Pixels(20));

    for viewport in &viewports {
        let mut display = create_test_display::<Rgb565>();
        let result = grid.draw(*viewport, &mut display);
        assert!(result.is_ok());
    }
}

#[test]
fn test_grid_style_integration() {
    let mut display = create_test_display::<Rgb565>();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(64, 64));

    let mut style = GridStyle::new();
    style.major.enabled = false; // Disable major grid lines

    let grid = LinearGrid::vertical(GridSpacing::Pixels(20)).with_style(style);

    let result = grid.draw(viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_grid_with_different_color_types() {
    // Test with BinaryColor
    let grid_binary: LinearGrid<BinaryColor> = LinearGrid::horizontal(GridSpacing::Auto);
    assert_eq!(grid_binary.orientation(), GridOrientation::Horizontal);

    // Test with Gray8
    let grid_gray: TickBasedGrid<f32, Gray8> = TickBasedGrid::vertical();
    assert_eq!(grid_gray.orientation(), GridOrientation::Vertical);
}

#[test]
fn test_grid_as_any() {
    let grid: LinearGrid<Rgb565> = LinearGrid::horizontal(GridSpacing::Pixels(20));
    let any_ref = grid.as_any();
    assert!(any_ref.is::<LinearGrid<Rgb565>>());
}

#[test]
fn test_linear_grid_data_units_spacing() {
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 100));
    let grid: LinearGrid<Rgb565> = LinearGrid::horizontal(GridSpacing::DataUnits(5.0));

    // For linear grids, DataUnits falls back to estimated pixel spacing
    let positions = grid.calculate_positions(viewport);
    assert!(!positions.is_empty());
}

#[test]
fn test_tick_based_grid_with_invisible_style() {
    let mut display = create_test_display::<Rgb565>();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(64, 64));

    let mut style = GridStyle::new();
    style.visibility.major = false;
    style.visibility.minor = false;

    let grid: TickBasedGrid<f32, Rgb565> = TickBasedGrid::horizontal().with_style(style);

    let result = grid.draw(viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_custom_grid_add_lines_method() {
    let mut grid: CustomGrid<Rgb565> = CustomGrid::vertical();
    grid.add_lines(&[10, 20, 30, 40, 50]);

    let positions = grid.calculate_positions(Rectangle::new(Point::zero(), Size::new(100, 100)));
    assert_eq!(positions.len(), 5);
}

#[test]
fn test_grid_style_mutation() {
    let mut grid: LinearGrid<Rgb565> = LinearGrid::horizontal(GridSpacing::Pixels(20));

    let new_style = GridStyle::new();
    grid.set_style(new_style.clone());

    assert_eq!(grid.style().major.enabled, new_style.major.enabled);
}

#[test]
fn test_grid_position_boundary_conditions() {
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(100, 50));

    // Test with spacing equal to viewport dimension - grid starts at spacing offset
    let grid: LinearGrid<Rgb565> = LinearGrid::horizontal(GridSpacing::Pixels(50));
    let positions = grid.calculate_positions(viewport);
    // With spacing=50 and height=50, first line would be at y=50 which is outside viewport
    assert!(positions.is_empty());

    // Test with very small spacing
    let grid_small: LinearGrid<Rgb565> = LinearGrid::vertical(GridSpacing::Pixels(1));
    let positions_small = grid_small.calculate_positions(viewport);
    assert!(positions_small.len() <= 64); // Limited by Vec capacity
}

#[test]
fn test_tick_based_grid_fallback_behavior() {
    let mut display = create_test_display::<Rgb565>();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(60, 50));

    // Test drawing without axis (fallback to linear spacing)
    let grid: TickBasedGrid<f32, Rgb565> = TickBasedGrid::horizontal();
    let result = grid.draw(viewport, &mut display);
    assert!(result.is_ok());

    let positions = grid.calculate_positions(viewport);
    assert!(!positions.is_empty());
}
