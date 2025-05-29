//! Core traits for grid implementations.

use crate::error::ChartResult;
use embedded_graphics::{prelude::*, primitives::Rectangle};

use crate::math::{Math, NumericConversion};

/// Core trait for all grid types
pub trait Grid<C: PixelColor> {
    /// Draw the grid lines to the target
    ///
    /// # Arguments
    /// * `viewport` - The area to draw the grid in
    /// * `target` - The display target to draw to
    fn draw<D>(&self, viewport: Rectangle, target: &mut D) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>;

    /// Get the grid orientation (horizontal or vertical)
    fn orientation(&self) -> GridOrientation;

    /// Check if the grid is visible
    fn is_visible(&self) -> bool;

    /// Set grid visibility
    fn set_visible(&mut self, visible: bool);

    /// Get the grid style
    fn style(&self) -> &crate::grid::style::GridStyle<C>;

    /// Set the grid style
    fn set_style(&mut self, style: crate::grid::style::GridStyle<C>);

    /// Calculate the positions where grid lines should be drawn
    /// Returns a vector of positions in screen coordinates
    fn calculate_positions(&self, viewport: Rectangle) -> heapless::Vec<i32, 64>;

    /// Get the spacing between grid lines
    fn spacing(&self) -> f32;

    /// Set the spacing between grid lines
    fn set_spacing(&mut self, spacing: f32);

    /// Support for downcasting to concrete types
    fn as_any(&self) -> &dyn core::any::Any;
}

/// Trait for rendering grid lines
pub trait GridRenderer<C: PixelColor> {
    /// Draw a major grid line
    ///
    /// # Arguments
    /// * `start` - Start point of the grid line
    /// * `end` - End point of the grid line
    /// * `style` - Line style to use
    /// * `target` - The display target to draw to
    fn draw_major_line<D>(
        &self,
        start: Point,
        end: Point,
        style: &crate::style::LineStyle<C>,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>;

    /// Draw a minor grid line
    ///
    /// # Arguments
    /// * `start` - Start point of the grid line
    /// * `end` - End point of the grid line
    /// * `style` - Line style to use
    /// * `target` - The display target to draw to
    fn draw_minor_line<D>(
        &self,
        start: Point,
        end: Point,
        style: &crate::style::LineStyle<C>,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>;

    /// Draw a grid line with custom style
    ///
    /// # Arguments
    /// * `start` - Start point of the grid line
    /// * `end` - End point of the grid line
    /// * `style` - Line style to use
    /// * `target` - The display target to draw to
    fn draw_grid_line<D>(
        &self,
        start: Point,
        end: Point,
        style: &crate::style::LineStyle<C>,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>;
}

/// Trait for configuring grid systems
pub trait GridConfiguration<C: PixelColor> {
    /// Configure major grid lines
    ///
    /// # Arguments
    /// * `enabled` - Whether major grid lines are enabled
    /// * `spacing` - Spacing between major grid lines
    /// * `style` - Style for major grid lines
    fn configure_major_grid(
        &mut self,
        enabled: bool,
        spacing: f32,
        style: crate::grid::style::MajorGridStyle<C>,
    );

    /// Configure minor grid lines
    ///
    /// # Arguments
    /// * `enabled` - Whether minor grid lines are enabled
    /// * `spacing` - Spacing between minor grid lines
    /// * `style` - Style for minor grid lines
    fn configure_minor_grid(
        &mut self,
        enabled: bool,
        spacing: f32,
        style: crate::grid::style::MinorGridStyle<C>,
    );

    /// Set the overall grid visibility
    ///
    /// # Arguments
    /// * `visible` - Whether the grid is visible
    fn set_grid_visible(&mut self, visible: bool);

    /// Get the current grid configuration
    fn grid_config(&self) -> &crate::grid::style::GridStyle<C>;
}

/// Trait for grids that align with axis ticks
pub trait TickAlignedGrid<T, C>: Grid<C>
where
    T: crate::axes::traits::AxisValue,
    C: PixelColor,
{
    /// Draw grid lines aligned with axis ticks
    ///
    /// # Arguments
    /// * `viewport` - The area to draw the grid in
    /// * `axis` - The axis to align with
    /// * `target` - The display target to draw to
    fn draw_with_axis<D, A>(
        &self,
        viewport: Rectangle,
        axis: &A,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
        A: crate::axes::traits::Axis<T, C>;

    /// Calculate grid positions based on axis ticks
    ///
    /// # Arguments
    /// * `viewport` - The viewport area
    /// * `axis` - The axis to get ticks from
    fn calculate_tick_positions<A>(&self, viewport: Rectangle, axis: &A) -> heapless::Vec<i32, 64>
    where
        A: crate::axes::traits::Axis<T, C>;

    /// Set whether to show grid lines for major ticks only
    ///
    /// # Arguments
    /// * `major_only` - If true, only show grid lines for major ticks
    fn set_major_ticks_only(&mut self, major_only: bool);

    /// Check if only major tick grid lines are shown
    fn is_major_ticks_only(&self) -> bool;
}

/// Grid orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridOrientation {
    /// Horizontal grid lines (parallel to X-axis)
    Horizontal,
    /// Vertical grid lines (parallel to Y-axis)
    Vertical,
}

/// Default grid renderer implementation
#[derive(Debug, Clone)]
pub struct DefaultGridRenderer;

impl<C: PixelColor> GridRenderer<C> for DefaultGridRenderer {
    fn draw_major_line<D>(
        &self,
        start: Point,
        end: Point,
        style: &crate::style::LineStyle<C>,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        self.draw_grid_line(start, end, style, target)
    }

    fn draw_minor_line<D>(
        &self,
        start: Point,
        end: Point,
        style: &crate::style::LineStyle<C>,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        self.draw_grid_line(start, end, style, target)
    }

    fn draw_grid_line<D>(
        &self,
        start: Point,
        end: Point,
        style: &crate::style::LineStyle<C>,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        use crate::error::ChartError;
        use embedded_graphics::primitives::{Line, PrimitiveStyle};

        let primitive_style = PrimitiveStyle::with_stroke(style.color, style.width);

        // For dashed/dotted lines, we need to implement pattern drawing
        match style.pattern {
            crate::style::LinePattern::Solid => {
                Line::new(start, end)
                    .into_styled(primitive_style)
                    .draw(target)
                    .map_err(|_| ChartError::RenderingError)?;
            }
            crate::style::LinePattern::Dashed => {
                self.draw_dashed_line(start, end, style, target)?;
            }
            crate::style::LinePattern::Dotted => {
                self.draw_dotted_line(start, end, style, target)?;
            }
            crate::style::LinePattern::DashDot => {
                self.draw_dash_dot_line(start, end, style, target)?;
            }
            crate::style::LinePattern::Custom => {
                // Fall back to solid for custom patterns
                Line::new(start, end)
                    .into_styled(primitive_style)
                    .draw(target)
                    .map_err(|_| ChartError::RenderingError)?;
            }
        }

        Ok(())
    }
}

impl DefaultGridRenderer {
    /// Draw a dashed line
    fn draw_dashed_line<C, D>(
        &self,
        start: Point,
        end: Point,
        style: &crate::style::LineStyle<C>,
        target: &mut D,
    ) -> ChartResult<()>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
    {
        use crate::error::ChartError;
        use embedded_graphics::primitives::{Line, PrimitiveStyle};

        let primitive_style = PrimitiveStyle::with_stroke(style.color, style.width);
        let dash_length = 8;
        let gap_length = 4;

        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let dx_f32 = dx as f32;
        let dy_f32 = dy as f32;
        let dx_num = dx_f32.to_number();
        let dy_num = dy_f32.to_number();
        let length_squared = dx_num * dx_num + dy_num * dy_num;
        let length_num = Math::sqrt(length_squared);
        let length = f32::from_number(length_num);

        let one = 1.0f32.to_number();
        if length_num < one {
            return Ok(());
        }

        let unit_x = dx as f32 / length;
        let unit_y = dy as f32 / length;

        let mut current_pos = 0.0;
        let mut drawing = true;

        while current_pos < length {
            let segment_length = if drawing {
                dash_length as f32
            } else {
                gap_length as f32
            };
            let next_pos = (current_pos + segment_length).min(length);

            if drawing {
                let seg_start = Point::new(
                    start.x + (current_pos * unit_x) as i32,
                    start.y + (current_pos * unit_y) as i32,
                );
                let seg_end = Point::new(
                    start.x + (next_pos * unit_x) as i32,
                    start.y + (next_pos * unit_y) as i32,
                );

                Line::new(seg_start, seg_end)
                    .into_styled(primitive_style)
                    .draw(target)
                    .map_err(|_| ChartError::RenderingError)?;
            }

            current_pos = next_pos;
            drawing = !drawing;
        }

        Ok(())
    }

    /// Draw a dotted line
    fn draw_dotted_line<C, D>(
        &self,
        start: Point,
        end: Point,
        style: &crate::style::LineStyle<C>,
        target: &mut D,
    ) -> ChartResult<()>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
    {
        use crate::error::ChartError;
        use embedded_graphics::primitives::{Circle, PrimitiveStyle};

        let primitive_style = PrimitiveStyle::with_fill(style.color);
        let dot_spacing = 6;

        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let dx_f32 = dx as f32;
        let dy_f32 = dy as f32;
        let dx_num = dx_f32.to_number();
        let dy_num = dy_f32.to_number();
        let length_squared = dx_num * dx_num + dy_num * dy_num;
        let length_num = Math::sqrt(length_squared);
        let length = f32::from_number(length_num);

        let one = 1.0f32.to_number();
        if length_num < one {
            return Ok(());
        }

        let unit_x = dx as f32 / length;
        let unit_y = dy as f32 / length;

        let mut current_pos = 0.0;

        while current_pos <= length {
            let dot_center = Point::new(
                start.x + (current_pos * unit_x) as i32,
                start.y + (current_pos * unit_y) as i32,
            );

            Circle::new(Point::new(dot_center.x - 1, dot_center.y - 1), 2)
                .into_styled(primitive_style)
                .draw(target)
                .map_err(|_| ChartError::RenderingError)?;

            current_pos += dot_spacing as f32;
        }

        Ok(())
    }

    /// Draw a dash-dot line
    fn draw_dash_dot_line<C, D>(
        &self,
        start: Point,
        end: Point,
        style: &crate::style::LineStyle<C>,
        target: &mut D,
    ) -> ChartResult<()>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
    {
        use crate::error::ChartError;
        use embedded_graphics::primitives::{Circle, Line, PrimitiveStyle};

        let line_style = PrimitiveStyle::with_stroke(style.color, style.width);
        let dot_style = PrimitiveStyle::with_fill(style.color);
        let dash_length = 8;
        let gap_length = 3;
        let dot_gap = 3;

        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let dx_f32 = dx as f32;
        let dy_f32 = dy as f32;
        let dx_num = dx_f32.to_number();
        let dy_num = dy_f32.to_number();
        let length_squared = dx_num * dx_num + dy_num * dy_num;
        let length_num = Math::sqrt(length_squared);
        let length = f32::from_number(length_num);

        let one = 1.0f32.to_number();
        if length_num < one {
            return Ok(());
        }

        let unit_x = dx as f32 / length;
        let unit_y = dy as f32 / length;

        let mut current_pos = 0.0;
        let pattern = [dash_length as f32, gap_length as f32, 2.0, dot_gap as f32]; // dash, gap, dot, gap
        let mut pattern_index = 0;

        while current_pos < length {
            let segment_length = pattern[pattern_index % pattern.len()];
            let next_pos = (current_pos + segment_length).min(length);

            match pattern_index % 4 {
                0 => {
                    // Draw dash
                    let seg_start = Point::new(
                        start.x + (current_pos * unit_x) as i32,
                        start.y + (current_pos * unit_y) as i32,
                    );
                    let seg_end = Point::new(
                        start.x + (next_pos * unit_x) as i32,
                        start.y + (next_pos * unit_y) as i32,
                    );

                    Line::new(seg_start, seg_end)
                        .into_styled(line_style)
                        .draw(target)
                        .map_err(|_| ChartError::RenderingError)?;
                }
                2 => {
                    // Draw dot
                    let dot_center = Point::new(
                        start.x + (current_pos * unit_x) as i32,
                        start.y + (current_pos * unit_y) as i32,
                    );

                    Circle::new(Point::new(dot_center.x - 1, dot_center.y - 1), 2)
                        .into_styled(dot_style)
                        .draw(target)
                        .map_err(|_| ChartError::RenderingError)?;
                }
                _ => {
                    // Gap - do nothing
                }
            }

            current_pos = next_pos;
            pattern_index += 1;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_orientation() {
        assert_eq!(GridOrientation::Horizontal, GridOrientation::Horizontal);
        assert_ne!(GridOrientation::Horizontal, GridOrientation::Vertical);
    }

    #[test]
    fn test_default_grid_renderer() {
        let renderer = DefaultGridRenderer;
        // Basic instantiation test
        assert_eq!(core::mem::size_of_val(&renderer), 0); // Zero-sized type
    }
}
