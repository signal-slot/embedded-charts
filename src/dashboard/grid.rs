//! Grid-based layout system for dashboards

use embedded_graphics::{prelude::*, primitives::Rectangle};
use heapless::Vec;

/// Position in a grid layout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridPosition {
    /// Row index (0-based)
    pub row: u8,
    /// Column index (0-based)
    pub col: u8,
    /// Number of rows this panel spans
    pub row_span: u8,
    /// Number of columns this panel spans  
    pub col_span: u8,
}

impl GridPosition {
    /// Create a new grid position with single cell
    pub fn new(row: u8, col: u8) -> Self {
        Self {
            row,
            col,
            row_span: 1,
            col_span: 1,
        }
    }

    /// Create a position that spans multiple cells
    pub fn with_span(row: u8, col: u8, row_span: u8, col_span: u8) -> Self {
        Self {
            row,
            col,
            row_span: row_span.max(1),
            col_span: col_span.max(1),
        }
    }
}

/// Grid-based layout for arranging charts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridLayout {
    /// Number of rows in the grid
    pub rows: u8,
    /// Number of columns in the grid
    pub cols: u8,
}

impl GridLayout {
    /// Create a new grid layout
    pub fn new(rows: u8, cols: u8) -> Self {
        Self {
            rows: rows.max(1),
            cols: cols.max(1),
        }
    }

    /// Calculate the viewport for a specific grid position
    pub fn calculate_cell_viewport(
        &self,
        total_viewport: Rectangle,
        position: GridPosition,
        spacing: u32,
    ) -> Rectangle {
        let total_width = total_viewport.size.width;
        let total_height = total_viewport.size.height;

        // Calculate spacing requirements
        let h_spacing = spacing * (self.cols as u32 - 1);
        let v_spacing = spacing * (self.rows as u32 - 1);

        // Calculate cell dimensions
        let cell_width = (total_width.saturating_sub(h_spacing)) / self.cols as u32;
        let cell_height = (total_height.saturating_sub(v_spacing)) / self.rows as u32;

        // Calculate position
        let x = total_viewport.top_left.x + 
            (position.col as i32 * (cell_width as i32 + spacing as i32));
        let y = total_viewport.top_left.y + 
            (position.row as i32 * (cell_height as i32 + spacing as i32));

        // Calculate size with span
        let width = (cell_width * position.col_span as u32) + 
            (spacing * position.col_span.saturating_sub(1) as u32);
        let height = (cell_height * position.row_span as u32) + 
            (spacing * position.row_span.saturating_sub(1) as u32);

        Rectangle::new(
            Point::new(x, y),
            Size::new(width, height),
        )
    }

    /// Calculate viewports for all panels in order
    pub fn calculate_viewports<const N: usize>(
        &self,
        total_viewport: Rectangle,
        positions: &[GridPosition],
        spacing: u32,
    ) -> crate::error::ChartResult<Vec<Rectangle, N>> {
        let mut viewports = Vec::new();

        for position in positions {
            let viewport = self.calculate_cell_viewport(total_viewport, *position, spacing);
            viewports.push(viewport)
                .map_err(|_| crate::error::ChartError::MemoryFull)?;
        }

        Ok(viewports)
    }
}

impl Default for GridLayout {
    fn default() -> Self {
        Self::new(2, 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_position() {
        let pos = GridPosition::new(1, 2);
        assert_eq!(pos.row, 1);
        assert_eq!(pos.col, 2);
        assert_eq!(pos.row_span, 1);
        assert_eq!(pos.col_span, 1);

        let span_pos = GridPosition::with_span(0, 0, 2, 3);
        assert_eq!(span_pos.row_span, 2);
        assert_eq!(span_pos.col_span, 3);
    }

    #[test]
    fn test_grid_layout_2x2() {
        let layout = GridLayout::new(2, 2);
        let viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 200));
        
        // Test top-left cell
        let cell = layout.calculate_cell_viewport(
            viewport,
            GridPosition::new(0, 0),
            10, // spacing
        );
        assert_eq!(cell.top_left, Point::new(0, 0));
        assert_eq!(cell.size, Size::new(95, 95)); // (200 - 10) / 2 = 95

        // Test bottom-right cell
        let cell = layout.calculate_cell_viewport(
            viewport,
            GridPosition::new(1, 1),
            10,
        );
        assert_eq!(cell.top_left, Point::new(105, 105)); // 95 + 10 = 105
        assert_eq!(cell.size, Size::new(95, 95));
    }

    #[test]
    fn test_grid_layout_with_span() {
        let layout = GridLayout::new(3, 3);
        let viewport = Rectangle::new(Point::new(0, 0), Size::new(320, 320));
        
        // Test 2x2 span starting at (0,0)
        let cell = layout.calculate_cell_viewport(
            viewport,
            GridPosition::with_span(0, 0, 2, 2),
            10,
        );
        assert_eq!(cell.top_left, Point::new(0, 0));
        assert_eq!(cell.size, Size::new(210, 210)); // 2*100 + 10 = 210
    }
}