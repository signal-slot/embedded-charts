//! Comprehensive tests for grid traits

use embedded_charts::{
    axes::{
        linear::LinearAxis,
        traits::{Axis, AxisValue},
        AxisOrientation, AxisPosition,
    },
    error::ChartError,
    grid::{
        traits::{
            DefaultGridRenderer, Grid, GridConfiguration, GridOrientation, GridRenderer,
            TickAlignedGrid,
        },
        types::LinearGrid,
        GridSpacing, GridStyle, MajorGridStyle, MinorGridStyle,
    },
    style::{LinePattern, LineStyle},
};
use embedded_graphics::{
    mock_display::MockDisplay, pixelcolor::Rgb565, prelude::*, primitives::Rectangle,
};

/// Helper function to create a test display
fn create_test_display() -> MockDisplay<Rgb565> {
    MockDisplay::new()
}

/// Mock grid implementation for testing
struct MockGrid<C: PixelColor> {
    orientation: GridOrientation,
    visible: bool,
    style: GridStyle<C>,
    spacing: f32,
    positions: heapless::Vec<i32, 64>,
}

impl<C: PixelColor + 'static> Grid<C> for MockGrid<C> {
    fn draw<D>(&self, _viewport: Rectangle, _target: &mut D) -> Result<(), ChartError>
    where
        D: DrawTarget<Color = C>,
    {
        Ok(())
    }

    fn orientation(&self) -> GridOrientation {
        self.orientation
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    fn style(&self) -> &GridStyle<C> {
        &self.style
    }

    fn set_style(&mut self, style: GridStyle<C>) {
        self.style = style;
    }

    fn calculate_positions(&self, _viewport: Rectangle) -> heapless::Vec<i32, 64> {
        self.positions.clone()
    }

    fn spacing(&self) -> f32 {
        self.spacing
    }

    fn set_spacing(&mut self, spacing: f32) {
        self.spacing = spacing;
    }

    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}

/// Mock grid configuration implementation
struct MockGridConfig<C: PixelColor> {
    style: GridStyle<C>,
    visible: bool,
}

impl<C: PixelColor> GridConfiguration<C> for MockGridConfig<C> {
    fn configure_major_grid(&mut self, enabled: bool, _spacing: f32, style: MajorGridStyle<C>) {
        self.style.major.enabled = enabled;
        self.style.major = style;
    }

    fn configure_minor_grid(&mut self, enabled: bool, _spacing: f32, style: MinorGridStyle<C>) {
        self.style.minor.enabled = enabled;
        self.style.minor = style;
    }

    fn set_grid_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    fn grid_config(&self) -> &GridStyle<C> {
        &self.style
    }
}

/// Mock tick-aligned grid implementation
struct MockTickAlignedGrid<T, C>
where
    T: AxisValue,
    C: PixelColor,
{
    orientation: GridOrientation,
    visible: bool,
    style: GridStyle<C>,
    major_ticks_only: bool,
    _phantom: core::marker::PhantomData<T>,
}

impl<T, C> Grid<C> for MockTickAlignedGrid<T, C>
where
    T: AxisValue + 'static,
    C: PixelColor + 'static,
{
    fn draw<D>(&self, _viewport: Rectangle, _target: &mut D) -> Result<(), ChartError>
    where
        D: DrawTarget<Color = C>,
    {
        Ok(())
    }

    fn orientation(&self) -> GridOrientation {
        self.orientation
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    fn style(&self) -> &GridStyle<C> {
        &self.style
    }

    fn set_style(&mut self, style: GridStyle<C>) {
        self.style = style;
    }

    fn calculate_positions(&self, _viewport: Rectangle) -> heapless::Vec<i32, 64> {
        heapless::Vec::new()
    }

    fn spacing(&self) -> f32 {
        1.0
    }

    fn set_spacing(&mut self, _spacing: f32) {}

    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}

impl<T, C> TickAlignedGrid<T, C> for MockTickAlignedGrid<T, C>
where
    T: AxisValue + 'static,
    C: PixelColor + 'static,
{
    fn draw_with_axis<D, A>(
        &self,
        _viewport: Rectangle,
        _axis: &A,
        _target: &mut D,
    ) -> Result<(), ChartError>
    where
        D: DrawTarget<Color = C>,
        A: Axis<T, C>,
    {
        Ok(())
    }

    fn calculate_tick_positions<A>(&self, viewport: Rectangle, axis: &A) -> heapless::Vec<i32, 64>
    where
        A: Axis<T, C>,
    {
        let mut positions = heapless::Vec::new();
        // Simple implementation: just transform min and max
        let _ = positions.push(axis.transform_value(axis.min(), viewport));
        let _ = positions.push(axis.transform_value(axis.max(), viewport));
        positions
    }

    fn set_major_ticks_only(&mut self, major_only: bool) {
        self.major_ticks_only = major_only;
    }

    fn is_major_ticks_only(&self) -> bool {
        self.major_ticks_only
    }
}

#[test]
fn test_grid_orientation_enum() {
    assert_eq!(GridOrientation::Horizontal, GridOrientation::Horizontal);
    assert_ne!(GridOrientation::Horizontal, GridOrientation::Vertical);

    // Test copy trait
    let orientation = GridOrientation::Vertical;
    let orientation_copy = orientation;
    assert_eq!(orientation, orientation_copy);
}

#[test]
fn test_grid_trait_basic_operations() {
    let mut grid: MockGrid<Rgb565> = MockGrid {
        orientation: GridOrientation::Horizontal,
        visible: true,
        style: GridStyle::default(),
        spacing: 20.0,
        positions: heapless::Vec::new(),
    };

    // Test orientation
    assert_eq!(grid.orientation(), GridOrientation::Horizontal);

    // Test visibility
    assert!(grid.is_visible());
    grid.set_visible(false);
    assert!(!grid.is_visible());

    // Test spacing
    assert_eq!(grid.spacing(), 20.0);
    grid.set_spacing(30.0);
    assert_eq!(grid.spacing(), 30.0);

    // Test as_any
    let any_ref = grid.as_any();
    assert!(any_ref.is::<MockGrid<Rgb565>>());
}

#[test]
fn test_grid_configuration_trait() {
    let mut config: MockGridConfig<Rgb565> = MockGridConfig {
        style: GridStyle::default(),
        visible: true,
    };

    // Test grid visibility
    config.set_grid_visible(false);
    assert!(!config.visible);

    // Test major grid configuration
    let major_style = MajorGridStyle::default();
    config.configure_major_grid(true, 25.0, major_style);
    assert!(config.grid_config().major.enabled);

    // Test minor grid configuration - note that MinorGridStyle defaults to enabled=true
    let minor_style = MinorGridStyle { 
        enabled: false, 
        ..MinorGridStyle::default() 
    };
    config.configure_minor_grid(false, 10.0, minor_style);
    assert!(!config.style.minor.enabled);
}

#[test]
fn test_tick_aligned_grid_trait() {
    let mut grid: MockTickAlignedGrid<f32, Rgb565> = MockTickAlignedGrid {
        orientation: GridOrientation::Vertical,
        visible: true,
        style: GridStyle::default(),
        major_ticks_only: false,
        _phantom: core::marker::PhantomData,
    };

    // Test major ticks only
    assert!(!grid.is_major_ticks_only());
    grid.set_major_ticks_only(true);
    assert!(grid.is_major_ticks_only());

    // Test with axis
    let axis = LinearAxis::new(
        0.0f32,
        100.0,
        AxisOrientation::Horizontal,
        AxisPosition::Bottom,
    );
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 100));

    let positions = grid.calculate_tick_positions(viewport, &axis);
    assert_eq!(positions.len(), 2); // Min and max
}

#[test]
fn test_default_grid_renderer() {
    let renderer = DefaultGridRenderer;
    let mut display = create_test_display();

    // Test drawing solid line (within 64x64 display)
    let solid_style = LineStyle::solid(Rgb565::BLACK).width(1);
    let result = renderer.draw_grid_line(
        Point::new(0, 0),
        Point::new(60, 0),
        &solid_style,
        &mut display,
    );
    assert!(result.is_ok());

    // Test drawing major line
    let result = renderer.draw_major_line(
        Point::new(0, 10),
        Point::new(60, 10),
        &solid_style,
        &mut display,
    );
    assert!(result.is_ok());

    // Test drawing minor line
    let result = renderer.draw_minor_line(
        Point::new(0, 20),
        Point::new(60, 20),
        &solid_style,
        &mut display,
    );
    assert!(result.is_ok());
}

#[test]
fn test_grid_renderer_line_patterns() {
    let renderer = DefaultGridRenderer;
    let mut display = create_test_display();

    // Test dashed line
    let dashed_style = LineStyle::solid(Rgb565::RED)
        .width(2)
        .pattern(LinePattern::Dashed);
    let result = renderer.draw_grid_line(
        Point::new(2, 2),
        Point::new(60, 2),
        &dashed_style,
        &mut display,
    );
    assert!(result.is_ok());

    // Test dotted line (avoid edges due to circle drawing)
    let dotted_style = LineStyle::solid(Rgb565::GREEN)
        .width(1)
        .pattern(LinePattern::Dotted);
    let result = renderer.draw_grid_line(
        Point::new(2, 10),
        Point::new(60, 10),
        &dotted_style,
        &mut display,
    );
    assert!(result.is_ok());

    // Test dash-dot line
    let dash_dot_style = LineStyle::solid(Rgb565::BLUE)
        .width(1)
        .pattern(LinePattern::DashDot);
    let result = renderer.draw_grid_line(
        Point::new(0, 20),
        Point::new(60, 20),
        &dash_dot_style,
        &mut display,
    );
    assert!(result.is_ok());

    // Test custom pattern (falls back to solid)
    let custom_style = LineStyle::solid(Rgb565::CYAN)
        .width(1)
        .pattern(LinePattern::Custom);
    let result = renderer.draw_grid_line(
        Point::new(0, 30),
        Point::new(60, 30),
        &custom_style,
        &mut display,
    );
    assert!(result.is_ok());
}

#[test]
fn test_grid_renderer_edge_cases() {
    let renderer = DefaultGridRenderer;
    let mut display = create_test_display();

    // Test zero-length line
    let style = LineStyle::solid(Rgb565::BLACK).width(1);
    let result =
        renderer.draw_grid_line(Point::new(30, 30), Point::new(30, 30), &style, &mut display);
    assert!(result.is_ok());

    // Reset display to avoid double drawing
    display = create_test_display();

    // Test diagonal lines
    let result =
        renderer.draw_grid_line(Point::new(0, 0), Point::new(60, 60), &style, &mut display);
    assert!(result.is_ok());

    // Reset display
    display = create_test_display();

    // Test lines partially outside display
    let result =
        renderer.draw_grid_line(Point::new(0, 0), Point::new(10, 10), &style, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_grid_trait_style_operations() {
    let mut grid: MockGrid<Rgb565> = MockGrid {
        orientation: GridOrientation::Vertical,
        visible: true,
        style: GridStyle::default(),
        spacing: 15.0,
        positions: heapless::Vec::new(),
    };

    // Test style getter
    let initial_style = grid.style();
    assert!(initial_style.major.enabled);

    // Test style setter
    let mut new_style = GridStyle::new();
    new_style.visibility.major = false;
    grid.set_style(new_style.clone());

    assert!(!grid.style().visibility.major);
}

#[test]
fn test_grid_calculate_positions() {
    let mut positions = heapless::Vec::new();
    let _ = positions.push(10);
    let _ = positions.push(30);
    let _ = positions.push(50);

    let grid: MockGrid<Rgb565> = MockGrid {
        orientation: GridOrientation::Horizontal,
        visible: true,
        style: GridStyle::default(),
        spacing: 20.0,
        positions: positions.clone(),
    };

    let viewport = Rectangle::new(Point::new(0, 0), Size::new(100, 100));
    let calculated = grid.calculate_positions(viewport);

    assert_eq!(calculated.len(), 3);
    assert_eq!(calculated[0], 10);
    assert_eq!(calculated[1], 30);
    assert_eq!(calculated[2], 50);
}

#[test]
fn test_grid_renderer_memory_size() {
    let renderer = DefaultGridRenderer;
    // DefaultGridRenderer should be a zero-sized type
    assert_eq!(core::mem::size_of_val(&renderer), 0);
}

#[test]
fn test_grid_with_real_implementation() {
    let mut display = create_test_display();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(60, 60));

    // Test with real LinearGrid implementation
    let grid: LinearGrid<Rgb565> = LinearGrid::horizontal(GridSpacing::Pixels(25));

    // Test Grid trait methods
    assert_eq!(grid.orientation(), GridOrientation::Horizontal);
    assert!(grid.is_visible());
    assert_eq!(grid.spacing(), 25.0);

    // Test drawing
    let result = grid.draw(viewport, &mut display);
    assert!(result.is_ok());

    // Test position calculation
    let positions = grid.calculate_positions(viewport);
    assert!(!positions.is_empty());
}

#[test]
fn test_tick_aligned_grid_with_different_axis_types() {
    let grid: MockTickAlignedGrid<i32, Rgb565> = MockTickAlignedGrid {
        orientation: GridOrientation::Horizontal,
        visible: true,
        style: GridStyle::default(),
        major_ticks_only: false,
        _phantom: core::marker::PhantomData,
    };

    // Test with integer axis
    let axis = LinearAxis::new(0i32, 100, AxisOrientation::Vertical, AxisPosition::Left);
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(100, 200));

    let positions = grid.calculate_tick_positions(viewport, &axis);
    assert!(!positions.is_empty());

    // Test visibility with axis
    let mut display = create_test_display();
    let result = grid.draw_with_axis(viewport, &axis, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_grid_renderer_pattern_edge_cases() {
    let renderer = DefaultGridRenderer;
    let mut display = create_test_display();

    // Test very short line with patterns
    let style = LineStyle::solid(Rgb565::BLACK)
        .width(1)
        .pattern(LinePattern::Dashed);
    let result = renderer.draw_grid_line(Point::new(0, 0), Point::new(5, 0), &style, &mut display);
    assert!(result.is_ok());

    // Reset display
    display = create_test_display();

    // Test vertical dashed line
    let result =
        renderer.draw_grid_line(Point::new(50, 0), Point::new(50, 60), &style, &mut display);
    assert!(result.is_ok());
}

#[test]
fn test_grid_configuration_comprehensive() {
    let mut config: MockGridConfig<Rgb565> = MockGridConfig {
        style: GridStyle::default(),
        visible: true,
    };

    // Configure with custom styles
    let major_style = MajorGridStyle::new(LineStyle::solid(Rgb565::RED).width(2));

    let minor_style = MinorGridStyle::new(
        LineStyle::solid(Rgb565::BLUE)
            .width(1)
            .pattern(LinePattern::Dotted),
    );

    config.configure_major_grid(true, 50.0, major_style);
    config.configure_minor_grid(true, 10.0, minor_style);

    assert!(config.grid_config().major.enabled);
    assert!(config.grid_config().minor.enabled);
}
