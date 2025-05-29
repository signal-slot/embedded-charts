//! Grid system for embedded graphics charts.
//!
//! This module provides a comprehensive grid system that integrates with the chart
//! and axis systems to provide professional-looking grid lines for data visualization.

pub mod builder;
pub mod style;
pub mod traits;
pub mod types;

#[cfg(all(feature = "no_std", not(feature = "std")))]
extern crate alloc;

#[cfg(all(feature = "no_std", not(feature = "std")))]
use alloc::boxed::Box;

#[cfg(not(all(feature = "no_std", not(feature = "std"))))]
use std::boxed::Box;

// Re-export main types
pub use builder::{CustomGridBuilder, GridBuilder, LinearGridBuilder, TickBasedGridBuilder};
pub use style::{GridLineStyle, GridStyle, GridVisibility, MajorGridStyle, MinorGridStyle};
pub use traits::{DefaultGridRenderer, Grid, GridConfiguration, GridOrientation, GridRenderer};
pub use types::{CustomGrid, GridSpacing, GridType, LinearGrid, TickBasedGrid};

pub use traits::TickAlignedGrid;

use crate::axes::traits::TickGenerator;
use crate::error::{ChartError, ChartResult};
use embedded_graphics::{
    prelude::*,
    primitives::{Line, PrimitiveStyle, Rectangle},
};

/// Main grid renderer that coordinates different grid types
#[derive(Debug)]
pub struct GridSystem<C: PixelColor> {
    /// Horizontal grid configuration
    pub horizontal: Option<GridContainer<C>>,
    /// Vertical grid configuration
    pub vertical: Option<GridContainer<C>>,
    /// Overall grid style
    pub style: GridStyle<C>,
    /// Whether the grid is enabled
    pub enabled: bool,
}

/// Container for different grid types
#[derive(Debug)]
pub enum GridContainer<C: PixelColor> {
    /// Linear grid
    Linear(LinearGrid<C>),
    /// Tick-based grid for f32 values
    TickBasedF32(TickBasedGrid<f32, C>),
    /// Tick-based grid for i32 values
    TickBasedI32(TickBasedGrid<i32, C>),
    /// Custom grid
    Custom(Box<CustomGrid<C>>),
}

impl<C: PixelColor + 'static> GridContainer<C> {
    /// Draw the grid
    pub fn draw<D>(&self, viewport: Rectangle, target: &mut D) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        match self {
            GridContainer::Linear(grid) => grid.draw(viewport, target),
            GridContainer::TickBasedF32(grid) => grid.draw(viewport, target),
            GridContainer::TickBasedI32(grid) => grid.draw(viewport, target),
            GridContainer::Custom(grid) => grid.draw(viewport, target),
        }
    }

    /// Get grid orientation
    pub fn orientation(&self) -> traits::GridOrientation {
        match self {
            GridContainer::Linear(grid) => grid.orientation(),
            GridContainer::TickBasedF32(grid) => grid.orientation(),
            GridContainer::TickBasedI32(grid) => grid.orientation(),
            GridContainer::Custom(grid) => grid.orientation(),
        }
    }

    /// Check if grid is visible
    pub fn is_visible(&self) -> bool {
        match self {
            GridContainer::Linear(grid) => grid.is_visible(),
            GridContainer::TickBasedF32(grid) => grid.is_visible(),
            GridContainer::TickBasedI32(grid) => grid.is_visible(),
            GridContainer::Custom(grid) => grid.is_visible(),
        }
    }
}

impl<C: PixelColor + 'static> GridSystem<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new empty grid system
    pub fn new() -> Self {
        Self {
            horizontal: None,
            vertical: None,
            style: GridStyle::default(),
            enabled: true,
        }
    }

    /// Create a builder for configuring the grid system
    pub fn builder() -> GridBuilder<C> {
        GridBuilder::new()
    }

    /// Set the horizontal grid
    pub fn set_horizontal_grid(&mut self, grid: GridContainer<C>) {
        self.horizontal = Some(grid);
    }

    /// Set the vertical grid
    pub fn set_vertical_grid(&mut self, grid: GridContainer<C>) {
        self.vertical = Some(grid);
    }

    /// Enable or disable the grid
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if the grid is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Draw the grid to the target
    pub fn draw<D>(&self, viewport: Rectangle, target: &mut D) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        if !self.enabled {
            return Ok(());
        }

        // Draw horizontal grid lines
        if let Some(ref horizontal_grid) = self.horizontal {
            horizontal_grid.draw(viewport, target)?;
        }

        // Draw vertical grid lines
        if let Some(ref vertical_grid) = self.vertical {
            vertical_grid.draw(viewport, target)?;
        }

        Ok(())
    }

    /// Draw grid lines that align with axis ticks
    pub fn draw_with_axes<T, D, XA, YA>(
        &self,
        viewport: Rectangle,
        x_axis: Option<&XA>,
        y_axis: Option<&YA>,
        target: &mut D,
    ) -> ChartResult<()>
    where
        T: crate::axes::traits::AxisValue,
        D: DrawTarget<Color = C>,
        XA: crate::axes::traits::Axis<T, C>,
        YA: crate::axes::traits::Axis<T, C>,
    {
        if !self.enabled {
            return Ok(());
        }

        // Draw grid lines aligned with axis ticks
        if let Some(x_axis) = x_axis {
            // Draw vertical grid lines at X-axis tick positions
            let ticks = TickGenerator::generate_ticks(
                x_axis.tick_generator(),
                x_axis.min(),
                x_axis.max(),
                10, // max ticks
            );

            for tick in &ticks {
                let x_pos = x_axis.transform_value(tick.value, viewport);
                if x_pos >= viewport.top_left.x
                    && x_pos <= viewport.top_left.x + viewport.size.width as i32
                {
                    let start = Point::new(x_pos, viewport.top_left.y);
                    let end = Point::new(x_pos, viewport.top_left.y + viewport.size.height as i32);

                    Line::new(start, end)
                        .into_styled(PrimitiveStyle::with_stroke(
                            self.style.major.line.line_style.color,
                            self.style.major.line.line_style.width,
                        ))
                        .draw(target)
                        .map_err(|_| ChartError::RenderingError)?;
                }
            }
        }

        if let Some(y_axis) = y_axis {
            // Draw horizontal grid lines at Y-axis tick positions
            let ticks = TickGenerator::generate_ticks(
                y_axis.tick_generator(),
                y_axis.min(),
                y_axis.max(),
                10, // max ticks
            );

            for tick in &ticks {
                let y_pos = y_axis.transform_value(tick.value, viewport);
                if y_pos >= viewport.top_left.y
                    && y_pos <= viewport.top_left.y + viewport.size.height as i32
                {
                    let start = Point::new(viewport.top_left.x, y_pos);
                    let end = Point::new(viewport.top_left.x + viewport.size.width as i32, y_pos);

                    Line::new(start, end)
                        .into_styled(PrimitiveStyle::with_stroke(
                            self.style.major.line.line_style.color,
                            self.style.major.line.line_style.width,
                        ))
                        .draw(target)
                        .map_err(|_| ChartError::RenderingError)?;
                }
            }
        }

        Ok(())
    }
}

impl<C: PixelColor + 'static> Default for GridSystem<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::pixelcolor::Rgb565;

    #[test]
    fn test_grid_system_creation() {
        let grid: GridSystem<Rgb565> = GridSystem::new();
        assert!(grid.is_enabled());
        assert!(grid.horizontal.is_none());
        assert!(grid.vertical.is_none());
    }

    #[test]
    fn test_grid_system_enable_disable() {
        let mut grid: GridSystem<Rgb565> = GridSystem::new();
        assert!(grid.is_enabled());

        grid.set_enabled(false);
        assert!(!grid.is_enabled());

        grid.set_enabled(true);
        assert!(grid.is_enabled());
    }
}
