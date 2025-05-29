//! Legend system for charts.
//!
//! This module provides a comprehensive legend system for embedded graphics charts,
//! supporting multiple legend types, flexible positioning, and customizable styling.

pub mod builder;
pub mod position;
pub mod style;
pub mod traits;
pub mod types;

// Re-export main types
pub use builder::{
    CompactLegendBuilder, CustomLegendBuilder, LegendBuilder, StandardLegendBuilder,
};
pub use position::{LegendAlignment, LegendMargins, LegendPosition, PositionCalculator};
pub use style::{BackgroundStyle, LegendStyle, SpacingStyle, SymbolStyle, TextStyle};
pub use traits::{
    DefaultLegendRenderer, Legend, LegendEntry, LegendRenderer, StandardLegendRenderer,
};
pub use types::{CompactLegend, CustomLegend, LegendEntryType, LegendOrientation, StandardLegend};

use crate::error::ChartResult;
use embedded_graphics::{prelude::*, primitives::Rectangle};

/// Default legend configuration
#[derive(Debug, Clone)]
pub struct DefaultLegend<C: PixelColor> {
    /// Legend entries
    pub entries: heapless::Vec<DefaultLegendEntry<C>, 8>,
    /// Legend position
    pub position: LegendPosition,
    /// Legend orientation
    pub orientation: LegendOrientation,
    /// Legend style
    pub style: LegendStyle<C>,
}

/// Default legend entry implementation
#[derive(Debug, Clone)]
pub struct DefaultLegendEntry<C: PixelColor> {
    /// Label text
    pub label: heapless::String<32>,
    /// Entry type (determines symbol)
    pub entry_type: LegendEntryType<C>,
    /// Whether this entry is visible
    pub visible: bool,
}

impl<C: PixelColor + From<embedded_graphics::pixelcolor::Rgb565>> DefaultLegend<C> {
    /// Create a new default legend
    pub fn new(position: LegendPosition) -> Self {
        Self {
            entries: heapless::Vec::new(),
            position,
            orientation: LegendOrientation::Vertical,
            style: LegendStyle::default(),
        }
    }

    /// Add an entry to the legend
    pub fn add_entry(&mut self, label: &str, entry_type: LegendEntryType<C>) -> ChartResult<()> {
        let label_string = heapless::String::try_from(label)
            .map_err(|_| crate::error::ChartError::ConfigurationError)?;

        let entry = DefaultLegendEntry {
            label: label_string,
            entry_type,
            visible: true,
        };

        self.entries
            .push(entry)
            .map_err(|_| crate::error::ChartError::ConfigurationError)?;

        Ok(())
    }

    /// Set the legend orientation
    pub fn set_orientation(&mut self, orientation: LegendOrientation) {
        self.orientation = orientation;
    }

    /// Set the legend style
    pub fn set_style(&mut self, style: LegendStyle<C>) {
        self.style = style;
    }

    /// Calculate the required size for this legend
    pub fn calculate_size(&self) -> Size {
        if self.entries.is_empty() {
            return Size::zero();
        }

        let entry_count = self.entries.iter().filter(|e| e.visible).count();
        if entry_count == 0 {
            return Size::zero();
        }

        match self.orientation {
            LegendOrientation::Vertical => {
                let width = self.style.spacing.symbol_width
                    + self.style.spacing.symbol_text_gap
                    + self.style.text.max_text_width;
                let height = entry_count as u32 * self.style.text.line_height
                    + (entry_count.saturating_sub(1)) as u32 * self.style.spacing.entry_spacing;
                Size::new(width, height)
            }
            LegendOrientation::Horizontal => {
                let height = self.style.text.line_height;
                let total_width: u32 = self
                    .entries
                    .iter()
                    .filter(|e| e.visible)
                    .map(|e| {
                        self.style.spacing.symbol_width
                            + self.style.spacing.symbol_text_gap
                            + e.label.len() as u32 * self.style.text.char_width
                    })
                    .sum();
                let spacing_width =
                    (entry_count.saturating_sub(1)) as u32 * self.style.spacing.entry_spacing;
                Size::new(total_width + spacing_width, height)
            }
        }
    }
}

impl<C: PixelColor> LegendEntry<C> for DefaultLegendEntry<C> {
    fn label(&self) -> &str {
        &self.label
    }

    fn set_label(&mut self, label: &str) -> ChartResult<()> {
        self.label = heapless::String::try_from(label)
            .map_err(|_| crate::error::ChartError::ConfigurationError)?;
        Ok(())
    }

    fn entry_type(&self) -> &LegendEntryType<C> {
        &self.entry_type
    }

    fn set_entry_type(&mut self, entry_type: LegendEntryType<C>) {
        self.entry_type = entry_type;
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    fn calculate_size(&self, style: &LegendStyle<C>) -> Size {
        let text_width = self.label.len() as u32 * style.text.char_width;
        let total_width = style.spacing.symbol_width + style.spacing.symbol_text_gap + text_width;
        Size::new(total_width, style.text.line_height)
    }

    fn render_symbol<D>(
        &self,
        bounds: Rectangle,
        _style: &SymbolStyle<C>,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        use embedded_graphics::primitives::{
            Circle, Line, PrimitiveStyle, Rectangle as EgRectangle,
        };

        match &self.entry_type {
            LegendEntryType::Line { color, .. } => {
                let line_y = bounds.top_left.y + bounds.size.height as i32 / 2;
                let line_start = Point::new(bounds.top_left.x + 2, line_y);
                let line_end = Point::new(bounds.top_left.x + bounds.size.width as i32 - 2, line_y);

                Line::new(line_start, line_end)
                    .into_styled(PrimitiveStyle::with_stroke(*color, 1))
                    .draw(target)
                    .map_err(|_| crate::error::ChartError::RenderingError)?;
            }
            LegendEntryType::Bar { color, .. } | LegendEntryType::Pie { color, .. } => {
                let rect_size = Size::new(bounds.size.width.min(16), bounds.size.height.min(12));
                let rect_pos = Point::new(
                    bounds.top_left.x + (bounds.size.width as i32 - rect_size.width as i32) / 2,
                    bounds.top_left.y + (bounds.size.height as i32 - rect_size.height as i32) / 2,
                );

                EgRectangle::new(rect_pos, rect_size)
                    .into_styled(PrimitiveStyle::with_fill(*color))
                    .draw(target)
                    .map_err(|_| crate::error::ChartError::RenderingError)?;
            }
            LegendEntryType::Custom {
                color,
                shape: _,
                size,
            } => {
                let symbol_size = (*size).min(bounds.size.width).min(bounds.size.height);
                let center = Point::new(
                    bounds.top_left.x + bounds.size.width as i32 / 2,
                    bounds.top_left.y + bounds.size.height as i32 / 2,
                );

                Circle::with_center(center, symbol_size)
                    .into_styled(PrimitiveStyle::with_fill(*color))
                    .draw(target)
                    .map_err(|_| crate::error::ChartError::RenderingError)?;
            }
        }

        Ok(())
    }
}

impl<C: PixelColor + From<embedded_graphics::pixelcolor::Rgb565>> Legend<C> for DefaultLegend<C> {
    type Entry = DefaultLegendEntry<C>;

    fn entries(&self) -> &[Self::Entry] {
        &self.entries
    }

    fn entries_mut(&mut self) -> &mut [Self::Entry] {
        &mut self.entries
    }

    fn add_entry(&mut self, entry: Self::Entry) -> ChartResult<()> {
        self.entries
            .push(entry)
            .map_err(|_| crate::error::ChartError::ConfigurationError)
    }

    fn remove_entry(&mut self, index: usize) -> ChartResult<()> {
        if index < self.entries.len() {
            self.entries.remove(index);
            Ok(())
        } else {
            Err(crate::error::ChartError::ConfigurationError)
        }
    }

    fn clear_entries(&mut self) {
        self.entries.clear();
    }

    fn position(&self) -> LegendPosition {
        self.position
    }

    fn set_position(&mut self, position: LegendPosition) {
        self.position = position;
    }

    fn orientation(&self) -> LegendOrientation {
        self.orientation
    }

    fn set_orientation(&mut self, orientation: LegendOrientation) {
        self.orientation = orientation;
    }

    fn calculate_size(&self) -> Size {
        DefaultLegend::calculate_size(self)
    }
}
