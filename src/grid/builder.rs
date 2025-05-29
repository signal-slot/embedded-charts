//! Grid builder implementations for fluent configuration.

#[cfg(all(feature = "no_std", not(feature = "std")))]
extern crate alloc;

#[cfg(all(feature = "no_std", not(feature = "std")))]
use alloc::boxed::Box;

#[cfg(not(all(feature = "no_std", not(feature = "std"))))]
use std::boxed::Box;

use crate::grid::{
    style::{GridStyle, GridVisibility, MajorGridStyle, MinorGridStyle},
    traits::{Grid, GridOrientation},
    types::{CustomGrid, GridSpacing, LinearGrid, TickBasedGrid},
    GridContainer, GridSystem,
};
use embedded_graphics::prelude::*;

/// Main grid builder for configuring grid systems
#[derive(Debug)]
pub struct GridBuilder<C: PixelColor> {
    horizontal_grid: Option<GridContainer<C>>,
    vertical_grid: Option<GridContainer<C>>,
    style: GridStyle<C>,
    enabled: bool,
}

impl<C: PixelColor + 'static> GridBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new grid builder
    pub fn new() -> Self {
        Self {
            horizontal_grid: None,
            vertical_grid: None,
            style: GridStyle::default(),
            enabled: true,
        }
    }

    /// Set a linear horizontal grid
    pub fn horizontal_linear(mut self, spacing: GridSpacing) -> Self {
        self.horizontal_grid = Some(GridContainer::Linear(LinearGrid::horizontal(spacing)));
        self
    }

    /// Set a linear vertical grid
    pub fn vertical_linear(mut self, spacing: GridSpacing) -> Self {
        self.vertical_grid = Some(GridContainer::Linear(LinearGrid::vertical(spacing)));
        self
    }

    /// Set a tick-based horizontal grid for f32 values
    pub fn horizontal_tick_based_f32(mut self) -> Self {
        self.horizontal_grid = Some(GridContainer::TickBasedF32(
            TickBasedGrid::<f32, C>::horizontal(),
        ));
        self
    }

    /// Set a tick-based vertical grid for f32 values
    pub fn vertical_tick_based_f32(mut self) -> Self {
        self.vertical_grid = Some(GridContainer::TickBasedF32(
            TickBasedGrid::<f32, C>::vertical(),
        ));
        self
    }

    /// Set a tick-based horizontal grid for i32 values
    pub fn horizontal_tick_based_i32(mut self) -> Self {
        self.horizontal_grid = Some(GridContainer::TickBasedI32(
            TickBasedGrid::<i32, C>::horizontal(),
        ));
        self
    }

    /// Set a tick-based vertical grid for i32 values
    pub fn vertical_tick_based_i32(mut self) -> Self {
        self.vertical_grid = Some(GridContainer::TickBasedI32(
            TickBasedGrid::<i32, C>::vertical(),
        ));
        self
    }

    /// Set a custom horizontal grid
    pub fn horizontal_custom(mut self, positions: &[i32]) -> Self {
        let mut grid = CustomGrid::horizontal();
        grid.add_lines(positions);
        self.horizontal_grid = Some(GridContainer::Custom(Box::new(grid)));
        self
    }

    /// Set a custom vertical grid
    pub fn vertical_custom(mut self, positions: &[i32]) -> Self {
        let mut grid = CustomGrid::vertical();
        grid.add_lines(positions);
        self.vertical_grid = Some(GridContainer::Custom(Box::new(grid)));
        self
    }

    /// Set the overall grid style
    pub fn style(mut self, style: GridStyle<C>) -> Self {
        self.style = style;
        self
    }

    /// Use professional grid styling
    pub fn professional(mut self) -> Self {
        self.style = GridStyle::professional();
        self
    }

    /// Use minimal grid styling
    pub fn minimal(mut self) -> Self {
        self.style = GridStyle::minimal();
        self
    }

    /// Use dashed grid styling
    pub fn dashed(mut self) -> Self {
        self.style = GridStyle::dashed();
        self
    }

    /// Set grid visibility
    pub fn visibility(mut self, visibility: GridVisibility) -> Self {
        self.style.visibility = visibility;
        self
    }

    /// Enable or disable the grid
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set grid opacity
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.style.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Configure major grid lines
    pub fn major_grid(mut self, style: MajorGridStyle<C>) -> Self {
        self.style.major = style;
        self
    }

    /// Configure minor grid lines
    pub fn minor_grid(mut self, style: MinorGridStyle<C>) -> Self {
        self.style.minor = style;
        self
    }

    /// Build the grid system
    pub fn build(self) -> GridSystem<C> {
        let mut grid_system = GridSystem::new();
        grid_system.style = self.style;
        grid_system.enabled = self.enabled;

        if let Some(horizontal) = self.horizontal_grid {
            grid_system.horizontal = Some(horizontal);
        }

        if let Some(vertical) = self.vertical_grid {
            grid_system.vertical = Some(vertical);
        }

        grid_system
    }
}

impl<C: PixelColor + 'static> Default for GridBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for linear grids
#[derive(Debug)]
pub struct LinearGridBuilder<C: PixelColor> {
    orientation: GridOrientation,
    spacing: GridSpacing,
    style: GridStyle<C>,
    visible: bool,
}

impl<C: PixelColor> LinearGridBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new linear grid builder
    pub fn new(orientation: GridOrientation) -> Self {
        Self {
            orientation,
            spacing: GridSpacing::Auto,
            style: GridStyle::default(),
            visible: true,
        }
    }

    /// Create a horizontal linear grid builder
    pub fn horizontal() -> Self {
        Self::new(GridOrientation::Horizontal)
    }

    /// Create a vertical linear grid builder
    pub fn vertical() -> Self {
        Self::new(GridOrientation::Vertical)
    }

    /// Set the grid spacing
    pub fn spacing(mut self, spacing: GridSpacing) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set spacing in pixels
    pub fn spacing_pixels(mut self, pixels: u32) -> Self {
        self.spacing = GridSpacing::Pixels(pixels);
        self
    }

    /// Set spacing in data units
    pub fn spacing_data_units(mut self, units: f32) -> Self {
        self.spacing = GridSpacing::DataUnits(units);
        self
    }

    /// Use automatic spacing
    pub fn spacing_auto(mut self) -> Self {
        self.spacing = GridSpacing::Auto;
        self
    }

    /// Set the grid style
    pub fn style(mut self, style: GridStyle<C>) -> Self {
        self.style = style;
        self
    }

    /// Set grid visibility
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Build the linear grid
    pub fn build(self) -> LinearGrid<C> {
        LinearGrid::new(self.orientation, self.spacing)
            .with_style(self.style)
            .with_visibility(self.visible)
    }
}

/// Builder for tick-based grids
#[derive(Debug)]
pub struct TickBasedGridBuilder<T, C>
where
    T: Copy + PartialOrd + core::fmt::Display,
    C: PixelColor,
{
    orientation: GridOrientation,
    style: GridStyle<C>,
    visible: bool,
    major_ticks_only: bool,
    _phantom: core::marker::PhantomData<T>,
}

impl<T, C> TickBasedGridBuilder<T, C>
where
    T: Copy + PartialOrd + core::fmt::Display,
    C: PixelColor,
{
    /// Create a new tick-based grid builder
    pub fn new(orientation: GridOrientation) -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self {
            orientation,
            style: GridStyle::default(),
            visible: true,
            major_ticks_only: false,
            _phantom: core::marker::PhantomData,
        }
    }

    /// Create a horizontal tick-based grid builder
    pub fn horizontal() -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self::new(GridOrientation::Horizontal)
    }

    /// Create a vertical tick-based grid builder
    pub fn vertical() -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self::new(GridOrientation::Vertical)
    }

    /// Set the grid style
    pub fn style(mut self, style: GridStyle<C>) -> Self {
        self.style = style;
        self
    }

    /// Set grid visibility
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Set whether to show only major tick grid lines
    pub fn major_ticks_only(mut self, major_only: bool) -> Self {
        self.major_ticks_only = major_only;
        self
    }

    /// Build the tick-based grid
    pub fn build(self) -> TickBasedGrid<T, C>
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        TickBasedGrid::new(self.orientation)
            .with_style(self.style)
            .with_major_ticks_only(self.major_ticks_only)
    }
}

/// Builder for custom grids
#[derive(Debug)]
pub struct CustomGridBuilder<C: PixelColor> {
    orientation: GridOrientation,
    positions: heapless::Vec<i32, 64>,
    style: GridStyle<C>,
    visible: bool,
}

impl<C: PixelColor + 'static> CustomGridBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new custom grid builder
    pub fn new(orientation: GridOrientation) -> Self {
        Self {
            orientation,
            positions: heapless::Vec::new(),
            style: GridStyle::default(),
            visible: true,
        }
    }

    /// Create a horizontal custom grid builder
    pub fn horizontal() -> Self {
        Self::new(GridOrientation::Horizontal)
    }

    /// Create a vertical custom grid builder
    pub fn vertical() -> Self {
        Self::new(GridOrientation::Vertical)
    }

    /// Add a grid line position
    pub fn add_line(mut self, position: i32) -> Self {
        let _ = self.positions.push(position);
        self
    }

    /// Add multiple grid line positions
    pub fn add_lines(mut self, positions: &[i32]) -> Self {
        for &pos in positions {
            let _ = self.positions.push(pos);
        }
        self
    }

    /// Set evenly spaced lines
    pub fn evenly_spaced(mut self, start: i32, end: i32, count: usize) -> Self {
        if count > 1 {
            let step = (end - start) / (count - 1) as i32;
            for i in 0..count {
                let pos = start + i as i32 * step;
                let _ = self.positions.push(pos);
            }
        }
        self
    }

    /// Set the grid style
    pub fn style(mut self, style: GridStyle<C>) -> Self {
        self.style = style;
        self
    }

    /// Set grid visibility
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Build the custom grid
    pub fn build(self) -> CustomGrid<C> {
        let mut grid = CustomGrid::new(self.orientation).with_style(self.style);
        grid.set_visible(self.visible);

        for &pos in self.positions.iter() {
            let _ = grid.add_line(pos);
        }

        grid
    }
}

/// Convenience functions for quick grid creation
pub mod presets {
    use super::*;
    use embedded_graphics::pixelcolor::Rgb565;

    /// Create a professional grid system with both horizontal and vertical grids
    pub fn professional_grid() -> GridSystem<Rgb565> {
        GridBuilder::new()
            .horizontal_linear(GridSpacing::Auto)
            .vertical_linear(GridSpacing::Auto)
            .professional()
            .build()
    }

    /// Create a minimal grid system with major lines only
    pub fn minimal_grid() -> GridSystem<Rgb565> {
        GridBuilder::new()
            .horizontal_linear(GridSpacing::Auto)
            .vertical_linear(GridSpacing::Auto)
            .minimal()
            .build()
    }

    /// Create a dashed grid system
    pub fn dashed_grid() -> GridSystem<Rgb565> {
        GridBuilder::new()
            .horizontal_linear(GridSpacing::Auto)
            .vertical_linear(GridSpacing::Auto)
            .dashed()
            .build()
    }

    /// Create a tick-aligned grid system for f32 values
    pub fn tick_aligned_grid_f32() -> GridSystem<Rgb565> {
        GridBuilder::new()
            .horizontal_tick_based_f32()
            .vertical_tick_based_f32()
            .professional()
            .build()
    }

    /// Create a tick-aligned grid system for i32 values
    pub fn tick_aligned_grid_i32() -> GridSystem<Rgb565> {
        GridBuilder::new()
            .horizontal_tick_based_i32()
            .vertical_tick_based_i32()
            .professional()
            .build()
    }

    /// Create a horizontal-only grid
    pub fn horizontal_only_grid() -> GridSystem<Rgb565> {
        GridBuilder::new()
            .horizontal_linear(GridSpacing::Auto)
            .visibility(GridVisibility::horizontal_only())
            .professional()
            .build()
    }

    /// Create a vertical-only grid
    pub fn vertical_only_grid() -> GridSystem<Rgb565> {
        GridBuilder::new()
            .vertical_linear(GridSpacing::Auto)
            .visibility(GridVisibility::vertical_only())
            .professional()
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::pixelcolor::Rgb565;

    #[test]
    fn test_grid_builder() {
        let grid = GridBuilder::<Rgb565>::new()
            .horizontal_linear(GridSpacing::Pixels(20))
            .vertical_linear(GridSpacing::Pixels(30))
            .professional()
            .build();

        assert!(grid.is_enabled());
        assert!(grid.horizontal.is_some());
        assert!(grid.vertical.is_some());
    }

    #[test]
    fn test_linear_grid_builder() {
        let grid = LinearGridBuilder::<Rgb565>::horizontal()
            .spacing_pixels(25)
            .visible(true)
            .build();

        assert_eq!(grid.orientation(), GridOrientation::Horizontal);
        assert!(grid.is_visible());
    }

    #[test]
    fn test_tick_based_grid_builder() {
        let grid = TickBasedGridBuilder::<f32, Rgb565>::vertical()
            .major_ticks_only(true)
            .build();

        assert_eq!(grid.orientation(), GridOrientation::Vertical);
        assert!(grid.is_major_ticks_only());
    }

    #[test]
    fn test_custom_grid_builder() {
        let grid = CustomGridBuilder::<Rgb565>::horizontal()
            .add_line(100)
            .add_line(200)
            .add_line(300)
            .build();

        assert_eq!(grid.orientation(), GridOrientation::Horizontal);
        let positions = grid.calculate_positions(embedded_graphics::primitives::Rectangle::new(
            embedded_graphics::prelude::Point::zero(),
            embedded_graphics::prelude::Size::new(400, 300),
        ));
        assert_eq!(positions.len(), 3);
    }

    #[test]
    fn test_preset_grids() {
        let professional = presets::professional_grid();
        assert!(professional.is_enabled());

        let minimal = presets::minimal_grid();
        assert!(minimal.is_enabled());

        let dashed = presets::dashed_grid();
        assert!(dashed.is_enabled());
    }
}
