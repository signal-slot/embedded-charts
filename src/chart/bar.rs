//! Bar chart implementation with no_std compatibility.
//!
//! This module provides a comprehensive bar chart implementation optimized for embedded systems.
//! It supports both vertical and horizontal orientations, multiple styling options, and efficient
//! rendering while maintaining memory efficiency suitable for resource-constrained environments.
//!
//! # Features
//!
//! - **Dual orientation**: Support for both vertical and horizontal bar charts
//! - **Flexible bar width**: Fixed width, proportional width, or automatic sizing
//! - **Multi-color support**: Up to 16 different bar colors with automatic cycling
//! - **Customizable spacing**: Configurable spacing between bars
//! - **Border styling**: Optional borders with customizable styles
//! - **Stacked bars**: Support for stacked bar charts (feature-gated)
//! - **Memory efficient**: Static allocation with compile-time bounds
//!
//! # Basic Usage
//!
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! // Create sample data
//! let mut data = StaticDataSeries::new();
//! data.push(Point2D::new(0.0, 10.0))?;
//! data.push(Point2D::new(1.0, 20.0))?;
//! data.push(Point2D::new(2.0, 15.0))?;
//!
//! // Create a basic bar chart
//! let chart = BarChart::builder()
//!     .orientation(BarOrientation::Vertical)
//!     .bar_width(BarWidth::Fixed(20))
//!     .bar_color(Rgb565::BLUE)
//!     .build()?;
//!
//! // Configure the chart
//! let config = ChartConfig::default();
//! let viewport = Rectangle::new(Point::zero(), Size::new(320, 240));
//!
//! // Render to display
//! chart.draw(&data, &config, viewport, &mut display)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Advanced Styling
//!
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! let chart = BarChart::builder()
//!     .orientation(BarOrientation::Horizontal)
//!     .bar_width(BarWidth::Proportional(0.8)) // 80% of available space
//!     .spacing(5)
//!     .with_border(BorderStyle {
//!         color: Rgb565::BLACK,
//!         width: 1,
//!         pattern: LinePattern::Solid,
//!         radius: 2,
//!     })
//!     .add_bar_color(Rgb565::BLUE)
//!     .add_bar_color(Rgb565::RED)
//!     .add_bar_color(Rgb565::GREEN)
//!     .build()?;
//! # Ok::<(), embedded_charts::error::ChartError>(())
//! ```
//!
//! # Multi-Series Bar Charts
//!
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! // Create multi-series data
//! let mut multi_series = MultiSeries::new();
//! let series1 = data_points![(0.0, 10.0), (1.0, 15.0), (2.0, 12.0)];
//! let series2 = data_points![(0.0, 8.0), (1.0, 18.0), (2.0, 14.0)];
//!
//! multi_series.add_series("Series 1", series1)?;
//! multi_series.add_series("Series 2", series2)?;
//!
//! // Create chart with multiple colors
//! let chart = BarChart::builder()
//!     .bar_width(BarWidth::Auto)
//!     .spacing(2)
//!     .add_bar_color(Rgb565::BLUE)
//!     .add_bar_color(Rgb565::RED)
//!     .build()?;
//! # Ok::<(), embedded_charts::error::ChartError>(())
//! ```

#[cfg(feature = "animations")]
use crate::chart::traits::{Chart, ChartBuilder, ChartConfig};
use crate::data::{DataBounds, DataPoint, DataSeries};
use crate::error::{ChartError, ChartResult};
use crate::style::BorderStyle;
use embedded_graphics::{
    draw_target::DrawTarget,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};
use heapless::Vec;

/// Bar chart implementation for displaying categorical data.
///
/// A bar chart displays data using rectangular bars with lengths proportional to the values
/// they represent. This implementation supports both vertical and horizontal orientations,
/// making it suitable for various data visualization needs on embedded displays.
///
/// # Features
///
/// - Configurable bar orientation (vertical/horizontal)
/// - Flexible bar width options (fixed, proportional, automatic)
/// - Multi-color support with automatic color cycling
/// - Customizable spacing and borders
/// - Support for stacked bars (feature-gated)
/// - Memory-efficient static allocation
///
/// # Memory Usage
///
/// The bar chart uses static allocation with:
/// - Up to 16 bar colors stored in a heapless vector
/// - Screen coordinate calculations for bar positioning
/// - Efficient rendering with minimal temporary storage
///
/// # Examples
///
/// Basic vertical bar chart:
/// ```rust
/// use embedded_charts::prelude::*;
/// use embedded_graphics::pixelcolor::Rgb565;
///
/// let chart = BarChart::new();
/// let mut data = StaticDataSeries::new();
/// data.push(Point2D::new(0.0, 10.0))?;
/// data.push(Point2D::new(1.0, 20.0))?;
/// # Ok::<(), embedded_charts::error::DataError>(())
/// ```
///
/// Horizontal bar chart with custom styling:
/// ```rust
/// use embedded_charts::prelude::*;
/// use embedded_graphics::pixelcolor::Rgb565;
///
/// let chart = BarChart::builder()
///     .orientation(BarOrientation::Horizontal)
///     .bar_width(BarWidth::Fixed(25))
///     .spacing(3)
///     .bar_color(Rgb565::GREEN)
///     .build()?;
/// # Ok::<(), embedded_charts::error::ChartError>(())
/// ```
#[derive(Debug, Clone)]
pub struct BarChart<C: PixelColor> {
    style: BarChartStyle<C>,
    config: ChartConfig<C>,
    orientation: BarOrientation,
}

/// Style configuration for bar charts.
///
/// This structure contains all visual styling options for bar charts,
/// including colors, dimensions, spacing, and border styles.
///
/// # Examples
///
/// ```rust
/// use embedded_charts::prelude::*;
/// use embedded_graphics::pixelcolor::Rgb565;
///
/// let mut colors = heapless::Vec::new();
/// colors.push(Rgb565::BLUE).unwrap();
/// colors.push(Rgb565::RED).unwrap();
///
/// let style = BarChartStyle {
///     bar_colors: colors,
///     bar_width: BarWidth::Fixed(20),
///     spacing: 5,
///     border: None,
///     stacked: false,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct BarChartStyle<C: PixelColor> {
    /// Colors for the bars.
    ///
    /// The chart cycles through these colors for multiple data series.
    /// Maximum of 16 colors supported for memory efficiency.
    pub bar_colors: Vec<C, 16>,
    /// Width configuration for bars.
    ///
    /// Determines how bar widths are calculated based on available space.
    pub bar_width: BarWidth,
    /// Spacing between bars in pixels.
    ///
    /// This spacing is applied between individual bars and between groups
    /// in multi-series charts.
    pub spacing: u32,
    /// Optional border style for bars.
    ///
    /// When `Some`, draws borders around each bar with the specified style.
    /// When `None`, bars are drawn without borders.
    pub border: Option<BorderStyle<C>>,
    /// Whether bars should be stacked.
    ///
    /// When `true`, multiple data series are stacked on top of each other.
    /// When `false`, series are displayed side by side.
    pub stacked: bool,
}

/// Bar orientation options.
///
/// Determines whether bars extend vertically (from bottom to top) or
/// horizontally (from left to right).
///
/// # Examples
///
/// ```rust
/// use embedded_charts::prelude::*;
///
/// // Vertical bars (traditional bar chart)
/// let vertical = BarOrientation::Vertical;
///
/// // Horizontal bars (useful for long category names)
/// let horizontal = BarOrientation::Horizontal;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarOrientation {
    /// Vertical bars extending from bottom to top.
    ///
    /// This is the traditional bar chart orientation where bars grow upward
    /// from a baseline. Best for time-series data or when category names are short.
    Vertical,
    /// Horizontal bars extending from left to right.
    ///
    /// Useful when category names are long or when you want to emphasize
    /// the comparison between values. The baseline is on the left side.
    Horizontal,
}

/// Bar width configuration options.
///
/// Determines how the width of bars is calculated based on the available
/// chart space and the number of data points.
///
/// # Examples
///
/// ```rust
/// use embedded_charts::prelude::*;
///
/// // Fixed width bars
/// let fixed = BarWidth::Fixed(20); // 20 pixels wide
///
/// // Proportional width (80% of available space per bar)
/// let proportional = BarWidth::Proportional(0.8);
///
/// // Automatic width calculation
/// let auto = BarWidth::Auto;
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BarWidth {
    /// Fixed width in pixels.
    ///
    /// Each bar will have exactly the specified width regardless of the
    /// available space or number of data points. This provides consistent
    /// bar appearance but may cause bars to overlap or leave unused space.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use embedded_charts::prelude::*;
    ///
    /// let width = BarWidth::Fixed(25); // 25 pixels wide
    /// ```
    Fixed(u32),
    /// Percentage of available space per bar (0.0 to 1.0).
    ///
    /// Each bar will occupy the specified percentage of the space allocated
    /// to it. For example, 0.8 means each bar uses 80% of its allocated space,
    /// leaving 20% for spacing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use embedded_charts::prelude::*;
    ///
    /// let width = BarWidth::Percentage(0.7); // 70% of allocated space
    /// ```
    Percentage(f32),
    /// Automatic width calculation based on available space and data count.
    ///
    /// The chart automatically calculates optimal bar width based on the
    /// viewport size, number of data points, and spacing requirements.
    /// This provides the best balance between bar visibility and spacing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use embedded_charts::prelude::*;
    ///
    /// let width = BarWidth::Auto; // Automatic calculation
    /// ```
    Auto,
}

impl<C: PixelColor> BarChart<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new bar chart with default styling.
    ///
    /// This creates a bar chart with:
    /// - Vertical orientation
    /// - Single blue bar color
    /// - Automatic bar width
    /// - 5-pixel spacing between bars
    /// - No borders
    /// - Default margins (10 pixels on all sides)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use embedded_charts::prelude::*;
    /// use embedded_graphics::pixelcolor::Rgb565;
    ///
    /// let chart: BarChart<Rgb565> = BarChart::new();
    /// ```
    pub fn new() -> Self {
        Self {
            style: BarChartStyle::default(),
            config: ChartConfig::default(),
            orientation: BarOrientation::Vertical,
        }
    }

    /// Create a builder for configuring the bar chart.
    ///
    /// The builder pattern provides a fluent interface for configuring
    /// all aspects of the bar chart before creation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use embedded_charts::prelude::*;
    /// use embedded_graphics::pixelcolor::Rgb565;
    ///
    /// let chart = BarChart::builder()
    ///     .orientation(BarOrientation::Horizontal)
    ///     .bar_width(BarWidth::Fixed(30))
    ///     .spacing(8)
    ///     .bar_color(Rgb565::GREEN)
    ///     .build()?;
    /// # Ok::<(), embedded_charts::error::ChartError>(())
    /// ```
    pub fn builder() -> BarChartBuilder<C> {
        BarChartBuilder::new()
    }

    /// Set the bar chart style configuration.
    ///
    /// This replaces the entire style configuration with the provided one.
    /// Use the builder pattern for more granular control.
    ///
    /// # Arguments
    ///
    /// * `style` - The new bar chart style configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use embedded_charts::prelude::*;
    /// use embedded_graphics::pixelcolor::Rgb565;
    ///
    /// let mut chart = BarChart::new();
    /// let mut colors = heapless::Vec::new();
    /// colors.push(Rgb565::BLUE).unwrap();
    /// colors.push(Rgb565::RED).unwrap();
    ///
    /// let style = BarChartStyle {
    ///     bar_colors: colors,
    ///     bar_width: BarWidth::Fixed(25),
    ///     spacing: 3,
    ///     border: None,
    ///     stacked: false,
    /// };
    /// chart.set_style(style);
    /// ```
    pub fn set_style(&mut self, style: BarChartStyle<C>) {
        self.style = style;
    }

    /// Get the current bar chart style configuration.
    ///
    /// Returns a reference to the current style configuration,
    /// allowing inspection of current settings.
    ///
    /// # Returns
    ///
    /// A reference to the current `BarChartStyle`
    pub fn style(&self) -> &BarChartStyle<C> {
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

    /// Get the chart configuration
    pub fn config(&self) -> &ChartConfig<C> {
        &self.config
    }

    /// Set the bar orientation
    pub fn set_orientation(&mut self, orientation: BarOrientation) {
        self.orientation = orientation;
    }

    /// Get the bar orientation
    pub fn orientation(&self) -> BarOrientation {
        self.orientation
    }

    /// Calculate bar dimensions and positions
    fn calculate_bar_layout(
        &self,
        data: &crate::data::series::StaticDataSeries<crate::data::point::Point2D, 256>,
        data_bounds: &DataBounds<f32, f32>,
        viewport: Rectangle,
    ) -> ChartResult<Vec<Rectangle, 256>> {
        let mut bars = Vec::new();
        let draw_area = self.config.margins.apply_to(viewport);

        let data_count = data.len();
        if data_count == 0 {
            return Ok(bars);
        }

        // Calculate bar width
        let bar_width = match self.style.bar_width {
            BarWidth::Fixed(width) => width,
            BarWidth::Percentage(pct) => {
                let available_width = match self.orientation {
                    BarOrientation::Vertical => draw_area.size.width,
                    BarOrientation::Horizontal => draw_area.size.height,
                };
                (available_width as f32 * pct.clamp(0.0, 1.0)) as u32
            }
            BarWidth::Auto => {
                let available_width = match self.orientation {
                    BarOrientation::Vertical => draw_area.size.width,
                    BarOrientation::Horizontal => draw_area.size.height,
                };
                let total_spacing = self.style.spacing * (data_count as u32).saturating_sub(1);
                let calculated_width =
                    (available_width.saturating_sub(total_spacing)) / data_count as u32;
                // Ensure minimum bar width for visibility
                calculated_width.max(5)
            }
        };

        // Calculate positions and sizes for each bar
        let mut current_pos = 0;
        for point in data.iter() {
            let bar_rect = match self.orientation {
                BarOrientation::Vertical => {
                    let x = draw_area.top_left.x + current_pos as i32;
                    let data_y: f32 = point.y();
                    let min_y: f32 = data_bounds.min_y;
                    let max_y: f32 = data_bounds.max_y;

                    // Normalize Y value (0.0 to 1.0)
                    let norm_y = if max_y > min_y {
                        (data_y - min_y) / (max_y - min_y)
                    } else {
                        0.5
                    };

                    // Ensure minimum bar height for visibility
                    let bar_height = ((norm_y * draw_area.size.height as f32) as u32).max(1);
                    let y = draw_area.top_left.y + draw_area.size.height as i32 - bar_height as i32;

                    Rectangle::new(Point::new(x, y), Size::new(bar_width, bar_height))
                }
                BarOrientation::Horizontal => {
                    let y = draw_area.top_left.y + current_pos as i32;
                    let data_y: f32 = point.y();
                    let min_y: f32 = data_bounds.min_y;
                    let max_y: f32 = data_bounds.max_y;

                    // Normalize Y value (0.0 to 1.0)
                    let norm_y = if max_y > min_y {
                        (data_y - min_y) / (max_y - min_y)
                    } else {
                        0.5
                    };

                    // Ensure minimum bar width for visibility
                    let bar_width_horizontal =
                        ((norm_y * draw_area.size.width as f32) as u32).max(1);
                    let x = draw_area.top_left.x;

                    Rectangle::new(Point::new(x, y), Size::new(bar_width_horizontal, bar_width))
                }
            };

            bars.push(bar_rect).map_err(|_| ChartError::MemoryFull)?;
            current_pos += bar_width + self.style.spacing;
        }

        Ok(bars)
    }

    /// Draw a single bar
    fn draw_bar<D>(
        &self,
        bar_rect: Rectangle,
        color_index: usize,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        // Get bar color (cycle through available colors)
        let bar_color = if !self.style.bar_colors.is_empty() {
            self.style.bar_colors[color_index % self.style.bar_colors.len()]
        } else {
            return Err(ChartError::InvalidConfiguration);
        };

        // Draw filled bar directly
        bar_rect
            .into_styled(PrimitiveStyle::with_fill(bar_color))
            .draw(target)
            .map_err(|_| ChartError::RenderingError)?;

        // Draw border if specified
        if let Some(border) = &self.style.border {
            if border.visible {
                bar_rect
                    .into_styled(PrimitiveStyle::with_stroke(
                        border.line.color,
                        border.line.width,
                    ))
                    .draw(target)
                    .map_err(|_| ChartError::RenderingError)?;
            }
        }

        Ok(())
    }
}

impl<C: PixelColor> Default for BarChart<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<C: PixelColor> Chart<C> for BarChart<C>
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

        // Draw background if specified
        if let Some(bg_color) = config.background_color {
            Rectangle::new(viewport.top_left, viewport.size)
                .into_styled(PrimitiveStyle::with_fill(bg_color))
                .draw(target)
                .map_err(|_| ChartError::RenderingError)?;
        }

        // Calculate data bounds
        let data_bounds = data.bounds()?;

        // Calculate bar layout
        let bars = self.calculate_bar_layout(data, &data_bounds, viewport)?;

        // Draw each bar
        for (index, bar_rect) in bars.iter().enumerate() {
            self.draw_bar(*bar_rect, index, target)?;
        }

        Ok(())
    }
}

impl<C: PixelColor> Default for BarChartStyle<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        let mut colors = Vec::new();
        let _ = colors.push(embedded_graphics::pixelcolor::Rgb565::BLUE.into());
        let _ = colors.push(embedded_graphics::pixelcolor::Rgb565::RED.into());
        let _ = colors.push(embedded_graphics::pixelcolor::Rgb565::GREEN.into());
        let _ = colors.push(embedded_graphics::pixelcolor::Rgb565::YELLOW.into());

        Self {
            bar_colors: colors,
            bar_width: BarWidth::Auto,
            spacing: 2,
            border: None,
            stacked: false,
        }
    }
}

/// Builder for bar charts
#[derive(Debug)]
pub struct BarChartBuilder<C: PixelColor> {
    style: BarChartStyle<C>,
    config: ChartConfig<C>,
    orientation: BarOrientation,
}

impl<C: PixelColor> BarChartBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new bar chart builder
    pub fn new() -> Self {
        Self {
            style: BarChartStyle::default(),
            config: ChartConfig::default(),
            orientation: BarOrientation::Vertical,
        }
    }

    /// Set the bar orientation
    pub fn orientation(mut self, orientation: BarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set bar colors
    pub fn colors(mut self, colors: &[C]) -> Self {
        self.style.bar_colors.clear();
        for &color in colors {
            if self.style.bar_colors.push(color).is_err() {
                break; // Reached capacity
            }
        }
        self
    }

    /// Set bar width
    pub fn bar_width(mut self, width: BarWidth) -> Self {
        self.style.bar_width = width;
        self
    }

    /// Set spacing between bars
    pub fn spacing(mut self, spacing: u32) -> Self {
        self.style.spacing = spacing;
        self
    }

    /// Add a border to bars
    pub fn with_border(mut self, border: BorderStyle<C>) -> Self {
        self.style.border = Some(border);
        self
    }

    /// Enable stacked bars
    pub fn stacked(mut self, stacked: bool) -> Self {
        self.style.stacked = stacked;
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
}

impl<C: PixelColor> ChartBuilder<C> for BarChartBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    type Chart = BarChart<C>;
    type Error = ChartError;

    fn build(self) -> Result<Self::Chart, Self::Error> {
        Ok(BarChart {
            style: self.style,
            config: self.config,
            orientation: self.orientation,
        })
    }
}

impl<C: PixelColor> Default for BarChartBuilder<C>
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
    fn test_bar_chart_creation() {
        let chart: BarChart<Rgb565> = BarChart::new();
        assert_eq!(chart.orientation(), BarOrientation::Vertical);
        assert!(!chart.style().stacked);
    }

    #[test]
    fn test_bar_chart_builder() {
        let chart: BarChart<Rgb565> = BarChart::builder()
            .orientation(BarOrientation::Horizontal)
            .colors(&[Rgb565::RED, Rgb565::BLUE])
            .bar_width(BarWidth::Fixed(30))
            .spacing(5)
            .with_title("Test Bar Chart")
            .build()
            .unwrap();

        assert_eq!(chart.orientation(), BarOrientation::Horizontal);
        assert_eq!(chart.style().bar_colors.len(), 2);
        assert_eq!(chart.style().spacing, 5);
        assert_eq!(
            chart.config().title.as_ref().map(|s| s.as_str()),
            Some("Test Bar Chart")
        );
    }

    #[test]
    fn test_bar_width_types() {
        assert_eq!(BarWidth::Fixed(20), BarWidth::Fixed(20));

        let percentage = BarWidth::Percentage(0.8);
        if let BarWidth::Percentage(pct) = percentage {
            assert_eq!(pct, 0.8);
        }

        assert_eq!(BarWidth::Auto, BarWidth::Auto);
    }
}

/// Animated bar chart that extends BarChart with animation capabilities
#[cfg(feature = "animations")]
#[derive(Debug, Clone)]
pub struct AnimatedBarChart<C: PixelColor> {
    /// Base bar chart
    base_chart: BarChart<C>,
    /// Current animated data (interpolated values)
    current_data: Option<crate::data::series::StaticDataSeries<crate::data::point::Point2D, 256>>,
}

#[cfg(feature = "animations")]
impl<C: PixelColor> AnimatedBarChart<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new animated bar chart
    pub fn new() -> Self {
        Self {
            base_chart: BarChart::new(),
            current_data: None,
        }
    }

    /// Create a builder for configuring the animated bar chart
    pub fn builder() -> AnimatedBarChartBuilder<C> {
        AnimatedBarChartBuilder::new()
    }

    /// Set the bar chart style
    pub fn set_style(&mut self, style: BarChartStyle<C>) {
        self.base_chart.set_style(style);
    }

    /// Get the current bar chart style
    pub fn style(&self) -> &BarChartStyle<C> {
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

    /// Set the bar orientation
    pub fn set_orientation(&mut self, orientation: BarOrientation) {
        self.base_chart.set_orientation(orientation);
    }

    /// Get the bar orientation
    pub fn orientation(&self) -> BarOrientation {
        self.base_chart.orientation()
    }

    /// Get the current animated data or fallback to empty series
    fn get_render_data(
        &self,
    ) -> crate::data::series::StaticDataSeries<crate::data::point::Point2D, 256> {
        self.current_data.clone().unwrap_or_default()
    }

    /// Interpolate between two data series based on animation progress
    #[allow(dead_code)]
    fn interpolate_data(
        &self,
        from_data: &crate::data::series::StaticDataSeries<crate::data::point::Point2D, 256>,
        to_data: &crate::data::series::StaticDataSeries<crate::data::point::Point2D, 256>,
        progress: f32,
    ) -> ChartResult<crate::data::series::StaticDataSeries<crate::data::point::Point2D, 256>> {
        let mut result = crate::data::series::StaticDataSeries::new();

        // Handle different data sizes by taking the minimum
        let min_len = from_data.len().min(to_data.len());

        for i in 0..min_len {
            if let (Some(from_point), Some(to_point)) = (from_data.get(i), to_data.get(i)) {
                let interpolated_x = from_point.x() + (to_point.x() - from_point.x()) * progress;
                let interpolated_y = from_point.y() + (to_point.y() - from_point.y()) * progress;

                result
                    .push(crate::data::point::Point2D::new(
                        interpolated_x,
                        interpolated_y,
                    ))
                    .map_err(|_| crate::error::ChartError::MemoryFull)?;
            }
        }

        Ok(result)
    }
}

#[cfg(feature = "animations")]
impl<C: PixelColor> Default for AnimatedBarChart<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "animations")]
impl<C: PixelColor> Chart<C> for AnimatedBarChart<C>
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
        if self.current_data.is_some() {
            let render_data = self.get_render_data();
            self.base_chart.draw(&render_data, config, viewport, target)
        } else {
            self.base_chart.draw(data, config, viewport, target)
        }
    }
}

#[cfg(feature = "animations")]
impl<C: PixelColor> crate::chart::traits::AnimatedChart<C> for AnimatedBarChart<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    type AnimatedData = crate::data::series::StaticDataSeries<crate::data::point::Point2D, 256>;

    fn draw_animated<D>(
        &self,
        data: &Self::Data,
        config: &Self::Config,
        viewport: embedded_graphics::primitives::Rectangle,
        target: &mut D,
        _progress: crate::animation::Progress,
    ) -> ChartResult<()>
    where
        D: embedded_graphics::draw_target::DrawTarget<Color = C>,
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

/// Builder for animated bar charts
#[cfg(feature = "animations")]
#[derive(Debug)]
pub struct AnimatedBarChartBuilder<C: PixelColor> {
    base_builder: BarChartBuilder<C>,
    frame_rate: u32,
}

#[cfg(feature = "animations")]
impl<C: PixelColor> AnimatedBarChartBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new animated bar chart builder
    pub fn new() -> Self {
        Self {
            base_builder: BarChartBuilder::new(),
            frame_rate: 60,
        }
    }

    /// Set the target frame rate
    pub fn frame_rate(mut self, fps: u32) -> Self {
        self.frame_rate = fps.clamp(1, 120);
        self
    }

    /// Set the bar orientation
    pub fn orientation(mut self, orientation: BarOrientation) -> Self {
        self.base_builder = self.base_builder.orientation(orientation);
        self
    }

    /// Set bar colors
    pub fn colors(mut self, colors: &[C]) -> Self {
        self.base_builder = self.base_builder.colors(colors);
        self
    }

    /// Set bar width
    pub fn bar_width(mut self, width: BarWidth) -> Self {
        self.base_builder = self.base_builder.bar_width(width);
        self
    }

    /// Set spacing between bars
    pub fn spacing(mut self, spacing: u32) -> Self {
        self.base_builder = self.base_builder.spacing(spacing);
        self
    }

    /// Add a border to bars
    pub fn with_border(mut self, border: BorderStyle<C>) -> Self {
        self.base_builder = self.base_builder.with_border(border);
        self
    }

    /// Enable stacked bars
    pub fn stacked(mut self, stacked: bool) -> Self {
        self.base_builder = self.base_builder.stacked(stacked);
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

    /// Build the animated bar chart
    pub fn build(self) -> ChartResult<AnimatedBarChart<C>> {
        let base_chart = self.base_builder.build()?;

        Ok(AnimatedBarChart {
            base_chart,
            current_data: None,
        })
    }
}

#[cfg(feature = "animations")]
impl<C: PixelColor> Default for AnimatedBarChartBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::new()
    }
}
