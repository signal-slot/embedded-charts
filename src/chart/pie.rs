//! Pie chart implementation.

use crate::chart::traits::{Chart, ChartBuilder, ChartConfig};
use crate::data::{DataPoint, DataSeries};
use crate::error::{ChartError, ChartResult};
use crate::math::Math;
use crate::math::NumericConversion;
use crate::style::BorderStyle;
use embedded_graphics::{
    draw_target::DrawTarget,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
};
use heapless::Vec;

/// Pie chart implementation
#[derive(Debug, Clone)]
pub struct PieChart<C: PixelColor> {
    style: PieChartStyle<C>,
    config: ChartConfig<C>,
    center: Point,
    radius: u32,
}

/// Style configuration for pie charts
#[derive(Debug, Clone)]
pub struct PieChartStyle<C: PixelColor> {
    /// Colors for pie slices
    pub colors: Vec<C, 16>,
    /// Border style for slices
    pub border: Option<BorderStyle<C>>,
    /// Label style configuration
    pub labels: LabelStyle,
    /// Starting angle in degrees (0 = right, 90 = top)
    pub start_angle: f32,
    /// Inner radius for donut charts (None = full pie)
    pub donut_inner_radius: Option<u32>,
}

/// Label style for pie chart slices
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LabelStyle {
    /// Whether to show labels
    pub visible: bool,
    /// Whether to show percentage values
    pub show_percentage: bool,
    /// Whether to show actual values
    pub show_values: bool,
    /// Distance from pie edge to label
    pub offset: u32,
}

/// Represents a pie slice with its properties
#[derive(Debug, Clone, Copy)]
pub struct PieSlice {
    /// Start angle in radians
    pub start_angle: f32,
    /// End angle in radians
    pub end_angle: f32,
    /// Value of this slice
    pub value: f32,
    /// Percentage of total
    pub percentage: f32,
}

impl<C: PixelColor> PieChart<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new pie chart with default styling
    pub fn new(center: Point, radius: u32) -> Self {
        Self {
            style: PieChartStyle::default(),
            config: ChartConfig::default(),
            center,
            radius,
        }
    }

    /// Create a builder for configuring the pie chart
    pub fn builder() -> PieChartBuilder<C> {
        PieChartBuilder::new()
    }

    /// Set the pie chart style
    pub fn set_style(&mut self, style: PieChartStyle<C>) {
        self.style = style;
    }

    /// Get the current pie chart style
    pub fn style(&self) -> &PieChartStyle<C> {
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

    /// Set the center point
    pub fn set_center(&mut self, center: Point) {
        self.center = center;
    }

    /// Get the center point
    pub fn center(&self) -> Point {
        self.center
    }

    /// Set the radius
    pub fn set_radius(&mut self, radius: u32) {
        self.radius = radius;
    }

    /// Get the radius
    pub fn radius(&self) -> u32 {
        self.radius
    }

    /// Calculate pie slices from data
    fn calculate_slices(
        &self,
        data: &crate::data::series::StaticDataSeries<crate::data::point::Point2D, 256>,
    ) -> ChartResult<Vec<PieSlice, 16>> {
        let mut slices = Vec::new();

        // Calculate total value
        let total: f32 = data
            .iter()
            .map(|point| point.y())
            .filter(|&value: &f32| value >= 0.0) // Only positive values
            .sum();

        if total <= 0.0 {
            return Err(ChartError::InsufficientData);
        }

        // Convert start angle to radians
        let start_angle_rad = self.style.start_angle.to_radians();
        let mut current_angle = start_angle_rad;

        // Create slices
        for point in data.iter() {
            let value: f32 = point.y();
            if value < 0.0 {
                continue; // Skip negative values
            }

            let percentage = value / total;
            let angle_span = percentage * 2.0 * core::f32::consts::PI;
            let end_angle = current_angle + angle_span;

            let slice = PieSlice {
                start_angle: current_angle,
                end_angle,
                value,
                percentage: percentage * 100.0,
            };

            slices.push(slice).map_err(|_| ChartError::MemoryFull)?;
            current_angle = end_angle;
        }

        Ok(slices)
    }

    /// Draw a pie slice using a custom implementation to avoid pixel overlap
    fn draw_slice<D>(&self, slice: &PieSlice, color_index: usize, target: &mut D) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        // Get slice color
        let slice_color = if !self.style.colors.is_empty() {
            self.style.colors[color_index % self.style.colors.len()]
        } else {
            return Err(ChartError::InvalidConfiguration);
        };

        // Custom pie slice drawing to avoid embedded-graphics Sector overlap issues
        self.draw_pie_slice_custom(slice, slice_color, target)?;

        Ok(())
    }

    /// Custom pie slice drawing implementation that avoids pixel overlap
    fn draw_pie_slice_custom<D>(
        &self,
        slice: &PieSlice,
        color: C,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        use embedded_graphics::Drawable;
        use embedded_graphics::Pixel;

        let center_x = self.center.x;
        let center_y = self.center.y;
        let radius_num = (self.radius as i32).to_number();

        // Fill the slice by checking each pixel in the bounding box
        let min_x = (center_x - self.radius as i32).max(0);
        let max_x = center_x + self.radius as i32;
        let min_y = (center_y - self.radius as i32).max(0);
        let max_y = center_y + self.radius as i32;

        // Constants using Number type
        let zero = 0i32.to_number();
        let pi = core::f32::consts::PI.to_number();
        let two_pi = pi + pi;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let dx_num = (x - center_x).to_number();
                let dy_num = (y - center_y).to_number();
                let distance_squared = dx_num * dx_num + dy_num * dy_num;
                let distance = Math::sqrt(distance_squared);

                // Skip pixels outside the circle or at the exact center (to avoid overlap)
                // Add small tolerance for better boundary handling
                let tolerance = 0.5f32.to_number();
                if distance > radius_num + tolerance || distance < tolerance {
                    continue;
                }

                // Calculate angle from center to this pixel using proper atan2
                // Note: Screen coordinates have y-axis flipped, so we negate dy for proper mathematical angles
                let angle = Math::atan2(-dy_num, dx_num);

                // Normalize angle to [0, 2π] using modulo operations
                let normalized_angle = {
                    let mut a = angle;
                    if a < zero {
                        a += two_pi;
                    }
                    // Use a simple normalization since we don't have modulo for Number type
                    while a >= two_pi {
                        a -= two_pi;
                    }
                    while a < zero {
                        a += two_pi;
                    }
                    a
                };

                // Check if this pixel is within the slice
                let start_angle_num = slice.start_angle.to_number();
                let end_angle_num = slice.end_angle.to_number();

                // Normalize slice angles to [0, 2π] using modulo operations
                let start_norm = {
                    let mut a = start_angle_num;
                    while a >= two_pi {
                        a -= two_pi;
                    }
                    while a < zero {
                        a += two_pi;
                    }
                    a
                };
                let end_norm = {
                    let mut a = end_angle_num;
                    while a >= two_pi {
                        a -= two_pi;
                    }
                    while a < zero {
                        a += two_pi;
                    }
                    a
                };

                let in_slice = if start_norm <= end_norm {
                    normalized_angle >= start_norm && normalized_angle <= end_norm
                } else {
                    // Handle wrap-around case
                    normalized_angle >= start_norm || normalized_angle <= end_norm
                };

                if in_slice {
                    let point = Point::new(x, y);
                    Pixel(point, color)
                        .draw(target)
                        .map_err(|_| ChartError::RenderingError)?;
                }
            }
        }

        Ok(())
    }

    /// Draw the center circle for donut charts
    fn draw_donut_center<D>(&self, target: &mut D) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        if let Some(inner_radius) = self.style.donut_inner_radius {
            if let Some(bg_color) = self.config.background_color {
                let fill_style = PrimitiveStyle::with_fill(bg_color);
                Circle::new(
                    Point::new(
                        self.center.x - inner_radius as i32,
                        self.center.y - inner_radius as i32,
                    ),
                    inner_radius * 2,
                )
                .into_styled(fill_style)
                .draw(target)
                .map_err(|_| ChartError::RenderingError)?;
            }
        }

        Ok(())
    }
}
impl<C: PixelColor> Default for PieChart<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::new(Point::new(50, 50), 40)
    }
}

impl<C: PixelColor> Chart<C> for PieChart<C>
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

        // Calculate the actual center position within the viewport
        let title_height = if config.title.is_some() { 30 } else { 0 };
        let available_height = viewport.size.height.saturating_sub(title_height);

        // Center the pie chart in the available space
        let center_x = viewport.top_left.x + (viewport.size.width as i32) / 2;
        let center_y = viewport.top_left.y + title_height as i32 + (available_height as i32) / 2;
        let actual_center = Point::new(center_x, center_y);

        // Create a temporary pie chart with the calculated center for drawing
        let mut chart_for_drawing = self.clone();
        chart_for_drawing.center = actual_center;

        // Calculate slices
        let slices = chart_for_drawing.calculate_slices(data)?;

        // Draw each slice using the chart with correct center
        for (index, slice) in slices.iter().enumerate() {
            chart_for_drawing.draw_slice(slice, index, target)?;
        }

        // Draw donut center if applicable
        chart_for_drawing.draw_donut_center(target)?;

        // Draw title if present
        if let Some(title) = &config.title {
            use embedded_graphics::{
                mono_font::{ascii::FONT_6X10, MonoTextStyle},
                text::{Alignment, Text},
            };

            let text_color = embedded_graphics::pixelcolor::Rgb565::BLACK.into();
            let text_style = MonoTextStyle::new(&FONT_6X10, text_color);

            let title_x = viewport.top_left.x + (viewport.size.width as i32) / 2;
            let title_y = viewport.top_left.y + 15;

            Text::with_alignment(
                title,
                Point::new(title_x, title_y),
                text_style,
                Alignment::Center,
            )
            .draw(target)
            .map_err(|_| ChartError::RenderingError)?;
        }

        Ok(())
    }
}

impl<C: PixelColor> Default for PieChartStyle<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        let mut colors = Vec::new();
        let _ = colors.push(embedded_graphics::pixelcolor::Rgb565::BLUE.into());
        let _ = colors.push(embedded_graphics::pixelcolor::Rgb565::RED.into());
        let _ = colors.push(embedded_graphics::pixelcolor::Rgb565::GREEN.into());
        let _ = colors.push(embedded_graphics::pixelcolor::Rgb565::YELLOW.into());
        let _ = colors.push(embedded_graphics::pixelcolor::Rgb565::MAGENTA.into());
        let _ = colors.push(embedded_graphics::pixelcolor::Rgb565::CYAN.into());

        Self {
            colors,
            border: None,
            labels: LabelStyle::default(),
            start_angle: 0.0,
            donut_inner_radius: None,
        }
    }
}

impl Default for LabelStyle {
    fn default() -> Self {
        Self {
            visible: false,
            show_percentage: true,
            show_values: false,
            offset: 10,
        }
    }
}

/// Builder for pie charts
#[derive(Debug)]
pub struct PieChartBuilder<C: PixelColor> {
    style: PieChartStyle<C>,
    config: ChartConfig<C>,
    center: Point,
    radius: u32,
}

impl<C: PixelColor> PieChartBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new pie chart builder
    pub fn new() -> Self {
        Self {
            style: PieChartStyle::default(),
            config: ChartConfig::default(),
            center: Point::new(50, 50),
            radius: 40,
        }
    }

    /// Set the center point
    pub fn center(mut self, center: Point) -> Self {
        self.center = center;
        self
    }

    /// Set the radius
    pub fn radius(mut self, radius: u32) -> Self {
        self.radius = radius;
        self
    }

    /// Set slice colors
    pub fn colors(mut self, colors: &[C]) -> Self {
        self.style.colors.clear();
        for &color in colors {
            if self.style.colors.push(color).is_err() {
                break; // Reached capacity
            }
        }
        self
    }

    /// Set the starting angle
    pub fn start_angle(mut self, angle: f32) -> Self {
        self.style.start_angle = angle;
        self
    }

    /// Make this a donut chart with the specified inner radius
    pub fn donut(mut self, inner_radius: u32) -> Self {
        self.style.donut_inner_radius = Some(inner_radius);
        self
    }

    /// Add a border to slices
    pub fn with_border(mut self, border: BorderStyle<C>) -> Self {
        self.style.border = Some(border);
        self
    }

    /// Configure labels
    pub fn labels(mut self, labels: LabelStyle) -> Self {
        self.style.labels = labels;
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

impl<C: PixelColor> ChartBuilder<C> for PieChartBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    type Chart = PieChart<C>;
    type Error = ChartError;

    fn build(self) -> Result<Self::Chart, Self::Error> {
        Ok(PieChart {
            style: self.style,
            config: self.config,
            center: self.center,
            radius: self.radius,
        })
    }
}

impl<C: PixelColor> Default for PieChartBuilder<C>
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
    fn test_pie_chart_creation() {
        let chart: PieChart<Rgb565> = PieChart::new(Point::new(100, 100), 50);
        assert_eq!(chart.center(), Point::new(100, 100));
        assert_eq!(chart.radius(), 50);
        assert!(chart.style().donut_inner_radius.is_none());
    }

    #[test]
    fn test_pie_chart_builder() {
        let chart: PieChart<Rgb565> = PieChart::builder()
            .center(Point::new(150, 150))
            .radius(60)
            .colors(&[Rgb565::RED, Rgb565::BLUE, Rgb565::GREEN])
            .start_angle(90.0)
            .donut(20)
            .with_title("Test Pie Chart")
            .build()
            .unwrap();

        assert_eq!(chart.center(), Point::new(150, 150));
        assert_eq!(chart.radius(), 60);
        assert_eq!(chart.style().colors.len(), 3);
        assert_eq!(chart.style().start_angle, 90.0);
        assert_eq!(chart.style().donut_inner_radius, Some(20));
        assert_eq!(
            chart.config().title.as_ref().map(|s| s.as_str()),
            Some("Test Pie Chart")
        );
    }

    #[test]
    fn test_label_style() {
        let labels = LabelStyle {
            visible: true,
            show_percentage: true,
            show_values: false,
            offset: 15,
        };

        assert!(labels.visible);
        assert!(labels.show_percentage);
        assert!(!labels.show_values);
        assert_eq!(labels.offset, 15);
    }

    #[test]
    fn test_pie_slice() {
        let slice = PieSlice {
            start_angle: 0.0,
            end_angle: core::f32::consts::PI / 2.0,
            value: 25.0,
            percentage: 25.0,
        };

        assert_eq!(slice.value, 25.0);
        assert_eq!(slice.percentage, 25.0);
        assert_eq!(slice.start_angle, 0.0);
    }
}
