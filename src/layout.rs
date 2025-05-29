//! Layout management for chart components.

use crate::chart::traits::Margins;
use crate::error::{LayoutError, LayoutResult};
use embedded_graphics::{prelude::*, primitives::Rectangle};

/// Layout manager for chart components
#[derive(Debug, Clone)]
pub struct ChartLayout {
    /// Total available area
    pub total_area: Rectangle,
    /// Chart drawing area (after margins)
    pub chart_area: Rectangle,
    /// Title area
    pub title_area: Option<Rectangle>,
    /// Legend area
    pub legend_area: Option<Rectangle>,
    /// X-axis area
    pub x_axis_area: Option<Rectangle>,
    /// Y-axis area
    pub y_axis_area: Option<Rectangle>,
}

impl ChartLayout {
    /// Create a new chart layout
    pub fn new(total_area: Rectangle) -> Self {
        Self {
            total_area,
            chart_area: total_area,
            title_area: None,
            legend_area: None,
            x_axis_area: None,
            y_axis_area: None,
        }
    }

    /// Apply margins to the layout
    pub fn with_margins(mut self, margins: Margins) -> Self {
        self.chart_area = margins.apply_to(self.total_area);
        self
    }

    /// Reserve space for a title at the top
    pub fn with_title(mut self, height: u32) -> LayoutResult<Self> {
        if height >= self.chart_area.size.height {
            return Err(LayoutError::InsufficientSpace);
        }

        self.title_area = Some(Rectangle::new(
            self.chart_area.top_left,
            Size::new(self.chart_area.size.width, height),
        ));

        // Adjust chart area
        self.chart_area = Rectangle::new(
            Point::new(
                self.chart_area.top_left.x,
                self.chart_area.top_left.y + height as i32,
            ),
            Size::new(
                self.chart_area.size.width,
                self.chart_area.size.height - height,
            ),
        );

        Ok(self)
    }

    /// Reserve space for a legend
    pub fn with_legend(mut self, position: LegendPosition, size: Size) -> LayoutResult<Self> {
        match position {
            LegendPosition::Right => {
                if size.width >= self.chart_area.size.width {
                    return Err(LayoutError::InsufficientSpace);
                }

                self.legend_area = Some(Rectangle::new(
                    Point::new(
                        self.chart_area.top_left.x + self.chart_area.size.width as i32
                            - size.width as i32,
                        self.chart_area.top_left.y,
                    ),
                    size,
                ));

                // Adjust chart area
                self.chart_area = Rectangle::new(
                    self.chart_area.top_left,
                    Size::new(
                        self.chart_area.size.width - size.width,
                        self.chart_area.size.height,
                    ),
                );
            }
            LegendPosition::Bottom => {
                if size.height >= self.chart_area.size.height {
                    return Err(LayoutError::InsufficientSpace);
                }

                self.legend_area = Some(Rectangle::new(
                    Point::new(
                        self.chart_area.top_left.x,
                        self.chart_area.top_left.y + self.chart_area.size.height as i32
                            - size.height as i32,
                    ),
                    size,
                ));

                // Adjust chart area
                self.chart_area = Rectangle::new(
                    self.chart_area.top_left,
                    Size::new(
                        self.chart_area.size.width,
                        self.chart_area.size.height - size.height,
                    ),
                );
            }
            LegendPosition::Top => {
                if size.height >= self.chart_area.size.height {
                    return Err(LayoutError::InsufficientSpace);
                }

                self.legend_area = Some(Rectangle::new(self.chart_area.top_left, size));

                // Adjust chart area
                self.chart_area = Rectangle::new(
                    Point::new(
                        self.chart_area.top_left.x,
                        self.chart_area.top_left.y + size.height as i32,
                    ),
                    Size::new(
                        self.chart_area.size.width,
                        self.chart_area.size.height - size.height,
                    ),
                );
            }
            LegendPosition::Left => {
                if size.width >= self.chart_area.size.width {
                    return Err(LayoutError::InsufficientSpace);
                }

                self.legend_area = Some(Rectangle::new(self.chart_area.top_left, size));

                // Adjust chart area
                self.chart_area = Rectangle::new(
                    Point::new(
                        self.chart_area.top_left.x + size.width as i32,
                        self.chart_area.top_left.y,
                    ),
                    Size::new(
                        self.chart_area.size.width - size.width,
                        self.chart_area.size.height,
                    ),
                );
            }
        }

        Ok(self)
    }

    /// Reserve space for X-axis
    pub fn with_x_axis(mut self, height: u32) -> LayoutResult<Self> {
        if height >= self.chart_area.size.height {
            return Err(LayoutError::InsufficientSpace);
        }

        self.x_axis_area = Some(Rectangle::new(
            Point::new(
                self.chart_area.top_left.x,
                self.chart_area.top_left.y + self.chart_area.size.height as i32 - height as i32,
            ),
            Size::new(self.chart_area.size.width, height),
        ));

        // Adjust chart area
        self.chart_area = Rectangle::new(
            self.chart_area.top_left,
            Size::new(
                self.chart_area.size.width,
                self.chart_area.size.height - height,
            ),
        );

        Ok(self)
    }

    /// Reserve space for Y-axis
    pub fn with_y_axis(mut self, width: u32) -> LayoutResult<Self> {
        if width >= self.chart_area.size.width {
            return Err(LayoutError::InsufficientSpace);
        }

        self.y_axis_area = Some(Rectangle::new(
            self.chart_area.top_left,
            Size::new(width, self.chart_area.size.height),
        ));

        // Adjust chart area
        self.chart_area = Rectangle::new(
            Point::new(
                self.chart_area.top_left.x + width as i32,
                self.chart_area.top_left.y,
            ),
            Size::new(
                self.chart_area.size.width - width,
                self.chart_area.size.height,
            ),
        );

        Ok(self)
    }

    /// Get the final chart drawing area
    pub fn chart_area(&self) -> Rectangle {
        self.chart_area
    }

    /// Validate that the layout has sufficient space
    pub fn validate(&self) -> LayoutResult<()> {
        if self.chart_area.size.width < 10 || self.chart_area.size.height < 10 {
            return Err(LayoutError::InsufficientSpace);
        }
        Ok(())
    }
}

/// Legend position options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegendPosition {
    /// Legend on the top
    Top,
    /// Legend on the right
    Right,
    /// Legend on the bottom
    Bottom,
    /// Legend on the left
    Left,
}

/// Viewport management for chart rendering
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    /// The visible area
    pub area: Rectangle,
    /// Zoom level (1.0 = normal, >1.0 = zoomed in)
    pub zoom: f32,
    /// Pan offset
    pub offset: Point,
}

impl Viewport {
    /// Create a new viewport
    pub fn new(area: Rectangle) -> Self {
        Self {
            area,
            zoom: 1.0,
            offset: Point::zero(),
        }
    }

    /// Set the zoom level
    pub fn with_zoom(mut self, zoom: f32) -> Self {
        self.zoom = zoom.clamp(0.1, 10.0); // Clamp zoom to reasonable range
        self
    }

    /// Set the pan offset
    pub fn with_offset(mut self, offset: Point) -> Self {
        self.offset = offset;
        self
    }

    /// Transform a point from data coordinates to screen coordinates
    pub fn transform_point(&self, data_point: Point, data_bounds: Rectangle) -> Point {
        // Normalize to 0-1 range
        let norm_x = if data_bounds.size.width > 0 {
            (data_point.x - data_bounds.top_left.x) as f32 / data_bounds.size.width as f32
        } else {
            0.5
        };

        let norm_y = if data_bounds.size.height > 0 {
            (data_point.y - data_bounds.top_left.y) as f32 / data_bounds.size.height as f32
        } else {
            0.5
        };

        // Apply zoom and offset
        let zoomed_x = norm_x * self.zoom;
        let zoomed_y = norm_y * self.zoom;

        // Transform to screen coordinates
        let screen_x =
            self.area.top_left.x + (zoomed_x * self.area.size.width as f32) as i32 + self.offset.x;
        let screen_y =
            self.area.top_left.y + (zoomed_y * self.area.size.height as f32) as i32 + self.offset.y;

        Point::new(screen_x, screen_y)
    }

    /// Check if a point is visible in the viewport
    pub fn is_point_visible(&self, point: Point) -> bool {
        point.x >= self.area.top_left.x
            && point.x < self.area.top_left.x + self.area.size.width as i32
            && point.y >= self.area.top_left.y
            && point.y < self.area.top_left.y + self.area.size.height as i32
    }

    /// Get the visible data bounds for the current viewport
    pub fn visible_data_bounds(&self, full_data_bounds: Rectangle) -> Rectangle {
        // This is a simplified implementation
        // In a full implementation, you'd calculate the actual visible bounds based on zoom and offset
        full_data_bounds
    }
}

/// Component positioning utilities
pub struct ComponentPositioning;

impl ComponentPositioning {
    /// Center a component within a container
    pub fn center_in_container(component_size: Size, container: Rectangle) -> Point {
        let x =
            container.top_left.x + (container.size.width as i32 - component_size.width as i32) / 2;
        let y = container.top_left.y
            + (container.size.height as i32 - component_size.height as i32) / 2;
        Point::new(x, y)
    }

    /// Align a component to the top-left of a container
    pub fn align_top_left(container: Rectangle, margin: u32) -> Point {
        Point::new(
            container.top_left.x + margin as i32,
            container.top_left.y + margin as i32,
        )
    }

    /// Align a component to the top-right of a container
    pub fn align_top_right(component_size: Size, container: Rectangle, margin: u32) -> Point {
        Point::new(
            container.top_left.x + container.size.width as i32
                - component_size.width as i32
                - margin as i32,
            container.top_left.y + margin as i32,
        )
    }

    /// Align a component to the bottom-left of a container
    pub fn align_bottom_left(component_size: Size, container: Rectangle, margin: u32) -> Point {
        Point::new(
            container.top_left.x + margin as i32,
            container.top_left.y + container.size.height as i32
                - component_size.height as i32
                - margin as i32,
        )
    }

    /// Align a component to the bottom-right of a container
    pub fn align_bottom_right(component_size: Size, container: Rectangle, margin: u32) -> Point {
        Point::new(
            container.top_left.x + container.size.width as i32
                - component_size.width as i32
                - margin as i32,
            container.top_left.y + container.size.height as i32
                - component_size.height as i32
                - margin as i32,
        )
    }

    /// Distribute components evenly in a horizontal layout
    pub fn distribute_horizontal(
        component_sizes: &[Size],
        container: Rectangle,
        spacing: u32,
    ) -> LayoutResult<heapless::Vec<Point, 16>> {
        let mut positions = heapless::Vec::new();

        if component_sizes.is_empty() {
            return Ok(positions);
        }

        let total_width: u32 = component_sizes.iter().map(|s| s.width).sum();
        let total_spacing = spacing * (component_sizes.len() as u32).saturating_sub(1);

        if total_width + total_spacing > container.size.width {
            return Err(LayoutError::InsufficientSpace);
        }

        let start_x =
            container.top_left.x + (container.size.width - total_width - total_spacing) as i32 / 2;
        let mut current_x = start_x;

        for size in component_sizes {
            let y = container.top_left.y + (container.size.height as i32 - size.height as i32) / 2;
            positions
                .push(Point::new(current_x, y))
                .map_err(|_| LayoutError::InsufficientSpace)?;
            current_x += size.width as i32 + spacing as i32;
        }

        Ok(positions)
    }

    /// Distribute components evenly in a vertical layout
    pub fn distribute_vertical(
        component_sizes: &[Size],
        container: Rectangle,
        spacing: u32,
    ) -> LayoutResult<heapless::Vec<Point, 16>> {
        let mut positions = heapless::Vec::new();

        if component_sizes.is_empty() {
            return Ok(positions);
        }

        let total_height: u32 = component_sizes.iter().map(|s| s.height).sum();
        let total_spacing = spacing * (component_sizes.len() as u32).saturating_sub(1);

        if total_height + total_spacing > container.size.height {
            return Err(LayoutError::InsufficientSpace);
        }

        let start_y = container.top_left.y
            + (container.size.height - total_height - total_spacing) as i32 / 2;
        let mut current_y = start_y;

        for size in component_sizes {
            let x = container.top_left.x + (container.size.width as i32 - size.width as i32) / 2;
            positions
                .push(Point::new(x, current_y))
                .map_err(|_| LayoutError::InsufficientSpace)?;
            current_y += size.height as i32 + spacing as i32;
        }

        Ok(positions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chart_layout_creation() {
        let area = Rectangle::new(Point::zero(), Size::new(400, 300));
        let layout = ChartLayout::new(area);

        assert_eq!(layout.total_area, area);
        assert_eq!(layout.chart_area, area);
        assert!(layout.title_area.is_none());
    }

    #[test]
    fn test_layout_with_margins() {
        let area = Rectangle::new(Point::zero(), Size::new(400, 300));
        let margins = Margins::all(20);
        let layout = ChartLayout::new(area).with_margins(margins);

        assert_eq!(layout.chart_area.top_left, Point::new(20, 20));
        assert_eq!(layout.chart_area.size, Size::new(360, 260));
    }

    #[test]
    fn test_layout_with_title() {
        let area = Rectangle::new(Point::zero(), Size::new(400, 300));
        let layout = ChartLayout::new(area).with_title(30).unwrap();

        assert!(layout.title_area.is_some());
        let title_area = layout.title_area.unwrap();
        assert_eq!(title_area.size.height, 30);
        assert_eq!(layout.chart_area.size.height, 270);
    }

    #[test]
    fn test_viewport_creation() {
        let area = Rectangle::new(Point::zero(), Size::new(200, 150));
        let viewport = Viewport::new(area);

        assert_eq!(viewport.area, area);
        assert_eq!(viewport.zoom, 1.0);
        assert_eq!(viewport.offset, Point::zero());
    }

    #[test]
    fn test_viewport_with_zoom() {
        let area = Rectangle::new(Point::zero(), Size::new(200, 150));
        let viewport = Viewport::new(area).with_zoom(2.0);

        assert_eq!(viewport.zoom, 2.0);
    }

    #[test]
    fn test_component_positioning_center() {
        let container = Rectangle::new(Point::new(10, 10), Size::new(100, 80));
        let component_size = Size::new(20, 10);

        let position = ComponentPositioning::center_in_container(component_size, container);
        assert_eq!(position, Point::new(50, 45));
    }

    #[test]
    fn test_component_positioning_corners() {
        let container = Rectangle::new(Point::new(0, 0), Size::new(100, 80));
        let component_size = Size::new(20, 10);
        let margin = 5;

        let top_left = ComponentPositioning::align_top_left(container, margin);
        assert_eq!(top_left, Point::new(5, 5));

        let top_right = ComponentPositioning::align_top_right(component_size, container, margin);
        assert_eq!(top_right, Point::new(75, 5));

        let bottom_left =
            ComponentPositioning::align_bottom_left(component_size, container, margin);
        assert_eq!(bottom_left, Point::new(5, 65));

        let bottom_right =
            ComponentPositioning::align_bottom_right(component_size, container, margin);
        assert_eq!(bottom_right, Point::new(75, 65));
    }
}
