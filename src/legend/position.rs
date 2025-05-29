//! Legend positioning and layout calculation.

use crate::error::ChartResult;
use embedded_graphics::{prelude::*, primitives::Rectangle};

/// Legend position options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegendPosition {
    /// Top of the chart
    Top,
    /// Bottom of the chart
    Bottom,
    /// Left side of the chart
    Left,
    /// Right side of the chart
    Right,
    /// Top-left corner
    TopLeft,
    /// Top-right corner
    TopRight,
    /// Bottom-left corner
    BottomLeft,
    /// Bottom-right corner
    BottomRight,
    /// Custom position with specific coordinates
    Custom(Point),
    /// Floating position (overlays the chart)
    Floating(Point),
}

/// Legend alignment within its position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegendAlignment {
    /// Start alignment (left for horizontal, top for vertical)
    Start,
    /// Center alignment
    Center,
    /// End alignment (right for horizontal, bottom for vertical)
    End,
}

/// Margins around the legend
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegendMargins {
    /// Top margin
    pub top: u32,
    /// Right margin
    pub right: u32,
    /// Bottom margin
    pub bottom: u32,
    /// Left margin
    pub left: u32,
}

/// Position calculator for legend layout
#[derive(Debug, Clone)]
pub struct PositionCalculator {
    /// Chart area (total available space)
    chart_area: Rectangle,
    /// Plot area (area for actual chart content)
    plot_area: Rectangle,
    /// Legend margins
    margins: LegendMargins,
    /// Legend alignment
    alignment: LegendAlignment,
}

impl PositionCalculator {
    /// Create a new position calculator
    pub fn new(chart_area: Rectangle, plot_area: Rectangle) -> Self {
        Self {
            chart_area,
            plot_area,
            margins: LegendMargins::default(),
            alignment: LegendAlignment::Start,
        }
    }

    /// Set legend margins
    pub fn with_margins(mut self, margins: LegendMargins) -> Self {
        self.margins = margins;
        self
    }

    /// Set legend alignment
    pub fn with_alignment(mut self, alignment: LegendAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Calculate the legend rectangle for a given position and size
    pub fn calculate_legend_rect(
        &self,
        position: LegendPosition,
        legend_size: Size,
    ) -> ChartResult<Rectangle> {
        match position {
            LegendPosition::Top => self.calculate_top_position(legend_size),
            LegendPosition::Bottom => self.calculate_bottom_position(legend_size),
            LegendPosition::Left => self.calculate_left_position(legend_size),
            LegendPosition::Right => self.calculate_right_position(legend_size),
            LegendPosition::TopLeft => self.calculate_corner_position(legend_size, true, true),
            LegendPosition::TopRight => self.calculate_corner_position(legend_size, true, false),
            LegendPosition::BottomLeft => self.calculate_corner_position(legend_size, false, true),
            LegendPosition::BottomRight => {
                self.calculate_corner_position(legend_size, false, false)
            }
            LegendPosition::Custom(point) => Ok(Rectangle::new(point, legend_size)),
            LegendPosition::Floating(point) => Ok(Rectangle::new(point, legend_size)),
        }
    }

    /// Calculate the adjusted plot area when legend is positioned outside the chart
    pub fn calculate_adjusted_plot_area(
        &self,
        position: LegendPosition,
        legend_size: Size,
    ) -> ChartResult<Rectangle> {
        match position {
            LegendPosition::Top => {
                let height_reduction = legend_size.height + self.margins.vertical();
                Ok(Rectangle::new(
                    Point::new(
                        self.plot_area.top_left.x,
                        self.plot_area.top_left.y + height_reduction as i32,
                    ),
                    Size::new(
                        self.plot_area.size.width,
                        self.plot_area.size.height.saturating_sub(height_reduction),
                    ),
                ))
            }
            LegendPosition::Bottom => {
                let height_reduction = legend_size.height + self.margins.vertical();
                Ok(Rectangle::new(
                    self.plot_area.top_left,
                    Size::new(
                        self.plot_area.size.width,
                        self.plot_area.size.height.saturating_sub(height_reduction),
                    ),
                ))
            }
            LegendPosition::Left => {
                let width_reduction = legend_size.width + self.margins.horizontal();
                Ok(Rectangle::new(
                    Point::new(
                        self.plot_area.top_left.x + width_reduction as i32,
                        self.plot_area.top_left.y,
                    ),
                    Size::new(
                        self.plot_area.size.width.saturating_sub(width_reduction),
                        self.plot_area.size.height,
                    ),
                ))
            }
            LegendPosition::Right => {
                let width_reduction = legend_size.width + self.margins.horizontal();
                Ok(Rectangle::new(
                    self.plot_area.top_left,
                    Size::new(
                        self.plot_area.size.width.saturating_sub(width_reduction),
                        self.plot_area.size.height,
                    ),
                ))
            }
            // Corner and floating positions don't affect plot area
            _ => Ok(self.plot_area),
        }
    }

    /// Check if the legend fits within the available space
    pub fn validate_legend_fit(
        &self,
        position: LegendPosition,
        legend_size: Size,
    ) -> ChartResult<bool> {
        let legend_rect = self.calculate_legend_rect(position, legend_size)?;

        // For legends positioned outside the plot area (like Right position),
        // we need to check if they fit within a reasonable extended area
        // For all positions, check if legend rectangle is within chart area
        let fits_horizontally = legend_rect.top_left.x >= self.chart_area.top_left.x
            && legend_rect.top_left.x + legend_size.width as i32
                <= self.chart_area.top_left.x + self.chart_area.size.width as i32;

        let fits_vertically = legend_rect.top_left.y >= self.chart_area.top_left.y
            && legend_rect.top_left.y + legend_size.height as i32
                <= self.chart_area.top_left.y + self.chart_area.size.height as i32;

        Ok(fits_horizontally && fits_vertically)
    }

    // Private helper methods

    fn calculate_top_position(&self, legend_size: Size) -> ChartResult<Rectangle> {
        let x = match self.alignment {
            LegendAlignment::Start => self.chart_area.top_left.x + self.margins.left as i32,
            LegendAlignment::Center => {
                self.chart_area.top_left.x
                    + (self.chart_area.size.width as i32 - legend_size.width as i32) / 2
            }
            LegendAlignment::End => {
                self.chart_area.top_left.x + self.chart_area.size.width as i32
                    - legend_size.width as i32
                    - self.margins.right as i32
            }
        };

        let y = self.chart_area.top_left.y + self.margins.top as i32;

        Ok(Rectangle::new(Point::new(x, y), legend_size))
    }

    fn calculate_bottom_position(&self, legend_size: Size) -> ChartResult<Rectangle> {
        let x = match self.alignment {
            LegendAlignment::Start => self.chart_area.top_left.x + self.margins.left as i32,
            LegendAlignment::Center => {
                self.chart_area.top_left.x
                    + (self.chart_area.size.width as i32 - legend_size.width as i32) / 2
            }
            LegendAlignment::End => {
                self.chart_area.top_left.x + self.chart_area.size.width as i32
                    - legend_size.width as i32
                    - self.margins.right as i32
            }
        };

        let y = self.chart_area.top_left.y + self.chart_area.size.height as i32
            - legend_size.height as i32
            - self.margins.bottom as i32;

        Ok(Rectangle::new(Point::new(x, y), legend_size))
    }

    fn calculate_left_position(&self, legend_size: Size) -> ChartResult<Rectangle> {
        let x = self.chart_area.top_left.x + self.margins.left as i32;

        let y = match self.alignment {
            LegendAlignment::Start => self.chart_area.top_left.y + self.margins.top as i32,
            LegendAlignment::Center => {
                self.chart_area.top_left.y
                    + (self.chart_area.size.height as i32 - legend_size.height as i32) / 2
            }
            LegendAlignment::End => {
                self.chart_area.top_left.y + self.chart_area.size.height as i32
                    - legend_size.height as i32
                    - self.margins.bottom as i32
            }
        };

        Ok(Rectangle::new(Point::new(x, y), legend_size))
    }

    fn calculate_right_position(&self, legend_size: Size) -> ChartResult<Rectangle> {
        // Position legend on the right side within the chart area bounds
        let x = self.chart_area.top_left.x + self.chart_area.size.width as i32
            - legend_size.width as i32
            - self.margins.right as i32;

        let y = match self.alignment {
            LegendAlignment::Start => self.chart_area.top_left.y + self.margins.top as i32,
            LegendAlignment::Center => {
                self.chart_area.top_left.y
                    + (self.chart_area.size.height as i32 - legend_size.height as i32) / 2
            }
            LegendAlignment::End => {
                self.chart_area.top_left.y + self.chart_area.size.height as i32
                    - legend_size.height as i32
                    - self.margins.bottom as i32
            }
        };

        Ok(Rectangle::new(Point::new(x, y), legend_size))
    }

    fn calculate_corner_position(
        &self,
        legend_size: Size,
        top: bool,
        left: bool,
    ) -> ChartResult<Rectangle> {
        let x = if left {
            self.chart_area.top_left.x + self.margins.left as i32
        } else {
            self.chart_area.top_left.x + self.chart_area.size.width as i32
                - legend_size.width as i32
                - self.margins.right as i32
        };

        let y = if top {
            self.chart_area.top_left.y + self.margins.top as i32
        } else {
            self.chart_area.top_left.y + self.chart_area.size.height as i32
                - legend_size.height as i32
                - self.margins.bottom as i32
        };

        Ok(Rectangle::new(Point::new(x, y), legend_size))
    }
}

impl LegendMargins {
    /// Create uniform margins
    pub const fn all(value: u32) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    /// Create symmetric margins
    pub const fn symmetric(horizontal: u32, vertical: u32) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    /// Create custom margins
    pub const fn new(top: u32, right: u32, bottom: u32, left: u32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    /// Get total horizontal margins
    pub const fn horizontal(&self) -> u32 {
        self.left + self.right
    }

    /// Get total vertical margins
    pub const fn vertical(&self) -> u32 {
        self.top + self.bottom
    }
}

impl Default for LegendPosition {
    fn default() -> Self {
        Self::Right
    }
}

impl Default for LegendAlignment {
    fn default() -> Self {
        Self::Start
    }
}

impl Default for LegendMargins {
    fn default() -> Self {
        Self::all(8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legend_margins() {
        let margins = LegendMargins::all(10);
        assert_eq!(margins.horizontal(), 20);
        assert_eq!(margins.vertical(), 20);

        let margins = LegendMargins::symmetric(5, 8);
        assert_eq!(margins.horizontal(), 10);
        assert_eq!(margins.vertical(), 16);
    }

    #[test]
    fn test_position_calculator() {
        let chart_area = Rectangle::new(Point::zero(), Size::new(200, 150));
        let plot_area = Rectangle::new(Point::new(20, 20), Size::new(160, 110));
        let calculator = PositionCalculator::new(chart_area, plot_area);

        let legend_size = Size::new(60, 40);

        // Test right position
        let legend_rect = calculator
            .calculate_legend_rect(LegendPosition::Right, legend_size)
            .unwrap();
        // Legend should be positioned within chart area, on the right side
        assert!(
            legend_rect.top_left.x + legend_size.width as i32
                <= chart_area.top_left.x + chart_area.size.width as i32
        );
        assert!(legend_rect.top_left.x >= chart_area.top_left.x);

        // Test that legend fits
        assert!(calculator
            .validate_legend_fit(LegendPosition::Right, legend_size)
            .unwrap());
    }

    #[test]
    fn test_adjusted_plot_area() {
        let chart_area = Rectangle::new(Point::zero(), Size::new(200, 150));
        let plot_area = Rectangle::new(Point::new(20, 20), Size::new(160, 110));
        let calculator = PositionCalculator::new(chart_area, plot_area);

        let legend_size = Size::new(60, 40);

        // Test right position adjustment
        let adjusted = calculator
            .calculate_adjusted_plot_area(LegendPosition::Right, legend_size)
            .unwrap();
        assert!(adjusted.size.width < plot_area.size.width);

        // Test bottom position adjustment
        let adjusted = calculator
            .calculate_adjusted_plot_area(LegendPosition::Bottom, legend_size)
            .unwrap();
        assert!(adjusted.size.height < plot_area.size.height);
    }
}
