//! Legend type implementations.

use crate::error::{ChartError, ChartResult};
use crate::legend::position::LegendPosition;
use crate::legend::style::{LegendStyle, SymbolStyle};
use crate::legend::traits::{Legend, LegendEntry};
use embedded_graphics::{prelude::*, primitives::Rectangle};

#[cfg(feature = "std")]
use std::vec::Vec;

#[cfg(all(feature = "no_std", not(feature = "std")))]
extern crate alloc;

#[cfg(all(feature = "no_std", not(feature = "std")))]
use alloc::vec::Vec;

/// Legend orientation options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegendOrientation {
    /// Vertical layout (entries stacked vertically)
    Vertical,
    /// Horizontal layout (entries arranged horizontally)
    Horizontal,
}

/// Types of legend entries
#[derive(Debug, Clone)]
pub enum LegendEntryType<C: PixelColor> {
    /// Line entry for line charts
    Line {
        /// Line color
        color: C,
        /// Line width
        width: u32,
        /// Line pattern (solid, dashed, etc.)
        pattern: crate::style::LinePattern,
        /// Optional marker
        marker: Option<MarkerStyle<C>>,
    },
    /// Bar entry for bar charts
    Bar {
        /// Fill color
        color: C,
        /// Optional border color
        border_color: Option<C>,
        /// Border width
        border_width: u32,
    },
    /// Pie entry for pie charts
    Pie {
        /// Fill color
        color: C,
        /// Optional border color
        border_color: Option<C>,
        /// Border width
        border_width: u32,
    },
    /// Custom symbol entry
    Custom {
        /// Symbol color
        color: C,
        /// Symbol shape
        shape: SymbolShape,
        /// Symbol size
        size: u32,
    },
}

/// Marker styles for line entries
#[derive(Debug, Clone)]
pub struct MarkerStyle<C: PixelColor> {
    /// Marker shape
    pub shape: MarkerShape,
    /// Marker color
    pub color: C,
    /// Marker size
    pub size: u32,
    /// Whether to fill the marker
    pub filled: bool,
}

/// Available marker shapes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkerShape {
    /// Circle marker
    Circle,
    /// Square marker
    Square,
    /// Triangle marker
    Triangle,
    /// Diamond marker
    Diamond,
    /// Cross marker
    Cross,
    /// Plus marker
    Plus,
}

/// Available symbol shapes for custom entries
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolShape {
    /// Circle symbol
    Circle,
    /// Square symbol
    Square,
    /// Triangle symbol
    Triangle,
    /// Diamond symbol
    Diamond,
    /// Star symbol
    Star,
    /// Cross symbol
    Cross,
}

/// Standard legend implementation
#[derive(Debug, Clone)]
pub struct StandardLegend<C: PixelColor> {
    /// Legend entries
    entries: heapless::Vec<StandardLegendEntry<C>, 16>,
    /// Legend position
    position: LegendPosition,
    /// Legend orientation
    orientation: LegendOrientation,
    /// Legend style
    style: LegendStyle<C>,
}

/// Standard legend entry
#[derive(Debug, Clone)]
pub struct StandardLegendEntry<C: PixelColor> {
    /// Entry label
    label: heapless::String<64>,
    /// Entry type
    entry_type: LegendEntryType<C>,
    /// Visibility flag
    visible: bool,
}

/// Compact legend for space-constrained environments
#[derive(Debug, Clone)]
pub struct CompactLegend<C: PixelColor> {
    /// Legend entries (limited capacity)
    entries: heapless::Vec<CompactLegendEntry<C>, 8>,
    /// Legend position
    position: LegendPosition,
    /// Legend orientation
    orientation: LegendOrientation,
    /// Legend style
    style: LegendStyle<C>,
}

impl<C: PixelColor> CompactLegend<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new compact legend
    pub fn new(position: LegendPosition) -> Self {
        Self {
            entries: heapless::Vec::new(),
            position,
            orientation: LegendOrientation::Vertical,
            style: LegendStyle::compact(),
        }
    }

    /// Set the legend orientation
    pub fn set_orientation(&mut self, orientation: LegendOrientation) {
        self.orientation = orientation;
    }

    /// Set the legend style
    pub fn set_style(&mut self, style: LegendStyle<C>) {
        self.style = style;
    }

    /// Add an entry to the legend
    pub fn add_entry(&mut self, entry: CompactLegendEntry<C>) -> ChartResult<()> {
        self.entries
            .push(entry)
            .map_err(|_| ChartError::ConfigurationError)?;
        Ok(())
    }
}

impl<C: PixelColor> crate::legend::traits::Legend<C> for CompactLegend<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    type Entry = CompactLegendEntry<C>;

    fn position(&self) -> LegendPosition {
        self.position
    }

    fn orientation(&self) -> LegendOrientation {
        self.orientation
    }

    fn entries(&self) -> &[Self::Entry] {
        &self.entries
    }

    fn entries_mut(&mut self) -> &mut [Self::Entry] {
        &mut self.entries
    }

    fn add_entry(&mut self, entry: Self::Entry) -> ChartResult<()> {
        self.entries
            .push(entry)
            .map_err(|_| ChartError::ConfigurationError)?;
        Ok(())
    }

    fn remove_entry(&mut self, index: usize) -> ChartResult<()> {
        if index < self.entries.len() {
            self.entries.swap_remove(index);
            Ok(())
        } else {
            Err(ChartError::InvalidConfiguration)
        }
    }

    fn clear_entries(&mut self) {
        self.entries.clear();
    }

    fn set_position(&mut self, position: LegendPosition) {
        self.position = position;
    }

    fn set_orientation(&mut self, orientation: LegendOrientation) {
        self.orientation = orientation;
    }

    fn calculate_size(&self) -> embedded_graphics::prelude::Size {
        // Simple size calculation for compact legend
        let entry_count = self.entries.len() as u32;
        match self.orientation {
            LegendOrientation::Vertical => {
                embedded_graphics::prelude::Size::new(80, entry_count * 16 + 8)
            }
            LegendOrientation::Horizontal => {
                embedded_graphics::prelude::Size::new(entry_count * 60 + 8, 20)
            }
        }
    }
}

/// Compact legend entry with shorter labels
#[derive(Debug, Clone)]
pub struct CompactLegendEntry<C: PixelColor> {
    /// Entry label (shorter for compact display)
    pub label: heapless::String<16>,
    /// Entry type
    pub entry_type: LegendEntryType<C>,
    /// Visibility flag
    pub visible: bool,
}

impl<C: PixelColor> CompactLegendEntry<C> {
    /// Create a new compact legend entry
    pub fn new(label: &str, entry_type: LegendEntryType<C>) -> ChartResult<Self> {
        let mut label_string = heapless::String::new();
        label_string
            .push_str(label)
            .map_err(|_| ChartError::ConfigurationError)?;
        Ok(Self {
            label: label_string,
            entry_type,
            visible: true,
        })
    }
}

/// Custom legend for specialized layouts
#[derive(Debug, Clone)]
pub struct CustomLegend<C: PixelColor> {
    /// Legend entries
    entries: heapless::Vec<CustomLegendEntry<C>, 12>,
    /// Legend position
    position: LegendPosition,
    /// Legend orientation
    orientation: LegendOrientation,
    /// Legend style
    style: LegendStyle<C>,
    /// Custom layout parameters
    layout_params: CustomLayoutParams,
}

impl<C: PixelColor> CustomLegend<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a new custom legend
    pub fn new(position: LegendPosition) -> Self {
        Self {
            entries: heapless::Vec::new(),
            position,
            orientation: LegendOrientation::Vertical,
            style: LegendStyle::new(),
            layout_params: CustomLayoutParams::default(),
        }
    }

    /// Set the legend orientation
    pub fn set_orientation(&mut self, orientation: LegendOrientation) {
        self.orientation = orientation;
    }

    /// Set the legend style
    pub fn set_style(&mut self, style: LegendStyle<C>) {
        self.style = style;
    }

    /// Set custom layout parameters
    pub fn set_layout_params(&mut self, params: CustomLayoutParams) {
        self.layout_params = params;
    }

    /// Add an entry to the legend
    pub fn add_entry(&mut self, entry: CustomLegendEntry<C>) -> ChartResult<()> {
        self.entries
            .push(entry)
            .map_err(|_| ChartError::ConfigurationError)?;
        Ok(())
    }
}

impl<C: PixelColor> crate::legend::traits::Legend<C> for CustomLegend<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    type Entry = CustomLegendEntry<C>;

    fn position(&self) -> LegendPosition {
        self.position
    }

    fn orientation(&self) -> LegendOrientation {
        self.orientation
    }

    fn entries(&self) -> &[Self::Entry] {
        &self.entries
    }

    fn entries_mut(&mut self) -> &mut [Self::Entry] {
        &mut self.entries
    }

    fn add_entry(&mut self, entry: Self::Entry) -> ChartResult<()> {
        self.entries
            .push(entry)
            .map_err(|_| ChartError::ConfigurationError)?;
        Ok(())
    }

    fn remove_entry(&mut self, index: usize) -> ChartResult<()> {
        if index < self.entries.len() {
            self.entries.swap_remove(index);
            Ok(())
        } else {
            Err(ChartError::InvalidConfiguration)
        }
    }

    fn clear_entries(&mut self) {
        self.entries.clear();
    }

    fn set_position(&mut self, position: LegendPosition) {
        self.position = position;
    }

    fn set_orientation(&mut self, orientation: LegendOrientation) {
        self.orientation = orientation;
    }

    fn calculate_size(&self) -> embedded_graphics::prelude::Size {
        // Size calculation based on layout parameters
        let entry_count = self.entries.len() as u32;
        let symbol_size = self.layout_params.symbol_size;
        let entry_spacing = self.layout_params.entry_spacing;

        match self.orientation {
            LegendOrientation::Vertical => {
                let width = symbol_size + 100; // Symbol + text space
                let height = entry_count * (symbol_size + entry_spacing) + 16;
                embedded_graphics::prelude::Size::new(width, height)
            }
            LegendOrientation::Horizontal => {
                let width = entry_count * (symbol_size + 80 + entry_spacing) + 16;
                let height = symbol_size + 16;
                embedded_graphics::prelude::Size::new(width, height)
            }
        }
    }
}

/// Custom legend entry with additional metadata
#[derive(Debug, Clone)]
pub struct CustomLegendEntry<C: PixelColor> {
    /// Entry label
    label: heapless::String<32>,
    /// Entry type
    entry_type: LegendEntryType<C>,
    /// Visibility flag
    visible: bool,
    /// Custom positioning offset
    #[allow(dead_code)]
    offset: Point,
    /// Custom size override
    size_override: Option<Size>,
}

impl<C: PixelColor> CustomLegendEntry<C> {
    /// Create a new custom legend entry
    pub fn new(label: &str, entry_type: LegendEntryType<C>) -> ChartResult<Self> {
        let mut label_string = heapless::String::new();
        label_string
            .push_str(label)
            .map_err(|_| ChartError::ConfigurationError)?;
        Ok(Self {
            label: label_string,
            entry_type,
            visible: true,
            offset: embedded_graphics::prelude::Point::zero(),
            size_override: None,
        })
    }
}

/// Custom layout parameters
#[derive(Debug, Clone)]
pub struct CustomLayoutParams {
    /// Custom entry spacing
    pub entry_spacing: u32,
    /// Custom symbol size
    pub symbol_size: u32,
    /// Custom text offset
    pub text_offset: Point,
    /// Whether to use automatic layout
    pub auto_layout: bool,
}

// Implementation for StandardLegend
impl<C: PixelColor + From<embedded_graphics::pixelcolor::Rgb565>> StandardLegend<C> {
    /// Create a new standard legend
    pub fn new(position: LegendPosition) -> Self {
        Self {
            entries: heapless::Vec::new(),
            position,
            orientation: LegendOrientation::Vertical,
            style: LegendStyle::default(),
        }
    }

    /// Set the legend style
    pub fn set_style(&mut self, style: LegendStyle<C>) {
        self.style = style;
    }

    /// Get the legend style
    pub fn style(&self) -> &LegendStyle<C> {
        &self.style
    }
}

impl<C: PixelColor> Legend<C> for StandardLegend<C> {
    type Entry = StandardLegendEntry<C>;

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
        if self.entries.is_empty() {
            return Size::zero();
        }

        let visible_entries: Vec<_> = self.entries.iter().filter(|e| e.visible).collect();
        if visible_entries.is_empty() {
            return Size::zero();
        }

        match self.orientation {
            LegendOrientation::Vertical => {
                let max_width = visible_entries
                    .iter()
                    .map(|e| e.calculate_size(&self.style).width)
                    .max()
                    .unwrap_or(0);
                let total_height = visible_entries
                    .iter()
                    .map(|e| e.calculate_size(&self.style).height)
                    .sum::<u32>()
                    + (visible_entries.len().saturating_sub(1) as u32
                        * self.style.spacing.entry_spacing);
                Size::new(max_width, total_height)
            }
            LegendOrientation::Horizontal => {
                let total_width = visible_entries
                    .iter()
                    .map(|e| e.calculate_size(&self.style).width)
                    .sum::<u32>()
                    + (visible_entries.len().saturating_sub(1) as u32
                        * self.style.spacing.entry_spacing);
                let max_height = visible_entries
                    .iter()
                    .map(|e| e.calculate_size(&self.style).height)
                    .max()
                    .unwrap_or(0);
                Size::new(total_width, max_height)
            }
        }
    }
}

// Implementation for StandardLegendEntry
impl<C: PixelColor> StandardLegendEntry<C> {
    /// Create a new standard legend entry
    pub fn new(label: &str, entry_type: LegendEntryType<C>) -> ChartResult<Self> {
        let label_string = heapless::String::try_from(label)
            .map_err(|_| crate::error::ChartError::ConfigurationError)?;

        Ok(Self {
            label: label_string,
            entry_type,
            visible: true,
        })
    }
}

impl<C: PixelColor> LegendEntry<C> for StandardLegendEntry<C> {
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
        use embedded_graphics::primitives::{Circle, PrimitiveStyle, Rectangle as EgRectangle};

        match &self.entry_type {
            LegendEntryType::Line { color, .. } => {
                // Draw a small line segment
                let line_y = bounds.top_left.y + bounds.size.height as i32 / 2;
                let line_start = Point::new(bounds.top_left.x + 2, line_y);
                let line_end = Point::new(bounds.top_left.x + bounds.size.width as i32 - 2, line_y);

                use embedded_graphics::primitives::Line;
                Line::new(line_start, line_end)
                    .into_styled(PrimitiveStyle::with_stroke(*color, 1))
                    .draw(target)
                    .map_err(|_| crate::error::ChartError::RenderingError)?;
            }
            LegendEntryType::Bar { color, .. } | LegendEntryType::Pie { color, .. } => {
                // Draw a small rectangle
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
            LegendEntryType::Custom { color, shape, size } => {
                let symbol_size = (*size).min(bounds.size.width).min(bounds.size.height);
                let center = Point::new(
                    bounds.top_left.x + bounds.size.width as i32 / 2,
                    bounds.top_left.y + bounds.size.height as i32 / 2,
                );

                match shape {
                    SymbolShape::Circle => {
                        Circle::with_center(center, symbol_size)
                            .into_styled(PrimitiveStyle::with_fill(*color))
                            .draw(target)
                            .map_err(|_| crate::error::ChartError::RenderingError)?;
                    }
                    SymbolShape::Square => {
                        let rect_size = Size::new(symbol_size, symbol_size);
                        let rect_pos = Point::new(
                            center.x - symbol_size as i32 / 2,
                            center.y - symbol_size as i32 / 2,
                        );
                        EgRectangle::new(rect_pos, rect_size)
                            .into_styled(PrimitiveStyle::with_fill(*color))
                            .draw(target)
                            .map_err(|_| crate::error::ChartError::RenderingError)?;
                    }
                    _ => {
                        // For other shapes, draw a circle as fallback
                        Circle::with_center(center, symbol_size)
                            .into_styled(PrimitiveStyle::with_fill(*color))
                            .draw(target)
                            .map_err(|_| crate::error::ChartError::RenderingError)?;
                    }
                }
            }
        }

        Ok(())
    }
}

impl<C: PixelColor> LegendEntry<C> for CompactLegendEntry<C> {
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
        use embedded_graphics::primitives::{Circle, PrimitiveStyle, Rectangle as EgRectangle};

        match &self.entry_type {
            LegendEntryType::Line { color, .. } => {
                // Draw a small line segment
                let line_y = bounds.top_left.y + bounds.size.height as i32 / 2;
                let line_start = Point::new(bounds.top_left.x + 2, line_y);
                let line_end = Point::new(bounds.top_left.x + bounds.size.width as i32 - 2, line_y);

                use embedded_graphics::primitives::Line;
                Line::new(line_start, line_end)
                    .into_styled(PrimitiveStyle::with_stroke(*color, 1))
                    .draw(target)
                    .map_err(|_| crate::error::ChartError::RenderingError)?;
            }
            LegendEntryType::Bar { color, .. } | LegendEntryType::Pie { color, .. } => {
                // Draw a small rectangle
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
            LegendEntryType::Custom { color, shape, size } => {
                let symbol_size = (*size).min(bounds.size.width).min(bounds.size.height);
                let center = Point::new(
                    bounds.top_left.x + bounds.size.width as i32 / 2,
                    bounds.top_left.y + bounds.size.height as i32 / 2,
                );

                match shape {
                    SymbolShape::Circle => {
                        Circle::with_center(center, symbol_size)
                            .into_styled(PrimitiveStyle::with_fill(*color))
                            .draw(target)
                            .map_err(|_| crate::error::ChartError::RenderingError)?;
                    }
                    SymbolShape::Square => {
                        let rect_size = Size::new(symbol_size, symbol_size);
                        let rect_pos = Point::new(
                            center.x - symbol_size as i32 / 2,
                            center.y - symbol_size as i32 / 2,
                        );
                        EgRectangle::new(rect_pos, rect_size)
                            .into_styled(PrimitiveStyle::with_fill(*color))
                            .draw(target)
                            .map_err(|_| crate::error::ChartError::RenderingError)?;
                    }
                    _ => {
                        // For other shapes, draw a circle as fallback
                        Circle::with_center(center, symbol_size)
                            .into_styled(PrimitiveStyle::with_fill(*color))
                            .draw(target)
                            .map_err(|_| crate::error::ChartError::RenderingError)?;
                    }
                }
            }
        }

        Ok(())
    }
}

// Similar implementations for CompactLegend and CustomLegend would follow the same pattern
// but with their respective constraints and features

impl Default for LegendOrientation {
    fn default() -> Self {
        Self::Vertical
    }
}

impl Default for CustomLayoutParams {
    fn default() -> Self {
        Self {
            entry_spacing: 8,
            symbol_size: 16,
            text_offset: Point::new(20, 0),
            auto_layout: true,
        }
    }
}

// Implementation of LegendEntry trait for CustomLegendEntry
impl<C: PixelColor> LegendEntry<C> for CustomLegendEntry<C> {
    fn label(&self) -> &str {
        &self.label
    }

    fn set_label(&mut self, label: &str) -> ChartResult<()> {
        self.label =
            heapless::String::try_from(label).map_err(|_| ChartError::ConfigurationError)?;
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

        // Use size override if available
        if let Some(size_override) = self.size_override {
            size_override
        } else {
            Size::new(total_width, style.text.line_height)
        }
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
                    .map_err(|_| ChartError::RenderingError)?;
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
                    .map_err(|_| ChartError::RenderingError)?;
            }
            LegendEntryType::Custom { color, shape, size } => {
                let symbol_size = (*size).min(bounds.size.width).min(bounds.size.height);
                let center = Point::new(
                    bounds.top_left.x + bounds.size.width as i32 / 2,
                    bounds.top_left.y + bounds.size.height as i32 / 2,
                );

                match shape {
                    SymbolShape::Circle => {
                        Circle::with_center(center, symbol_size)
                            .into_styled(PrimitiveStyle::with_fill(*color))
                            .draw(target)
                            .map_err(|_| ChartError::RenderingError)?;
                    }
                    SymbolShape::Square => {
                        let half_size = symbol_size / 2;
                        let rect_pos =
                            Point::new(center.x - half_size as i32, center.y - half_size as i32);
                        EgRectangle::new(rect_pos, Size::new(symbol_size, symbol_size))
                            .into_styled(PrimitiveStyle::with_fill(*color))
                            .draw(target)
                            .map_err(|_| ChartError::RenderingError)?;
                    }
                    _ => {
                        // For other shapes, default to circle
                        Circle::with_center(center, symbol_size)
                            .into_styled(PrimitiveStyle::with_fill(*color))
                            .draw(target)
                            .map_err(|_| ChartError::RenderingError)?;
                    }
                }
            }
        }

        Ok(())
    }
}
