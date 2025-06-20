//! Layout strategies for dashboard arrangement

use super::{GridLayout, GridPosition, MAX_DASHBOARD_CHARTS};
use crate::error::ChartResult;
use embedded_graphics::primitives::Rectangle;
use heapless::Vec;

/// Layout strategy for arranging charts in a dashboard
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DashboardLayout {
    /// Grid-based layout with fixed rows and columns
    Grid(GridLayout),
    /// Flexible layout (future implementation)
    Flexible,
}

impl DashboardLayout {
    /// Calculate viewports for all panels based on the layout strategy
    pub fn calculate_viewports(
        &self,
        total_viewport: Rectangle,
        panel_count: usize,
        spacing: u32,
    ) -> ChartResult<Vec<Rectangle, MAX_DASHBOARD_CHARTS>> {
        match self {
            DashboardLayout::Grid(grid) => {
                // For now, auto-arrange panels in grid order
                let mut positions: Vec<GridPosition, MAX_DASHBOARD_CHARTS> = Vec::new();
                let mut index = 0;

                'outer: for row in 0..grid.rows {
                    for col in 0..grid.cols {
                        if index >= panel_count {
                            break 'outer;
                        }
                        positions
                            .push(GridPosition::new(row, col))
                            .map_err(|_| crate::error::ChartError::MemoryFull)?;
                        index += 1;
                    }
                }

                grid.calculate_viewports(total_viewport, &positions, spacing)
            }
            DashboardLayout::Flexible => {
                // Future implementation
                Err(crate::error::ChartError::ConfigurationError)
            }
        }
    }
}

/// Layout presets for common dashboard configurations
pub enum LayoutPreset {
    /// Single chart (1x1)
    Single,
    /// Side by side (1x2)
    SideBySide,
    /// Stacked vertically (2x1)
    Stacked,
    /// Four quadrants (2x2)
    Quadrants,
    /// Three columns (1x3)
    ThreeColumns,
    /// Three rows (3x1)
    ThreeRows,
    /// 3x3 grid
    Grid3x3,
    /// 4x4 grid
    Grid4x4,
}

impl LayoutPreset {
    /// Convert preset to dashboard layout
    pub fn to_layout(self) -> DashboardLayout {
        match self {
            LayoutPreset::Single => DashboardLayout::Grid(GridLayout::new(1, 1)),
            LayoutPreset::SideBySide => DashboardLayout::Grid(GridLayout::new(1, 2)),
            LayoutPreset::Stacked => DashboardLayout::Grid(GridLayout::new(2, 1)),
            LayoutPreset::Quadrants => DashboardLayout::Grid(GridLayout::new(2, 2)),
            LayoutPreset::ThreeColumns => DashboardLayout::Grid(GridLayout::new(1, 3)),
            LayoutPreset::ThreeRows => DashboardLayout::Grid(GridLayout::new(3, 1)),
            LayoutPreset::Grid3x3 => DashboardLayout::Grid(GridLayout::new(3, 3)),
            LayoutPreset::Grid4x4 => DashboardLayout::Grid(GridLayout::new(4, 4)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::prelude::*;

    #[test]
    fn test_layout_preset_conversion() {
        let layout = LayoutPreset::Quadrants.to_layout();
        match layout {
            DashboardLayout::Grid(grid) => {
                assert_eq!(grid.rows, 2);
                assert_eq!(grid.cols, 2);
            }
            _ => panic!("Expected grid layout"),
        }
    }

    #[test]
    fn test_dashboard_layout_calculate_viewports() {
        let layout = DashboardLayout::Grid(GridLayout::new(2, 2));
        let viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 200));

        let viewports = layout.calculate_viewports(viewport, 3, 10).unwrap();
        assert_eq!(viewports.len(), 3);

        // First viewport should be top-left
        assert_eq!(viewports[0].top_left, Point::new(0, 0));
        assert_eq!(viewports[0].size, Size::new(95, 95));

        // Second viewport should be top-right
        assert_eq!(viewports[1].top_left, Point::new(105, 0));
        assert_eq!(viewports[1].size, Size::new(95, 95));

        // Third viewport should be bottom-left
        assert_eq!(viewports[2].top_left, Point::new(0, 105));
        assert_eq!(viewports[2].size, Size::new(95, 95));
    }
}
