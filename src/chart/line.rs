//! Line chart implementation with no_std compatibility.
//!
//! This module provides a comprehensive line chart implementation optimized for embedded systems.
//! It supports multiple styling options, markers, area fills, and smooth curves while maintaining
//! memory efficiency and performance suitable for resource-constrained environments.
//!
//! # Features
//!
//! - **Multi-series support**: Display multiple data series with different colors
//! - **Marker customization**: Various shapes (circle, square, diamond, triangle) with configurable size and color
//! - **Area filling**: Fill the area under the line with customizable colors
//! - **Smooth curves**: Optional bezier curve smoothing for professional appearance
//! - **Grid integration**: Support for both legacy and modern grid systems
//! - **Axis integration**: Full support for linear axes with labels and ticks
//! - **Animation support**: Real-time data streaming and smooth transitions (feature-gated)
//! - **Memory efficient**: Static allocation with compile-time bounds
//!
//! # Basic Usage
//!
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! // Create sample data
//! let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
//! data.push(Point2D::new(0.0, 10.0))?;
//! data.push(Point2D::new(1.0, 20.0))?;
//! data.push(Point2D::new(2.0, 15.0))?;
//!
//! // Create a basic line chart
//! let chart = LineChart::builder()
//!     .line_color(Rgb565::BLUE)
//!     .line_width(2)
//!     .build()?;
//!
//! // Configure the chart
//! let config: ChartConfig<Rgb565> = ChartConfig::default();
//! let viewport = Rectangle::new(Point::zero(), Size::new(320, 240));
//!
//! // Render to display (display would be provided by your embedded target)  
//! // chart.draw(&data, &config, viewport, &mut display)?;
//! # Ok::<(), embedded_charts::error::ChartError>(())
//! ```
//!
//! # Advanced Styling
//!
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! let chart = LineChart::builder()
//!     .line_color(Rgb565::BLUE)
//!     .line_width(3)
//!     .fill_area(Rgb565::CSS_LIGHT_BLUE)
//!     .with_markers(MarkerStyle {
//!         shape: MarkerShape::Circle,
//!         size: 8,
//!         color: Rgb565::RED,
//!         visible: true,
//!     })
//!     .smooth(true)
//!     .build()?;
//! # Ok::<(), embedded_charts::error::ChartError>(())
//! ```
//!
//! # With Axes and Grid
//!
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! // Simple example with line chart styling
//! let chart = LineChart::builder()
//!     .line_color(Rgb565::BLUE)
//!     .line_width(2)
//!     .build()?;
//!
//! // Axes and grids can be configured through the chart config
//! // This is a simplified example focusing on basic line chart usage
//! # Ok::<(), embedded_charts::error::ChartError>(())
//! ```

use crate::axes::traits::Axis;
use crate::chart::traits::AxisChart;
use crate::chart::traits::{Chart, ChartBuilder, ChartConfig, Margins};
use crate::data::{DataBounds, DataPoint, DataSeries};
use crate::error::{ChartError, ChartResult};
use crate::math::NumericConversion;
use embedded_graphics::{
    draw_target::DrawTarget,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, Rectangle},
};

/// Line chart implementation for displaying continuous data series.
///
/// A line chart connects data points with straight lines (or smooth curves when enabled),
/// making it ideal for showing trends over time or continuous relationships between variables.
/// This implementation is optimized for embedded systems with static memory allocation
/// and efficient rendering.
///
/// # Features
///
/// - Configurable line styling (color, width, smoothing)
/// - Optional markers at data points with various shapes
/// - Area filling under the line
/// - Integration with grid systems and axes
/// - Support for animations and real-time data streaming
///
/// # Memory Usage
///
/// The line chart uses static allocation with a maximum of 256 data points per series.
/// Additional memory is used for:
/// - Screen coordinate transformation (256 points)
/// - Area fill polygon vertices (258 points maximum)
/// - Grid and axis rendering buffers
///
/// # Examples
///
/// Basic line chart:
/// ```rust
/// use embedded_charts::prelude::*;
/// use embedded_graphics::pixelcolor::Rgb565;
///
/// let chart: LineChart<Rgb565> = LineChart::new();
/// let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
/// data.push(Point2D::new(0.0, 10.0))?;
/// data.push(Point2D::new(1.0, 20.0))?;
/// # Ok::<(), embedded_charts::error::DataError>(())
/// ```
///
/// Styled line chart with markers:
/// ```rust
/// use embedded_charts::prelude::*;
/// use embedded_graphics::pixelcolor::Rgb565;
///
/// let chart = LineChart::builder()
///     .line_color(Rgb565::BLUE)
///     .line_width(2)
///     .with_markers(MarkerStyle {
///         shape: MarkerShape::Circle,
///         size: 6,
///         color: Rgb565::RED,
///         visible: true,
///     })
///     .build()?;
/// # Ok::<(), embedded_charts::error::ChartError>(())
/// ```
#[derive(Debug)]
pub struct LineChart<C: PixelColor> {
    style: LineChartStyle<C>,
    config: ChartConfig<C>,
    grid: Option<crate::grid::GridSystem<C>>,
    x_axis: Option<crate::axes::LinearAxis<f32, C>>,
    y_axis: Option<crate::axes::LinearAxis<f32, C>>,
}

/// Style configuration for line charts.
///
/// This structure contains all visual styling options for line charts,
/// including line appearance, markers, and area fills.
///
/// # Examples
///
/// ```rust
/// use embedded_charts::prelude::*;
/// use embedded_graphics::pixelcolor::Rgb565;
///
/// let style = LineChartStyle {
///     line_color: Rgb565::BLUE,
///     line_width: 2,
///     fill_area: true,
///     fill_color: Some(Rgb565::CSS_LIGHT_BLUE),
///     markers: Some(MarkerStyle::default()),
///     smooth: false,
///     smooth_subdivisions: 8,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct LineChartStyle<C: PixelColor> {
    /// Color of the line connecting data points.
    pub line_color: C,
    /// Width of the line in pixels (recommended range: 1-10).
    ///
    /// Larger widths may impact performance on resource-constrained devices.
    pub line_width: u32,
    /// Whether to fill the area under the line.
    ///
    /// When enabled, creates a filled polygon from the line to the chart baseline.
    pub fill_area: bool,
    /// Fill color for the area under the line.
    ///
    /// Only used when `fill_area` is `true`. If `None`, no fill is drawn.
    pub fill_color: Option<C>,
    /// Marker style for data points.
    ///
    /// When `Some`, markers are drawn at each data point. When `None`, no markers are shown.
    pub markers: Option<MarkerStyle<C>>,
    /// Whether to smooth the line using interpolation.
    ///
    /// When enabled, creates smooth curves between data points instead of straight lines.
    /// Uses Catmull-Rom spline interpolation for balanced smoothness and performance.
    /// This feature may impact performance and is recommended for larger displays.
    pub smooth: bool,
    /// Number of subdivisions for smooth curves (only used when smooth = true)
    pub smooth_subdivisions: u32,
}

/// Marker style configuration for data points.
///
/// Markers are visual indicators drawn at each data point to make individual
/// values easier to identify. This is particularly useful for sparse data
/// or when precise values need to be highlighted.
///
/// # Examples
///
/// ```rust
/// use embedded_charts::prelude::*;
/// use embedded_graphics::pixelcolor::Rgb565;
///
/// let marker_style = MarkerStyle {
///     shape: MarkerShape::Circle,
///     size: 8,
///     color: Rgb565::RED,
///     visible: true,
/// };
/// ```
#[derive(Debug, Clone, Copy)]
pub struct MarkerStyle<C: PixelColor> {
    /// Shape of the marker.
    pub shape: MarkerShape,
    /// Size of the marker in pixels.
    ///
    /// This represents the diameter for circles or the side length for squares.
    /// Recommended range: 4-16 pixels for optimal visibility.
    pub size: u32,
    /// Color of the marker.
    pub color: C,
    /// Whether markers should be visible.
    ///
    /// When `false`, markers are not drawn even if a `MarkerStyle` is provided.
    pub visible: bool,
}

/// Available shapes for data point markers.
///
/// Each shape provides different visual characteristics:
/// - `Circle`: Smooth, traditional marker shape
/// - `Square`: Sharp, geometric appearance
/// - `Diamond`: Distinctive diamond shape
/// - `Triangle`: Directional appearance, good for indicating trends
///
/// # Performance Notes
///
/// - `Circle` and `Square` use embedded-graphics primitives (fastest)
/// - `Diamond` and `Triangle` use custom rendering (slightly slower)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkerShape {
    /// Circular marker - smooth and traditional appearance.
    Circle,
    /// Square marker - sharp, geometric appearance.
    Square,
    /// Diamond marker - distinctive diamond shape.
    Diamond,
    /// Triangle marker - directional appearance.
    Triangle,
}

impl<C: PixelColor> LineChart<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new line chart with default styling.
    ///
    /// This creates a line chart with:
    /// - Blue line color
    /// - 1-pixel line width
    /// - No area fill
    /// - No markers
    /// - No smoothing
    /// - Default margins (10 pixels on all sides)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use embedded_charts::prelude::*;
    /// use embedded_graphics::pixelcolor::Rgb565;
    ///
    /// let chart: LineChart<Rgb565> = LineChart::new();
    /// ```
    pub fn new() -> Self {
        Self {
            style: LineChartStyle::default(),
            config: ChartConfig::default(),
            grid: None,
            x_axis: None,
            y_axis: None,
        }
    }

    /// Create a builder for configuring the line chart.
    ///
    /// The builder pattern provides a fluent interface for configuring
    /// all aspects of the line chart before creation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use embedded_charts::prelude::*;
    /// use embedded_graphics::pixelcolor::Rgb565;
    ///
    /// let chart = LineChart::builder()
    ///     .line_color(Rgb565::BLUE)
    ///     .line_width(2)
    ///     .with_markers(MarkerStyle::default())
    ///     .build()?;
    /// # Ok::<(), embedded_charts::error::ChartError>(())
    /// ```
    pub fn builder() -> LineChartBuilder<C> {
        LineChartBuilder::new()
    }

    /// Set the line style configuration.
    ///
    /// This replaces the entire style configuration with the provided one.
    /// Use the builder pattern for more granular control.
    ///
    /// # Arguments
    ///
    /// * `style` - The new line chart style configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use embedded_charts::prelude::*;
    /// use embedded_graphics::pixelcolor::Rgb565;
    ///
    /// let mut chart = LineChart::new();
    /// let style = LineChartStyle {
    ///     line_color: Rgb565::RED,
    ///     line_width: 3,
    ///     fill_area: true,
    ///     fill_color: Some(Rgb565::CSS_LIGHT_CORAL),
    ///     markers: None,
    ///     smooth: false,
    ///     smooth_subdivisions: 8,
    /// };
    /// chart.set_style(style);
    /// ```
    pub fn set_style(&mut self, style: LineChartStyle<C>) {
        self.style = style;
    }

    /// Get the current line style configuration.
    ///
    /// Returns a reference to the current style configuration,
    /// allowing inspection of current settings.
    ///
    /// # Returns
    ///
    /// A reference to the current `LineChartStyle`
    pub fn style(&self) -> &LineChartStyle<C> {
        &self.style
    }

    /// Set the chart configuration.
    ///
    /// This includes general chart settings like title, background color,
    /// margins, and grid visibility.
    ///
    /// # Arguments
    ///
    /// * `config` - The new chart configuration
    pub fn set_config(&mut self, config: ChartConfig<C>) {
        self.config = config;
    }

    /// Get the current chart configuration.
    ///
    /// # Returns
    ///
    /// A reference to the current `ChartConfig`
    pub fn config(&self) -> &ChartConfig<C> {
        &self.config
    }

    /// Set the grid system for the chart.
    ///
    /// The grid system draws background grid lines to help with data reading.
    /// Pass `None` to disable the grid.
    ///
    /// # Arguments
    ///
    /// * `grid` - Optional grid system configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use embedded_charts::prelude::*;
    /// use embedded_graphics::pixelcolor::Rgb565;
    ///
    /// let mut chart: LineChart<Rgb565> = LineChart::new();
    /// // Grid configuration is simplified in this example
    /// let grid: LinearGrid<Rgb565> = LinearGrid::new(GridOrientation::Horizontal, GridSpacing::DataUnits(10.0));
    /// // chart.set_grid(Some(grid)); // Grid system API still evolving
    /// # Ok::<(), embedded_charts::error::ChartError>(())
    /// ```
    pub fn set_grid(&mut self, grid: Option<crate::grid::GridSystem<C>>) {
        self.grid = grid;
    }

    /// Get the current grid system configuration.
    ///
    /// # Returns
    ///
    /// An optional reference to the current grid system
    pub fn grid(&self) -> Option<&crate::grid::GridSystem<C>> {
        self.grid.as_ref()
    }

    /// Transform data coordinates to screen coordinates using math abstraction
    fn transform_point<P>(
        &self,
        point: &P,
        data_bounds: &DataBounds<P::X, P::Y>,
        viewport: Rectangle,
    ) -> Point
    where
        P: DataPoint,
        P::X: NumericConversion<P::X> + Into<f32> + Copy,
        P::Y: NumericConversion<P::Y> + Into<f32> + Copy,
    {
        // Convert to our math abstraction layer
        let data_x = point.x().into().to_number();
        let data_y = point.y().into().to_number();

        // Use axis ranges if available, otherwise fall back to data bounds
        let (min_x, max_x) = if let Some(ref x_axis) = self.x_axis {
            let axis_min: f32 = x_axis.min();
            let axis_max: f32 = x_axis.max();
            (axis_min.to_number(), axis_max.to_number())
        } else {
            (
                data_bounds.min_x.into().to_number(),
                data_bounds.max_x.into().to_number(),
            )
        };

        let (min_y, max_y) = if let Some(ref y_axis) = self.y_axis {
            let axis_min: f32 = y_axis.min();
            let axis_max: f32 = y_axis.max();
            (axis_min.to_number(), axis_max.to_number())
        } else {
            (
                data_bounds.min_y.into().to_number(),
                data_bounds.max_y.into().to_number(),
            )
        };

        // Apply margins to get the actual drawing area
        let draw_area = self.config.margins.apply_to(viewport);

        // Normalize to 0-1 range using math abstraction
        let norm_x = if f32::from_number(max_x) > f32::from_number(min_x) {
            let range_x = f32::from_number(max_x - min_x);
            let offset_x = f32::from_number(data_x - min_x);
            (offset_x / range_x).to_number()
        } else {
            0.5f32.to_number()
        };

        let norm_y = if f32::from_number(max_y) > f32::from_number(min_y) {
            let range_y = f32::from_number(max_y - min_y);
            let offset_y = f32::from_number(data_y - min_y);
            (offset_y / range_y).to_number()
        } else {
            0.5f32.to_number()
        };

        // Transform to screen coordinates (Y is flipped)
        let norm_x_f32 = f32::from_number(norm_x);
        let norm_y_f32 = f32::from_number(norm_y);

        let screen_x =
            draw_area.top_left.x + (norm_x_f32 * (draw_area.size.width as f32 - 1.0)) as i32;
        let screen_y = draw_area.top_left.y + draw_area.size.height as i32
            - 1
            - (norm_y_f32 * (draw_area.size.height as f32 - 1.0)) as i32;

        Point::new(screen_x, screen_y)
    }

    /// Draw markers at data points
    fn draw_markers<D>(
        &self,
        data: &crate::data::series::StaticDataSeries<crate::data::point::Point2D, 256>,
        data_bounds: &DataBounds<f32, f32>,
        viewport: Rectangle,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        if let Some(marker_style) = &self.style.markers {
            if marker_style.visible {
                for point in data.iter() {
                    let screen_point = self.transform_point(&point, data_bounds, viewport);
                    self.draw_marker(screen_point, marker_style, target)?;
                }
            }
        }
        Ok(())
    }

    /// Draw a single marker
    fn draw_marker<D>(
        &self,
        center: Point,
        marker_style: &MarkerStyle<C>,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        let style = PrimitiveStyle::with_fill(marker_style.color);
        let radius = marker_style.size / 2;

        match marker_style.shape {
            MarkerShape::Circle => {
                Circle::new(
                    Point::new(center.x - radius as i32, center.y - radius as i32),
                    marker_style.size,
                )
                .into_styled(style)
                .draw(target)
                .map_err(|_| ChartError::RenderingError)?;
            }
            MarkerShape::Square => {
                Rectangle::new(
                    Point::new(center.x - radius as i32, center.y - radius as i32),
                    Size::new(marker_style.size, marker_style.size),
                )
                .into_styled(style)
                .draw(target)
                .map_err(|_| ChartError::RenderingError)?;
            }
            MarkerShape::Diamond => {
                use crate::render::PrimitiveRenderer;
                use crate::style::FillStyle;

                let fill_style = FillStyle::solid(marker_style.color);
                PrimitiveRenderer::draw_diamond(
                    center,
                    marker_style.size,
                    None,
                    Some(&fill_style),
                    target,
                )
                .map_err(|_| ChartError::RenderingError)?;
            }
            MarkerShape::Triangle => {
                use crate::render::PrimitiveRenderer;
                use crate::style::FillStyle;

                let fill_style = FillStyle::solid(marker_style.color);
                let half_size = marker_style.size as i32 / 2;
                let p1 = Point::new(center.x, center.y - half_size);
                let p2 = Point::new(center.x - half_size, center.y + half_size);
                let p3 = Point::new(center.x + half_size, center.y + half_size);

                PrimitiveRenderer::draw_triangle(p1, p2, p3, None, Some(&fill_style), target)
                    .map_err(|_| ChartError::RenderingError)?;
            }
        }

        Ok(())
    }

    /// Draw area fill under the line
    fn draw_area_fill<D>(
        &self,
        screen_points: &heapless::Vec<Point, 512>,
        fill_color: C,
        viewport: Rectangle,
        _data_bounds: &DataBounds<f32, f32>,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        if screen_points.len() < 2 {
            return Ok(());
        }

        // Get the chart area (with margins applied)
        let chart_area = self.config.margins.apply_to(viewport);
        let baseline_y = chart_area.top_left.y + chart_area.size.height as i32 - 1;

        use embedded_graphics::primitives::{Line, PrimitiveStyle};
        let line_style = PrimitiveStyle::with_stroke(fill_color, 1);

        // Draw horizontal fill lines using scanline approach
        let min_x = screen_points
            .iter()
            .map(|p| p.x)
            .min()
            .unwrap_or(chart_area.top_left.x);
        let max_x = screen_points
            .iter()
            .map(|p| p.x)
            .max()
            .unwrap_or(chart_area.top_left.x);

        // For each x position, find the curve y and draw a vertical line to baseline
        for x in min_x..=max_x {
            if x < chart_area.top_left.x
                || x >= chart_area.top_left.x + chart_area.size.width as i32
            {
                continue;
            }

            // Find the y value on the curve at this x position
            let mut curve_y = baseline_y;

            // Linear interpolation between adjacent points
            for window in screen_points.windows(2) {
                if let [p1, p2] = window {
                    if (p1.x <= x && x <= p2.x) || (p2.x <= x && x <= p1.x) {
                        if p1.x == p2.x {
                            curve_y = p1.y.min(p2.y);
                        } else {
                            let t = (x - p1.x) as f32 / (p2.x - p1.x) as f32;
                            curve_y = (p1.y as f32 + t * (p2.y - p1.y) as f32) as i32;
                        }
                        break;
                    }
                }
            }

            // Clip curve_y to chart area
            curve_y = curve_y.clamp(
                chart_area.top_left.y,
                chart_area.top_left.y + chart_area.size.height as i32 - 1,
            );

            // Draw vertical line from curve to baseline
            if curve_y <= baseline_y {
                let top_point = Point::new(x, curve_y);
                let bottom_point = Point::new(x, baseline_y);

                Line::new(top_point, bottom_point)
                    .into_styled(line_style)
                    .draw(target)
                    .map_err(|_| ChartError::RenderingError)?;
            }
        }

        Ok(())
    }
}

impl<C: PixelColor> Default for LineChart<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<C: PixelColor + 'static> Chart<C> for LineChart<C>
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

        // First, draw grid lines from axes (background layer)
        {
            let chart_area = config.margins.apply_to(viewport);

            // Draw grid lines from X-axis
            if let Some(ref x_axis) = self.x_axis {
                x_axis.draw_grid_lines(chart_area, chart_area, target)?;
            }

            // Draw grid lines from Y-axis
            if let Some(ref y_axis) = self.y_axis {
                y_axis.draw_grid_lines(chart_area, chart_area, target)?;
            }
        }

        // Draw grid if present (legacy grid system)
        if let Some(ref grid) = self.grid {
            let chart_area = config.margins.apply_to(viewport);
            grid.draw(chart_area, target)?;
        }

        // Collect and potentially smooth the data points
        let data_to_render = if self.style.smooth && data.len() > 2 {
            // Create interpolated smooth curve
            use crate::math::interpolation::{
                CurveInterpolator, InterpolationConfig, InterpolationType,
            };

            let mut input_points = heapless::Vec::<crate::data::Point2D, 256>::new();
            for point in data.iter() {
                input_points
                    .push(point)
                    .map_err(|_| ChartError::MemoryFull)?;
            }

            let interpolation_config = InterpolationConfig {
                interpolation_type: InterpolationType::CatmullRom,
                subdivisions: self.style.smooth_subdivisions,
                tension: 0.5,
                closed: false,
            };

            let interpolated =
                CurveInterpolator::interpolate(&input_points, &interpolation_config)?;

            // Create a temporary data series with interpolated points
            let mut smooth_data = crate::data::series::StaticDataSeries::new();
            for point in interpolated.iter() {
                smooth_data
                    .push(*point)
                    .map_err(|_| ChartError::MemoryFull)?;
            }
            smooth_data
        } else {
            // Use original data
            data.clone()
        };

        // Transform data points to screen coordinates
        let mut screen_points = heapless::Vec::<Point, 512>::new();
        for point in data_to_render.iter() {
            let screen_point = self.transform_point(&point, &data_bounds, viewport);
            screen_points
                .push(screen_point)
                .map_err(|_| ChartError::MemoryFull)?;
        }

        // Draw area fill if enabled
        if self.style.fill_area {
            if let Some(fill_color) = self.style.fill_color {
                self.draw_area_fill(&screen_points, fill_color, viewport, &data_bounds, target)?;
            }
        }

        // Draw lines between consecutive points
        let line_style = PrimitiveStyle::with_stroke(self.style.line_color, self.style.line_width);
        for window in screen_points.windows(2) {
            if let [p1, p2] = window {
                Line::new(*p1, *p2)
                    .into_styled(line_style)
                    .draw(target)
                    .map_err(|_| ChartError::RenderingError)?;
            }
        }

        // Draw markers
        self.draw_markers(data, &data_bounds, viewport, target)?;

        // Finally, draw axis lines, ticks, and labels (foreground layer)
        {
            let chart_area = config.margins.apply_to(viewport);

            // Draw X-axis (without grid lines)
            if let Some(ref x_axis) = self.x_axis {
                x_axis.draw_axis_only(chart_area, target)?;
            }

            // Draw Y-axis (without grid lines)
            if let Some(ref y_axis) = self.y_axis {
                y_axis.draw_axis_only(chart_area, target)?;
            }
        }

        Ok(())
    }
}

impl<C: PixelColor> Default for LineChartStyle<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self {
            line_color: embedded_graphics::pixelcolor::Rgb565::BLUE.into(),
            line_width: 1,
            fill_area: false,
            fill_color: None,
            markers: None,
            smooth: false,
            smooth_subdivisions: 8,
        }
    }
}

impl<C: PixelColor> Default for MarkerStyle<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self {
            shape: MarkerShape::Circle,
            size: 4,
            color: embedded_graphics::pixelcolor::Rgb565::RED.into(),
            visible: true,
        }
    }
}

/// Builder for line charts
#[derive(Debug)]
pub struct LineChartBuilder<C: PixelColor> {
    style: LineChartStyle<C>,
    config: ChartConfig<C>,
    grid: Option<crate::grid::GridSystem<C>>,
    x_axis: Option<crate::axes::LinearAxis<f32, C>>,
    y_axis: Option<crate::axes::LinearAxis<f32, C>>,
}

impl<C: PixelColor> LineChartBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new line chart builder
    pub fn new() -> Self {
        Self {
            style: LineChartStyle::default(),
            config: ChartConfig::default(),
            grid: None,
            x_axis: None,
            y_axis: None,
        }
    }

    /// Set the line color
    pub fn line_color(mut self, color: C) -> Self {
        self.style.line_color = color;
        self
    }

    /// Set the line width
    pub fn line_width(mut self, width: u32) -> Self {
        self.style.line_width = width.clamp(1, 10);
        self
    }

    /// Enable area filling with the specified color
    pub fn fill_area(mut self, color: C) -> Self {
        self.style.fill_area = true;
        self.style.fill_color = Some(color);
        self
    }

    /// Add markers to data points
    pub fn with_markers(mut self, marker_style: MarkerStyle<C>) -> Self {
        self.style.markers = Some(marker_style);
        self
    }

    /// Set the chart title
    pub fn with_title(mut self, title: &str) -> Self {
        if let Ok(title_string) = heapless::String::try_from(title) {
            self.config.title = Some(title_string);
        }
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

    /// Enable smooth line rendering
    pub fn smooth(mut self, smooth: bool) -> Self {
        self.style.smooth = smooth;
        self
    }

    /// Set the number of subdivisions for smooth curves
    pub fn smooth_subdivisions(mut self, subdivisions: u32) -> Self {
        self.style.smooth_subdivisions = subdivisions.clamp(2, 16);
        self
    }

    /// Set the grid system
    pub fn with_grid(mut self, grid: crate::grid::GridSystem<C>) -> Self {
        self.grid = Some(grid);
        self
    }

    /// Set the X-axis configuration
    pub fn with_x_axis(mut self, axis: crate::axes::LinearAxis<f32, C>) -> Self {
        self.x_axis = Some(axis);
        self
    }

    /// Set the Y-axis configuration
    pub fn with_y_axis(mut self, axis: crate::axes::LinearAxis<f32, C>) -> Self {
        self.y_axis = Some(axis);
        self
    }
}

impl<C: PixelColor + 'static> ChartBuilder<C> for LineChartBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    type Chart = LineChart<C>;
    type Error = ChartError;

    fn build(self) -> Result<Self::Chart, Self::Error> {
        Ok(LineChart {
            style: self.style,
            config: self.config,
            grid: self.grid,
            x_axis: self.x_axis,
            y_axis: self.y_axis,
        })
    }
}

impl<C: PixelColor> Default for LineChartBuilder<C>
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
    use crate::data::series::StaticDataSeries;
    use crate::data::{DataBounds, Point2D};
    use crate::grid::GridSystem;
    use crate::axes::{LinearAxis, AxisOrientation, AxisPosition};
    use embedded_graphics::pixelcolor::Rgb565;
    use embedded_graphics::primitives::Rectangle;
    use embedded_graphics::mock_display::MockDisplay;

    #[test]
    fn test_line_chart_creation() {
        let chart: LineChart<Rgb565> = LineChart::new();
        assert_eq!(chart.style().line_width, 1);
        assert_eq!(chart.style().line_color, Rgb565::BLUE);
        assert!(!chart.style().fill_area);
        assert!(chart.style().fill_color.is_none());
        assert!(chart.style().markers.is_none());
        assert!(!chart.style().smooth);
        assert_eq!(chart.style().smooth_subdivisions, 8);
    }

    #[test]
    fn test_line_chart_builder() {
        let chart = LineChart::builder()
            .line_color(Rgb565::RED)
            .line_width(3)
            .build()
            .unwrap();

        assert_eq!(chart.style().line_color, Rgb565::RED);
        assert_eq!(chart.style().line_width, 3);
    }

    #[test]
    fn test_marker_style() {
        let marker = MarkerStyle {
            shape: MarkerShape::Diamond,
            size: 8,
            color: Rgb565::GREEN,
            visible: true,
        };

        assert_eq!(marker.shape, MarkerShape::Diamond);
        assert_eq!(marker.size, 8);
        assert_eq!(marker.color, Rgb565::GREEN);
        assert!(marker.visible);
    }

    #[test]
    fn test_line_chart_default() {
        let chart: LineChart<Rgb565> = LineChart::default();
        assert_eq!(chart.style().line_color, Rgb565::BLUE);
        assert_eq!(chart.style().line_width, 1);
    }

    #[test]
    fn test_line_chart_style_default() {
        let style: LineChartStyle<Rgb565> = LineChartStyle::default();
        assert_eq!(style.line_color, Rgb565::BLUE);
        assert_eq!(style.line_width, 1);
        assert!(!style.fill_area);
        assert!(style.fill_color.is_none());
        assert!(style.markers.is_none());
        assert!(!style.smooth);
        assert_eq!(style.smooth_subdivisions, 8);
    }

    #[test]
    fn test_marker_style_default() {
        let marker: MarkerStyle<Rgb565> = MarkerStyle::default();
        assert_eq!(marker.shape, MarkerShape::Circle);
        assert_eq!(marker.size, 4);
        assert_eq!(marker.color, Rgb565::RED);
        assert!(marker.visible);
    }

    #[test]
    fn test_line_chart_builder_default() {
        let builder: LineChartBuilder<Rgb565> = LineChartBuilder::default();
        let chart = builder.build().unwrap();
        assert_eq!(chart.style().line_color, Rgb565::BLUE);
    }

    #[test]
    fn test_setters() {
        let mut chart: LineChart<Rgb565> = LineChart::new();
        
        // Test style setter
        let style = LineChartStyle {
            line_color: Rgb565::MAGENTA,
            line_width: 5,
            fill_area: true,
            fill_color: Some(Rgb565::CYAN),
            markers: Some(MarkerStyle::default()),
            smooth: true,
            smooth_subdivisions: 12,
        };
        chart.set_style(style.clone());
        assert_eq!(chart.style().line_color, Rgb565::MAGENTA);
        assert_eq!(chart.style().line_width, 5);
        assert!(chart.style().fill_area);
        
        // Test config setter
        let config = ChartConfig {
            title: None,
            background_color: Some(Rgb565::BLACK),
            margins: Margins::all(20),
            show_grid: true,
            grid_color: Some(Rgb565::CSS_GRAY),
        };
        chart.set_config(config);
        assert_eq!(chart.config().margins.top, 20);
        
        // Test grid setter
        let grid = GridSystem::new();
        chart.set_grid(Some(grid));
        assert!(chart.grid().is_some());
        
        chart.set_grid(None);
        assert!(chart.grid().is_none());
    }

    #[test]
    fn test_builder_all_options() {
        let grid = GridSystem::new();
        let x_axis = LinearAxis::new(0.0, 100.0, AxisOrientation::Horizontal, AxisPosition::Bottom);
        let y_axis = LinearAxis::new(0.0, 50.0, AxisOrientation::Vertical, AxisPosition::Left);
        
        let chart = LineChart::builder()
            .line_color(Rgb565::GREEN)
            .line_width(4)
            .fill_area(Rgb565::CSS_LIGHT_GREEN)
            .with_markers(MarkerStyle {
                shape: MarkerShape::Square,
                size: 6,
                color: Rgb565::RED,
                visible: true,
            })
            .smooth(true)
            .smooth_subdivisions(16)
            .with_title("Test Chart")
            .background_color(Rgb565::BLACK)
            .margins(Margins::new(5, 10, 15, 20))
            .with_grid(grid)
            .with_x_axis(x_axis)
            .with_y_axis(y_axis)
            .build()
            .unwrap();
        
        assert_eq!(chart.style().line_color, Rgb565::GREEN);
        assert_eq!(chart.style().line_width, 4);
        assert!(chart.style().fill_area);
        assert_eq!(chart.style().fill_color, Some(Rgb565::CSS_LIGHT_GREEN));
        assert!(chart.style().markers.is_some());
        assert!(chart.style().smooth);
        assert_eq!(chart.style().smooth_subdivisions, 16);
        assert_eq!(chart.config().margins.top, 5);
        assert_eq!(chart.config().margins.right, 10);
        assert_eq!(chart.config().margins.bottom, 15);
        assert_eq!(chart.config().margins.left, 20);
        assert!(chart.grid().is_some());
    }

    #[test]
    fn test_builder_edge_cases() {
        // Test line width clamping
        let chart: LineChart<Rgb565> = LineChart::builder()
            .line_width(50) // Should be clamped to 10
            .build()
            .unwrap();
        assert_eq!(chart.style().line_width, 10); // Clamped to 10, not 20
        
        // Test smooth subdivisions clamping
        let chart: LineChart<Rgb565> = LineChart::builder()
            .smooth(true)
            .smooth_subdivisions(100) // Should be clamped to 16
            .build()
            .unwrap();
        assert_eq!(chart.style().smooth_subdivisions, 16);
        
        // Test minimum subdivisions
        let chart: LineChart<Rgb565> = LineChart::builder()
            .smooth(true)
            .smooth_subdivisions(0) // Should be clamped to 2
            .build()
            .unwrap();
        assert_eq!(chart.style().smooth_subdivisions, 2);
    }

    #[test]
    fn test_transform_point_no_axes() {
        let chart: LineChart<Rgb565> = LineChart::new();
        let viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 100));
        let bounds = DataBounds::<f32, f32> {
            min_x: 0.0,
            max_x: 10.0,
            min_y: 0.0,
            max_y: 20.0,
        };
        
        // Test origin point
        let point = Point2D::new(0.0, 0.0);
        let screen_point = chart.transform_point(&point, &bounds, viewport);
        assert_eq!(screen_point.x, 10); // Left margin
        assert_eq!(screen_point.y, 89); // Bottom minus margin
        
        // Test max point
        let point = Point2D::new(10.0, 20.0);
        let screen_point = chart.transform_point(&point, &bounds, viewport);
        assert_eq!(screen_point.x, 189); // Right minus margin
        assert_eq!(screen_point.y, 10); // Top margin
    }

    #[test]
    fn test_transform_point_equal_bounds() {
        let chart: LineChart<Rgb565> = LineChart::new();
        let viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 100));
        
        // Test with equal min/max bounds
        let bounds = DataBounds::<f32, f32> {
            min_x: 5.0,
            max_x: 5.0,
            min_y: 10.0,
            max_y: 10.0,
        };
        
        let point = Point2D::new(5.0, 10.0);
        let screen_point = chart.transform_point(&point, &bounds, viewport);
        
        // Should center the point
        assert_eq!(screen_point.x, 99); // Center X
        assert_eq!(screen_point.y, 50); // Center Y
    }

    #[test]
    fn test_draw_empty_data() {
        let chart: LineChart<Rgb565> = LineChart::new();
        let config = ChartConfig::default();
        let viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 100));
        let mut display: MockDisplay<Rgb565> = MockDisplay::new();
        display.set_allow_overdraw(true);
        display.set_allow_out_of_bounds_drawing(true);
        
        let data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
        
        let result = chart.draw(&data, &config, viewport, &mut display);
        assert!(matches!(result, Err(ChartError::InsufficientData)));
    }

    #[test]
    fn test_draw_single_point() {
        let chart: LineChart<Rgb565> = LineChart::new();
        let config = ChartConfig::default();
        let viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 100));
        let mut display: MockDisplay<Rgb565> = MockDisplay::new();
        display.set_allow_overdraw(true);
        display.set_allow_out_of_bounds_drawing(true);
        
        let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
        data.push(Point2D::new(5.0, 10.0)).unwrap();
        
        let result = chart.draw(&data, &config, viewport, &mut display);
        assert!(result.is_ok());
    }

    #[test]
    fn test_draw_with_background() {
        let chart = LineChart::builder()
            .background_color(Rgb565::BLACK)
            .build()
            .unwrap();
            
        let mut config = ChartConfig::default();
        config.background_color = Some(Rgb565::WHITE);
        
        let viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 100));
        let mut display: MockDisplay<Rgb565> = MockDisplay::new();
        display.set_allow_overdraw(true);
        display.set_allow_out_of_bounds_drawing(true);
        
        let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
        data.push(Point2D::new(0.0, 0.0)).unwrap();
        data.push(Point2D::new(10.0, 10.0)).unwrap();
        
        let result = chart.draw(&data, &config, viewport, &mut display);
        assert!(result.is_ok());
    }

    #[test]
    fn test_draw_all_marker_shapes() {
        let shapes = [
            MarkerShape::Circle,
            MarkerShape::Square,
            MarkerShape::Diamond,
            MarkerShape::Triangle,
        ];
        
        for shape in shapes {
            let chart = LineChart::builder()
                .with_markers(MarkerStyle {
                    shape,
                    size: 6,
                    color: Rgb565::RED,
                    visible: true,
                })
                .build()
                .unwrap();
                
            let config = ChartConfig::default();
            let viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 100));
            let mut display: MockDisplay<Rgb565> = MockDisplay::new();
            display.set_allow_overdraw(true);
            display.set_allow_out_of_bounds_drawing(true);
            
            let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
            data.push(Point2D::new(0.0, 0.0)).unwrap();
            data.push(Point2D::new(5.0, 10.0)).unwrap();
            data.push(Point2D::new(10.0, 5.0)).unwrap();
            
            let result = chart.draw(&data, &config, viewport, &mut display);
            assert!(result.is_ok(), "Failed to draw marker shape: {:?}", shape);
        }
    }

    #[test]
    fn test_draw_invisible_markers() {
        let chart = LineChart::builder()
            .with_markers(MarkerStyle {
                shape: MarkerShape::Circle,
                size: 6,
                color: Rgb565::RED,
                visible: false, // Invisible
            })
            .build()
            .unwrap();
            
        let config = ChartConfig::default();
        let viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 100));
        let mut display: MockDisplay<Rgb565> = MockDisplay::new();
        display.set_allow_overdraw(true);
        display.set_allow_out_of_bounds_drawing(true);
        
        let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
        data.push(Point2D::new(0.0, 0.0)).unwrap();
        data.push(Point2D::new(10.0, 10.0)).unwrap();
        
        let result = chart.draw(&data, &config, viewport, &mut display);
        assert!(result.is_ok());
    }

    #[test]
    fn test_draw_with_area_fill() {
        let chart = LineChart::builder()
            .line_color(Rgb565::BLUE)
            .fill_area(Rgb565::CSS_LIGHT_BLUE)
            .build()
            .unwrap();
            
        let config = ChartConfig::default();
        let viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 100));
        let mut display: MockDisplay<Rgb565> = MockDisplay::new();
        display.set_allow_overdraw(true);
        display.set_allow_out_of_bounds_drawing(true);
        
        let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
        data.push(Point2D::new(0.0, 5.0)).unwrap();
        data.push(Point2D::new(5.0, 15.0)).unwrap();
        data.push(Point2D::new(10.0, 10.0)).unwrap();
        
        let result = chart.draw(&data, &config, viewport, &mut display);
        assert!(result.is_ok());
    }

    #[test]
    fn test_draw_smooth_curve() {
        let chart = LineChart::builder()
            .line_color(Rgb565::GREEN)
            .smooth(true)
            .smooth_subdivisions(8)
            .build()
            .unwrap();
            
        let config = ChartConfig::default();
        let viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 100));
        let mut display: MockDisplay<Rgb565> = MockDisplay::new();
        display.set_allow_overdraw(true);
        display.set_allow_out_of_bounds_drawing(true);
        
        let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
        data.push(Point2D::new(0.0, 0.0)).unwrap();
        data.push(Point2D::new(5.0, 20.0)).unwrap();
        data.push(Point2D::new(10.0, 10.0)).unwrap();
        
        let result = chart.draw(&data, &config, viewport, &mut display);
        assert!(result.is_ok());
    }

    #[test]
    fn test_draw_smooth_curve_insufficient_points() {
        let chart = LineChart::builder()
            .smooth(true)
            .build()
            .unwrap();
            
        let config = ChartConfig::default();
        let viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 100));
        let mut display: MockDisplay<Rgb565> = MockDisplay::new();
        display.set_allow_overdraw(true);
        display.set_allow_out_of_bounds_drawing(true);
        
        let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
        data.push(Point2D::new(0.0, 0.0)).unwrap();
        data.push(Point2D::new(10.0, 10.0)).unwrap();
        
        // Should fall back to regular line with only 2 points
        let result = chart.draw(&data, &config, viewport, &mut display);
        assert!(result.is_ok());
    }

    #[test]
    fn test_draw_with_axes() {
        let mut chart: LineChart<Rgb565> = LineChart::new();
        let x_axis = LinearAxis::new(0.0, 100.0, AxisOrientation::Horizontal, AxisPosition::Bottom);
        let y_axis = LinearAxis::new(0.0, 50.0, AxisOrientation::Vertical, AxisPosition::Left);
        
        chart.set_x_axis(x_axis);
        chart.set_y_axis(y_axis);
        
        let config = ChartConfig::default();
        let viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 100));
        let mut display: MockDisplay<Rgb565> = MockDisplay::new();
        display.set_allow_overdraw(true);
        display.set_allow_out_of_bounds_drawing(true);
        
        let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
        data.push(Point2D::new(0.0, 0.0)).unwrap();
        data.push(Point2D::new(50.0, 25.0)).unwrap();
        data.push(Point2D::new(100.0, 50.0)).unwrap();
        
        let result = chart.draw(&data, &config, viewport, &mut display);
        assert!(result.is_ok());
    }

    #[test]
    fn test_axis_getters() {
        let mut chart: LineChart<Rgb565> = LineChart::new();
        
        // Test missing axes
        assert!(matches!(chart.x_axis(), Err(ChartError::InvalidConfiguration)));
        assert!(matches!(chart.y_axis(), Err(ChartError::InvalidConfiguration)));
        
        // Test with axes
        let x_axis = LinearAxis::new(0.0, 100.0, AxisOrientation::Horizontal, AxisPosition::Bottom);
        let y_axis = LinearAxis::new(0.0, 50.0, AxisOrientation::Vertical, AxisPosition::Left);
        
        chart.set_x_axis(x_axis);
        chart.set_y_axis(y_axis);
        
        assert!(chart.x_axis().is_ok());
        assert!(chart.y_axis().is_ok());
    }

    #[test]
    fn test_marker_shape_equality() {
        assert_eq!(MarkerShape::Circle, MarkerShape::Circle);
        assert_ne!(MarkerShape::Circle, MarkerShape::Square);
        assert_ne!(MarkerShape::Square, MarkerShape::Diamond);
        assert_ne!(MarkerShape::Diamond, MarkerShape::Triangle);
    }

    #[test]
    fn test_large_data_set() {
        let chart = LineChart::new();
        let config = ChartConfig::default();
        let viewport = Rectangle::new(Point::new(0, 0), Size::new(320, 240));
        let mut display: MockDisplay<Rgb565> = MockDisplay::new();
        display.set_allow_overdraw(true);
        display.set_allow_out_of_bounds_drawing(true);
        
        let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
        
        // Fill with maximum points
        for i in 0..100 {
            data.push(Point2D::new(i as f32, (i * 2) as f32)).unwrap();
        }
        
        let result = chart.draw(&data, &config, viewport, &mut display);
        assert!(result.is_ok());
    }

    #[test]
    fn test_viewport_edge_cases() {
        let chart = LineChart::new();
        let config = ChartConfig::default();
        
        // Very small viewport
        let viewport = Rectangle::new(Point::new(0, 0), Size::new(10, 10));
        let mut display: MockDisplay<Rgb565> = MockDisplay::new();
        display.set_allow_overdraw(true);
        display.set_allow_out_of_bounds_drawing(true);
        
        let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
        data.push(Point2D::new(0.0, 0.0)).unwrap();
        data.push(Point2D::new(10.0, 10.0)).unwrap();
        
        let result = chart.draw(&data, &config, viewport, &mut display);
        assert!(result.is_ok());
    }

    #[test]
    fn test_negative_data_values() {
        let chart = LineChart::new();
        let config = ChartConfig::default();
        let viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 100));
        let mut display: MockDisplay<Rgb565> = MockDisplay::new();
        display.set_allow_overdraw(true);
        display.set_allow_out_of_bounds_drawing(true);
        
        let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
        data.push(Point2D::new(-10.0, -20.0)).unwrap();
        data.push(Point2D::new(0.0, 0.0)).unwrap();
        data.push(Point2D::new(10.0, -10.0)).unwrap();
        
        let result = chart.draw(&data, &config, viewport, &mut display);
        assert!(result.is_ok());
    }

    #[test]
    fn test_transform_point_with_axes() {
        let mut chart: LineChart<Rgb565> = LineChart::new();
        let x_axis = LinearAxis::new(-50.0, 50.0, AxisOrientation::Horizontal, AxisPosition::Bottom);
        let y_axis = LinearAxis::new(-100.0, 100.0, AxisOrientation::Vertical, AxisPosition::Left);
        
        chart.set_x_axis(x_axis);
        chart.set_y_axis(y_axis);
        
        let viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 100));
        let bounds = DataBounds::<f32, f32> {
            min_x: -10.0,
            max_x: 10.0,
            min_y: -20.0,
            max_y: 20.0,
        };
        
        // Test origin point (0,0) which should be in the center
        let point = Point2D::new(0.0, 0.0);
        let screen_point = chart.transform_point(&point, &bounds, viewport);
        
        // Since axes range from -50 to 50 and -100 to 100, origin should be centered
        assert_eq!(screen_point.x, 99); // Center X with margins
        assert_eq!(screen_point.y, 50); // Center Y with margins
    }
}

impl<C: PixelColor + 'static> AxisChart<C> for LineChart<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    type XAxis = crate::axes::LinearAxis<f32, C>;
    type YAxis = crate::axes::LinearAxis<f32, C>;

    fn set_x_axis(&mut self, axis: crate::axes::LinearAxis<f32, C>) {
        self.x_axis = Some(axis);
    }

    fn set_y_axis(&mut self, axis: crate::axes::LinearAxis<f32, C>) {
        self.y_axis = Some(axis);
    }

    fn x_axis(&self) -> ChartResult<&crate::axes::LinearAxis<f32, C>> {
        self.x_axis.as_ref().ok_or(ChartError::InvalidConfiguration)
    }

    fn y_axis(&self) -> ChartResult<&crate::axes::LinearAxis<f32, C>> {
        self.y_axis.as_ref().ok_or(ChartError::InvalidConfiguration)
    }
}

/// Animated line chart that extends LineChart with animation capabilities
#[cfg(feature = "animations")]
#[derive(Debug)]
pub struct AnimatedLineChart<C: PixelColor> {
    /// Base line chart
    base_chart: LineChart<C>,
    /// Current animated data (interpolated values)
    current_data: Option<crate::data::series::StaticDataSeries<crate::data::point::Point2D, 256>>,
}

#[cfg(feature = "animations")]
impl<C: PixelColor + 'static> AnimatedLineChart<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new animated line chart
    pub fn new() -> Self {
        Self {
            base_chart: LineChart::new(),
            current_data: None,
        }
    }

    /// Create a builder for configuring the animated line chart
    pub fn builder() -> AnimatedLineChartBuilder<C> {
        AnimatedLineChartBuilder::new()
    }

    /// Set the line style
    pub fn set_style(&mut self, style: LineChartStyle<C>) {
        self.base_chart.set_style(style);
    }

    /// Get the current line style
    pub fn style(&self) -> &LineChartStyle<C> {
        self.base_chart.style()
    }

    /// Set the chart configuration
    pub fn set_config(&mut self, config: ChartConfig<C>) {
        self.base_chart.set_config(config);
    }

    /// Get the chart configuration
    pub fn config(&self) -> &ChartConfig<C> {
        self.base_chart.config()
    }

    /// Set the grid system
    pub fn set_grid(&mut self, grid: Option<crate::grid::GridSystem<C>>) {
        self.base_chart.set_grid(grid);
    }

    /// Get the grid system
    pub fn grid(&self) -> Option<&crate::grid::GridSystem<C>> {
        self.base_chart.grid()
    }

    /// Set the current animated data for rendering
    pub fn set_animated_data(
        &mut self,
        data: Option<crate::data::series::StaticDataSeries<crate::data::point::Point2D, 256>>,
    ) {
        self.current_data = data;
    }

    /// Get the current animated data
    pub fn animated_data(
        &self,
    ) -> Option<&crate::data::series::StaticDataSeries<crate::data::point::Point2D, 256>> {
        self.current_data.as_ref()
    }

    /// Get access to the base chart for configuration
    pub fn base_chart(&self) -> &LineChart<C> {
        &self.base_chart
    }

    /// Get mutable access to the base chart for configuration
    pub fn base_chart_mut(&mut self) -> &mut LineChart<C> {
        &mut self.base_chart
    }

    /// Interpolate between two data series using a ChartAnimator
    pub fn interpolate_with_animator(
        animator: &crate::animation::ChartAnimator<
            crate::data::series::StaticDataSeries<crate::data::point::Point2D, 256>,
        >,
        progress: crate::animation::Progress,
    ) -> Option<crate::data::series::StaticDataSeries<crate::data::point::Point2D, 256>> {
        animator.value_at(progress)
    }
}

#[cfg(feature = "animations")]
impl<C: PixelColor + 'static> Default for AnimatedLineChart<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "animations")]
impl<C: PixelColor + 'static> Chart<C> for AnimatedLineChart<C>
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
        Self::Data: crate::data::DataSeries,
        <Self::Data as crate::data::DataSeries>::Item: crate::data::DataPoint,
        <<Self::Data as crate::data::DataSeries>::Item as crate::data::DataPoint>::X:
            Into<f32> + Copy + PartialOrd,
        <<Self::Data as crate::data::DataSeries>::Item as crate::data::DataPoint>::Y:
            Into<f32> + Copy + PartialOrd,
    {
        // Use animated data if available, otherwise use provided data
        if let Some(ref animated_data) = self.current_data {
            self.base_chart
                .draw(animated_data, config, viewport, target)
        } else {
            self.base_chart.draw(data, config, viewport, target)
        }
    }
}

#[cfg(feature = "animations")]
impl<C: PixelColor + 'static> crate::chart::traits::AnimatedChart<C> for AnimatedLineChart<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    type AnimatedData = crate::data::series::StaticDataSeries<crate::data::point::Point2D, 256>;

    fn draw_animated<D>(
        &self,
        data: &Self::Data,
        config: &Self::Config,
        viewport: Rectangle,
        target: &mut D,
        _progress: crate::animation::Progress,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        // Use the provided data which should already be interpolated by the caller
        self.base_chart.draw(data, config, viewport, target)
    }

    fn create_transition_animator(
        &self,
        from_data: Self::AnimatedData,
        to_data: Self::AnimatedData,
        easing: crate::animation::EasingFunction,
    ) -> crate::animation::ChartAnimator<Self::AnimatedData> {
        crate::animation::ChartAnimator::new(from_data, to_data, easing)
    }

    fn extract_animated_data(&self, data: &Self::Data) -> ChartResult<Self::AnimatedData> {
        // Clone the data series for animation
        Ok(data.clone())
    }
}

/// Builder for animated line charts
#[cfg(feature = "animations")]
#[derive(Debug)]
pub struct AnimatedLineChartBuilder<C: PixelColor> {
    base_builder: LineChartBuilder<C>,
    frame_rate: u32,
}

#[cfg(feature = "animations")]
impl<C: PixelColor + 'static> AnimatedLineChartBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new animated line chart builder
    pub fn new() -> Self {
        Self {
            base_builder: LineChartBuilder::new(),
            frame_rate: 60,
        }
    }

    /// Set the target frame rate
    pub fn frame_rate(mut self, fps: u32) -> Self {
        self.frame_rate = fps.clamp(1, 120);
        self
    }

    /// Set the line color
    pub fn line_color(mut self, color: C) -> Self {
        self.base_builder = self.base_builder.line_color(color);
        self
    }

    /// Set the line width
    pub fn line_width(mut self, width: u32) -> Self {
        self.base_builder = self.base_builder.line_width(width);
        self
    }

    /// Enable area fill with color
    pub fn fill_area(mut self, color: C) -> Self {
        self.base_builder = self.base_builder.fill_area(color);
        self
    }

    /// Add markers to data points
    pub fn with_markers(mut self, marker_style: MarkerStyle<C>) -> Self {
        self.base_builder = self.base_builder.with_markers(marker_style);
        self
    }

    /// Set the chart title
    pub fn with_title(mut self, title: &str) -> Self {
        self.base_builder = self.base_builder.with_title(title);
        self
    }

    /// Set the background color
    pub fn background_color(mut self, color: C) -> Self {
        self.base_builder = self.base_builder.background_color(color);
        self
    }

    /// Set chart margins
    pub fn margins(mut self, margins: Margins) -> Self {
        self.base_builder = self.base_builder.margins(margins);
        self
    }

    /// Enable smooth lines
    pub fn smooth(mut self, smooth: bool) -> Self {
        self.base_builder = self.base_builder.smooth(smooth);
        self
    }

    /// Set the number of subdivisions for smooth curves
    pub fn smooth_subdivisions(mut self, subdivisions: u32) -> Self {
        self.base_builder = self.base_builder.smooth_subdivisions(subdivisions);
        self
    }

    /// Add grid system
    pub fn with_grid(mut self, grid: crate::grid::GridSystem<C>) -> Self {
        self.base_builder = self.base_builder.with_grid(grid);
        self
    }

    /// Add X-axis
    pub fn with_x_axis(mut self, axis: crate::axes::LinearAxis<f32, C>) -> Self {
        self.base_builder = self.base_builder.with_x_axis(axis);
        self
    }

    /// Add Y-axis
    pub fn with_y_axis(mut self, axis: crate::axes::LinearAxis<f32, C>) -> Self {
        self.base_builder = self.base_builder.with_y_axis(axis);
        self
    }

    /// Build the animated line chart
    pub fn build(self) -> ChartResult<AnimatedLineChart<C>> {
        let base_chart = self.base_builder.build()?;

        Ok(AnimatedLineChart {
            base_chart,
            current_data: None,
        })
    }
}

#[cfg(feature = "animations")]
impl<C: PixelColor + 'static> Default for AnimatedLineChartBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::new()
    }
}
