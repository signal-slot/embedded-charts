//! Styling configuration for axes.

use crate::style::LineStyle;
use embedded_graphics::prelude::*;

/// Style configuration for an axis
#[derive(Debug, Clone)]
pub struct AxisStyle<C: PixelColor> {
    /// Style for the main axis line
    pub axis_line: LineStyle<C>,
    /// Style for major tick marks
    pub major_ticks: TickStyle<C>,
    /// Style for minor tick marks
    pub minor_ticks: TickStyle<C>,
    /// Style for grid lines
    pub grid_lines: Option<LineStyle<C>>,
    /// Style for axis labels
    pub labels: LabelStyle<C>,
    /// Spacing between the axis and labels
    pub label_offset: u32,
}

/// Style configuration for tick marks
#[derive(Debug, Clone)]
pub struct TickStyle<C: PixelColor> {
    /// Line style for the tick mark
    pub line: LineStyle<C>,
    /// Length of the tick mark in pixels
    pub length: u32,
    /// Whether to show this type of tick
    pub visible: bool,
}

/// Style configuration for axis labels
#[derive(Debug, Clone)]
pub struct LabelStyle<C: PixelColor> {
    /// Text color
    pub color: C,
    /// Font size (if supported by the font system)
    pub font_size: u32,
    /// Whether to show labels
    pub visible: bool,
    /// Text alignment relative to tick position
    pub alignment: TextAlignment,
    /// Rotation angle in degrees (0, 90, 180, 270)
    pub rotation: u16,
}

/// Text alignment options for labels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlignment {
    /// Align text to the left/top
    Start,
    /// Center text
    Center,
    /// Align text to the right/bottom
    End,
}

impl<C: PixelColor> AxisStyle<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new axis style with default values
    pub fn new() -> Self {
        Self {
            axis_line: LineStyle::solid(embedded_graphics::pixelcolor::Rgb565::RED.into()).width(3),
            major_ticks: TickStyle::new(embedded_graphics::pixelcolor::Rgb565::RED.into(), 10),
            minor_ticks: TickStyle::new(embedded_graphics::pixelcolor::Rgb565::BLUE.into(), 5),
            grid_lines: None,
            labels: LabelStyle::new(embedded_graphics::pixelcolor::Rgb565::BLACK.into()),
            label_offset: 8,
        }
    }

    /// Set the axis line style
    pub fn with_axis_line(mut self, style: LineStyle<C>) -> Self {
        self.axis_line = style;
        self
    }

    /// Set the major tick style
    pub fn with_major_ticks(mut self, style: TickStyle<C>) -> Self {
        self.major_ticks = style;
        self
    }

    /// Set the minor tick style
    pub fn with_minor_ticks(mut self, style: TickStyle<C>) -> Self {
        self.minor_ticks = style;
        self
    }

    /// Enable grid lines with the specified style
    pub fn with_grid_lines(mut self, style: LineStyle<C>) -> Self {
        self.grid_lines = Some(style);
        self
    }

    /// Disable grid lines
    pub fn without_grid_lines(mut self) -> Self {
        self.grid_lines = None;
        self
    }

    /// Set the label style
    pub fn with_labels(mut self, style: LabelStyle<C>) -> Self {
        self.labels = style;
        self
    }

    /// Set the label offset
    pub fn with_label_offset(mut self, offset: u32) -> Self {
        self.label_offset = offset;
        self
    }

    /// Create a minimal style for small displays
    pub fn minimal() -> Self {
        Self {
            axis_line: LineStyle::solid(embedded_graphics::pixelcolor::Rgb565::BLACK.into()),
            major_ticks: TickStyle::new(embedded_graphics::pixelcolor::Rgb565::BLACK.into(), 3),
            minor_ticks: TickStyle::new(embedded_graphics::pixelcolor::Rgb565::BLACK.into(), 1)
                .hidden(),
            grid_lines: None,
            labels: LabelStyle::new(embedded_graphics::pixelcolor::Rgb565::BLACK.into())
                .with_font_size(8),
            label_offset: 4,
        }
    }

    /// Create a professional style
    pub fn professional() -> Self {
        Self {
            axis_line: LineStyle::solid(embedded_graphics::pixelcolor::Rgb565::BLACK.into()),
            major_ticks: TickStyle::new(embedded_graphics::pixelcolor::Rgb565::BLACK.into(), 8),
            minor_ticks: TickStyle::new(
                embedded_graphics::pixelcolor::Rgb565::new(16, 32, 16).into(), // Gray
                4,                                                             // Normal length
            )
            .with_width(1), // Normal width
            grid_lines: Some(LineStyle::solid(
                embedded_graphics::pixelcolor::Rgb565::new(25, 50, 25).into(),
            )),
            labels: LabelStyle::new(embedded_graphics::pixelcolor::Rgb565::BLACK.into()),
            label_offset: 10,
        }
    }
}

impl<C: PixelColor> TickStyle<C> {
    /// Create a new tick style
    pub fn new(color: C, length: u32) -> Self {
        Self {
            line: LineStyle::solid(color),
            length,
            visible: true,
        }
    }

    /// Set the tick color
    pub fn with_color(mut self, color: C) -> Self {
        self.line = self.line.color(color);
        self
    }

    /// Set the tick width
    pub fn with_width(mut self, width: u32) -> Self {
        self.line = self.line.width(width);
        self
    }

    /// Set the tick length
    pub fn with_length(mut self, length: u32) -> Self {
        self.length = length;
        self
    }

    /// Hide this type of tick
    pub fn hidden(mut self) -> Self {
        self.visible = false;
        self
    }

    /// Show this type of tick
    pub fn visible(mut self) -> Self {
        self.visible = true;
        self
    }
}

impl<C: PixelColor> LabelStyle<C> {
    /// Create a new label style
    pub fn new(color: C) -> Self {
        Self {
            color,
            font_size: 12,
            visible: true,
            alignment: TextAlignment::Center,
            rotation: 0,
        }
    }

    /// Set the label color
    pub fn with_color(mut self, color: C) -> Self {
        self.color = color;
        self
    }

    /// Set the font size
    pub fn with_font_size(mut self, size: u32) -> Self {
        self.font_size = size;
        self
    }

    /// Set the text alignment
    pub fn with_alignment(mut self, alignment: TextAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Set the rotation angle (0, 90, 180, 270 degrees)
    pub fn with_rotation(mut self, degrees: u16) -> Self {
        self.rotation = match degrees {
            0..=45 => 0,
            46..=135 => 90,
            136..=225 => 180,
            226..=315 => 270,
            _ => 0,
        };
        self
    }

    /// Hide labels
    pub fn hidden(mut self) -> Self {
        self.visible = false;
        self
    }

    /// Show labels
    pub fn visible(mut self) -> Self {
        self.visible = true;
        self
    }
}

impl<C: PixelColor> Default for AxisStyle<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<C: PixelColor> Default for TickStyle<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::new(embedded_graphics::pixelcolor::Rgb565::BLACK.into(), 4)
    }
}

impl<C: PixelColor> Default for LabelStyle<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::new(embedded_graphics::pixelcolor::Rgb565::BLACK.into())
    }
}

impl Default for TextAlignment {
    fn default() -> Self {
        Self::Center
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::pixelcolor::Rgb565;

    #[test]
    fn test_axis_style_creation() {
        let style: AxisStyle<Rgb565> = AxisStyle::new();
        assert!(style.major_ticks.visible);
        assert!(style.minor_ticks.visible);
        assert!(style.grid_lines.is_none());
    }

    #[test]
    fn test_tick_style_builder() {
        let style = TickStyle::new(Rgb565::RED, 5).with_width(2).with_length(8);

        assert_eq!(style.length, 8);
        assert!(style.visible);
    }

    #[test]
    fn test_label_style_builder() {
        let style = LabelStyle::new(Rgb565::BLUE)
            .with_font_size(14)
            .with_alignment(TextAlignment::Start)
            .with_rotation(90);

        assert_eq!(style.font_size, 14);
        assert_eq!(style.alignment, TextAlignment::Start);
        assert_eq!(style.rotation, 90);
    }

    #[test]
    fn test_professional_style() {
        let style: AxisStyle<Rgb565> = AxisStyle::professional();
        assert!(style.grid_lines.is_some());
        assert_eq!(style.label_offset, 10);
    }

    #[test]
    fn test_minimal_style() {
        let style: AxisStyle<Rgb565> = AxisStyle::minimal();
        assert!(!style.minor_ticks.visible);
        assert_eq!(style.label_offset, 4);
    }
}
