//! Core traits for legend implementations.

use crate::error::ChartResult;
use embedded_graphics::{prelude::*, primitives::Rectangle};

#[cfg(feature = "std")]
use std::vec::Vec;

#[cfg(all(feature = "no_std", not(feature = "std")))]
extern crate alloc;

#[cfg(all(feature = "no_std", not(feature = "std")))]
use alloc::vec::Vec;

/// Main trait for legend implementations
pub trait Legend<C: PixelColor> {
    /// The type of legend entries this legend contains
    type Entry: LegendEntry<C>;

    /// Get all legend entries
    fn entries(&self) -> &[Self::Entry];

    /// Get mutable access to legend entries
    fn entries_mut(&mut self) -> &mut [Self::Entry];

    /// Add a new entry to the legend
    fn add_entry(&mut self, entry: Self::Entry) -> ChartResult<()>;

    /// Remove an entry by index
    fn remove_entry(&mut self, index: usize) -> ChartResult<()>;

    /// Clear all entries
    fn clear_entries(&mut self);

    /// Get the legend position
    fn position(&self) -> crate::legend::position::LegendPosition;

    /// Set the legend position
    fn set_position(&mut self, position: crate::legend::position::LegendPosition);

    /// Get the legend orientation
    fn orientation(&self) -> crate::legend::types::LegendOrientation;

    /// Set the legend orientation
    fn set_orientation(&mut self, orientation: crate::legend::types::LegendOrientation);

    /// Calculate the required size for this legend
    fn calculate_size(&self) -> Size;

    /// Check if the legend is empty
    fn is_empty(&self) -> bool {
        self.entries().is_empty()
    }

    /// Get the number of visible entries
    fn visible_entry_count(&self) -> usize {
        self.entries().iter().filter(|e| e.is_visible()).count()
    }
}

/// Trait for rendering legends to a display target
pub trait LegendRenderer<C: PixelColor> {
    /// The legend type this renderer can handle
    type Legend: Legend<C>;

    /// Render the legend to the target display
    ///
    /// # Arguments
    /// * `legend` - The legend to render
    /// * `viewport` - The area to render the legend in
    /// * `target` - The display target to render to
    fn render<D>(
        &self,
        legend: &Self::Legend,
        viewport: Rectangle,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>;

    /// Calculate the layout for legend entries within the viewport
    ///
    /// # Arguments
    /// * `legend` - The legend to calculate layout for
    /// * `viewport` - The available area for the legend
    fn calculate_layout(
        &self,
        legend: &Self::Legend,
        viewport: Rectangle,
    ) -> ChartResult<heapless::Vec<Rectangle, 8>>;

    /// Render a single legend entry
    ///
    /// # Arguments
    /// * `entry` - The legend entry to render
    /// * `bounds` - The area to render the entry in
    /// * `target` - The display target to render to
    fn render_entry<D>(
        &self,
        entry: &<Self::Legend as Legend<C>>::Entry,
        bounds: Rectangle,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>;
}

/// Trait for individual legend entries
pub trait LegendEntry<C: PixelColor> {
    /// Get the label text for this entry
    fn label(&self) -> &str;

    /// Set the label text for this entry
    fn set_label(&mut self, label: &str) -> ChartResult<()>;

    /// Get the entry type (determines the symbol)
    fn entry_type(&self) -> &crate::legend::types::LegendEntryType<C>;

    /// Set the entry type
    fn set_entry_type(&mut self, entry_type: crate::legend::types::LegendEntryType<C>);

    /// Check if this entry is visible
    fn is_visible(&self) -> bool;

    /// Set the visibility of this entry
    fn set_visible(&mut self, visible: bool);

    /// Calculate the required size for this entry
    fn calculate_size(&self, style: &crate::legend::style::LegendStyle<C>) -> Size;

    /// Render the symbol for this entry
    fn render_symbol<D>(
        &self,
        bounds: Rectangle,
        style: &crate::legend::style::SymbolStyle<C>,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>;
}

/// Trait for legends that can automatically generate entries from chart data
pub trait AutoLegend<C: PixelColor>: Legend<C> {
    /// The type of data series this legend can generate entries for
    type DataSeries;

    /// Generate legend entries from data series
    fn generate_from_series(&mut self, series: &[Self::DataSeries]) -> ChartResult<()>;

    /// Generate a single entry from a data series
    fn generate_entry_from_series(
        &self,
        series: &Self::DataSeries,
        index: usize,
    ) -> ChartResult<Self::Entry>;

    /// Update existing entries to match current data series
    fn update_from_series(&mut self, series: &[Self::DataSeries]) -> ChartResult<()>;
}

/// Trait for legends that support interactive features
pub trait InteractiveLegend<C: PixelColor>: Legend<C> {
    /// Event type for legend interactions
    type Event;
    /// Response type for legend interactions
    type Response;

    /// Handle an interaction event
    ///
    /// # Arguments
    /// * `event` - The interaction event
    /// * `viewport` - The legend viewport
    fn handle_event(
        &mut self,
        event: Self::Event,
        viewport: Rectangle,
    ) -> ChartResult<Self::Response>;

    /// Check if a point is within a legend entry
    ///
    /// # Arguments
    /// * `point` - The point to check
    /// * `viewport` - The legend viewport
    fn hit_test(&self, point: Point, viewport: Rectangle) -> Option<usize>;

    /// Toggle the visibility of an entry
    fn toggle_entry(&mut self, index: usize) -> ChartResult<()>;

    /// Get the currently selected entry index
    fn selected_entry(&self) -> Option<usize>;

    /// Set the selected entry
    fn set_selected_entry(&mut self, index: Option<usize>);
}

/// Default legend renderer implementation
#[derive(Debug, Clone)]
pub struct DefaultLegendRenderer<C: PixelColor> {
    _phantom: core::marker::PhantomData<C>,
}

impl<C: PixelColor> DefaultLegendRenderer<C> {
    /// Create a new default legend renderer
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<C: PixelColor> Default for DefaultLegendRenderer<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: PixelColor + From<embedded_graphics::pixelcolor::Rgb565>> LegendRenderer<C>
    for DefaultLegendRenderer<C>
{
    type Legend = crate::legend::DefaultLegend<C>;

    fn render<D>(
        &self,
        legend: &Self::Legend,
        viewport: Rectangle,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        if legend.entries.is_empty() {
            return Ok(());
        }

        let entry_bounds = self.calculate_layout(legend, viewport)?;

        // Render background if configured
        if let Some(bg_color) = legend.style.background.color {
            use embedded_graphics::primitives::PrimitiveStyle;
            use embedded_graphics::primitives::Rectangle as EgRectangle;

            EgRectangle::new(viewport.top_left, viewport.size)
                .into_styled(PrimitiveStyle::with_fill(bg_color))
                .draw(target)
                .map_err(|_| crate::error::ChartError::RenderingError)?;
        }

        // Render each visible entry
        for (entry, bounds) in legend
            .entries
            .iter()
            .filter(|e| e.visible)
            .zip(entry_bounds.iter())
        {
            self.render_entry(entry, *bounds, target)?;
        }

        Ok(())
    }

    fn calculate_layout(
        &self,
        legend: &Self::Legend,
        viewport: Rectangle,
    ) -> ChartResult<heapless::Vec<Rectangle, 8>> {
        let mut layouts = heapless::Vec::new();
        let visible_entries: Vec<_> = legend.entries.iter().filter(|e| e.visible).collect();

        if visible_entries.is_empty() {
            return Ok(layouts);
        }

        match legend.orientation {
            crate::legend::types::LegendOrientation::Vertical => {
                let entry_height = legend.style.text.line_height;
                let spacing = legend.style.spacing.entry_spacing;

                for (i, _) in visible_entries.iter().enumerate() {
                    let y_offset = i as u32 * (entry_height + spacing);
                    let bounds = Rectangle::new(
                        Point::new(viewport.top_left.x, viewport.top_left.y + y_offset as i32),
                        Size::new(viewport.size.width, entry_height),
                    );
                    if layouts.push(bounds).is_err() {
                        return Err(crate::error::ChartError::ConfigurationError);
                    }
                }
            }
            crate::legend::types::LegendOrientation::Horizontal => {
                let mut x_offset = 0u32;
                let entry_height = legend.style.text.line_height;

                for entry in visible_entries.iter() {
                    let entry_width = legend.style.spacing.symbol_width
                        + legend.style.spacing.symbol_text_gap
                        + entry.label.len() as u32 * legend.style.text.char_width;

                    let bounds = Rectangle::new(
                        Point::new(viewport.top_left.x + x_offset as i32, viewport.top_left.y),
                        Size::new(entry_width, entry_height),
                    );
                    if layouts.push(bounds).is_err() {
                        return Err(crate::error::ChartError::ConfigurationError);
                    }

                    x_offset += entry_width + legend.style.spacing.entry_spacing;
                }
            }
        }

        /// Standard legend renderer implementation
        #[derive(Debug, Clone)]
        pub struct StandardLegendRenderer<C: PixelColor> {
            _phantom: core::marker::PhantomData<C>,
        }

        impl<C: PixelColor> StandardLegendRenderer<C> {
            /// Create a new standard legend renderer
            pub fn new() -> Self {
                Self {
                    _phantom: core::marker::PhantomData,
                }
            }
        }

        impl<C: PixelColor> Default for StandardLegendRenderer<C> {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<C: PixelColor + From<embedded_graphics::pixelcolor::Rgb565>> LegendRenderer<C>
            for StandardLegendRenderer<C>
        {
            type Legend = crate::legend::types::StandardLegend<C>;

            fn render<D>(
                &self,
                legend: &Self::Legend,
                viewport: Rectangle,
                target: &mut D,
            ) -> ChartResult<()>
            where
                D: DrawTarget<Color = C>,
            {
                if legend.entries().is_empty() {
                    return Ok(());
                }

                let entry_bounds = self.calculate_layout(legend, viewport)?;

                // Render background if configured
                if let Some(bg_color) = legend.style().background.color {
                    use embedded_graphics::primitives::PrimitiveStyle;
                    use embedded_graphics::primitives::Rectangle as EgRectangle;

                    EgRectangle::new(viewport.top_left, viewport.size)
                        .into_styled(PrimitiveStyle::with_fill(bg_color))
                        .draw(target)
                        .map_err(|_| crate::error::ChartError::RenderingError)?;
                }

                // Render each visible entry
                for (entry, bounds) in legend
                    .entries()
                    .iter()
                    .filter(|e| e.is_visible())
                    .zip(entry_bounds.iter())
                {
                    self.render_entry(entry, *bounds, target)?;
                }

                Ok(())
            }

            fn calculate_layout(
                &self,
                legend: &Self::Legend,
                viewport: Rectangle,
            ) -> ChartResult<heapless::Vec<Rectangle, 8>> {
                let mut layouts = heapless::Vec::new();
                let visible_entries: Vec<_> =
                    legend.entries().iter().filter(|e| e.is_visible()).collect();

                if visible_entries.is_empty() {
                    return Ok(layouts);
                }

                match legend.orientation() {
                    crate::legend::types::LegendOrientation::Vertical => {
                        let entry_height = legend.style().text.line_height;
                        let spacing = legend.style().spacing.entry_spacing;

                        for (i, _) in visible_entries.iter().enumerate() {
                            let y_offset = i as u32 * (entry_height + spacing);
                            let bounds = Rectangle::new(
                                Point::new(
                                    viewport.top_left.x,
                                    viewport.top_left.y + y_offset as i32,
                                ),
                                Size::new(viewport.size.width, entry_height),
                            );
                            if layouts.push(bounds).is_err() {
                                return Err(crate::error::ChartError::ConfigurationError);
                            }
                        }
                    }
                    crate::legend::types::LegendOrientation::Horizontal => {
                        let mut x_offset = 0u32;
                        let entry_height = legend.style().text.line_height;

                        for entry in visible_entries.iter() {
                            let entry_width = legend.style().spacing.symbol_width
                                + legend.style().spacing.symbol_text_gap
                                + entry.label().len() as u32 * legend.style().text.char_width;

                            let bounds = Rectangle::new(
                                Point::new(
                                    viewport.top_left.x + x_offset as i32,
                                    viewport.top_left.y,
                                ),
                                Size::new(entry_width, entry_height),
                            );
                            if layouts.push(bounds).is_err() {
                                return Err(crate::error::ChartError::ConfigurationError);
                            }

                            x_offset += entry_width + legend.style().spacing.entry_spacing;
                        }
                    }
                }

                Ok(layouts)
            }

            fn render_entry<D>(
                &self,
                entry: &crate::legend::types::StandardLegendEntry<C>,
                bounds: Rectangle,
                target: &mut D,
            ) -> ChartResult<()>
            where
                D: DrawTarget<Color = C>,
            {
                // Render symbol
                let symbol_bounds = Rectangle::new(
                    bounds.top_left,
                    Size::new(bounds.size.width.min(20), bounds.size.height),
                );
                entry.render_symbol(
                    symbol_bounds,
                    &crate::legend::style::SymbolStyle::default(),
                    target,
                )?;

                // Render text (simplified - would need proper text rendering in full implementation)
                // For now, we'll skip text rendering as it requires font support

                Ok(())
            }
        }

        Ok(layouts)
    }

    fn render_entry<D>(
        &self,
        entry: &crate::legend::DefaultLegendEntry<C>,
        bounds: Rectangle,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        // Render symbol
        let symbol_bounds = Rectangle::new(
            bounds.top_left,
            Size::new(bounds.size.width.min(20), bounds.size.height),
        );
        entry.render_symbol(
            symbol_bounds,
            &crate::legend::style::SymbolStyle::default(),
            target,
        )?;

        // Render text label
        let text_x = bounds.top_left.x + 25; // Symbol width + gap
        let text_y = bounds.top_left.y + (bounds.size.height as i32 / 2);

        // Use embedded-graphics text rendering
        use embedded_graphics::{
            mono_font::{ascii::FONT_6X10, MonoTextStyle},
            text::{Baseline, Text},
        };

        let text_style = MonoTextStyle::new(
            &FONT_6X10,
            C::from(embedded_graphics::pixelcolor::Rgb565::BLACK),
        );

        Text::with_baseline(
            entry.label(),
            Point::new(text_x, text_y),
            text_style,
            Baseline::Middle,
        )
        .draw(target)
        .map_err(|_| crate::error::ChartError::RenderingError)?;

        Ok(())
    }
}

/// Standard legend renderer implementation
#[derive(Debug, Clone)]
pub struct StandardLegendRenderer<C: PixelColor> {
    _phantom: core::marker::PhantomData<C>,
}

impl<C: PixelColor> StandardLegendRenderer<C> {
    /// Create a new standard legend renderer
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<C: PixelColor> Default for StandardLegendRenderer<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: PixelColor + From<embedded_graphics::pixelcolor::Rgb565>> LegendRenderer<C>
    for StandardLegendRenderer<C>
{
    type Legend = crate::legend::types::StandardLegend<C>;

    fn render<D>(
        &self,
        legend: &Self::Legend,
        viewport: Rectangle,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        if legend.entries().is_empty() {
            return Ok(());
        }

        let entry_bounds = self.calculate_layout(legend, viewport)?;

        // Render background if configured
        if let Some(bg_color) = legend.style().background.color {
            use embedded_graphics::primitives::PrimitiveStyle;
            use embedded_graphics::primitives::Rectangle as EgRectangle;

            EgRectangle::new(viewport.top_left, viewport.size)
                .into_styled(PrimitiveStyle::with_fill(bg_color))
                .draw(target)
                .map_err(|_| crate::error::ChartError::RenderingError)?;
        }

        // Render each visible entry
        for (entry, bounds) in legend
            .entries()
            .iter()
            .filter(|e| e.is_visible())
            .zip(entry_bounds.iter())
        {
            self.render_entry(entry, *bounds, target)?;
        }

        Ok(())
    }

    fn calculate_layout(
        &self,
        legend: &Self::Legend,
        viewport: Rectangle,
    ) -> ChartResult<heapless::Vec<Rectangle, 8>> {
        let mut layouts = heapless::Vec::new();
        let visible_entries: Vec<_> = legend.entries().iter().filter(|e| e.is_visible()).collect();

        if visible_entries.is_empty() {
            return Ok(layouts);
        }

        match legend.orientation() {
            crate::legend::types::LegendOrientation::Vertical => {
                let entry_height = legend.style().text.line_height;
                let spacing = legend.style().spacing.entry_spacing;

                for (i, _) in visible_entries.iter().enumerate() {
                    let y_offset = i as u32 * (entry_height + spacing);
                    let bounds = Rectangle::new(
                        Point::new(viewport.top_left.x, viewport.top_left.y + y_offset as i32),
                        Size::new(viewport.size.width, entry_height),
                    );
                    if layouts.push(bounds).is_err() {
                        return Err(crate::error::ChartError::ConfigurationError);
                    }
                }
            }
            crate::legend::types::LegendOrientation::Horizontal => {
                let mut x_offset = 0u32;
                let entry_height = legend.style().text.line_height;

                for entry in visible_entries.iter() {
                    let entry_width = legend.style().spacing.symbol_width
                        + legend.style().spacing.symbol_text_gap
                        + entry.label().len() as u32 * legend.style().text.char_width;

                    let bounds = Rectangle::new(
                        Point::new(viewport.top_left.x + x_offset as i32, viewport.top_left.y),
                        Size::new(entry_width, entry_height),
                    );
                    if layouts.push(bounds).is_err() {
                        return Err(crate::error::ChartError::ConfigurationError);
                    }

                    x_offset += entry_width + legend.style().spacing.entry_spacing;
                }
            }
        }

        Ok(layouts)
    }

    fn render_entry<D>(
        &self,
        entry: &crate::legend::types::StandardLegendEntry<C>,
        bounds: Rectangle,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        // Render symbol
        let symbol_bounds = Rectangle::new(
            bounds.top_left,
            Size::new(bounds.size.width.min(20), bounds.size.height),
        );
        entry.render_symbol(
            symbol_bounds,
            &crate::legend::style::SymbolStyle::default(),
            target,
        )?;

        // Render text label
        let text_x = bounds.top_left.x + 25; // Symbol width + gap
        let text_y = bounds.top_left.y + (bounds.size.height as i32 / 2);

        // Use embedded-graphics text rendering
        use embedded_graphics::{
            mono_font::{ascii::FONT_6X10, MonoTextStyle},
            text::{Baseline, Text},
        };

        let text_style = MonoTextStyle::new(
            &FONT_6X10,
            C::from(embedded_graphics::pixelcolor::Rgb565::BLACK),
        );

        Text::with_baseline(
            entry.label(),
            Point::new(text_x, text_y),
            text_style,
            Baseline::Middle,
        )
        .draw(target)
        .map_err(|_| crate::error::ChartError::RenderingError)?;

        Ok(())
    }
}
