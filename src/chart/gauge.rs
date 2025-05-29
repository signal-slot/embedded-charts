//! Gauge chart implementation.
//!
//! This module provides gauge chart functionality for displaying single values with ranges,
//! needle animations, and threshold zones. Supports various gauge styles including speedometer,
//! temperature gauge, and progress indicators.

use crate::chart::traits::{Chart, ChartBuilder, ChartConfig};
use crate::data::{DataPoint, DataSeries};
use crate::error::{ChartError, ChartResult};
use crate::math::{Math, NumericConversion};
use embedded_graphics::{
    draw_target::DrawTarget,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, Rectangle},
};
use heapless::Vec;

/// A gauge chart for displaying single values with ranges and needle animation
#[derive(Debug, Clone)]
pub struct GaugeChart<C: PixelColor> {
    style: GaugeChartStyle<C>,
    config: ChartConfig<C>,
    gauge_type: GaugeType,
    value_range: ValueRange,
}

/// Style configuration for gauge charts
#[derive(Debug, Clone)]
pub struct GaugeChartStyle<C: PixelColor> {
    /// Arc configuration for the gauge background
    pub arc_style: ArcStyle<C>,
    /// Needle configuration
    pub needle_style: NeedleStyle<C>,
    /// Threshold zones with colors
    pub threshold_zones: Vec<ThresholdZone<C>, 8>,
    /// Center hub style
    pub center_style: CenterStyle<C>,
    /// Tick marks configuration
    pub tick_style: Option<TickStyle<C>>,
    /// Value display configuration
    pub value_display: Option<ValueDisplayStyle<C>>,
}

/// Arc style configuration for the gauge background
#[derive(Debug, Clone, Copy)]
pub struct ArcStyle<C: PixelColor> {
    /// Background arc color
    pub background_color: C,
    /// Background arc width
    pub background_width: u32,
    /// Value arc color (if different from threshold zones)
    pub value_color: Option<C>,
    /// Value arc width
    pub value_width: u32,
    /// Arc radius
    pub radius: u32,
}

/// Needle style configuration
#[derive(Debug, Clone, Copy)]
pub struct NeedleStyle<C: PixelColor> {
    /// Needle shape
    pub shape: NeedleShape,
    /// Needle color
    pub color: C,
    /// Needle length as percentage of radius (0.0 to 1.0)
    pub length: f32,
    /// Needle width
    pub width: u32,
    /// Whether needle has animation
    pub animated: bool,
}

/// Available needle shapes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeedleShape {
    /// Simple line needle
    Line,
    /// Arrow-shaped needle
    Arrow,
    /// Pointer-shaped needle
    Pointer,
}

/// Threshold zone configuration
#[derive(Debug, Clone)]
pub struct ThresholdZone<C: PixelColor> {
    /// Start value of the zone
    pub start: f32,
    /// End value of the zone
    pub end: f32,
    /// Color for this zone
    pub color: C,
    /// Zone name/label
    pub label: Option<heapless::String<16>>,
}

/// Center hub style
#[derive(Debug, Clone, Copy)]
pub struct CenterStyle<C: PixelColor> {
    /// Center hub color
    pub color: C,
    /// Center hub radius
    pub radius: u32,
    /// Whether to show center hub
    pub visible: bool,
}

/// Tick marks style
#[derive(Debug, Clone, Copy)]
pub struct TickStyle<C: PixelColor> {
    /// Major tick color
    pub major_color: C,
    /// Minor tick color
    pub minor_color: C,
    /// Major tick length
    pub major_length: u32,
    /// Minor tick length
    pub minor_length: u32,
    /// Major tick width
    pub major_width: u32,
    /// Minor tick width
    pub minor_width: u32,
    /// Number of major ticks
    pub major_count: u32,
    /// Number of minor ticks between major ticks
    pub minor_count: u32,
}

/// Value display style
#[derive(Debug, Clone)]
pub struct ValueDisplayStyle<C: PixelColor> {
    /// Text color
    pub color: C,
    /// Font size (if supported)
    pub font_size: u32,
    /// Position relative to center
    pub position: ValueDisplayPosition,
    /// Number format
    pub format: ValueFormat,
    /// Whether to show units
    pub show_units: bool,
    /// Units text
    pub units: Option<heapless::String<8>>,
}

/// Value display position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueDisplayPosition {
    /// Center of the gauge
    Center,
    /// Below the center
    Below,
    /// Above the center
    Above,
}

/// Value format options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueFormat {
    /// Integer format
    Integer,
    /// One decimal place
    OneDecimal,
    /// Two decimal places
    TwoDecimal,
    /// Percentage
    Percentage,
}

/// Gauge type configuration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GaugeType {
    /// Semicircle gauge (180 degrees)
    Semicircle,
    /// Three-quarter circle gauge (270 degrees)
    ThreeQuarter,
    /// Full circle gauge (360 degrees)
    FullCircle,
    /// Custom angle range
    Custom {
        /// Start angle in degrees
        start_angle: f32,
        /// End angle in degrees
        end_angle: f32,
    },
}

/// Value range for the gauge
#[derive(Debug, Clone, Copy)]
pub struct ValueRange {
    /// Minimum value
    pub min: f32,
    /// Maximum value
    pub max: f32,
}

impl<C: PixelColor> GaugeChart<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new gauge chart with default styling
    pub fn new() -> Self {
        Self {
            style: GaugeChartStyle::default(),
            config: ChartConfig::default(),
            gauge_type: GaugeType::Semicircle,
            value_range: ValueRange {
                min: 0.0,
                max: 100.0,
            },
        }
    }

    /// Create a builder for configuring the gauge chart
    pub fn builder() -> GaugeChartBuilder<C> {
        GaugeChartBuilder::new()
    }

    /// Get the gauge type
    pub fn gauge_type(&self) -> GaugeType {
        self.gauge_type
    }

    /// Get the value range
    pub fn value_range(&self) -> ValueRange {
        self.value_range
    }

    /// Get the current gauge chart style
    pub fn style(&self) -> &GaugeChartStyle<C> {
        &self.style
    }

    /// Get the chart configuration
    pub fn config(&self) -> &ChartConfig<C> {
        &self.config
    }

    /// Calculate the angle for a given value
    fn value_to_angle(&self, value: f32) -> f32 {
        let normalized =
            (value - self.value_range.min) / (self.value_range.max - self.value_range.min);
        let normalized = normalized.clamp(0.0, 1.0);

        match self.gauge_type {
            GaugeType::Semicircle => -90.0 + (normalized * 180.0),
            GaugeType::ThreeQuarter => -135.0 + (normalized * 270.0),
            GaugeType::FullCircle => normalized * 360.0,
            GaugeType::Custom {
                start_angle,
                end_angle,
            } => start_angle + (normalized * (end_angle - start_angle)),
        }
    }

    /// Get the start and end angles for the gauge type
    fn get_angle_range(&self) -> (f32, f32) {
        match self.gauge_type {
            GaugeType::Semicircle => (-90.0, 90.0),
            GaugeType::ThreeQuarter => (-135.0, 135.0),
            GaugeType::FullCircle => (0.0, 360.0),
            GaugeType::Custom {
                start_angle,
                end_angle,
            } => (start_angle, end_angle),
        }
    }

    /// Draw the gauge background arc
    fn draw_background_arc<D>(&self, center: Point, target: &mut D) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        let (start_angle, end_angle) = self.get_angle_range();
        let radius = self.style.arc_style.radius;
        let segments = 60;
        let angle_step = (end_angle - start_angle) / segments as f32;

        for i in 0..segments {
            let angle1 = start_angle + (i as f32 * angle_step);
            let angle2 = start_angle + ((i + 1) as f32 * angle_step);
            let angle1_rad = angle1.to_radians();
            let angle2_rad = angle2.to_radians();
            let angle1_num = angle1_rad.to_number();
            let angle2_num = angle2_rad.to_number();
            let radius_num = (radius as f32).to_number();

            let cos1 = f32::from_number(Math::cos(angle1_num));
            let sin1 = f32::from_number(Math::sin(angle1_num));
            let cos2 = f32::from_number(Math::cos(angle2_num));
            let sin2 = f32::from_number(Math::sin(angle2_num));

            let x1 = center.x + (f32::from_number(radius_num) * cos1) as i32;
            let y1 = center.y + (f32::from_number(radius_num) * sin1) as i32;
            let x2 = center.x + (f32::from_number(radius_num) * cos2) as i32;
            let y2 = center.y + (f32::from_number(radius_num) * sin2) as i32;

            Line::new(Point::new(x1, y1), Point::new(x2, y2))
                .into_styled(PrimitiveStyle::with_stroke(
                    self.style.arc_style.background_color,
                    self.style.arc_style.background_width,
                ))
                .draw(target)
                .map_err(|_| ChartError::RenderingError)?;
        }
        Ok(())
    }

    /// Draw threshold zones
    fn draw_threshold_zones<D>(&self, center: Point, target: &mut D) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        let radius = self.style.arc_style.radius;
        let (start_angle, end_angle) = self.get_angle_range();

        for zone in &self.style.threshold_zones {
            let zone_start_angle = self
                .value_to_angle(zone.start)
                .max(start_angle)
                .min(end_angle);
            let zone_end_angle = self
                .value_to_angle(zone.end)
                .max(start_angle)
                .min(end_angle);

            if zone_start_angle >= zone_end_angle {
                continue;
            }

            let segments = ((zone_end_angle - zone_start_angle).abs() / 3.0).max(1.0) as u32;
            let angle_step = (zone_end_angle - zone_start_angle) / segments as f32;

            for i in 0..segments {
                let angle1 = zone_start_angle + (i as f32 * angle_step);
                let angle2 = zone_start_angle + ((i + 1) as f32 * angle_step);
                let angle1_rad = angle1.to_radians();
                let angle2_rad = angle2.to_radians();
                let angle1_num = angle1_rad.to_number();
                let angle2_num = angle2_rad.to_number();
                let radius_num = (radius as f32).to_number();

                let x1 = center.x + f32::from_number(radius_num * Math::cos(angle1_num)) as i32;
                let y1 = center.y + f32::from_number(radius_num * Math::sin(angle1_num)) as i32;
                let x2 = center.x + f32::from_number(radius_num * Math::cos(angle2_num)) as i32;
                let y2 = center.y + f32::from_number(radius_num * Math::sin(angle2_num)) as i32;

                Line::new(Point::new(x1, y1), Point::new(x2, y2))
                    .into_styled(PrimitiveStyle::with_stroke(
                        zone.color,
                        self.style.arc_style.value_width,
                    ))
                    .draw(target)
                    .map_err(|_| ChartError::RenderingError)?;
            }
        }
        Ok(())
    }

    /// Draw the needle
    fn draw_needle<D>(&self, center: Point, value: f32, target: &mut D) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        let angle = self.value_to_angle(value);
        let angle_rad = angle.to_radians();
        let needle_length =
            (self.style.arc_style.radius as f32 * self.style.needle_style.length) as u32;
        let angle_num = angle_rad.to_number();
        let needle_length_num = (needle_length as f32).to_number();

        let needle_end_x =
            center.x + f32::from_number(needle_length_num * Math::cos(angle_num)) as i32;
        let needle_end_y =
            center.y + f32::from_number(needle_length_num * Math::sin(angle_num)) as i32;

        Line::new(center, Point::new(needle_end_x, needle_end_y))
            .into_styled(PrimitiveStyle::with_stroke(
                self.style.needle_style.color,
                self.style.needle_style.width,
            ))
            .draw(target)
            .map_err(|_| ChartError::RenderingError)?;

        if matches!(
            self.style.needle_style.shape,
            NeedleShape::Arrow | NeedleShape::Pointer
        ) {
            let arrow_length = 8;
            let arrow_angle = 0.5;
            let arrow_angle1 = angle_rad + arrow_angle;
            let arrow_angle2 = angle_rad - arrow_angle;
            let arrow_angle1_num = arrow_angle1.to_number();
            let arrow_angle2_num = arrow_angle2.to_number();
            let arrow_length_num = (arrow_length as f32).to_number();

            let arrow_x1 = needle_end_x
                - f32::from_number(arrow_length_num * Math::cos(arrow_angle1_num)) as i32;
            let arrow_y1 = needle_end_y
                - f32::from_number(arrow_length_num * Math::sin(arrow_angle1_num)) as i32;
            let arrow_x2 = needle_end_x
                - f32::from_number(arrow_length_num * Math::cos(arrow_angle2_num)) as i32;
            let arrow_y2 = needle_end_y
                - f32::from_number(arrow_length_num * Math::sin(arrow_angle2_num)) as i32;

            Line::new(
                Point::new(needle_end_x, needle_end_y),
                Point::new(arrow_x1, arrow_y1),
            )
            .into_styled(PrimitiveStyle::with_stroke(
                self.style.needle_style.color,
                self.style.needle_style.width,
            ))
            .draw(target)
            .map_err(|_| ChartError::RenderingError)?;
            Line::new(
                Point::new(needle_end_x, needle_end_y),
                Point::new(arrow_x2, arrow_y2),
            )
            .into_styled(PrimitiveStyle::with_stroke(
                self.style.needle_style.color,
                self.style.needle_style.width,
            ))
            .draw(target)
            .map_err(|_| ChartError::RenderingError)?;
        }
        Ok(())
    }

    /// Draw the center hub
    fn draw_center_hub<D>(&self, center: Point, target: &mut D) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        if self.style.center_style.visible {
            Circle::new(
                Point::new(
                    center.x - self.style.center_style.radius as i32,
                    center.y - self.style.center_style.radius as i32,
                ),
                self.style.center_style.radius * 2,
            )
            .into_styled(PrimitiveStyle::with_fill(self.style.center_style.color))
            .draw(target)
            .map_err(|_| ChartError::RenderingError)?;
        }
        Ok(())
    }
}

impl<C: PixelColor> Default for GaugeChart<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<C: PixelColor> Chart<C> for GaugeChart<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    type Data = crate::data::series::StaticDataSeries<crate::data::point::Point2D, 1>;
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
    {
        if let Some(bg_color) = config.background_color {
            Rectangle::new(viewport.top_left, viewport.size)
                .into_styled(PrimitiveStyle::with_fill(bg_color))
                .draw(target)
                .map_err(|_| ChartError::RenderingError)?;
        }

        let draw_area = config.margins.apply_to(viewport);
        let center = Point::new(
            draw_area.top_left.x + draw_area.size.width as i32 / 2,
            draw_area.top_left.y + draw_area.size.height as i32 / 2,
        );

        let current_value = if let Some(point) = data.iter().next() {
            point.y()
        } else {
            0.0
        };

        self.draw_background_arc(center, target)?;
        self.draw_threshold_zones(center, target)?;
        self.draw_needle(center, current_value, target)?;
        self.draw_center_hub(center, target)?;

        Ok(())
    }
}

impl<C: PixelColor> Default for GaugeChartStyle<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        let mut threshold_zones = Vec::new();
        let _ = threshold_zones.push(ThresholdZone {
            start: 0.0,
            end: 30.0,
            color: embedded_graphics::pixelcolor::Rgb565::GREEN.into(),
            label: None,
        });
        let _ = threshold_zones.push(ThresholdZone {
            start: 30.0,
            end: 70.0,
            color: embedded_graphics::pixelcolor::Rgb565::YELLOW.into(),
            label: None,
        });
        let _ = threshold_zones.push(ThresholdZone {
            start: 70.0,
            end: 100.0,
            color: embedded_graphics::pixelcolor::Rgb565::RED.into(),
            label: None,
        });

        Self {
            arc_style: ArcStyle {
                background_color: embedded_graphics::pixelcolor::Rgb565::CSS_GRAY.into(),
                background_width: 8,
                value_color: None,
                value_width: 8,
                radius: 80,
            },
            needle_style: NeedleStyle {
                shape: NeedleShape::Arrow,
                color: embedded_graphics::pixelcolor::Rgb565::BLACK.into(),
                length: 0.8,
                width: 2,
                animated: true,
            },
            threshold_zones,
            center_style: CenterStyle {
                color: embedded_graphics::pixelcolor::Rgb565::BLACK.into(),
                radius: 5,
                visible: true,
            },
            tick_style: Some(TickStyle {
                major_color: embedded_graphics::pixelcolor::Rgb565::BLACK.into(),
                minor_color: embedded_graphics::pixelcolor::Rgb565::CSS_GRAY.into(),
                major_length: 10,
                minor_length: 5,
                major_width: 2,
                minor_width: 1,
                major_count: 10,
                minor_count: 5,
            }),
            value_display: Some(ValueDisplayStyle {
                color: embedded_graphics::pixelcolor::Rgb565::BLACK.into(),
                font_size: 12,
                position: ValueDisplayPosition::Below,
                format: ValueFormat::Integer,
                show_units: false,
                units: None,
            }),
        }
    }
}

/// Builder for gauge charts
#[derive(Debug)]
pub struct GaugeChartBuilder<C: PixelColor> {
    style: GaugeChartStyle<C>,
    config: ChartConfig<C>,
    gauge_type: GaugeType,
    value_range: ValueRange,
}

impl<C: PixelColor> GaugeChartBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new gauge chart builder
    pub fn new() -> Self {
        Self {
            style: GaugeChartStyle::default(),
            config: ChartConfig::default(),
            gauge_type: GaugeType::Semicircle,
            value_range: ValueRange {
                min: 0.0,
                max: 100.0,
            },
        }
    }

    /// Set the gauge type
    pub fn gauge_type(mut self, gauge_type: GaugeType) -> Self {
        self.gauge_type = gauge_type;
        self
    }

    /// Set the value range
    pub fn value_range(mut self, min: f32, max: f32) -> Self {
        self.value_range = ValueRange { min, max };
        self
    }

    /// Set the arc radius
    pub fn radius(mut self, radius: u32) -> Self {
        self.style.arc_style.radius = radius;
        self
    }

    /// Set the needle style
    pub fn needle_style(mut self, shape: NeedleShape, color: C, length: f32, width: u32) -> Self {
        self.style.needle_style = NeedleStyle {
            shape,
            color,
            length: length.clamp(0.0, 1.0),
            width,
            animated: self.style.needle_style.animated,
        };
        self
    }

    /// Add a threshold zone
    pub fn add_threshold_zone(mut self, start: f32, end: f32, color: C) -> Self {
        if self.style.threshold_zones.len() < 8 {
            let _ = self.style.threshold_zones.push(ThresholdZone {
                start,
                end,
                color,
                label: None,
            });
        }
        self
    }

    /// Set the chart title
    pub fn with_title(mut self, title: &str) -> Self {
        if let Ok(title_string) = heapless::String::try_from(title) {
            self.config.title = Some(title_string);
        }
        self
    }

    /// Build the gauge chart
    pub fn build(self) -> ChartResult<GaugeChart<C>> {
        Ok(GaugeChart {
            style: self.style,
            config: self.config,
            gauge_type: self.gauge_type,
            value_range: self.value_range,
        })
    }
}

impl<C: PixelColor> ChartBuilder<C> for GaugeChartBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    type Chart = GaugeChart<C>;
    type Error = ChartError;

    fn build(self) -> Result<Self::Chart, Self::Error> {
        Ok(GaugeChart {
            style: self.style,
            config: self.config,
            gauge_type: self.gauge_type,
            value_range: self.value_range,
        })
    }
}

impl<C: PixelColor> Default for GaugeChartBuilder<C>
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
    fn test_gauge_chart_creation() {
        let chart: GaugeChart<Rgb565> = GaugeChart::new();
        assert_eq!(chart.gauge_type(), GaugeType::Semicircle);
        assert_eq!(chart.value_range().min, 0.0);
        assert_eq!(chart.value_range().max, 100.0);
    }

    #[test]
    fn test_gauge_chart_builder() {
        let chart: GaugeChart<Rgb565> = GaugeChart::builder()
            .gauge_type(GaugeType::ThreeQuarter)
            .value_range(0.0, 200.0)
            .radius(100)
            .needle_style(NeedleShape::Arrow, Rgb565::RED, 0.9, 3)
            .with_title("Test Gauge")
            .build()
            .unwrap();

        assert_eq!(chart.gauge_type(), GaugeType::ThreeQuarter);
        assert_eq!(chart.value_range().min, 0.0);
        assert_eq!(chart.value_range().max, 200.0);
        assert_eq!(chart.style().arc_style.radius, 100);
    }

    #[test]
    fn test_value_to_angle_conversion() {
        let chart = GaugeChart::<Rgb565>::builder()
            .gauge_type(GaugeType::Semicircle)
            .value_range(0.0, 100.0)
            .build()
            .unwrap();

        assert_eq!(chart.value_to_angle(0.0), -90.0);
        assert_eq!(chart.value_to_angle(50.0), 0.0);
        assert_eq!(chart.value_to_angle(100.0), 90.0);
    }
}
