//! Common Display Abstraction for Examples
//!
//! This module provides a unified interface that works with both MockDisplay and SimulatorDisplay,
//! with automatic feature detection and standard viewport sizing conventions.

use embedded_graphics::prelude::*;

#[cfg(feature = "std")]
use embedded_graphics_simulator::SimulatorDisplay;

use embedded_graphics::mock_display::MockDisplay;

/// Unified display trait that works with both MockDisplay and SimulatorDisplay
pub trait UnifiedDisplay<C: PixelColor> {
    /// Clear the display with the given color
    fn clear(&mut self, color: C) -> Result<(), embedded_charts::error::ChartError>;

    /// Get the display size
    fn size(&self) -> Size;

    /// Get the bounding box of the display
    fn bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
        embedded_graphics::primitives::Rectangle::new(Point::zero(), self.size())
    }
}

impl<C: PixelColor> UnifiedDisplay<C> for MockDisplay<C> {
    fn clear(&mut self, color: C) -> Result<(), embedded_charts::error::ChartError> {
        <MockDisplay<C> as embedded_graphics::prelude::DrawTarget>::clear(self, color)
            .map_err(|_| embedded_charts::error::ChartError::RenderingError)
    }

    fn size(&self) -> Size {
        <MockDisplay<C> as embedded_graphics::prelude::OriginDimensions>::size(self)
    }
}

#[cfg(feature = "std")]
impl<C: PixelColor> UnifiedDisplay<C> for SimulatorDisplay<C> {
    fn clear(&mut self, color: C) -> Result<(), embedded_charts::error::ChartError> {
        <SimulatorDisplay<C> as embedded_graphics::prelude::DrawTarget>::clear(self, color)
            .map_err(|_| embedded_charts::error::ChartError::RenderingError)
    }

    fn size(&self) -> Size {
        <SimulatorDisplay<C> as embedded_graphics::prelude::OriginDimensions>::size(self)
    }
}
