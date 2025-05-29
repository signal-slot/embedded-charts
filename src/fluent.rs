//! Fluent API for creating charts with an intuitive, readable syntax.
//!
//! This module provides a streamlined interface for creating charts that reduces
//! boilerplate and makes common chart configurations more accessible. The fluent
//! API maintains all the performance and memory characteristics of the core library
//! while providing a more developer-friendly interface.
//!
//! # Examples
//!
//! ## Simple Line Chart
//! ```rust
//! use embedded_charts::fluent::Chart;
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! let data = [(0.0, 10.0), (1.0, 20.0), (2.0, 15.0)];
//! let chart = Chart::line()
//!     .data_from_tuples(&data)
//!     .color(Rgb565::BLUE)
//!     .title("Temperature")
//!     .build()?;
//! # Ok::<(), embedded_charts::error::ChartError>(())
//! ```
//!
//! ## Professional Styled Chart
//! ```rust
//! use embedded_charts::fluent::Chart;
//! use embedded_charts::prelude::*;
//!
//! let chart = Chart::line()
//!     .preset(ChartPreset::Professional)
//!     .data_from_tuples(&[(0.0, 10.0), (1.0, 20.0)])
//!     .title("Sales Data")
//!     .build()?;
//! # Ok::<(), embedded_charts::error::ChartError>(())
//! ```
//!
//! ## Multi-Series Chart
//! ```rust
//! use embedded_charts::fluent::Chart;
//! use embedded_charts::prelude::*;
//!
//! let chart = Chart::line()
//!     .series("Temperature", &[(0.0, 22.5), (1.0, 23.1)])
//!     .series("Humidity", &[(0.0, 65.0), (1.0, 68.0)])
//!     .colors(ColorPalette::professional())
//!     .build()?;
//! # Ok::<(), embedded_charts::error::ChartError>(())
//! ```

#[cfg(any(feature = "line", feature = "bar"))]
use crate::chart::traits::ChartBuilder;
#[cfg(any(feature = "line", feature = "bar"))]
use crate::data::{MultiSeries, Point2D, StaticDataSeries};
#[cfg(any(feature = "line", feature = "bar"))]
use crate::error::ChartResult;
#[cfg(any(feature = "line", feature = "bar"))]
use embedded_graphics::prelude::*;
#[cfg(any(feature = "line", feature = "bar"))]
use heapless::String;

/// Chart presets for common styling patterns
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChartPreset {
    /// Clean, minimal styling suitable for dashboards
    Professional,
    /// High contrast colors for embedded displays
    Embedded,
    /// Colorful, vibrant styling
    Vibrant,
    /// Subtle, pastel colors
    Pastel,
    /// Dark theme for low-light environments
    Dark,
}

/// Fluent API builder for creating charts
pub struct Chart;

impl Chart {
    /// Start building a line chart
    #[cfg(feature = "line")]
    pub fn line<C>() -> FluentLineChartBuilder<C>
    where
        C: PixelColor + From<embedded_graphics::pixelcolor::Rgb565> + 'static,
    {
        FluentLineChartBuilder::new()
    }

    /// Start building a bar chart
    #[cfg(feature = "bar")]
    pub fn bar<C>() -> FluentBarChartBuilder<C>
    where
        C: PixelColor + From<embedded_graphics::pixelcolor::Rgb565> + 'static,
    {
        FluentBarChartBuilder::new()
    }
}

/// Fluent builder for line charts
#[cfg(feature = "line")]
pub struct FluentLineChartBuilder<C: PixelColor> {
    data: Option<StaticDataSeries<Point2D, 256>>,
    multi_data: Option<MultiSeries<Point2D, 8, 256>>,
    color: Option<C>,
    title: Option<String<64>>,
    preset: Option<ChartPreset>,
    line_width: Option<u32>,
    show_markers: bool,
    marker_size: Option<u32>,
}

#[cfg(feature = "line")]
impl<C: PixelColor + 'static> FluentLineChartBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn new() -> Self {
        Self {
            data: None,
            multi_data: None,
            color: None,
            title: None,
            preset: None,
            line_width: None,
            show_markers: false,
            marker_size: None,
        }
    }

    /// Set data from an array of tuples
    pub fn data_from_tuples(mut self, tuples: &[(f32, f32)]) -> Self {
        let series =
            StaticDataSeries::from_tuples(tuples).unwrap_or_else(|_| StaticDataSeries::new());
        self.data = Some(series);
        self
    }

    /// Add a data series with a label
    pub fn series(mut self, label: &str, tuples: &[(f32, f32)]) -> Self {
        if self.multi_data.is_none() {
            self.multi_data = Some(MultiSeries::new());
        }

        if let Some(ref mut multi_data) = self.multi_data {
            let mut series = StaticDataSeries::with_label(label);
            for &(x, y) in tuples {
                if series.push(Point2D::new(x, y)).is_err() {
                    break; // Stop if buffer is full
                }
            }
            let _ = multi_data.add_series(series);
        }
        self
    }

    /// Set the line color
    pub fn color(mut self, color: C) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the chart title
    pub fn title(mut self, title: &str) -> Self {
        if let Ok(title_string) = String::try_from(title) {
            self.title = Some(title_string);
        }
        self
    }

    /// Apply a preset style
    pub fn preset(mut self, preset: ChartPreset) -> Self {
        self.preset = Some(preset);
        self
    }

    /// Set line width
    pub fn line_width(mut self, width: u32) -> Self {
        self.line_width = Some(width);
        self
    }

    /// Enable markers on data points
    pub fn with_markers(mut self) -> Self {
        self.show_markers = true;
        self
    }

    /// Set marker size
    pub fn marker_size(mut self, size: u32) -> Self {
        self.marker_size = Some(size);
        self.show_markers = true;
        self
    }

    /// Build the line chart
    pub fn build(self) -> ChartResult<crate::chart::LineChart<C>> {
        let mut builder = crate::chart::LineChart::builder();

        // Apply preset styling first
        if let Some(preset) = self.preset {
            builder = self.apply_preset_to_line_builder(builder, preset);
        }

        // Apply specific customizations
        if let Some(color) = self.color {
            builder = builder.line_color(color);
        }

        if let Some(width) = self.line_width {
            builder = builder.line_width(width);
        }

        if self.show_markers {
            let marker_style = crate::chart::MarkerStyle {
                shape: crate::chart::MarkerShape::Circle,
                size: self.marker_size.unwrap_or(4),
                color: self
                    .color
                    .unwrap_or_else(|| C::from(embedded_graphics::pixelcolor::Rgb565::BLUE)),
                visible: true,
            };
            builder = builder.with_markers(marker_style);
        }

        builder.build()
    }

    fn apply_preset_to_line_builder(
        &self,
        mut builder: crate::chart::LineChartBuilder<C>,
        preset: ChartPreset,
    ) -> crate::chart::LineChartBuilder<C> {
        match preset {
            ChartPreset::Professional => {
                let color = C::from(embedded_graphics::pixelcolor::Rgb565::new(
                    70 >> 3,
                    130 >> 2,
                    180 >> 3,
                ));
                builder = builder.line_color(color).line_width(2);
            }
            ChartPreset::Embedded => {
                let color = C::from(embedded_graphics::pixelcolor::Rgb565::new(0, 31, 0)); // Bright green
                builder = builder.line_color(color).line_width(1);
            }
            ChartPreset::Vibrant => {
                let color = C::from(embedded_graphics::pixelcolor::Rgb565::new(
                    236 >> 3,
                    72 >> 2,
                    153 >> 3,
                ));
                builder = builder.line_color(color).line_width(3);
            }
            ChartPreset::Pastel => {
                let color = C::from(embedded_graphics::pixelcolor::Rgb565::new(
                    147 >> 3,
                    197 >> 2,
                    253 >> 3,
                ));
                builder = builder.line_color(color).line_width(2);
            }
            ChartPreset::Dark => {
                let color = C::from(embedded_graphics::pixelcolor::Rgb565::new(
                    200 >> 3,
                    200 >> 2,
                    200 >> 3,
                ));
                builder = builder.line_color(color).line_width(1);
            }
        }
        builder
    }
}

/// Fluent builder for bar charts
#[cfg(feature = "bar")]
pub struct FluentBarChartBuilder<C: PixelColor> {
    data: Option<StaticDataSeries<Point2D, 256>>,
    color: Option<C>,
    title: Option<String<64>>,
    preset: Option<ChartPreset>,
    bar_width: Option<u32>,
}

#[cfg(feature = "bar")]
impl<C: PixelColor + 'static> FluentBarChartBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn new() -> Self {
        Self {
            data: None,
            color: None,
            title: None,
            preset: None,
            bar_width: None,
        }
    }

    /// Set data from an array of tuples
    pub fn data_from_tuples(mut self, tuples: &[(f32, f32)]) -> Self {
        let series =
            StaticDataSeries::from_tuples(tuples).unwrap_or_else(|_| StaticDataSeries::new());
        self.data = Some(series);
        self
    }

    /// Set the bar color
    pub fn color(mut self, color: C) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the chart title
    pub fn title(mut self, title: &str) -> Self {
        if let Ok(title_string) = String::try_from(title) {
            self.title = Some(title_string);
        }
        self
    }

    /// Apply a preset style
    pub fn preset(mut self, preset: ChartPreset) -> Self {
        self.preset = Some(preset);
        self
    }

    /// Set bar width
    pub fn bar_width(mut self, width: u32) -> Self {
        self.bar_width = Some(width);
        self
    }

    /// Build the bar chart
    pub fn build(self) -> ChartResult<crate::chart::BarChart<C>> {
        let mut builder = crate::chart::BarChart::builder();

        // Apply preset styling first
        if let Some(preset) = self.preset {
            builder = self.apply_preset_to_bar_builder(builder, preset);
        }

        // Apply specific customizations
        if let Some(color) = self.color {
            builder = builder.colors(&[color]);
        }

        if let Some(width) = self.bar_width {
            builder = builder.bar_width(crate::chart::bar::BarWidth::Fixed(width));
        }

        builder.build()
    }

    fn apply_preset_to_bar_builder(
        &self,
        mut builder: crate::chart::BarChartBuilder<C>,
        preset: ChartPreset,
    ) -> crate::chart::BarChartBuilder<C> {
        match preset {
            ChartPreset::Professional => {
                let color = C::from(embedded_graphics::pixelcolor::Rgb565::new(
                    59 >> 3,
                    130 >> 2,
                    246 >> 3,
                ));
                builder = builder.colors(&[color]);
            }
            ChartPreset::Embedded => {
                let color = C::from(embedded_graphics::pixelcolor::Rgb565::new(0, 31, 0));
                builder = builder.colors(&[color]);
            }
            ChartPreset::Vibrant => {
                let color = C::from(embedded_graphics::pixelcolor::Rgb565::new(
                    245 >> 3,
                    101 >> 2,
                    101 >> 3,
                ));
                builder = builder.colors(&[color]);
            }
            ChartPreset::Pastel => {
                let color = C::from(embedded_graphics::pixelcolor::Rgb565::new(
                    167 >> 3,
                    243 >> 2,
                    208 >> 3,
                ));
                builder = builder.colors(&[color]);
            }
            ChartPreset::Dark => {
                let color = C::from(embedded_graphics::pixelcolor::Rgb565::new(
                    150 >> 3,
                    150 >> 2,
                    150 >> 3,
                ));
                builder = builder.colors(&[color]);
            }
        }
        builder
    }
}

// Similar implementations for other chart types would follow...

/// Quick creation functions for common chart types
pub mod quick {
    #[cfg(any(feature = "line", feature = "bar"))]
    use super::*;

    /// Create a simple line chart from data tuples
    #[cfg(feature = "line")]
    pub fn line_chart<C>(data: &[(f32, f32)]) -> ChartResult<crate::chart::LineChart<C>>
    where
        C: PixelColor + From<embedded_graphics::pixelcolor::Rgb565> + 'static,
    {
        Chart::line().data_from_tuples(data).build()
    }

    /// Create a professional line chart from data tuples
    #[cfg(feature = "line")]
    pub fn professional_line_chart<C>(
        data: &[(f32, f32)],
    ) -> ChartResult<crate::chart::LineChart<C>>
    where
        C: PixelColor + From<embedded_graphics::pixelcolor::Rgb565> + 'static,
    {
        Chart::line()
            .data_from_tuples(data)
            .preset(ChartPreset::Professional)
            .with_markers()
            .build()
    }

    /// Create a simple bar chart from data tuples
    #[cfg(feature = "bar")]
    pub fn bar_chart<C>(data: &[(f32, f32)]) -> ChartResult<crate::chart::BarChart<C>>
    where
        C: PixelColor + From<embedded_graphics::pixelcolor::Rgb565> + 'static,
    {
        Chart::bar().data_from_tuples(data).build()
    }

    /// Create an embedded-optimized chart (high contrast, minimal styling)
    #[cfg(feature = "line")]
    pub fn embedded_line_chart<C>(data: &[(f32, f32)]) -> ChartResult<crate::chart::LineChart<C>>
    where
        C: PixelColor + From<embedded_graphics::pixelcolor::Rgb565> + 'static,
    {
        Chart::line()
            .data_from_tuples(data)
            .preset(ChartPreset::Embedded)
            .build()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "line")]
    use super::*;
    #[cfg(feature = "line")]
    use embedded_graphics::pixelcolor::Rgb565;

    #[test]
    #[cfg(feature = "line")]
    fn test_fluent_line_chart() {
        let data = [(0.0, 10.0), (1.0, 20.0), (2.0, 15.0)];
        let chart = Chart::line::<Rgb565>()
            .data_from_tuples(&data)
            .color(Rgb565::BLUE)
            .title("Test Chart")
            .with_markers()
            .build();

        assert!(chart.is_ok());
    }

    #[test]
    #[cfg(feature = "line")]
    fn test_quick_line_chart() {
        let data = [(0.0, 10.0), (1.0, 20.0), (2.0, 15.0)];
        let chart = quick::line_chart::<Rgb565>(&data);
        assert!(chart.is_ok());
    }

    #[test]
    #[cfg(feature = "line")]
    fn test_professional_preset() {
        let data = [(0.0, 10.0), (1.0, 20.0)];
        let chart = Chart::line::<Rgb565>()
            .data_from_tuples(&data)
            .preset(ChartPreset::Professional)
            .build();

        assert!(chart.is_ok());
    }

    #[test]
    #[cfg(feature = "line")]
    fn test_multi_series() {
        let chart = Chart::line::<Rgb565>()
            .series("Series 1", &[(0.0, 10.0), (1.0, 20.0)])
            .series("Series 2", &[(0.0, 15.0), (1.0, 25.0)])
            .preset(ChartPreset::Vibrant)
            .build();

        assert!(chart.is_ok());
    }
}
