//! Grid type implementations.

use crate::error::ChartResult;
use crate::grid::style::GridStyle;
use crate::grid::traits::{DefaultGridRenderer, Grid, GridOrientation, GridRenderer};
use embedded_graphics::{prelude::*, primitives::Rectangle};

use crate::axes::traits::TickGenerator;
use crate::grid::traits::TickAlignedGrid;

/// Grid spacing configuration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GridSpacing {
    /// Fixed spacing in pixels
    Pixels(u32),
    /// Fixed spacing in data units
    DataUnits(f32),
    /// Automatic spacing based on viewport size
    Auto,
}

/// Grid type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridType {
    /// Linear grid with evenly spaced lines
    Linear,
    /// Grid aligned with axis ticks
    TickBased,
    /// Custom grid with user-defined positions
    Custom,
}

/// Linear grid with evenly spaced grid lines
#[derive(Debug, Clone)]
pub struct LinearGrid<C: PixelColor> {
    /// Grid orientation
    orientation: GridOrientation,
    /// Spacing between grid lines
    spacing: GridSpacing,
    /// Grid style
    style: GridStyle<C>,
    /// Whether the grid is visible
    visible: bool,
    /// Grid renderer
    renderer: DefaultGridRenderer,
}

impl<C: PixelColor> LinearGrid<C> {
    /// Create a new linear grid
    pub fn new(orientation: GridOrientation, spacing: GridSpacing) -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self {
            orientation,
            spacing,
            style: GridStyle::default(),
            visible: true,
            renderer: DefaultGridRenderer,
        }
    }

    /// Create a horizontal linear grid
    pub fn horizontal(spacing: GridSpacing) -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self::new(GridOrientation::Horizontal, spacing)
    }

    /// Create a vertical linear grid
    pub fn vertical(spacing: GridSpacing) -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self::new(GridOrientation::Vertical, spacing)
    }

    /// Set the grid style
    pub fn with_style(mut self, style: GridStyle<C>) -> Self {
        self.style = style;
        self
    }

    /// Set grid visibility
    pub fn with_visibility(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Calculate spacing in pixels for the given viewport
    fn calculate_pixel_spacing(&self, viewport: Rectangle) -> u32 {
        match self.spacing {
            GridSpacing::Pixels(pixels) => pixels,
            GridSpacing::DataUnits(_) => {
                // For linear grids, we need to estimate pixel spacing
                // This is a simplified calculation
                match self.orientation {
                    GridOrientation::Horizontal => viewport.size.height / 10,
                    GridOrientation::Vertical => viewport.size.width / 10,
                }
            }
            GridSpacing::Auto => match self.orientation {
                GridOrientation::Horizontal => (viewport.size.height / 8).max(20),
                GridOrientation::Vertical => (viewport.size.width / 8).max(20),
            },
        }
    }
}

impl<C: PixelColor + 'static> Grid<C> for LinearGrid<C> {
    fn draw<D>(&self, viewport: Rectangle, target: &mut D) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        if !self.visible || !self.style.visibility.any_visible() {
            return Ok(());
        }

        let positions = self.calculate_positions(viewport);

        for &pos in positions.iter() {
            let (start, end) = match self.orientation {
                GridOrientation::Horizontal => (
                    Point::new(viewport.top_left.x, pos),
                    Point::new(viewport.top_left.x + viewport.size.width as i32, pos),
                ),
                GridOrientation::Vertical => (
                    Point::new(pos, viewport.top_left.y),
                    Point::new(pos, viewport.top_left.y + viewport.size.height as i32),
                ),
            };

            // Draw major grid lines
            if self.style.major.enabled && self.style.visibility.major {
                self.renderer.draw_major_line(
                    start,
                    end,
                    &self.style.major.line.line_style,
                    target,
                )?;
            }
        }

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

    fn calculate_positions(&self, viewport: Rectangle) -> heapless::Vec<i32, 64> {
        let mut positions = heapless::Vec::new();
        let spacing = self.calculate_pixel_spacing(viewport);

        match self.orientation {
            GridOrientation::Horizontal => {
                let mut y = viewport.top_left.y + spacing as i32;
                while y < viewport.top_left.y + viewport.size.height as i32 {
                    let _ = positions.push(y);
                    y += spacing as i32;
                }
            }
            GridOrientation::Vertical => {
                let mut x = viewport.top_left.x + spacing as i32;
                while x < viewport.top_left.x + viewport.size.width as i32 {
                    let _ = positions.push(x);
                    x += spacing as i32;
                }
            }
        }

        positions
    }

    fn spacing(&self) -> f32 {
        match self.spacing {
            GridSpacing::Pixels(pixels) => pixels as f32,
            GridSpacing::DataUnits(units) => units,
            GridSpacing::Auto => 1.0,
        }
    }

    fn set_spacing(&mut self, spacing: f32) {
        self.spacing = GridSpacing::DataUnits(spacing);
    }

    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}

/// Grid that aligns with axis ticks
#[derive(Debug, Clone)]
pub struct TickBasedGrid<T, C>
where
    T: Copy + PartialOrd + core::fmt::Display,
    C: PixelColor,
{
    /// Grid orientation
    orientation: GridOrientation,
    /// Grid style
    style: GridStyle<C>,
    /// Whether the grid is visible
    visible: bool,
    /// Whether to show only major tick grid lines
    major_ticks_only: bool,
    /// Grid renderer
    renderer: DefaultGridRenderer,
    /// Phantom data for axis value type
    _phantom: core::marker::PhantomData<T>,
}

impl<T, C> TickBasedGrid<T, C>
where
    T: Copy + PartialOrd + core::fmt::Display,
    C: PixelColor,
{
    /// Create a new tick-based grid
    pub fn new(orientation: GridOrientation) -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self {
            orientation,
            style: GridStyle::default(),
            visible: true,
            major_ticks_only: false,
            renderer: DefaultGridRenderer,
            _phantom: core::marker::PhantomData,
        }
    }

    /// Create a horizontal tick-based grid
    pub fn horizontal() -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self::new(GridOrientation::Horizontal)
    }

    /// Create a vertical tick-based grid
    pub fn vertical() -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self::new(GridOrientation::Vertical)
    }

    /// Set the grid style
    pub fn with_style(mut self, style: GridStyle<C>) -> Self {
        self.style = style;
        self
    }

    /// Set whether to show only major tick grid lines
    pub fn with_major_ticks_only(mut self, major_only: bool) -> Self {
        self.major_ticks_only = major_only;
        self
    }

    /// Check if only major tick grid lines are shown
    pub fn is_major_ticks_only(&self) -> bool {
        self.major_ticks_only
    }

    /// Set whether to show grid lines for major ticks only
    pub fn set_major_ticks_only(&mut self, major_only: bool) {
        self.major_ticks_only = major_only;
    }
}

impl<T, C> Grid<C> for TickBasedGrid<T, C>
where
    T: Copy + PartialOrd + core::fmt::Display + 'static,
    C: PixelColor + 'static,
{
    fn draw<D>(&self, viewport: Rectangle, target: &mut D) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        if !self.visible || !self.style.visibility.any_visible() {
            return Ok(());
        }

        // For basic grid drawing without axis, fall back to linear spacing
        let positions = self.calculate_positions(viewport);

        for &pos in positions.iter() {
            let (start, end) = match self.orientation {
                GridOrientation::Horizontal => (
                    Point::new(viewport.top_left.x, pos),
                    Point::new(viewport.top_left.x + viewport.size.width as i32, pos),
                ),
                GridOrientation::Vertical => (
                    Point::new(pos, viewport.top_left.y),
                    Point::new(pos, viewport.top_left.y + viewport.size.height as i32),
                ),
            };

            // Draw major grid lines
            if self.style.major.enabled && self.style.visibility.major {
                self.renderer.draw_major_line(
                    start,
                    end,
                    &self.style.major.line.line_style,
                    target,
                )?;
            }
        }

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

    fn calculate_positions(&self, viewport: Rectangle) -> heapless::Vec<i32, 64> {
        let mut positions = heapless::Vec::new();
        let spacing = match self.orientation {
            GridOrientation::Horizontal => viewport.size.height / 8,
            GridOrientation::Vertical => viewport.size.width / 8,
        };

        match self.orientation {
            GridOrientation::Horizontal => {
                let mut y = viewport.top_left.y + spacing as i32;
                while y < viewport.top_left.y + viewport.size.height as i32 {
                    let _ = positions.push(y);
                    y += spacing as i32;
                }
            }
            GridOrientation::Vertical => {
                let mut x = viewport.top_left.x + spacing as i32;
                while x < viewport.top_left.x + viewport.size.width as i32 {
                    let _ = positions.push(x);
                    x += spacing as i32;
                }
            }
        }

        positions
    }

    fn spacing(&self) -> f32 {
        1.0 // Default spacing for tick-based grids
    }

    fn set_spacing(&mut self, _spacing: f32) {
        // Spacing is determined by axis ticks, so this is a no-op
    }

    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}

impl<T, C> TickAlignedGrid<T, C> for TickBasedGrid<T, C>
where
    T: crate::axes::traits::AxisValue + 'static,
    C: PixelColor + 'static,
{
    fn draw_with_axis<D, A>(&self, viewport: Rectangle, axis: &A, target: &mut D) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
        A: crate::axes::traits::Axis<T, C>,
    {
        if !self.visible || !self.style.visibility.any_visible() {
            return Ok(());
        }

        let positions = self.calculate_tick_positions(viewport, axis);

        for &pos in positions.iter() {
            let (start, end) = match self.orientation {
                GridOrientation::Horizontal => (
                    Point::new(viewport.top_left.x, pos),
                    Point::new(viewport.top_left.x + viewport.size.width as i32, pos),
                ),
                GridOrientation::Vertical => (
                    Point::new(pos, viewport.top_left.y),
                    Point::new(pos, viewport.top_left.y + viewport.size.height as i32),
                ),
            };

            // Draw grid lines based on tick type
            if self.style.major.enabled && self.style.visibility.major {
                self.renderer.draw_major_line(
                    start,
                    end,
                    &self.style.major.line.line_style,
                    target,
                )?;
            }
        }

        Ok(())
    }

    fn calculate_tick_positions<A>(&self, viewport: Rectangle, axis: &A) -> heapless::Vec<i32, 64>
    where
        A: crate::axes::traits::Axis<T, C>,
    {
        let mut positions = heapless::Vec::new();

        // Generate ticks for the axis range
        let ticks = axis
            .tick_generator()
            .generate_ticks(axis.min(), axis.max(), 16);

        for tick in ticks.iter() {
            if self.major_ticks_only && !tick.is_major {
                continue;
            }

            let screen_pos = axis.transform_value(tick.value, viewport);
            let _ = positions.push(screen_pos);
        }

        positions
    }

    fn set_major_ticks_only(&mut self, major_only: bool) {
        self.major_ticks_only = major_only;
    }

    fn is_major_ticks_only(&self) -> bool {
        self.major_ticks_only
    }
}

/// Custom grid with user-defined positions
#[derive(Debug, Clone)]
pub struct CustomGrid<C: PixelColor> {
    /// Grid orientation
    orientation: GridOrientation,
    /// Custom grid line positions (in screen coordinates)
    positions: heapless::Vec<i32, 64>,
    /// Grid style
    style: GridStyle<C>,
    /// Whether the grid is visible
    visible: bool,
    /// Grid renderer
    renderer: DefaultGridRenderer,
}

impl<C: PixelColor> CustomGrid<C> {
    /// Create a new custom grid
    pub fn new(orientation: GridOrientation) -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self {
            orientation,
            positions: heapless::Vec::new(),
            style: GridStyle::default(),
            visible: true,
            renderer: DefaultGridRenderer,
        }
    }

    /// Create a horizontal custom grid
    pub fn horizontal() -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self::new(GridOrientation::Horizontal)
    }

    /// Create a vertical custom grid
    pub fn vertical() -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self::new(GridOrientation::Vertical)
    }

    /// Add a grid line at the specified position
    pub fn add_line(&mut self, position: i32) -> Result<(), crate::error::DataError> {
        self.positions
            .push(position)
            .map_err(|_| crate::error::DataError::buffer_full("add grid line", 32))
    }

    /// Add multiple grid lines
    pub fn add_lines(&mut self, positions: &[i32]) {
        for &pos in positions {
            let _ = self.add_line(pos);
        }
    }

    /// Clear all grid lines
    pub fn clear_lines(&mut self) {
        self.positions.clear();
    }

    /// Set the grid style
    pub fn with_style(mut self, style: GridStyle<C>) -> Self {
        self.style = style;
        self
    }

    /// Add lines with builder pattern
    pub fn with_lines(mut self, positions: &[i32]) -> Self {
        self.add_lines(positions);
        self
    }
}

impl<C: PixelColor + 'static> Grid<C> for CustomGrid<C> {
    fn draw<D>(&self, viewport: Rectangle, target: &mut D) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        if !self.visible || !self.style.visibility.any_visible() {
            return Ok(());
        }

        for &pos in self.positions.iter() {
            let (start, end) = match self.orientation {
                GridOrientation::Horizontal => (
                    Point::new(viewport.top_left.x, pos),
                    Point::new(viewport.top_left.x + viewport.size.width as i32, pos),
                ),
                GridOrientation::Vertical => (
                    Point::new(pos, viewport.top_left.y),
                    Point::new(pos, viewport.top_left.y + viewport.size.height as i32),
                ),
            };

            // Draw grid lines
            if self.style.major.enabled && self.style.visibility.major {
                self.renderer.draw_major_line(
                    start,
                    end,
                    &self.style.major.line.line_style,
                    target,
                )?;
            }
        }

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
        // Calculate average spacing
        if self.positions.len() < 2 {
            return 1.0;
        }

        let mut total_spacing = 0;
        for window in self.positions.windows(2) {
            if let [a, b] = window {
                total_spacing += (b - a).abs();
            }
        }

        total_spacing as f32 / (self.positions.len() - 1) as f32
    }

    fn set_spacing(&mut self, _spacing: f32) {
        // Custom grids don't use automatic spacing
    }

    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::pixelcolor::Rgb565;

    #[test]
    fn test_linear_grid_creation() {
        let grid: LinearGrid<Rgb565> = LinearGrid::horizontal(GridSpacing::Pixels(20));
        assert_eq!(grid.orientation(), GridOrientation::Horizontal);
        assert!(grid.is_visible());
    }

    #[test]
    fn test_tick_based_grid_creation() {
        let grid: TickBasedGrid<f32, Rgb565> = TickBasedGrid::vertical();
        assert_eq!(grid.orientation(), GridOrientation::Vertical);
        assert!(!grid.is_major_ticks_only());
    }

    #[test]
    fn test_custom_grid_creation() {
        let mut grid: CustomGrid<Rgb565> = CustomGrid::horizontal();
        assert_eq!(grid.orientation(), GridOrientation::Horizontal);

        grid.add_line(100).unwrap();
        grid.add_line(200).unwrap();

        let positions =
            grid.calculate_positions(Rectangle::new(Point::zero(), Size::new(400, 300)));
        assert_eq!(positions.len(), 2);
    }

    #[test]
    fn test_grid_spacing() {
        assert_eq!(GridSpacing::Pixels(20), GridSpacing::Pixels(20));
        assert_ne!(GridSpacing::Pixels(20), GridSpacing::Pixels(30));
        assert_ne!(GridSpacing::Pixels(20), GridSpacing::Auto);
    }

    #[test]
    fn test_grid_type() {
        assert_eq!(GridType::Linear, GridType::Linear);
        assert_ne!(GridType::Linear, GridType::TickBased);
    }
}
