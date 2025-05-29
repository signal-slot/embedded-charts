//! Scatter chart implementation.
//!
//! This module provides scatter chart functionality for plotting discrete data points
//! with various shapes, sizes, and colors. Supports bubble charts with size mapping
//! and collision detection for large datasets.

use crate::axes::traits::Axis;
use crate::chart::traits::AxisChart;
#[cfg(feature = "animations")]
use crate::chart::traits::{Chart, ChartBuilder, ChartConfig, Margins};
use crate::data::{DataBounds, DataPoint, DataSeries};
use crate::error::{ChartError, ChartResult};
use crate::math::{Math, NumericConversion};
use embedded_graphics::{
    draw_target::DrawTarget,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, Rectangle},
};
use heapless::Vec;

/// Scatter chart implementation for plotting discrete data points
#[derive(Debug)]
pub struct ScatterChart<C: PixelColor> {
    style: ScatterChartStyle<C>,
    config: ChartConfig<C>,
    grid: Option<crate::grid::GridSystem<C>>,
    x_axis: Option<crate::axes::LinearAxis<f32, C>>,
    y_axis: Option<crate::axes::LinearAxis<f32, C>>,
}

/// Style configuration for scatter charts
#[derive(Debug, Clone)]
pub struct ScatterChartStyle<C: PixelColor> {
    /// Default point style
    pub point_style: PointStyle<C>,
    /// Whether to enable size mapping for bubble charts
    pub size_mapping: Option<SizeMapping>,
    /// Whether to enable color mapping
    pub color_mapping: Option<ColorMapping<C>>,
    /// Collision detection settings
    pub collision_detection: CollisionSettings,
    /// Whether to show connecting lines between points
    pub show_connections: bool,
    /// Connection line style
    pub connection_style: Option<ConnectionStyle<C>>,
}

/// Style configuration for individual points
#[derive(Debug, Clone, Copy)]
pub struct PointStyle<C: PixelColor> {
    /// Shape of the point
    pub shape: PointShape,
    /// Size of the point in pixels
    pub size: u32,
    /// Color of the point
    pub color: C,
    /// Border style for the point
    pub border: Option<BorderStyle<C>>,
    /// Fill opacity (0.0 = transparent, 1.0 = opaque)
    pub opacity: f32,
}

/// Border style for points
#[derive(Debug, Clone, Copy)]
pub struct BorderStyle<C: PixelColor> {
    /// Border color
    pub color: C,
    /// Border width in pixels
    pub width: u32,
}

/// Connection line style
#[derive(Debug, Clone, Copy)]
pub struct ConnectionStyle<C: PixelColor> {
    /// Line color
    pub color: C,
    /// Line width
    pub width: u32,
    /// Line pattern
    pub pattern: LinePattern,
}

/// Line pattern for connections
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinePattern {
    /// Solid line
    Solid,
    /// Dashed line
    Dashed,
    /// Dotted line
    Dotted,
}

/// Available point shapes for scatter plots
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointShape {
    /// Circular point
    Circle,
    /// Square point
    Square,
    /// Diamond point
    Diamond,
    /// Triangle point (pointing up)
    Triangle,
    /// Cross/plus point
    Cross,
    /// X-shaped point
    X,
    /// Star point
    Star,
}

/// Size mapping configuration for bubble charts
#[derive(Debug, Clone, Copy)]
pub struct SizeMapping {
    /// Minimum point size
    pub min_size: u32,
    /// Maximum point size
    pub max_size: u32,
    /// Size scaling function
    pub scaling: SizeScaling,
}

/// Size scaling functions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeScaling {
    /// Linear scaling
    Linear,
    /// Square root scaling (better for area representation)
    SquareRoot,
    /// Logarithmic scaling
    Logarithmic,
}

/// Color mapping configuration
#[derive(Debug, Clone)]
pub struct ColorMapping<C: PixelColor> {
    /// Color palette for mapping
    pub colors: Vec<C, 16>,
    /// Mapping strategy
    pub strategy: ColorMappingStrategy,
}

/// Color mapping strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMappingStrategy {
    /// Map based on Y value
    ValueBased,
    /// Map based on data point index
    IndexBased,
    /// Map based on distance from origin
    DistanceBased,
}

/// Collision detection settings
#[derive(Debug, Clone, Copy)]
pub struct CollisionSettings {
    /// Whether collision detection is enabled
    pub enabled: bool,
    /// Minimum distance between points to avoid overlap
    pub min_distance: u32,
    /// Strategy for handling collisions
    pub strategy: CollisionStrategy,
}

/// Collision handling strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionStrategy {
    /// Hide overlapping points
    Hide,
    /// Offset overlapping points slightly
    Offset,
    /// Merge overlapping points
    Merge,
    /// Show all points (no collision handling)
    None,
}

/// Builder for scatter charts
#[derive(Debug)]
pub struct ScatterChartBuilder<C: PixelColor> {
    style: ScatterChartStyle<C>,
    config: ChartConfig<C>,
    grid: Option<crate::grid::GridSystem<C>>,
    x_axis: Option<crate::axes::LinearAxis<f32, C>>,
    y_axis: Option<crate::axes::LinearAxis<f32, C>>,
}

impl<C: PixelColor> ScatterChart<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new scatter chart with default styling
    pub fn new() -> Self {
        Self {
            style: ScatterChartStyle::default(),
            config: ChartConfig::default(),
            grid: None,
            x_axis: None,
            y_axis: None,
        }
    }

    /// Create a builder for configuring the scatter chart
    pub fn builder() -> ScatterChartBuilder<C> {
        ScatterChartBuilder::new()
    }

    /// Set the scatter chart style
    pub fn set_style(&mut self, style: ScatterChartStyle<C>) {
        self.style = style;
    }

    /// Get the current scatter chart style
    pub fn style(&self) -> &ScatterChartStyle<C> {
        &self.style
    }

    /// Set the chart configuration
    pub fn set_config(&mut self, config: ChartConfig<C>) {
        self.config = config;
    }

    /// Get the chart configuration
    pub fn config(&self) -> &ChartConfig<C> {
        &self.config
    }

    /// Set the grid system
    pub fn set_grid(&mut self, grid: Option<crate::grid::GridSystem<C>>) {
        self.grid = grid;
    }

    /// Get the grid system
    pub fn grid(&self) -> Option<&crate::grid::GridSystem<C>> {
        self.grid.as_ref()
    }

    /// Transform data coordinates to screen coordinates
    fn transform_point<P>(
        &self,
        point: &P,
        data_bounds: &DataBounds<P::X, P::Y>,
        viewport: Rectangle,
    ) -> Point
    where
        P: DataPoint,
        P::X: Into<f32> + Copy,
        P::Y: Into<f32> + Copy,
    {
        let data_x: f32 = point.x().into();
        let data_y: f32 = point.y().into();

        // Use axis ranges if available, otherwise fall back to data bounds
        let (min_x, max_x) = if let Some(ref x_axis) = self.x_axis {
            (x_axis.min(), x_axis.max())
        } else {
            (data_bounds.min_x.into(), data_bounds.max_x.into())
        };

        let (min_y, max_y) = if let Some(ref y_axis) = self.y_axis {
            (y_axis.min(), y_axis.max())
        } else {
            (data_bounds.min_y.into(), data_bounds.max_y.into())
        };

        // Apply margins to get the actual drawing area
        let draw_area = self.config.margins.apply_to(viewport);

        // Normalize to 0-1 range
        let norm_x = if max_x > min_x {
            (data_x - min_x) / (max_x - min_x)
        } else {
            0.5
        };

        let norm_y = if max_y > min_y {
            (data_y - min_y) / (max_y - min_y)
        } else {
            0.5
        };

        // Transform to screen coordinates (Y is flipped)
        // Ensure we don't go outside the drawing area bounds
        let screen_x = if draw_area.size.width > 0 {
            let x = draw_area.top_left.x + (norm_x * (draw_area.size.width as f32 - 1.0)) as i32;
            x.max(draw_area.top_left.x)
                .min(draw_area.top_left.x + draw_area.size.width as i32 - 1)
        } else {
            draw_area.top_left.x
        };

        let screen_y = if draw_area.size.height > 0 {
            let y = draw_area.top_left.y + draw_area.size.height as i32
                - 1
                - (norm_y * (draw_area.size.height as f32 - 1.0)) as i32;
            y.max(draw_area.top_left.y)
                .min(draw_area.top_left.y + draw_area.size.height as i32 - 1)
        } else {
            draw_area.top_left.y
        };

        Point::new(screen_x, screen_y)
    }

    /// Calculate point size based on size mapping
    fn calculate_point_size<P>(&self, point: &P, data_bounds: &DataBounds<P::X, P::Y>) -> u32
    where
        P: DataPoint,
        P::Y: Into<f32> + Copy,
    {
        if let Some(size_mapping) = &self.style.size_mapping {
            let data_y: f32 = point.y().into();
            let min_y: f32 = data_bounds.min_y.into();
            let max_y: f32 = data_bounds.max_y.into();

            let norm_value = if max_y > min_y {
                (data_y - min_y) / (max_y - min_y)
            } else {
                0.5
            };

            let scaled_value = match size_mapping.scaling {
                SizeScaling::Linear => norm_value,
                SizeScaling::SquareRoot => {
                    let norm_num = norm_value.to_number();
                    f32::from_number(Math::sqrt(norm_num))
                }
                SizeScaling::Logarithmic => {
                    if norm_value > 0.0 {
                        let norm_num = norm_value.to_number();
                        let one_num = 1.0f32.to_number();
                        let numerator = Math::ln(one_num + norm_num);
                        let denominator = Math::ln(one_num + one_num);
                        f32::from_number(numerator / denominator)
                    } else {
                        0.0
                    }
                }
            };

            let size_range = size_mapping.max_size - size_mapping.min_size;
            size_mapping.min_size + (scaled_value * size_range as f32) as u32
        } else {
            self.style.point_style.size
        }
    }

    /// Calculate point color based on color mapping
    fn calculate_point_color<P>(
        &self,
        point: &P,
        index: usize,
        data_bounds: &DataBounds<P::X, P::Y>,
    ) -> C
    where
        P: DataPoint,
        P::X: Into<f32> + Copy,
        P::Y: Into<f32> + Copy,
    {
        if let Some(color_mapping) = &self.style.color_mapping {
            let color_index = match color_mapping.strategy {
                ColorMappingStrategy::ValueBased => {
                    let data_y: f32 = point.y().into();
                    let min_y: f32 = data_bounds.min_y.into();
                    let max_y: f32 = data_bounds.max_y.into();

                    let norm_value = if max_y > min_y {
                        (data_y - min_y) / (max_y - min_y)
                    } else {
                        0.5
                    };

                    ((norm_value * (color_mapping.colors.len() - 1) as f32) as usize)
                        .min(color_mapping.colors.len() - 1)
                }
                ColorMappingStrategy::IndexBased => index % color_mapping.colors.len(),
                ColorMappingStrategy::DistanceBased => {
                    let data_x: f32 = point.x().into();
                    let data_y: f32 = point.y().into();
                    let data_x_num = data_x.to_number();
                    let data_y_num = data_y.to_number();
                    let distance_squared = data_x_num * data_x_num + data_y_num * data_y_num;
                    let distance = f32::from_number(Math::sqrt(distance_squared));

                    // Normalize distance and map to color index
                    let max_distance = {
                        let max_x: f32 = data_bounds.max_x.into();
                        let max_y: f32 = data_bounds.max_y.into();
                        let max_x_num = max_x.to_number();
                        let max_y_num = max_y.to_number();
                        let max_distance_squared = max_x_num * max_x_num + max_y_num * max_y_num;
                        f32::from_number(Math::sqrt(max_distance_squared))
                    };

                    let norm_distance = if max_distance > 0.0 {
                        distance / max_distance
                    } else {
                        0.0
                    };

                    ((norm_distance * (color_mapping.colors.len() - 1) as f32) as usize)
                        .min(color_mapping.colors.len() - 1)
                }
            };

            color_mapping.colors[color_index]
        } else {
            self.style.point_style.color
        }
    }

    /// Check for collision between two points
    fn check_collision(&self, p1: Point, p2: Point, size1: u32, size2: u32) -> bool {
        if !self.style.collision_detection.enabled {
            return false;
        }

        let dx = (p1.x - p2.x).unsigned_abs();
        let dy = (p1.y - p2.y).unsigned_abs();
        let dx_f32 = dx as f32;
        let dy_f32 = dy as f32;
        let dx_num = dx_f32.to_number();
        let dy_num = dy_f32.to_number();
        let distance_squared = dx_num * dx_num + dy_num * dy_num;
        let distance = f32::from_number(Math::sqrt(distance_squared)) as u32;
        let min_distance = (size1 + size2) / 2 + self.style.collision_detection.min_distance;

        distance < min_distance
    }

    /// Draw a single point with the specified style
    fn draw_point<D>(
        &self,
        center: Point,
        point_style: &PointStyle<C>,
        size: u32,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        // Get the target's bounding box to ensure we don't draw outside
        let target_bounds = target.bounding_box();

        // Check if the point is within the target bounds (with some margin for the point size)
        let radius = size / 2;
        if center.x < target_bounds.top_left.x - radius as i32
            || center.x
                >= target_bounds.top_left.x + target_bounds.size.width as i32 + radius as i32
            || center.y < target_bounds.top_left.y - radius as i32
            || center.y
                >= target_bounds.top_left.y + target_bounds.size.height as i32 + radius as i32
        {
            // Point is outside the target bounds, skip drawing
            return Ok(());
        }

        let fill_style = PrimitiveStyle::with_fill(point_style.color);

        match point_style.shape {
            PointShape::Circle => {
                Circle::new(
                    Point::new(center.x - radius as i32, center.y - radius as i32),
                    size,
                )
                .into_styled(fill_style)
                .draw(target)
                .map_err(|_| ChartError::RenderingError)?;
            }
            PointShape::Square => {
                Rectangle::new(
                    Point::new(center.x - radius as i32, center.y - radius as i32),
                    Size::new(size, size),
                )
                .into_styled(fill_style)
                .draw(target)
                .map_err(|_| ChartError::RenderingError)?;
            }
            PointShape::Diamond => {
                use crate::render::PrimitiveRenderer;
                use crate::style::FillStyle;

                let fill_style = FillStyle::solid(point_style.color);
                PrimitiveRenderer::draw_diamond(center, size, None, Some(&fill_style), target)
                    .map_err(|_| ChartError::RenderingError)?;
            }
            PointShape::Triangle => {
                use crate::render::PrimitiveRenderer;
                use crate::style::FillStyle;

                let fill_style = FillStyle::solid(point_style.color);
                let half_size = size as i32 / 2;
                let p1 = Point::new(center.x, center.y - half_size);
                let p2 = Point::new(center.x - half_size, center.y + half_size);
                let p3 = Point::new(center.x + half_size, center.y + half_size);

                PrimitiveRenderer::draw_triangle(p1, p2, p3, None, Some(&fill_style), target)
                    .map_err(|_| ChartError::RenderingError)?;
            }
            PointShape::Cross => {
                let stroke_style = PrimitiveStyle::with_stroke(point_style.color, 2);
                let half_size = radius as i32;

                // Vertical line
                Line::new(
                    Point::new(center.x, center.y - half_size),
                    Point::new(center.x, center.y + half_size),
                )
                .into_styled(stroke_style)
                .draw(target)
                .map_err(|_| ChartError::RenderingError)?;

                // Horizontal line
                Line::new(
                    Point::new(center.x - half_size, center.y),
                    Point::new(center.x + half_size, center.y),
                )
                .into_styled(stroke_style)
                .draw(target)
                .map_err(|_| ChartError::RenderingError)?;
            }
            PointShape::X => {
                let stroke_style = PrimitiveStyle::with_stroke(point_style.color, 2);
                let half_size = radius as i32;

                // Diagonal line 1
                Line::new(
                    Point::new(center.x - half_size, center.y - half_size),
                    Point::new(center.x + half_size, center.y + half_size),
                )
                .into_styled(stroke_style)
                .draw(target)
                .map_err(|_| ChartError::RenderingError)?;

                // Diagonal line 2
                Line::new(
                    Point::new(center.x - half_size, center.y + half_size),
                    Point::new(center.x + half_size, center.y - half_size),
                )
                .into_styled(stroke_style)
                .draw(target)
                .map_err(|_| ChartError::RenderingError)?;
            }
            PointShape::Star => {
                // For star, draw a simple filled circle for now
                // A full star implementation would be more complex
                Circle::new(
                    Point::new(center.x - radius as i32, center.y - radius as i32),
                    size,
                )
                .into_styled(fill_style)
                .draw(target)
                .map_err(|_| ChartError::RenderingError)?;
            }
        }

        // Draw border if specified
        if let Some(border) = &point_style.border {
            let border_style = PrimitiveStyle::with_stroke(border.color, border.width);
            match point_style.shape {
                PointShape::Circle | PointShape::Star => {
                    Circle::new(
                        Point::new(center.x - radius as i32, center.y - radius as i32),
                        size,
                    )
                    .into_styled(border_style)
                    .draw(target)
                    .map_err(|_| ChartError::RenderingError)?;
                }
                PointShape::Square => {
                    Rectangle::new(
                        Point::new(center.x - radius as i32, center.y - radius as i32),
                        Size::new(size, size),
                    )
                    .into_styled(border_style)
                    .draw(target)
                    .map_err(|_| ChartError::RenderingError)?;
                }
                _ => {
                    // For complex shapes, draw a simple circle border
                    Circle::new(
                        Point::new(center.x - radius as i32, center.y - radius as i32),
                        size,
                    )
                    .into_styled(border_style)
                    .draw(target)
                    .map_err(|_| ChartError::RenderingError)?;
                }
            }
        }

        Ok(())
    }

    /// Draw connection lines between points
    fn draw_connections<D>(
        &self,
        screen_points: &Vec<Point, 256>,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        if !self.style.show_connections || screen_points.len() < 2 {
            return Ok(());
        }

        if let Some(connection_style) = &self.style.connection_style {
            let line_style =
                PrimitiveStyle::with_stroke(connection_style.color, connection_style.width);
            let target_bounds = target.bounding_box();

            for window in screen_points.windows(2) {
                if let [p1, p2] = window {
                    // Check if both points are within reasonable bounds
                    let p1_in_bounds = p1.x >= target_bounds.top_left.x - 10
                        && p1.x < target_bounds.top_left.x + target_bounds.size.width as i32 + 10
                        && p1.y >= target_bounds.top_left.y - 10
                        && p1.y < target_bounds.top_left.y + target_bounds.size.height as i32 + 10;

                    let p2_in_bounds = p2.x >= target_bounds.top_left.x - 10
                        && p2.x < target_bounds.top_left.x + target_bounds.size.width as i32 + 10
                        && p2.y >= target_bounds.top_left.y - 10
                        && p2.y < target_bounds.top_left.y + target_bounds.size.height as i32 + 10;

                    // Only draw the line if at least one point is in bounds
                    if p1_in_bounds || p2_in_bounds {
                        Line::new(*p1, *p2)
                            .into_styled(line_style)
                            .draw(target)
                            .map_err(|_| ChartError::RenderingError)?;
                    }
                }
            }
        }

        Ok(())
    }
}

impl<C: PixelColor> Default for ScatterChart<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<C: PixelColor + 'static> Chart<C> for ScatterChart<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    type Data = crate::data::series::StaticDataSeries<crate::data::point::Point2D, 256>;
    type Config = ChartConfig<C>;

    fn draw<D>(
        &self,
        data: &Self::Data,
        config: &Self::Config,
        viewport: Rectangle,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
        Self::Data: DataSeries,
        <Self::Data as DataSeries>::Item: DataPoint,
        <<Self::Data as DataSeries>::Item as DataPoint>::X: Into<f32> + Copy + PartialOrd,
        <<Self::Data as DataSeries>::Item as DataPoint>::Y: Into<f32> + Copy + PartialOrd,
    {
        if data.is_empty() {
            return Err(ChartError::InsufficientData);
        }

        // Calculate data bounds
        let data_bounds = data.bounds()?;

        // Draw background if specified
        if let Some(bg_color) = config.background_color {
            Rectangle::new(viewport.top_left, viewport.size)
                .into_styled(PrimitiveStyle::with_fill(bg_color))
                .draw(target)
                .map_err(|_| ChartError::RenderingError)?;
        }

        // Draw grid if present
        if let Some(ref grid) = self.grid {
            let chart_area = config.margins.apply_to(viewport);
            grid.draw(chart_area, target)?;
        }

        // Collect screen points and handle collisions
        let mut screen_points = Vec::<Point, 256>::new();
        let mut point_data = Vec::<(Point, PointStyle<C>, u32), 256>::new();

        for (index, point) in data.iter().enumerate() {
            let screen_point = self.transform_point(&point, &data_bounds, viewport);
            let point_size = self.calculate_point_size(&point, &data_bounds);
            let point_color = self.calculate_point_color(&point, index, &data_bounds);

            let mut point_style = self.style.point_style;
            point_style.color = point_color;

            // Check for collisions if enabled
            let mut should_draw = true;
            if self.style.collision_detection.enabled {
                for (existing_point, _, existing_size) in &point_data {
                    if self.check_collision(
                        screen_point,
                        *existing_point,
                        point_size,
                        *existing_size,
                    ) {
                        match self.style.collision_detection.strategy {
                            CollisionStrategy::Hide => {
                                should_draw = false;
                                break;
                            }
                            CollisionStrategy::Offset => {
                                // Offset the point slightly
                                let offset = self.style.collision_detection.min_distance as i32;
                                let screen_point =
                                    Point::new(screen_point.x + offset, screen_point.y + offset);
                                screen_points
                                    .push(screen_point)
                                    .map_err(|_| ChartError::MemoryFull)?;
                                point_data
                                    .push((screen_point, point_style, point_size))
                                    .map_err(|_| ChartError::MemoryFull)?;
                                should_draw = false;
                                break;
                            }
                            CollisionStrategy::Merge => {
                                // For simplicity, just use the larger point
                                should_draw = false;
                                break;
                            }
                            CollisionStrategy::None => {
                                // No collision handling
                            }
                        }
                    }
                }
            }

            if should_draw {
                screen_points
                    .push(screen_point)
                    .map_err(|_| ChartError::MemoryFull)?;
                point_data
                    .push((screen_point, point_style, point_size))
                    .map_err(|_| ChartError::MemoryFull)?;
            }
        }

        // Draw connection lines if enabled
        self.draw_connections(&screen_points, target)?;

        // Draw all points
        for (screen_point, point_style, point_size) in &point_data {
            self.draw_point(*screen_point, point_style, *point_size, target)?;
        }

        // Draw axes if configured
        {
            let chart_area = config.margins.apply_to(viewport);

            // Draw X-axis using the axis system
            if let Some(ref x_axis) = self.x_axis {
                x_axis.draw(chart_area, target)?;
            }

            // Draw Y-axis using the axis system
            if let Some(ref y_axis) = self.y_axis {
                y_axis.draw(chart_area, target)?;
            }
        }

        Ok(())
    }
}

impl<C: PixelColor> Default for ScatterChartStyle<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self {
            point_style: PointStyle::default(),
            size_mapping: None,
            color_mapping: None,
            collision_detection: CollisionSettings::default(),
            show_connections: false,
            connection_style: None,
        }
    }
}

impl<C: PixelColor> Default for PointStyle<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self {
            shape: PointShape::Circle,
            size: 6,
            color: embedded_graphics::pixelcolor::Rgb565::BLUE.into(),
            border: None,
            opacity: 1.0,
        }
    }
}

impl Default for CollisionSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            min_distance: 5,
            strategy: CollisionStrategy::None,
        }
    }
}

impl Default for SizeMapping {
    fn default() -> Self {
        Self {
            min_size: 4,
            max_size: 20,
            scaling: SizeScaling::Linear,
        }
    }
}

impl<C: PixelColor> ScatterChartBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new scatter chart builder
    pub fn new() -> Self {
        Self {
            style: ScatterChartStyle::default(),
            config: ChartConfig::default(),
            grid: None,
            x_axis: None,
            y_axis: None,
        }
    }

    /// Set the point shape
    pub fn point_shape(mut self, shape: PointShape) -> Self {
        self.style.point_style.shape = shape;
        self
    }

    /// Set the point size
    pub fn point_size(mut self, size: u32) -> Self {
        self.style.point_style.size = size;
        self
    }

    /// Set the point color
    pub fn point_color(mut self, color: C) -> Self {
        self.style.point_style.color = color;
        self
    }

    /// Set the point border
    pub fn point_border(mut self, border: BorderStyle<C>) -> Self {
        self.style.point_style.border = Some(border);
        self
    }

    /// Enable size mapping for bubble charts
    pub fn with_size_mapping(mut self, mapping: SizeMapping) -> Self {
        self.style.size_mapping = Some(mapping);
        self
    }

    /// Enable color mapping
    pub fn with_color_mapping(mut self, mapping: ColorMapping<C>) -> Self {
        self.style.color_mapping = Some(mapping);
        self
    }

    /// Enable collision detection
    pub fn with_collision_detection(mut self, settings: CollisionSettings) -> Self {
        self.style.collision_detection = settings;
        self
    }

    /// Enable connection lines between points
    pub fn with_connections(mut self, style: ConnectionStyle<C>) -> Self {
        self.style.show_connections = true;
        self.style.connection_style = Some(style);
        self
    }

    /// Set the chart title
    pub fn with_title(mut self, title: &str) -> Self {
        self.config.title =
            Some(heapless::String::try_from(title).unwrap_or_else(|_| heapless::String::new()));
        self
    }

    /// Set the background color
    pub fn background_color(mut self, color: C) -> Self {
        self.config.background_color = Some(color);
        self
    }

    /// Set the chart margins
    pub fn margins(mut self, margins: Margins) -> Self {
        self.config.margins = margins;
        self
    }

    /// Add a grid system
    pub fn with_grid(mut self, grid: crate::grid::GridSystem<C>) -> Self {
        self.grid = Some(grid);
        self
    }

    /// Add an X-axis
    pub fn with_x_axis(mut self, axis: crate::axes::LinearAxis<f32, C>) -> Self {
        self.x_axis = Some(axis);
        self
    }

    /// Add a Y-axis
    pub fn with_y_axis(mut self, axis: crate::axes::LinearAxis<f32, C>) -> Self {
        self.y_axis = Some(axis);
        self
    }

    /// Build the scatter chart
    pub fn build(self) -> ChartResult<ScatterChart<C>> {
        Ok(ScatterChart {
            style: self.style,
            config: self.config,
            grid: self.grid,
            x_axis: self.x_axis,
            y_axis: self.y_axis,
        })
    }
}

impl<C: PixelColor + 'static> ChartBuilder<C> for ScatterChartBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    type Chart = ScatterChart<C>;
    type Error = ChartError;

    fn build(self) -> Result<Self::Chart, Self::Error> {
        self.build()
    }
}

impl<C: PixelColor> Default for ScatterChartBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<C: PixelColor + 'static> AxisChart<C> for ScatterChart<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    type XAxis = crate::axes::LinearAxis<f32, C>;
    type YAxis = crate::axes::LinearAxis<f32, C>;

    fn set_x_axis(&mut self, axis: Self::XAxis) {
        self.x_axis = Some(axis);
    }

    fn set_y_axis(&mut self, axis: Self::YAxis) {
        self.y_axis = Some(axis);
    }

    fn x_axis(&self) -> ChartResult<&Self::XAxis> {
        self.x_axis.as_ref().ok_or(ChartError::InvalidConfiguration)
    }

    fn y_axis(&self) -> ChartResult<&Self::YAxis> {
        self.y_axis.as_ref().ok_or(ChartError::InvalidConfiguration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::pixelcolor::Rgb565;

    #[test]
    fn test_scatter_chart_creation() {
        let chart = ScatterChart::<Rgb565>::new();
        assert_eq!(chart.style().point_style.shape, PointShape::Circle);
        assert_eq!(chart.style().point_style.size, 6);
    }

    #[test]
    fn test_scatter_chart_builder() {
        let chart = ScatterChart::<Rgb565>::builder()
            .point_shape(PointShape::Square)
            .point_size(8)
            .point_color(Rgb565::RED)
            .build()
            .unwrap();

        assert_eq!(chart.style().point_style.shape, PointShape::Square);
        assert_eq!(chart.style().point_style.size, 8);
        assert_eq!(chart.style().point_style.color, Rgb565::RED);
    }

    #[test]
    fn test_point_shapes() {
        let shapes = [
            PointShape::Circle,
            PointShape::Square,
            PointShape::Diamond,
            PointShape::Triangle,
            PointShape::Cross,
            PointShape::X,
            PointShape::Star,
        ];

        for shape in &shapes {
            let chart = ScatterChart::<Rgb565>::builder()
                .point_shape(*shape)
                .build()
                .unwrap();
            assert_eq!(chart.style().point_style.shape, *shape);
        }
    }

    #[test]
    fn test_size_mapping() {
        let mapping = SizeMapping {
            min_size: 4,
            max_size: 20,
            scaling: SizeScaling::Linear,
        };

        let chart = ScatterChart::<Rgb565>::builder()
            .with_size_mapping(mapping)
            .build()
            .unwrap();

        assert!(chart.style().size_mapping.is_some());
        let size_mapping = chart.style().size_mapping.unwrap();
        assert_eq!(size_mapping.min_size, 4);
        assert_eq!(size_mapping.max_size, 20);
        assert_eq!(size_mapping.scaling, SizeScaling::Linear);
    }

    #[test]
    fn test_collision_detection() {
        let settings = CollisionSettings {
            enabled: true,
            min_distance: 10,
            strategy: CollisionStrategy::Offset,
        };

        let chart = ScatterChart::<Rgb565>::builder()
            .with_collision_detection(settings)
            .build()
            .unwrap();

        assert!(chart.style().collision_detection.enabled);
        assert_eq!(chart.style().collision_detection.min_distance, 10);
        assert_eq!(
            chart.style().collision_detection.strategy,
            CollisionStrategy::Offset
        );
    }
}
