//! Simplified dashboard implementation without type erasure

use super::{GridLayout, GridPosition};
use crate::error::ChartResult;
use embedded_graphics::primitives::Rectangle;
use heapless::Vec;

/// Maximum number of charts in a dashboard
pub const MAX_DASHBOARD_CHARTS: usize = 16;

/// A simple dashboard that manages viewport layout
pub struct SimpleDashboard {
    /// Grid layout configuration
    grid: GridLayout,
    /// Spacing between panels
    spacing: u32,
}

impl SimpleDashboard {
    /// Create a new dashboard
    pub fn new(rows: u8, cols: u8, spacing: u32) -> Self {
        Self {
            grid: GridLayout::new(rows, cols),
            spacing,
        }
    }

    /// Calculate viewport for a specific grid position
    pub fn get_viewport(&self, position: GridPosition, total_viewport: Rectangle) -> Rectangle {
        self.grid.calculate_cell_viewport(total_viewport, position, self.spacing)
    }

    /// Calculate all viewports for a given number of panels
    pub fn get_all_viewports<const N: usize>(
        &self,
        total_viewport: Rectangle,
        panel_count: usize,
    ) -> ChartResult<Vec<Rectangle, N>> {
        let mut positions: Vec<GridPosition, N> = Vec::new();
        let mut index = 0;

        'outer: for row in 0..self.grid.rows {
            for col in 0..self.grid.cols {
                if index >= panel_count {
                    break 'outer;
                }
                positions.push(GridPosition::new(row, col))
                    .map_err(|_| crate::error::ChartError::MemoryFull)?;
                index += 1;
            }
        }

        self.grid.calculate_viewports(total_viewport, &positions, self.spacing)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::prelude::*;

    #[test]
    fn test_simple_dashboard() {
        let dashboard = SimpleDashboard::new(2, 2, 10);
        let total_viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 200));
        
        // Test single viewport
        let viewport = dashboard.get_viewport(GridPosition::new(0, 0), total_viewport);
        assert_eq!(viewport.top_left, Point::new(0, 0));
        assert_eq!(viewport.size, Size::new(95, 95));
        
        // Test all viewports
        let viewports: Vec<Rectangle, 4> = dashboard.get_all_viewports(total_viewport, 3).unwrap();
        assert_eq!(viewports.len(), 3);
    }
}