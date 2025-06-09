//! Smooth curve chart implementation with interpolation support.
//!
//! This module provides a specialized chart type for displaying smooth curves using
//! various interpolation algorithms. It extends the basic line chart functionality
//! with advanced curve generation capabilities.

use crate::chart::line::{LineChart, LineChartBuilder, LineChartStyle, MarkerStyle};
use crate::chart::traits::{Chart, ChartBuilder, ChartConfig};
use crate::data::{DataPoint, DataSeries, Point2D};
use crate::error::{ChartError, ChartResult};
use crate::math::interpolation::{CurveInterpolator, InterpolationConfig, InterpolationType};
use embedded_graphics::{draw_target::DrawTarget, prelude::*};
use heapless::Vec;

/// A smooth curve chart that uses interpolation to create fluid curves from discrete data points.
///
/// This chart type builds upon the LineChart foundation but adds sophisticated curve
/// interpolation capabilities including cubic splines, Catmull-Rom curves, and Bezier curves.
/// It automatically generates additional points between input data to create smooth, visually
/// appealing curves.
///
/// # Features
///
/// - Multiple interpolation algorithms (cubic spline, Catmull-Rom, Bezier, linear)
/// - Configurable curve smoothness and tension
/// - Memory-efficient implementation suitable for embedded systems
/// - Integration with existing chart styling and theming
/// - Support for markers, area fills, and grid systems
///
/// # Examples
///
/// Basic smooth curve:
/// ```rust
/// use embedded_charts::prelude::*;
/// use embedded_graphics::pixelcolor::Rgb565;
///
/// let chart = CurveChart::builder()
///     .line_color(Rgb565::BLUE)
///     .interpolation_type(InterpolationType::CubicSpline)
///     .subdivisions(12)
///     .build()?;
/// # Ok::<(), embedded_charts::error::ChartError>(())
/// ```
///
/// Artistic Bezier curves:
/// ```rust
/// use embedded_charts::prelude::*;
/// use embedded_graphics::pixelcolor::Rgb565;
///
/// let chart = CurveChart::builder()
///     .line_color(Rgb565::GREEN)
///     .interpolation_type(InterpolationType::Bezier)
///     .tension(0.8)
///     .subdivisions(16)
///     .with_markers(MarkerStyle::default())
///     .build()?;
/// # Ok::<(), embedded_charts::error::ChartError>(())
/// ```
#[derive(Debug)]
pub struct CurveChart<C: PixelColor> {
    /// Base line chart for rendering and styling
    base_chart: LineChart<C>,
    /// Interpolation configuration
    interpolation_config: InterpolationConfig,
}

impl<C: PixelColor + 'static> CurveChart<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new curve chart with default settings.
    ///
    /// Creates a curve chart with:
    /// - Cubic spline interpolation
    /// - 8 subdivisions per segment
    /// - Medium tension (0.5)
    /// - Default line chart styling
    pub fn new() -> Self {
        Self {
            base_chart: LineChart::new(),
            interpolation_config: InterpolationConfig::default(),
        }
    }

    /// Create a builder for configuring the curve chart.
    pub fn builder() -> CurveChartBuilder<C> {
        CurveChartBuilder::new()
    }

    /// Set the interpolation configuration.
    ///
    /// # Arguments
    /// * `config` - The interpolation configuration to use
    pub fn set_interpolation_config(&mut self, config: InterpolationConfig) {
        self.interpolation_config = config;
    }

    /// Get the current interpolation configuration.
    pub fn interpolation_config(&self) -> &InterpolationConfig {
        &self.interpolation_config
    }

    /// Set the line style configuration.
    pub fn set_style(&mut self, style: LineChartStyle<C>) {
        self.base_chart.set_style(style);
    }

    /// Get the current line style configuration.
    pub fn style(&self) -> &LineChartStyle<C> {
        self.base_chart.style()
    }

    /// Set the chart configuration.
    pub fn set_config(&mut self, config: ChartConfig<C>) {
        self.base_chart.set_config(config);
    }

    /// Get the current chart configuration.
    pub fn config(&self) -> &ChartConfig<C> {
        self.base_chart.config()
    }

    /// Set the grid system for the chart.
    pub fn set_grid(&mut self, grid: Option<crate::grid::GridSystem<C>>) {
        self.base_chart.set_grid(grid);
    }

    /// Get the current grid system configuration.
    pub fn grid(&self) -> Option<&crate::grid::GridSystem<C>> {
        self.base_chart.grid()
    }

    /// Get access to the underlying line chart for advanced configuration.
    pub fn base_chart(&self) -> &LineChart<C> {
        &self.base_chart
    }

    /// Get mutable access to the underlying line chart.
    pub fn base_chart_mut(&mut self) -> &mut LineChart<C> {
        &mut self.base_chart
    }

    /// Generate interpolated curve points from input data.
    fn interpolate_data(
        &self,
        data: &crate::data::series::StaticDataSeries<Point2D, 256>,
    ) -> ChartResult<Vec<Point2D, 512>> {
        // Convert data series to slice for interpolation
        let mut points = Vec::<Point2D, 256>::new();
        for point in data.iter() {
            points.push(point).map_err(|_| ChartError::MemoryFull)?;
        }

        // Perform interpolation
        CurveInterpolator::interpolate(&points, &self.interpolation_config)
    }

    /// Transform a data point to screen coordinates using the same logic as LineChart
    fn transform_curve_point(
        &self,
        point: &Point2D,
        data_bounds: &crate::data::DataBounds<f32, f32>,
        viewport: embedded_graphics::primitives::Rectangle,
    ) -> embedded_graphics::prelude::Point {
        use crate::math::NumericConversion;

        // Convert to math abstraction layer (same as LineChart)
        let data_x = point.x.to_number();
        let data_y = point.y.to_number();

        // Use the same bounds as LineChart would
        let min_x = data_bounds.min_x.to_number();
        let max_x = data_bounds.max_x.to_number();
        let min_y = data_bounds.min_y.to_number();
        let max_y = data_bounds.max_y.to_number();

        // Apply margins to get the actual drawing area (same as LineChart)
        let draw_area = self.base_chart.config().margins.apply_to(viewport);

        // Normalize to 0-1 range using math abstraction (same as LineChart)
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

        // Transform to screen coordinates (Y is flipped) - same as LineChart
        let norm_x_f32 = f32::from_number(norm_x);
        let norm_y_f32 = f32::from_number(norm_y);

        let screen_x =
            draw_area.top_left.x + (norm_x_f32 * (draw_area.size.width as f32 - 1.0)) as i32;
        let screen_y = draw_area.top_left.y + draw_area.size.height as i32
            - 1
            - (norm_y_f32 * (draw_area.size.height as f32 - 1.0)) as i32;

        embedded_graphics::prelude::Point::new(screen_x, screen_y)
    }
}

impl<C: PixelColor + 'static> Default for CurveChart<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<C: PixelColor + 'static> Chart<C> for CurveChart<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    type Data = crate::data::series::StaticDataSeries<Point2D, 256>;
    type Config = ChartConfig<C>;

    fn draw<D>(
        &self,
        data: &Self::Data,
        config: &Self::Config,
        viewport: embedded_graphics::primitives::Rectangle,
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

        // Handle case with only one point (can't interpolate)
        if data.len() == 1 {
            return self.base_chart.draw(data, config, viewport, target);
        }

        // Generate interpolated curve points
        let interpolated_points = self.interpolate_data(data)?;

        // Create a temporary data series with interpolated points
        let mut curve_data = crate::data::series::StaticDataSeries::new();
        for point in interpolated_points.iter() {
            curve_data
                .push(*point)
                .map_err(|_| ChartError::MemoryFull)?;
        }

        // Save the original marker style and remove it temporarily
        let original_markers = self.base_chart.style().markers;

        // Create a temporary chart without markers for drawing the curve
        let mut temp_chart = LineChart::builder()
            .line_color(self.base_chart.style().line_color)
            .line_width(self.base_chart.style().line_width)
            .fill_area(
                self.base_chart
                    .style()
                    .fill_color
                    .unwrap_or(self.base_chart.style().line_color),
            )
            .smooth(false) // Already interpolated
            .build()?;

        if self.base_chart.style().fill_area {
            if let Some(fill_color) = self.base_chart.style().fill_color {
                temp_chart = LineChart::builder()
                    .line_color(self.base_chart.style().line_color)
                    .line_width(self.base_chart.style().line_width)
                    .fill_area(fill_color)
                    .smooth(false)
                    .build()?;
            }
        } else {
            temp_chart = LineChart::builder()
                .line_color(self.base_chart.style().line_color)
                .line_width(self.base_chart.style().line_width)
                .smooth(false)
                .build()?;
        }

        // Draw the smooth curve without markers
        temp_chart.draw(&curve_data, config, viewport, target)?;

        // Now draw markers at original data points manually
        if let Some(marker_style) = original_markers {
            if marker_style.visible {
                use embedded_graphics::primitives::{Circle, PrimitiveStyle};

                let data_bounds = data.bounds()?;

                for original_point in data.iter() {
                    // Convert to Point2D for transformation
                    let point_2d = crate::data::Point2D::new(original_point.x, original_point.y);
                    // Transform original data point to screen coordinates
                    let screen_point =
                        self.transform_curve_point(&point_2d, &data_bounds, viewport);

                    // Draw marker
                    let marker_primitive_style = PrimitiveStyle::with_fill(marker_style.color);
                    let radius = marker_style.size / 2;

                    Circle::new(
                        embedded_graphics::prelude::Point::new(
                            screen_point.x - radius as i32,
                            screen_point.y - radius as i32,
                        ),
                        marker_style.size,
                    )
                    .into_styled(marker_primitive_style)
                    .draw(target)
                    .map_err(|_| ChartError::RenderingError)?;
                }
            }
        }

        Ok(())
    }
}

/// Builder for curve charts with fluent configuration API.
#[derive(Debug)]
pub struct CurveChartBuilder<C: PixelColor> {
    /// Base line chart builder
    line_builder: LineChartBuilder<C>,
    /// Interpolation configuration
    interpolation_config: InterpolationConfig,
}

impl<C: PixelColor + 'static> CurveChartBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new curve chart builder.
    pub fn new() -> Self {
        Self {
            line_builder: LineChartBuilder::new(),
            interpolation_config: InterpolationConfig::default(),
        }
    }

    /// Set the interpolation algorithm to use.
    ///
    /// # Arguments
    /// * `interpolation_type` - The type of curve interpolation
    pub fn interpolation_type(mut self, interpolation_type: InterpolationType) -> Self {
        self.interpolation_config.interpolation_type = interpolation_type;
        self
    }

    /// Set the number of subdivisions between data points.
    ///
    /// Higher values create smoother curves but require more memory and processing.
    /// Recommended range: 4-20 subdivisions.
    ///
    /// # Arguments
    /// * `subdivisions` - Number of interpolated points between each pair of data points
    pub fn subdivisions(mut self, subdivisions: u32) -> Self {
        self.interpolation_config.subdivisions = subdivisions.clamp(2, 32);
        self
    }

    /// Set the curve tension for spline interpolation.
    ///
    /// # Arguments
    /// * `tension` - Tension value (0.0 = loose curves, 1.0 = tight curves)
    pub fn tension(mut self, tension: f32) -> Self {
        self.interpolation_config.tension = tension.clamp(0.0, 1.0);
        self
    }

    /// Enable closed curve (connect last point to first).
    ///
    /// # Arguments
    /// * `closed` - Whether to create a closed curve
    pub fn closed(mut self, closed: bool) -> Self {
        self.interpolation_config.closed = closed;
        self
    }

    /// Set the line color.
    pub fn line_color(mut self, color: C) -> Self {
        self.line_builder = self.line_builder.line_color(color);
        self
    }

    /// Set the line width.
    pub fn line_width(mut self, width: u32) -> Self {
        self.line_builder = self.line_builder.line_width(width);
        self
    }

    /// Enable area filling with the specified color.
    pub fn fill_area(mut self, color: C) -> Self {
        self.line_builder = self.line_builder.fill_area(color);
        self
    }

    /// Add markers to original data points.
    ///
    /// Note: Markers are only placed at the original data points, not the interpolated points.
    pub fn with_markers(mut self, marker_style: MarkerStyle<C>) -> Self {
        self.line_builder = self.line_builder.with_markers(marker_style);
        self
    }

    /// Set the chart title.
    pub fn with_title(mut self, title: &str) -> Self {
        self.line_builder = self.line_builder.with_title(title);
        self
    }

    /// Set the background color.
    pub fn background_color(mut self, color: C) -> Self {
        self.line_builder = self.line_builder.background_color(color);
        self
    }

    /// Set the chart margins.
    pub fn margins(mut self, margins: crate::chart::traits::Margins) -> Self {
        self.line_builder = self.line_builder.margins(margins);
        self
    }

    /// Set the grid system.
    pub fn with_grid(mut self, grid: crate::grid::GridSystem<C>) -> Self {
        self.line_builder = self.line_builder.with_grid(grid);
        self
    }

    /// Set the X-axis configuration.
    pub fn with_x_axis(mut self, axis: crate::axes::LinearAxis<f32, C>) -> Self {
        self.line_builder = self.line_builder.with_x_axis(axis);
        self
    }

    /// Set the Y-axis configuration.
    pub fn with_y_axis(mut self, axis: crate::axes::LinearAxis<f32, C>) -> Self {
        self.line_builder = self.line_builder.with_y_axis(axis);
        self
    }

    /// Build the curve chart.
    pub fn build(self) -> ChartResult<CurveChart<C>> {
        let base_chart = self.line_builder.build()?;

        Ok(CurveChart {
            base_chart,
            interpolation_config: self.interpolation_config,
        })
    }
}

impl<C: PixelColor + 'static> Default for CurveChartBuilder<C>
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
    fn test_curve_chart_creation() {
        let chart: CurveChart<Rgb565> = CurveChart::new();
        assert_eq!(
            chart.interpolation_config().interpolation_type,
            InterpolationType::CubicSpline
        );
        assert_eq!(chart.interpolation_config().subdivisions, 8);
    }

    #[test]
    fn test_curve_chart_builder() {
        let chart: CurveChart<Rgb565> = CurveChart::builder()
            .line_color(Rgb565::RED)
            .line_width(3)
            .interpolation_type(InterpolationType::Bezier)
            .subdivisions(12)
            .tension(0.8)
            .build()
            .unwrap();

        assert_eq!(chart.style().line_color, Rgb565::RED);
        assert_eq!(chart.style().line_width, 3);
        assert_eq!(
            chart.interpolation_config().interpolation_type,
            InterpolationType::Bezier
        );
        assert_eq!(chart.interpolation_config().subdivisions, 12);
    }

    #[test]
    fn test_interpolation_config_clamping() {
        let chart: CurveChart<Rgb565> = CurveChart::builder()
            .subdivisions(100) // Should be clamped to 32
            .tension(2.0) // Should be clamped to 1.0
            .build()
            .unwrap();

        assert_eq!(chart.interpolation_config().subdivisions, 32);
        assert_eq!(chart.interpolation_config().tension, 1.0);
    }
}
