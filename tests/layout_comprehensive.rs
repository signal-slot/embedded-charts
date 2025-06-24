//! Comprehensive tests for layout module

use embedded_charts::{
    chart::traits::Margins,
    error::LayoutError,
    layout::{ChartLayout, ComponentPositioning, LegendPosition, Viewport},
};
use embedded_graphics::{prelude::*, primitives::Rectangle};

/// Helper function to create a test rectangle
fn create_test_area(width: u32, height: u32) -> Rectangle {
    Rectangle::new(Point::zero(), Size::new(width, height))
}

#[test]
fn test_chart_layout_creation() {
    let area = create_test_area(400, 300);
    let layout = ChartLayout::new(area);

    assert_eq!(layout.total_area, area);
    assert_eq!(layout.chart_area, area);
    assert!(layout.title_area.is_none());
    assert!(layout.legend_area.is_none());
    assert!(layout.x_axis_area.is_none());
    assert!(layout.y_axis_area.is_none());
}

#[test]
fn test_chart_layout_with_margins() {
    let area = create_test_area(400, 300);

    // Test with uniform margins
    let margins = Margins::all(20);
    let layout = ChartLayout::new(area).with_margins(margins);

    assert_eq!(layout.chart_area.top_left, Point::new(20, 20));
    assert_eq!(layout.chart_area.size, Size::new(360, 260));

    // Test with asymmetric margins
    let margins = Margins {
        top: 10,
        right: 20,
        bottom: 30,
        left: 40,
    };
    let layout = ChartLayout::new(area).with_margins(margins);

    assert_eq!(layout.chart_area.top_left, Point::new(40, 10));
    assert_eq!(layout.chart_area.size, Size::new(340, 260));
}

#[test]
fn test_chart_layout_with_title() {
    let area = create_test_area(400, 300);

    // Test normal title
    let layout = ChartLayout::new(area).with_title(30).unwrap();

    assert!(layout.title_area.is_some());
    let title_area = layout.title_area.unwrap();
    assert_eq!(title_area.top_left, Point::zero());
    assert_eq!(title_area.size, Size::new(400, 30));
    assert_eq!(layout.chart_area.top_left, Point::new(0, 30));
    assert_eq!(layout.chart_area.size, Size::new(400, 270));

    // Test title that's too large
    let result = ChartLayout::new(area).with_title(300);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        LayoutError::InsufficientSpace
    ));

    // Test title that exactly fits
    let result = ChartLayout::new(area).with_title(299);
    assert!(result.is_ok());
}

#[test]
fn test_chart_layout_with_title_and_margins() {
    let area = create_test_area(400, 300);
    let margins = Margins::all(20);

    let layout = ChartLayout::new(area)
        .with_margins(margins)
        .with_title(30)
        .unwrap();

    assert!(layout.title_area.is_some());
    let title_area = layout.title_area.unwrap();
    assert_eq!(title_area.top_left, Point::new(20, 20));
    assert_eq!(title_area.size, Size::new(360, 30));
    assert_eq!(layout.chart_area.top_left, Point::new(20, 50));
    assert_eq!(layout.chart_area.size, Size::new(360, 230));
}

#[test]
fn test_chart_layout_with_legend_right() {
    let area = create_test_area(400, 300);
    let legend_size = Size::new(80, 200);

    let layout = ChartLayout::new(area)
        .with_legend(LegendPosition::Right, legend_size)
        .unwrap();

    assert!(layout.legend_area.is_some());
    let legend_area = layout.legend_area.unwrap();
    assert_eq!(legend_area.top_left, Point::new(320, 0));
    assert_eq!(legend_area.size, legend_size);
    assert_eq!(layout.chart_area.size, Size::new(320, 300));

    // Test legend that's too wide
    let result = ChartLayout::new(area).with_legend(LegendPosition::Right, Size::new(400, 200));
    assert!(result.is_err());
}

#[test]
fn test_chart_layout_with_legend_bottom() {
    let area = create_test_area(400, 300);
    let legend_size = Size::new(300, 50);

    let layout = ChartLayout::new(area)
        .with_legend(LegendPosition::Bottom, legend_size)
        .unwrap();

    assert!(layout.legend_area.is_some());
    let legend_area = layout.legend_area.unwrap();
    assert_eq!(legend_area.top_left, Point::new(0, 250));
    assert_eq!(legend_area.size, legend_size);
    assert_eq!(layout.chart_area.size, Size::new(400, 250));

    // Test legend that's too tall
    let result = ChartLayout::new(area).with_legend(LegendPosition::Bottom, Size::new(300, 300));
    assert!(result.is_err());
}

#[test]
fn test_chart_layout_with_legend_top() {
    let area = create_test_area(400, 300);
    let legend_size = Size::new(300, 50);

    let layout = ChartLayout::new(area)
        .with_legend(LegendPosition::Top, legend_size)
        .unwrap();

    assert!(layout.legend_area.is_some());
    let legend_area = layout.legend_area.unwrap();
    assert_eq!(legend_area.top_left, Point::zero());
    assert_eq!(legend_area.size, legend_size);
    assert_eq!(layout.chart_area.top_left, Point::new(0, 50));
    assert_eq!(layout.chart_area.size, Size::new(400, 250));
}

#[test]
fn test_chart_layout_with_legend_left() {
    let area = create_test_area(400, 300);
    let legend_size = Size::new(80, 200);

    let layout = ChartLayout::new(area)
        .with_legend(LegendPosition::Left, legend_size)
        .unwrap();

    assert!(layout.legend_area.is_some());
    let legend_area = layout.legend_area.unwrap();
    assert_eq!(legend_area.top_left, Point::zero());
    assert_eq!(legend_area.size, legend_size);
    assert_eq!(layout.chart_area.top_left, Point::new(80, 0));
    assert_eq!(layout.chart_area.size, Size::new(320, 300));
}

#[test]
fn test_chart_layout_with_x_axis() {
    let area = create_test_area(400, 300);

    let layout = ChartLayout::new(area).with_x_axis(40).unwrap();

    assert!(layout.x_axis_area.is_some());
    let x_axis_area = layout.x_axis_area.unwrap();
    assert_eq!(x_axis_area.top_left, Point::new(0, 260));
    assert_eq!(x_axis_area.size, Size::new(400, 40));
    assert_eq!(layout.chart_area.size, Size::new(400, 260));

    // Test axis that's too tall
    let result = ChartLayout::new(area).with_x_axis(300);
    assert!(result.is_err());
}

#[test]
fn test_chart_layout_with_y_axis() {
    let area = create_test_area(400, 300);

    let layout = ChartLayout::new(area).with_y_axis(60).unwrap();

    assert!(layout.y_axis_area.is_some());
    let y_axis_area = layout.y_axis_area.unwrap();
    assert_eq!(y_axis_area.top_left, Point::zero());
    assert_eq!(y_axis_area.size, Size::new(60, 300));
    assert_eq!(layout.chart_area.top_left, Point::new(60, 0));
    assert_eq!(layout.chart_area.size, Size::new(340, 300));

    // Test axis that's too wide
    let result = ChartLayout::new(area).with_y_axis(400);
    assert!(result.is_err());
}

#[test]
fn test_chart_layout_complex_configuration() {
    let area = create_test_area(800, 600);
    let margins = Margins {
        top: 20,
        right: 30,
        bottom: 40,
        left: 50,
    };

    let layout = ChartLayout::new(area)
        .with_margins(margins)
        .with_title(50)
        .unwrap()
        .with_legend(LegendPosition::Right, Size::new(120, 400))
        .unwrap()
        .with_x_axis(60)
        .unwrap()
        .with_y_axis(80)
        .unwrap();

    // Verify all areas are set
    assert!(layout.title_area.is_some());
    assert!(layout.legend_area.is_some());
    assert!(layout.x_axis_area.is_some());
    assert!(layout.y_axis_area.is_some());

    // Verify final chart area
    // Starting area: 800x600
    // After margins: 720x540 at (50, 20)
    // After title: 720x490 at (50, 70)
    // After legend: 600x490 at (50, 70)
    // After x-axis: 600x430 at (50, 70)
    // After y-axis: 520x430 at (130, 70)
    assert_eq!(layout.chart_area.top_left, Point::new(130, 70));
    assert_eq!(layout.chart_area.size, Size::new(520, 430));
}

#[test]
fn test_chart_layout_validate() {
    // Test valid layout
    let area = create_test_area(400, 300);
    let layout = ChartLayout::new(area);
    assert!(layout.validate().is_ok());

    // Test layout with insufficient space
    let small_area = create_test_area(50, 50);
    let layout = ChartLayout::new(small_area).with_margins(Margins::all(25));

    let result = layout.validate();
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        LayoutError::InsufficientSpace
    ));

    // Test boundary case - exactly 10x10
    let boundary_area = create_test_area(30, 30);
    let layout = ChartLayout::new(boundary_area).with_margins(Margins::all(10));
    assert!(layout.validate().is_ok());

    // Test boundary case - 9x10 (should fail)
    let boundary_area = create_test_area(29, 30);
    let layout = ChartLayout::new(boundary_area).with_margins(Margins::all(10));
    assert!(layout.validate().is_err());
}

#[test]
fn test_chart_layout_chart_area() {
    let area = create_test_area(400, 300);
    let layout = ChartLayout::new(area);

    assert_eq!(layout.chart_area(), area);
}

#[test]
fn test_legend_position_enum() {
    // Test equality
    assert_eq!(LegendPosition::Top, LegendPosition::Top);
    assert_ne!(LegendPosition::Top, LegendPosition::Bottom);

    // Test copy trait
    let pos = LegendPosition::Right;
    let pos_copy = pos;
    assert_eq!(pos, pos_copy);
}

#[test]
fn test_viewport_creation() {
    let area = create_test_area(200, 150);
    let viewport = Viewport::new(area);

    assert_eq!(viewport.area, area);
    assert_eq!(viewport.zoom, 1.0);
    assert_eq!(viewport.offset, Point::zero());
}

#[test]
fn test_viewport_with_zoom() {
    let area = create_test_area(200, 150);

    // Test normal zoom
    let viewport = Viewport::new(area).with_zoom(2.0);
    assert_eq!(viewport.zoom, 2.0);

    // Test zoom clamping - too small
    let viewport = Viewport::new(area).with_zoom(0.05);
    assert_eq!(viewport.zoom, 0.1);

    // Test zoom clamping - too large
    let viewport = Viewport::new(area).with_zoom(15.0);
    assert_eq!(viewport.zoom, 10.0);

    // Test boundary values
    let viewport = Viewport::new(area).with_zoom(0.1);
    assert_eq!(viewport.zoom, 0.1);

    let viewport = Viewport::new(area).with_zoom(10.0);
    assert_eq!(viewport.zoom, 10.0);
}

#[test]
fn test_viewport_with_offset() {
    let area = create_test_area(200, 150);
    let offset = Point::new(10, -20);

    let viewport = Viewport::new(area).with_offset(offset);
    assert_eq!(viewport.offset, offset);
}

#[test]
fn test_viewport_transform_point() {
    let viewport_area = Rectangle::new(Point::new(100, 50), Size::new(200, 100));
    let viewport = Viewport::new(viewport_area);

    let data_bounds = Rectangle::new(Point::new(0, 0), Size::new(100, 50));

    // Test transform with no zoom/offset
    let data_point = Point::new(50, 25);
    let screen_point = viewport.transform_point(data_point, data_bounds);
    // Normalized: (0.5, 0.5) -> Screen: (200, 100)
    assert_eq!(screen_point, Point::new(200, 100));

    // Test corner points
    let bottom_left = Point::new(0, 0);
    let screen_bl = viewport.transform_point(bottom_left, data_bounds);
    assert_eq!(screen_bl, viewport_area.top_left);

    let top_right = Point::new(100, 50);
    let screen_tr = viewport.transform_point(top_right, data_bounds);
    assert_eq!(screen_tr, Point::new(300, 150));
}

#[test]
fn test_viewport_transform_with_zoom_and_offset() {
    let viewport_area = Rectangle::new(Point::new(0, 0), Size::new(200, 100));
    let viewport = Viewport::new(viewport_area)
        .with_zoom(2.0)
        .with_offset(Point::new(10, 5));

    let data_bounds = Rectangle::new(Point::new(0, 0), Size::new(100, 50));

    // Test center point with zoom
    let data_point = Point::new(50, 25);
    let screen_point = viewport.transform_point(data_point, data_bounds);
    // Normalized: (0.5, 0.5) -> Zoomed: (1.0, 1.0) -> Screen: (200, 100) + offset
    assert_eq!(screen_point, Point::new(210, 105));
}

#[test]
fn test_viewport_transform_edge_cases() {
    let viewport_area = Rectangle::new(Point::new(0, 0), Size::new(200, 100));
    let viewport = Viewport::new(viewport_area);

    // Test with zero-sized data bounds
    let zero_bounds = Rectangle::new(Point::new(10, 20), Size::new(0, 0));
    let point = Point::new(10, 20);
    let transformed = viewport.transform_point(point, zero_bounds);
    // Should use 0.5 as normalized value -> center of viewport
    assert_eq!(transformed, Point::new(100, 50));

    // Test with single dimension zero
    let partial_zero = Rectangle::new(Point::new(0, 0), Size::new(100, 0));
    let point = Point::new(50, 0);
    let transformed = viewport.transform_point(point, partial_zero);
    assert_eq!(transformed.x, 100); // Normal transform
    assert_eq!(transformed.y, 50); // Centered due to zero height
}

#[test]
fn test_viewport_is_point_visible() {
    let viewport_area = Rectangle::new(Point::new(10, 20), Size::new(100, 80));
    let viewport = Viewport::new(viewport_area);

    // Test visible points
    assert!(viewport.is_point_visible(Point::new(10, 20))); // Top-left
    assert!(viewport.is_point_visible(Point::new(50, 50))); // Center
    assert!(viewport.is_point_visible(Point::new(109, 99))); // Bottom-right (inclusive)

    // Test invisible points
    assert!(!viewport.is_point_visible(Point::new(9, 20))); // Just left
    assert!(!viewport.is_point_visible(Point::new(10, 19))); // Just above
    assert!(!viewport.is_point_visible(Point::new(110, 50))); // Just right
    assert!(!viewport.is_point_visible(Point::new(50, 100))); // Just below
    assert!(!viewport.is_point_visible(Point::new(0, 0))); // Way outside
}

#[test]
fn test_viewport_visible_data_bounds() {
    let viewport_area = Rectangle::new(Point::new(0, 0), Size::new(200, 100));
    let viewport = Viewport::new(viewport_area);

    let full_bounds = Rectangle::new(Point::new(-100, -50), Size::new(400, 200));
    let visible_bounds = viewport.visible_data_bounds(full_bounds);

    // Current implementation just returns full bounds
    assert_eq!(visible_bounds, full_bounds);
}

#[test]
fn test_component_positioning_center() {
    let container = Rectangle::new(Point::new(10, 20), Size::new(100, 80));
    let component_size = Size::new(20, 10);

    let position = ComponentPositioning::center_in_container(component_size, container);
    assert_eq!(position, Point::new(50, 55));

    // Test with component same size as container
    let position = ComponentPositioning::center_in_container(container.size, container);
    assert_eq!(position, container.top_left);

    // Test with odd sizes
    let container = Rectangle::new(Point::new(0, 0), Size::new(101, 81));
    let component_size = Size::new(21, 11);
    let position = ComponentPositioning::center_in_container(component_size, container);
    assert_eq!(position, Point::new(40, 35));
}

#[test]
fn test_component_positioning_align_top_left() {
    let container = Rectangle::new(Point::new(10, 20), Size::new(100, 80));

    let position = ComponentPositioning::align_top_left(container, 5);
    assert_eq!(position, Point::new(15, 25));

    let position = ComponentPositioning::align_top_left(container, 0);
    assert_eq!(position, container.top_left);
}

#[test]
fn test_component_positioning_align_top_right() {
    let container = Rectangle::new(Point::new(10, 20), Size::new(100, 80));
    let component_size = Size::new(20, 15);

    let position = ComponentPositioning::align_top_right(component_size, container, 5);
    assert_eq!(position, Point::new(85, 25));

    let position = ComponentPositioning::align_top_right(component_size, container, 0);
    assert_eq!(position, Point::new(90, 20));
}

#[test]
fn test_component_positioning_align_bottom_left() {
    let container = Rectangle::new(Point::new(10, 20), Size::new(100, 80));
    let component_size = Size::new(20, 15);

    let position = ComponentPositioning::align_bottom_left(component_size, container, 5);
    assert_eq!(position, Point::new(15, 80));

    let position = ComponentPositioning::align_bottom_left(component_size, container, 0);
    assert_eq!(position, Point::new(10, 85));
}

#[test]
fn test_component_positioning_align_bottom_right() {
    let container = Rectangle::new(Point::new(10, 20), Size::new(100, 80));
    let component_size = Size::new(20, 15);

    let position = ComponentPositioning::align_bottom_right(component_size, container, 5);
    assert_eq!(position, Point::new(85, 80));

    let position = ComponentPositioning::align_bottom_right(component_size, container, 0);
    assert_eq!(position, Point::new(90, 85));
}

#[test]
fn test_component_positioning_distribute_horizontal() {
    let container = Rectangle::new(Point::new(0, 0), Size::new(200, 100));

    // Test normal distribution
    let sizes = vec![Size::new(30, 20), Size::new(40, 25), Size::new(30, 15)];
    let spacing = 10;

    let positions =
        ComponentPositioning::distribute_horizontal(&sizes, container, spacing).unwrap();

    assert_eq!(positions.len(), 3);
    // Total width: 30 + 40 + 30 = 100
    // Total spacing: 10 * 2 = 20
    // Start x: (200 - 120) / 2 = 40
    assert_eq!(positions[0], Point::new(40, 40)); // (200-20)/2
    assert_eq!(positions[1], Point::new(80, 37)); // 40 + 30 + 10, (100-25)/2
    assert_eq!(positions[2], Point::new(130, 42)); // 80 + 40 + 10, (100-15)/2

    // Test empty components
    let positions = ComponentPositioning::distribute_horizontal(&[], container, spacing).unwrap();
    assert!(positions.is_empty());

    // Test single component
    let sizes = vec![Size::new(50, 30)];
    let positions =
        ComponentPositioning::distribute_horizontal(&sizes, container, spacing).unwrap();
    assert_eq!(positions.len(), 1);
    assert_eq!(positions[0], Point::new(75, 35)); // Centered
}

#[test]
fn test_component_positioning_distribute_horizontal_insufficient_space() {
    let container = Rectangle::new(Point::new(0, 0), Size::new(100, 50));

    let sizes = vec![Size::new(40, 20), Size::new(40, 20), Size::new(40, 20)];
    let spacing = 10;

    // Total needed: 120 + 20 = 140, but container is only 100 wide
    let result = ComponentPositioning::distribute_horizontal(&sizes, container, spacing);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        LayoutError::InsufficientSpace
    ));
}

#[test]
fn test_component_positioning_distribute_vertical() {
    let container = Rectangle::new(Point::new(0, 0), Size::new(100, 200));

    // Test normal distribution
    let sizes = vec![Size::new(20, 30), Size::new(25, 40), Size::new(15, 30)];
    let spacing = 10;

    let positions = ComponentPositioning::distribute_vertical(&sizes, container, spacing).unwrap();

    assert_eq!(positions.len(), 3);
    // Total height: 30 + 40 + 30 = 100
    // Total spacing: 10 * 2 = 20
    // Start y: (200 - 120) / 2 = 40
    assert_eq!(positions[0], Point::new(40, 40)); // (100-20)/2
    assert_eq!(positions[1], Point::new(37, 80)); // (100-25)/2, 40 + 30 + 10
    assert_eq!(positions[2], Point::new(42, 130)); // (100-15)/2, 80 + 40 + 10

    // Test empty components
    let positions = ComponentPositioning::distribute_vertical(&[], container, spacing).unwrap();
    assert!(positions.is_empty());
}

#[test]
fn test_component_positioning_distribute_vertical_insufficient_space() {
    let container = Rectangle::new(Point::new(0, 0), Size::new(50, 100));

    let sizes = vec![Size::new(20, 40), Size::new(20, 40), Size::new(20, 40)];
    let spacing = 10;

    // Total needed: 120 + 20 = 140, but container is only 100 tall
    let result = ComponentPositioning::distribute_vertical(&sizes, container, spacing);
    assert!(result.is_err());
}

#[test]
fn test_component_positioning_distribute_capacity_limit() {
    let container = Rectangle::new(Point::new(0, 0), Size::new(1000, 100));

    // Test that we can handle up to 16 components (heapless::Vec capacity)
    let mut sizes = Vec::new();
    for _ in 0..16 {
        sizes.push(Size::new(10, 10));
    }

    let result = ComponentPositioning::distribute_horizontal(&sizes, container, 5);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 16);

    // Test with 17 components (should fail due to capacity)
    sizes.push(Size::new(10, 10));
    let result = ComponentPositioning::distribute_horizontal(&sizes, container, 5);
    assert!(result.is_err());
}

#[test]
fn test_layout_sequential_operations() {
    let area = create_test_area(600, 400);
    let margins = Margins::all(10);

    // Test different ordering of operations
    let layout1 = ChartLayout::new(area)
        .with_margins(margins)
        .with_title(30)
        .unwrap()
        .with_x_axis(40)
        .unwrap()
        .with_y_axis(50)
        .unwrap();

    // Should produce consistent final chart area regardless of order
    // (after margins, title, axes are applied)
    assert_eq!(layout1.chart_area.top_left, Point::new(60, 40));
    assert_eq!(layout1.chart_area.size, Size::new(530, 310));
}

#[test]
fn test_viewport_builder_pattern() {
    let area = create_test_area(300, 200);

    let viewport = Viewport::new(area)
        .with_zoom(3.0)
        .with_offset(Point::new(15, -10))
        .with_zoom(2.5); // Should override previous zoom

    assert_eq!(viewport.zoom, 2.5);
    assert_eq!(viewport.offset, Point::new(15, -10));
}

#[test]
fn test_margins_edge_cases() {
    let area = create_test_area(100, 100);

    // Test with margins that consume all space
    let margins = Margins {
        top: 25,
        right: 25,
        bottom: 25,
        left: 25,
    };
    let layout = ChartLayout::new(area).with_margins(margins);
    assert_eq!(layout.chart_area.size, Size::new(50, 50));

    // Test with margins larger than area (should result in zero or negative size)
    let large_margins = Margins::all(60);
    let layout = ChartLayout::new(area).with_margins(large_margins);
    // The layout will be created but validate() should fail
    assert!(layout.validate().is_err());
}

#[test]
fn test_component_positioning_with_offset_containers() {
    let container = Rectangle::new(Point::new(50, 100), Size::new(200, 150));
    let component_size = Size::new(40, 30);

    // All positioning functions should work correctly with non-zero origin containers
    let center = ComponentPositioning::center_in_container(component_size, container);
    assert_eq!(center, Point::new(130, 160)); // 50 + 80, 100 + 60

    let tl = ComponentPositioning::align_top_left(container, 10);
    assert_eq!(tl, Point::new(60, 110));

    let tr = ComponentPositioning::align_top_right(component_size, container, 10);
    assert_eq!(tr, Point::new(200, 110)); // 50 + 200 - 40 - 10

    let bl = ComponentPositioning::align_bottom_left(component_size, container, 10);
    assert_eq!(bl, Point::new(60, 210)); // 50 + 10, 100 + 150 - 30 - 10

    let br = ComponentPositioning::align_bottom_right(component_size, container, 10);
    assert_eq!(br, Point::new(200, 210));
}

#[test]
fn test_saturating_arithmetic_in_distribution() {
    let container = Rectangle::new(Point::new(0, 0), Size::new(100, 100));

    // Test with single component (saturating_sub should handle len=1 case)
    let sizes = vec![Size::new(20, 20)];
    let positions = ComponentPositioning::distribute_horizontal(&sizes, container, 10).unwrap();
    assert_eq!(positions.len(), 1);
    assert_eq!(positions[0], Point::new(40, 40)); // Centered
}

#[test]
fn test_layout_error_types() {
    let area = create_test_area(50, 50);

    // Multiple ways to trigger InsufficientSpace error
    let result1 = ChartLayout::new(area).with_title(50);
    assert!(matches!(
        result1.unwrap_err(),
        LayoutError::InsufficientSpace
    ));

    let result2 = ChartLayout::new(area).with_x_axis(50);
    assert!(matches!(
        result2.unwrap_err(),
        LayoutError::InsufficientSpace
    ));

    let result3 = ChartLayout::new(area).with_legend(LegendPosition::Right, Size::new(50, 30));
    assert!(matches!(
        result3.unwrap_err(),
        LayoutError::InsufficientSpace
    ));
}

#[test]
fn test_viewport_copy_and_partialeq() {
    let area = create_test_area(200, 150);
    let viewport1 = Viewport::new(area)
        .with_zoom(2.0)
        .with_offset(Point::new(5, 10));

    // Test Copy trait
    let viewport2 = viewport1;
    assert_eq!(viewport1.area, viewport2.area);
    assert_eq!(viewport1.zoom, viewport2.zoom);
    assert_eq!(viewport1.offset, viewport2.offset);

    // Test PartialEq
    assert_eq!(viewport1, viewport2);

    let viewport3 = Viewport::new(area)
        .with_zoom(2.0)
        .with_offset(Point::new(5, 11));
    assert_ne!(viewport1, viewport3);
}
