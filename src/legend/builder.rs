//! Builder pattern for legend configuration.

use crate::error::{ChartError, ChartResult};
use crate::legend::{
    position::{LegendAlignment, LegendMargins, LegendPosition},
    style::LegendStyle,
    traits::Legend,
    types::{
        CompactLegend, CompactLegendEntry, CustomLegend, CustomLegendEntry, LegendEntryType,
        LegendOrientation, StandardLegend, StandardLegendEntry,
    },
};
use embedded_graphics::prelude::*;

/// Builder for creating legends with fluent configuration
pub trait LegendBuilder<C: PixelColor> {
    /// The legend type this builder creates
    type Legend: Legend<C>;
    /// Error type for building operations
    type Error;

    /// Build the legend with current configuration
    fn build(self) -> Result<Self::Legend, Self::Error>;
}

/// Builder for standard legends
#[derive(Debug)]
pub struct StandardLegendBuilder<C: PixelColor> {
    position: LegendPosition,
    orientation: LegendOrientation,
    style: LegendStyle<C>,
    alignment: LegendAlignment,
    margins: LegendMargins,
    entries: heapless::Vec<StandardLegendEntry<C>, 16>,
}

/// Builder for compact legends
#[derive(Debug)]
pub struct CompactLegendBuilder<C: PixelColor> {
    position: LegendPosition,
    orientation: LegendOrientation,
    style: LegendStyle<C>,
    #[allow(dead_code)]
    alignment: LegendAlignment,
    #[allow(dead_code)]
    margins: LegendMargins,
    entries: heapless::Vec<CompactLegendEntry<C>, 8>,
}

/// Builder for custom legends
#[derive(Debug)]
pub struct CustomLegendBuilder<C: PixelColor> {
    position: LegendPosition,
    orientation: LegendOrientation,
    style: LegendStyle<C>,
    #[allow(dead_code)]
    alignment: LegendAlignment,
    #[allow(dead_code)]
    margins: LegendMargins,
    entries: heapless::Vec<CustomLegendEntry<C>, 12>,
    layout_params: crate::legend::types::CustomLayoutParams,
}

impl<C: PixelColor> StandardLegendBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new standard legend builder
    pub fn new() -> Self {
        Self {
            position: LegendPosition::Right,
            orientation: LegendOrientation::Vertical,
            style: LegendStyle::new(),
            alignment: LegendAlignment::Start,
            margins: LegendMargins::default(),
            entries: heapless::Vec::new(),
        }
    }

    /// Set the legend position
    pub fn position(mut self, position: LegendPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the legend orientation
    pub fn orientation(mut self, orientation: LegendOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set the legend style
    pub fn style(mut self, style: LegendStyle<C>) -> Self {
        self.style = style;
        self
    }

    /// Use minimal styling for small displays
    pub fn minimal_style(mut self) -> Self {
        self.style = LegendStyle::minimal();
        self
    }

    /// Use professional styling
    pub fn professional_style(mut self) -> Self {
        self.style = LegendStyle::professional();
        self
    }

    /// Use compact styling
    pub fn compact_style(mut self) -> Self {
        self.style = LegendStyle::compact();
        self
    }

    /// Set the legend alignment
    pub fn alignment(mut self, alignment: LegendAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Set the legend margins
    pub fn margins(mut self, margins: LegendMargins) -> Self {
        self.margins = margins;
        self
    }

    /// Add a line entry to the legend
    pub fn add_line_entry(mut self, label: &str, color: C) -> ChartResult<Self> {
        let entry_type = LegendEntryType::Line {
            color,
            width: 2,
            pattern: crate::style::LinePattern::Solid,
            marker: None,
        };
        let entry = StandardLegendEntry::new(label, entry_type)?;
        self.entries
            .push(entry)
            .map_err(|_| ChartError::ConfigurationError)?;
        Ok(self)
    }

    /// Add a line entry with marker to the legend
    pub fn add_line_entry_with_marker(
        mut self,
        label: &str,
        color: C,
        marker: crate::legend::types::MarkerStyle<C>,
    ) -> ChartResult<Self> {
        let entry_type = LegendEntryType::Line {
            color,
            width: 2,
            pattern: crate::style::LinePattern::Solid,
            marker: Some(marker),
        };
        let entry = StandardLegendEntry::new(label, entry_type)?;
        self.entries
            .push(entry)
            .map_err(|_| ChartError::ConfigurationError)?;
        Ok(self)
    }

    /// Add a bar entry to the legend
    pub fn add_bar_entry(mut self, label: &str, color: C) -> ChartResult<Self> {
        let entry_type = LegendEntryType::Bar {
            color,
            border_color: None,
            border_width: 0,
        };
        let entry = StandardLegendEntry::new(label, entry_type)?;
        self.entries
            .push(entry)
            .map_err(|_| ChartError::ConfigurationError)?;
        Ok(self)
    }

    /// Add a bar entry with border to the legend
    pub fn add_bar_entry_with_border(
        mut self,
        label: &str,
        color: C,
        border_color: C,
        border_width: u32,
    ) -> ChartResult<Self> {
        let entry_type = LegendEntryType::Bar {
            color,
            border_color: Some(border_color),
            border_width,
        };
        let entry = StandardLegendEntry::new(label, entry_type)?;
        self.entries
            .push(entry)
            .map_err(|_| ChartError::ConfigurationError)?;
        Ok(self)
    }

    /// Add a pie entry to the legend
    pub fn add_pie_entry(mut self, label: &str, color: C) -> ChartResult<Self> {
        let entry_type = LegendEntryType::Pie {
            color,
            border_color: None,
            border_width: 0,
        };
        let entry = StandardLegendEntry::new(label, entry_type)?;
        self.entries
            .push(entry)
            .map_err(|_| ChartError::ConfigurationError)?;
        Ok(self)
    }

    /// Add a custom symbol entry to the legend
    pub fn add_custom_entry(
        mut self,
        label: &str,
        color: C,
        shape: crate::legend::types::SymbolShape,
        size: u32,
    ) -> ChartResult<Self> {
        let entry_type = LegendEntryType::Custom { color, shape, size };
        let entry = StandardLegendEntry::new(label, entry_type)?;
        self.entries
            .push(entry)
            .map_err(|_| ChartError::ConfigurationError)?;
        Ok(self)
    }

    /// Add a generic entry to the legend
    pub fn add_entry(mut self, label: &str, entry_type: LegendEntryType<C>) -> ChartResult<Self> {
        let entry = StandardLegendEntry::new(label, entry_type)?;
        self.entries
            .push(entry)
            .map_err(|_| ChartError::ConfigurationError)?;
        Ok(self)
    }

    /// Clear all entries
    pub fn clear_entries(mut self) -> Self {
        self.entries.clear();
        self
    }
}

impl<C: PixelColor> LegendBuilder<C> for StandardLegendBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    type Legend = StandardLegend<C>;
    type Error = ChartError;

    fn build(self) -> Result<Self::Legend, Self::Error> {
        let mut legend = StandardLegend::new(self.position);
        legend.set_orientation(self.orientation);
        legend.set_style(self.style);

        // Add all entries
        for entry in self.entries {
            legend.add_entry(entry)?;
        }

        Ok(legend)
    }
}

impl<C: PixelColor> Default for StandardLegendBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<C: PixelColor> CompactLegendBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new compact legend builder
    pub fn new() -> Self {
        Self {
            position: LegendPosition::Right,
            orientation: LegendOrientation::Vertical,
            style: LegendStyle::compact(),
            alignment: LegendAlignment::Start,
            margins: LegendMargins::all(4),
            entries: heapless::Vec::new(),
        }
    }

    /// Set the legend position
    pub fn position(mut self, position: LegendPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the legend orientation
    pub fn orientation(mut self, orientation: LegendOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Add a simple entry with just color
    pub fn add_simple_entry(mut self, label: &str, color: C) -> ChartResult<Self> {
        let entry_type = LegendEntryType::Custom {
            color,
            shape: crate::legend::types::SymbolShape::Circle,
            size: 8,
        };
        let entry = CompactLegendEntry::new(label, entry_type)?;
        self.entries
            .push(entry)
            .map_err(|_| ChartError::ConfigurationError)?;
        Ok(self)
    }
}

impl<C: PixelColor> LegendBuilder<C> for CompactLegendBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    type Legend = CompactLegend<C>;
    type Error = ChartError;

    fn build(self) -> Result<Self::Legend, Self::Error> {
        let mut legend = CompactLegend::new(self.position);
        legend.set_orientation(self.orientation);
        legend.set_style(self.style);

        // Add all entries
        for entry in self.entries {
            legend.add_entry(entry)?;
        }

        Ok(legend)
    }
}

impl<C: PixelColor> Default for CompactLegendBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<C: PixelColor> CustomLegendBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new custom legend builder
    pub fn new() -> Self {
        Self {
            position: LegendPosition::Right,
            orientation: LegendOrientation::Vertical,
            style: LegendStyle::new(),
            alignment: LegendAlignment::Start,
            margins: LegendMargins::default(),
            entries: heapless::Vec::new(),
            layout_params: crate::legend::types::CustomLayoutParams::default(),
        }
    }

    /// Set the legend position
    pub fn position(mut self, position: LegendPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the legend orientation
    pub fn orientation(mut self, orientation: LegendOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set custom layout parameters
    pub fn layout_params(mut self, params: crate::legend::types::CustomLayoutParams) -> Self {
        self.layout_params = params;
        self
    }

    /// Set custom entry spacing
    pub fn entry_spacing(mut self, spacing: u32) -> Self {
        self.layout_params.entry_spacing = spacing;
        self
    }

    /// Set custom symbol size
    pub fn symbol_size(mut self, size: u32) -> Self {
        self.layout_params.symbol_size = size;
        self
    }

    /// Enable or disable automatic layout
    pub fn auto_layout(mut self, enabled: bool) -> Self {
        self.layout_params.auto_layout = enabled;
        self
    }
}

impl<C: PixelColor> LegendBuilder<C> for CustomLegendBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    type Legend = CustomLegend<C>;
    type Error = ChartError;

    fn build(self) -> Result<Self::Legend, Self::Error> {
        let mut legend = CustomLegend::new(self.position);
        legend.set_orientation(self.orientation);
        legend.set_style(self.style);
        legend.set_layout_params(self.layout_params);

        // Add all entries
        for entry in self.entries {
            legend.add_entry(entry)?;
        }

        Ok(legend)
    }
}

impl<C: PixelColor> Default for CustomLegendBuilder<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::new()
    }
}

// Convenience functions for creating common legend configurations
/// Preset legend configurations for common use cases
pub mod presets {
    use super::*;
    use embedded_graphics::pixelcolor::Rgb565;

    /// Create a standard legend on the right side
    pub fn right_legend<C: PixelColor + From<Rgb565>>() -> StandardLegendBuilder<C> {
        StandardLegendBuilder::new()
            .position(LegendPosition::Right)
            .orientation(LegendOrientation::Vertical)
    }

    /// Create a standard legend at the bottom
    pub fn bottom_legend<C: PixelColor + From<Rgb565>>() -> StandardLegendBuilder<C> {
        StandardLegendBuilder::new()
            .position(LegendPosition::Bottom)
            .orientation(LegendOrientation::Horizontal)
    }

    /// Create a minimal legend for small displays
    pub fn minimal_legend<C: PixelColor + From<Rgb565>>() -> CompactLegendBuilder<C> {
        CompactLegendBuilder::new()
            .position(LegendPosition::TopRight)
            .orientation(LegendOrientation::Vertical)
    }

    /// Create a professional legend with styling
    pub fn professional_legend<C: PixelColor + From<Rgb565>>() -> StandardLegendBuilder<C> {
        StandardLegendBuilder::new()
            .position(LegendPosition::Right)
            .professional_style()
            .margins(LegendMargins::all(12))
    }

    /// Create a floating legend that overlays the chart
    pub fn floating_legend<C: PixelColor + From<Rgb565>>(
        position: Point,
    ) -> StandardLegendBuilder<C> {
        StandardLegendBuilder::new()
            .position(LegendPosition::Floating(position))
            .orientation(LegendOrientation::Vertical)
    }
}

// CompactLegendEntry implementation is in types.rs

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::pixelcolor::Rgb565;

    #[test]
    fn test_standard_legend_builder() {
        let legend = StandardLegendBuilder::new()
            .position(LegendPosition::Bottom)
            .orientation(LegendOrientation::Horizontal)
            .add_line_entry("Series 1", Rgb565::RED)
            .unwrap()
            .add_bar_entry("Series 2", Rgb565::BLUE)
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(legend.position(), LegendPosition::Bottom);
        assert_eq!(legend.orientation(), LegendOrientation::Horizontal);
        assert_eq!(legend.entries().len(), 2);
    }

    #[test]
    fn test_preset_legends() {
        let legend = presets::right_legend::<Rgb565>()
            .add_line_entry("Test", Rgb565::GREEN)
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(legend.position(), LegendPosition::Right);
        assert_eq!(legend.orientation(), LegendOrientation::Vertical);
    }
}
