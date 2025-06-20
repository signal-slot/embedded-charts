//! Dashboard layout system for composing multiple charts
//!
//! This module provides a flexible layout system for arranging multiple charts
//! on a single display, perfect for creating comprehensive dashboards in embedded
//! systems.
//!
//! ## Example
//!
//! ```rust,no_run
//! use embedded_charts::dashboard::{SimpleDashboard, GridPosition};
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//! use embedded_graphics::primitives::Rectangle;
//!
//! // Create a 2x2 dashboard
//! let dashboard = SimpleDashboard::new(2, 2, 10);
//! let total_viewport = Rectangle::new(Point::new(0, 0), Size::new(400, 300));
//!
//! // Get viewport for top-left position
//! let chart1_viewport = dashboard.get_viewport(
//!     GridPosition::new(0, 0),
//!     total_viewport
//! );
//!
//! // Draw your chart in this viewport
//! // chart.draw(data, config, chart1_viewport, &mut display)?;
//! ```

mod grid;
mod layout;
mod simple;

pub use grid::{GridLayout, GridPosition};
pub use layout::{DashboardLayout, LayoutPreset};
pub use simple::{SimpleDashboard, MAX_DASHBOARD_CHARTS};

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::prelude::*;
    use embedded_graphics::primitives::Rectangle;

    #[test]
    fn test_simple_dashboard_creation() {
        let dashboard = SimpleDashboard::new(2, 2, 10);
        let viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 200));
        
        // Test getting individual viewport
        let pos = GridPosition::new(0, 0);
        let chart_viewport = dashboard.get_viewport(pos, viewport);
        
        assert_eq!(chart_viewport.top_left, Point::new(0, 0));
        assert_eq!(chart_viewport.size.width, 95); // (200 - 10) / 2
    }

    #[test]
    fn test_layout_presets() {
        let preset = LayoutPreset::Quadrants;
        let layout = preset.to_layout();
        
        match layout {
            DashboardLayout::Grid(grid) => {
                assert_eq!(grid.rows, 2);
                assert_eq!(grid.cols, 2);
            }
            _ => panic!("Expected grid layout"),
        }
    }
}