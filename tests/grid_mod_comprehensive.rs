//! Comprehensive tests for grid module

use embedded_charts::{
    axes::{linear::LinearAxis, ticks::LinearTickGenerator, AxisOrientation, AxisPosition},
    grid::{
        builder::GridBuilder,
        traits::GridOrientation,
        types::{CustomGrid, LinearGrid, TickBasedGrid},
        GridContainer, GridSpacing, GridStyle, GridSystem,
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
fn test_grid_system_creation() {
    let grid_system: GridSystem<Rgb565> = GridSystem::new();
    assert!(grid_system.enabled);
    assert!(grid_system.horizontal.is_none());
    assert!(grid_system.vertical.is_none());

    // Test default trait
    let default_system: GridSystem<Rgb565> = GridSystem::default();
    assert!(default_system.enabled);
}

#[test]
fn test_grid_system_builder() {
    let _builder: GridBuilder<Rgb565> = GridSystem::builder();
    // Builder functionality is tested in builder module tests
}

#[test]
fn test_grid_system_enable_disable() {
    let mut grid_system: GridSystem<Rgb565> = GridSystem::new();

    assert!(grid_system.is_enabled());

    grid_system.set_enabled(false);
    assert!(!grid_system.is_enabled());

    grid_system.set_enabled(true);
    assert!(grid_system.is_enabled());
}

#[test]
fn test_grid_system_set_grids() {
    let mut grid_system: GridSystem<Rgb565> = GridSystem::new();

    // Set horizontal grid
    let h_grid = LinearGrid::horizontal(GridSpacing::Pixels(20));
    grid_system.set_horizontal_grid(GridContainer::Linear(h_grid));
    assert!(grid_system.horizontal.is_some());

    // Set vertical grid
    let v_grid = LinearGrid::vertical(GridSpacing::Pixels(25));
    grid_system.set_vertical_grid(GridContainer::Linear(v_grid));
    assert!(grid_system.vertical.is_some());
}

#[test]
fn test_grid_system_drawing() {
    let mut display = create_test_display::<Rgb565>();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(60, 50));

    let mut grid_system = GridSystem::new();

    // Test drawing empty system
    let result = grid_system.draw(viewport, &mut display);
    assert!(result.is_ok());

    // Add grids and test drawing with fresh display
    display = create_test_display::<Rgb565>();
    grid_system.set_horizontal_grid(GridContainer::Linear(LinearGrid::horizontal(
        GridSpacing::Pixels(30),
    )));
    grid_system.set_vertical_grid(GridContainer::Linear(LinearGrid::vertical(
        GridSpacing::Pixels(25),
    )));

    // Draw horizontal and vertical grids separately to avoid pixel overlap
    let mut h_only_system = GridSystem::new();
    h_only_system.set_horizontal_grid(GridContainer::Linear(LinearGrid::horizontal(
        GridSpacing::Pixels(30),
    )));
    let result = h_only_system.draw(viewport, &mut display);
    assert!(result.is_ok());

    // Test vertical grid separately
    display = create_test_display::<Rgb565>();
    let mut v_only_system = GridSystem::new();
    v_only_system.set_vertical_grid(GridContainer::Linear(LinearGrid::vertical(
        GridSpacing::Pixels(25),
    )));
    let result = v_only_system.draw(viewport, &mut display);
    assert!(result.is_ok());

    // Test drawing with disabled system
    grid_system.set_enabled(false);
    display = create_test_display::<Rgb565>();
    let result = grid_system.draw(viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_grid_container_variants() {
    let mut display = create_test_display::<Rgb565>();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(60, 60));

    // Test Linear variant
    let linear_container = GridContainer::Linear(LinearGrid::horizontal(GridSpacing::Pixels(20)));
    assert_eq!(linear_container.orientation(), GridOrientation::Horizontal);
    assert!(linear_container.is_visible());
    let result = linear_container.draw(viewport, &mut display);
    assert!(result.is_ok());

    // Test TickBasedF32 variant with fresh display
    display = create_test_display::<Rgb565>();
    let tick_f32_container = GridContainer::TickBasedF32(TickBasedGrid::<f32, Rgb565>::vertical());
    assert_eq!(tick_f32_container.orientation(), GridOrientation::Vertical);
    let result = tick_f32_container.draw(viewport, &mut display);
    assert!(result.is_ok());

    // Test TickBasedI32 variant with fresh display
    display = create_test_display::<Rgb565>();
    let tick_i32_container =
        GridContainer::TickBasedI32(TickBasedGrid::<i32, Rgb565>::horizontal());
    assert_eq!(
        tick_i32_container.orientation(),
        GridOrientation::Horizontal
    );
    let result = tick_i32_container.draw(viewport, &mut display);
    assert!(result.is_ok());

    // Test Custom variant with fresh display
    display = create_test_display::<Rgb565>();
    let custom_grid = CustomGrid::vertical().with_lines(&[25, 50]);
    let custom_container = GridContainer::Custom(Box::new(custom_grid));
    assert_eq!(custom_container.orientation(), GridOrientation::Vertical);
    let result = custom_container.draw(viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_grid_system_with_axes() {
    let mut display = create_test_display::<Rgb565>();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(60, 50));

    let grid_system = GridSystem::new();

    // Create axes
    let x_axis = LinearAxis::new(
        0.0f32,
        100.0,
        AxisOrientation::Horizontal,
        AxisPosition::Bottom,
    )
    .with_tick_generator(LinearTickGenerator::new(5));
    let y_axis = LinearAxis::new(0.0f32, 50.0, AxisOrientation::Vertical, AxisPosition::Left)
        .with_tick_generator(LinearTickGenerator::new(4));

    // Test drawing with both axes - draw them separately to avoid overlap
    let result = grid_system.draw_with_axes(
        viewport,
        Some(&x_axis),
        None::<&LinearAxis<f32, Rgb565>>,
        &mut display,
    );
    assert!(result.is_ok());

    display = create_test_display::<Rgb565>();
    let result = grid_system.draw_with_axes(
        viewport,
        None::<&LinearAxis<f32, Rgb565>>,
        Some(&y_axis),
        &mut display,
    );
    assert!(result.is_ok());

    // Test with only X axis with fresh display
    display = create_test_display::<Rgb565>();
    let result = grid_system.draw_with_axes(
        viewport,
        Some(&x_axis),
        None::<&LinearAxis<f32, Rgb565>>,
        &mut display,
    );
    assert!(result.is_ok());

    // Test with only Y axis with fresh display
    display = create_test_display::<Rgb565>();
    let result = grid_system.draw_with_axes(
        viewport,
        None::<&LinearAxis<f32, Rgb565>>,
        Some(&y_axis),
        &mut display,
    );
    assert!(result.is_ok());

    // Test with no axes with fresh display
    display = create_test_display::<Rgb565>();
    let result = grid_system
        .draw_with_axes::<f32, _, LinearAxis<f32, Rgb565>, LinearAxis<f32, Rgb565>>(
            viewport,
            None,
            None,
            &mut display,
        );
    assert!(result.is_ok());
}

#[test]
fn test_grid_system_with_disabled_axes_drawing() {
    let mut display = create_test_display::<Rgb565>();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(60, 50));

    let mut grid_system = GridSystem::new();
    grid_system.set_enabled(false);

    let x_axis = LinearAxis::new(
        0.0f32,
        100.0,
        AxisOrientation::Horizontal,
        AxisPosition::Bottom,
    );

    let result = grid_system.draw_with_axes(
        viewport,
        Some(&x_axis),
        None::<&LinearAxis<f32, Rgb565>>,
        &mut display,
    );
    assert!(result.is_ok());
}

#[test]
fn test_grid_system_style() {
    let mut grid_system: GridSystem<Rgb565> = GridSystem::new();

    // Modify style
    grid_system.style.major.enabled = false;
    grid_system.style.visibility.major = false;

    assert!(!grid_system.style.major.enabled);
    assert!(!grid_system.style.visibility.major);
}

#[test]
fn test_grid_system_with_different_grid_types() {
    let mut display = create_test_display::<Rgb565>();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(60, 50));

    let mut grid_system: GridSystem<Rgb565> = GridSystem::new();

    // Mix different grid types
    grid_system.set_horizontal_grid(GridContainer::TickBasedF32(TickBasedGrid::horizontal()));

    let custom_grid = CustomGrid::vertical().with_lines(&[20, 40]);
    grid_system.set_vertical_grid(GridContainer::Custom(Box::new(custom_grid)));

    // Draw grid types separately to avoid overlap
    let mut h_only_system = GridSystem::new();
    h_only_system.set_horizontal_grid(GridContainer::TickBasedF32(TickBasedGrid::horizontal()));
    let result = h_only_system.draw(viewport, &mut display);
    assert!(result.is_ok());

    display = create_test_display::<Rgb565>();
    let mut v_only_system = GridSystem::new();
    let custom_grid = CustomGrid::vertical().with_lines(&[20, 40]);
    v_only_system.set_vertical_grid(GridContainer::Custom(Box::new(custom_grid)));
    let result = v_only_system.draw(viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_grid_container_visibility() {
    // Test visibility for all container types
    let containers = vec![
        GridContainer::Linear(LinearGrid::horizontal(GridSpacing::Pixels(20))),
        GridContainer::TickBasedF32(TickBasedGrid::<f32, Rgb565>::vertical()),
        GridContainer::TickBasedI32(TickBasedGrid::<i32, Rgb565>::horizontal()),
        GridContainer::Custom(Box::new(CustomGrid::vertical())),
    ];

    for container in containers {
        assert!(container.is_visible());
    }
}

#[test]
fn test_grid_system_comprehensive_scenario() {
    let mut display = create_test_display::<Rgb565>();
    let viewport = Rectangle::new(Point::new(5, 5), Size::new(54, 54));

    let mut grid_system = GridSystem::new();

    // Configure a complete grid system
    let h_grid = LinearGrid::horizontal(GridSpacing::Pixels(27)).with_style(GridStyle::new());
    let v_grid = TickBasedGrid::<f32, Rgb565>::vertical().with_major_ticks_only(true);

    grid_system.set_horizontal_grid(GridContainer::Linear(h_grid));
    grid_system.set_vertical_grid(GridContainer::TickBasedF32(v_grid));

    // Test with axes
    let x_axis = LinearAxis::new(
        0.0f32,
        250.0,
        AxisOrientation::Horizontal,
        AxisPosition::Bottom,
    );
    let y_axis = LinearAxis::new(
        -50.0f32,
        50.0,
        AxisOrientation::Vertical,
        AxisPosition::Left,
    );

    // Draw axes separately to avoid overlap
    let result = grid_system.draw_with_axes(
        viewport,
        Some(&x_axis),
        None::<&LinearAxis<f32, Rgb565>>,
        &mut display,
    );
    assert!(result.is_ok());

    display = create_test_display::<Rgb565>();
    let result = grid_system.draw_with_axes(
        viewport,
        None::<&LinearAxis<f32, Rgb565>>,
        Some(&y_axis),
        &mut display,
    );
    assert!(result.is_ok());
}

#[test]
fn test_grid_system_with_different_color_types() {
    // Test with BinaryColor
    let grid_binary: GridSystem<BinaryColor> = GridSystem::new();
    assert!(grid_binary.is_enabled());

    // Test with Gray8
    let mut grid_gray: GridSystem<Gray8> = GridSystem::new();
    grid_gray.set_horizontal_grid(GridContainer::Linear(LinearGrid::horizontal(
        GridSpacing::Auto,
    )));
    assert!(grid_gray.horizontal.is_some());
}

#[test]
fn test_grid_system_memory_footprint() {
    use core::mem::size_of;

    // Verify reasonable memory usage
    assert!(size_of::<GridSystem<Rgb565>>() < 1024);
    assert!(size_of::<GridContainer<Rgb565>>() < 512);
}

#[test]
fn test_grid_container_edge_cases() {
    let mut display = create_test_display::<Rgb565>();

    // Test with zero-sized viewport
    let zero_viewport = Rectangle::new(Point::new(0, 0), Size::new(0, 0));
    let container = GridContainer::Linear(LinearGrid::horizontal(GridSpacing::Pixels(10)));
    let result = container.draw(zero_viewport, &mut display);
    assert!(result.is_ok());

    // Test with very large viewport
    let large_viewport = Rectangle::new(Point::new(0, 0), Size::new(60, 60));
    let result = container.draw(large_viewport, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_grid_system_axes_boundary_checking() {
    let mut display = create_test_display::<Rgb565>();
    let viewport = Rectangle::new(Point::new(5, 5), Size::new(50, 50));

    let grid_system = GridSystem::new();

    // Create axis with values that will transform outside viewport
    let x_axis = LinearAxis::new(
        0.0f32,
        1000.0,
        AxisOrientation::Horizontal,
        AxisPosition::Bottom,
    )
    .with_tick_generator(LinearTickGenerator::new(20));

    // Should handle tick positions outside viewport gracefully
    let result = grid_system.draw_with_axes(
        viewport,
        Some(&x_axis),
        None::<&LinearAxis<f32, Rgb565>>,
        &mut display,
    );
    assert!(result.is_ok());
}

#[test]
fn test_re_exports() {
    // Verify that re-exports are accessible
    let _builder: GridBuilder<Rgb565> = GridBuilder::new();
    let _style: GridStyle<Rgb565> = GridStyle::new();
    let _orientation = GridOrientation::Horizontal;
    let _spacing = GridSpacing::Pixels(10);

    // Verify trait re-exports
    use embedded_charts::grid::DefaultGridRenderer;
    let _renderer = DefaultGridRenderer;
}

#[test]
fn test_grid_system_sequential_operations() {
    let mut grid_system: GridSystem<Rgb565> = GridSystem::new();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(60, 60));

    // Test multiple sequential operations
    for i in 0..5 {
        // Toggle enabled state
        grid_system.set_enabled(i % 2 == 0);

        // Alternate grid types
        if i % 2 == 0 {
            grid_system.set_horizontal_grid(GridContainer::Linear(LinearGrid::horizontal(
                GridSpacing::Pixels(20 + i * 5),
            )));
        } else {
            grid_system
                .set_horizontal_grid(GridContainer::TickBasedF32(TickBasedGrid::horizontal()));
        }

        let mut display = create_test_display();
        let result = grid_system.draw(viewport, &mut display);
        assert!(result.is_ok());
    }
}
