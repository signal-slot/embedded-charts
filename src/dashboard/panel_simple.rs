//! Simplified chart panels for dashboard composition

use super::GridPosition;
use crate::error::ChartResult;
use embedded_graphics::{
    pixelcolor::PixelColor,
    prelude::*,
    primitives::Rectangle,
};

/// Unique identifier for a panel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PanelId(pub u16);

/// A function that can draw a chart
pub type DrawFn<C> = Box<dyn Fn(Rectangle, &mut dyn DrawTarget<Color = C, Error = core::convert::Infallible>) -> ChartResult<()>>;

/// A panel containing a chart in the dashboard
pub struct ChartPanel<C: PixelColor> {
    /// Unique identifier
    id: PanelId,
    /// Position in the grid
    position: GridPosition,
    /// Draw function
    draw_fn: DrawFn<C>,
}

impl<C: PixelColor> ChartPanel<C> {
    /// Create a new chart panel with a draw function
    pub fn new(
        id: PanelId,
        position: GridPosition,
        draw_fn: DrawFn<C>,
    ) -> Self {
        Self {
            id,
            position,
            draw_fn,
        }
    }

    /// Get the panel ID
    pub fn id(&self) -> PanelId {
        self.id
    }

    /// Get the grid position
    pub fn position(&self) -> GridPosition {
        self.position
    }

    /// Draw the panel
    pub fn draw<DT>(&self, viewport: Rectangle, target: &mut DT) -> ChartResult<()>
    where
        DT: DrawTarget<Color = C>,
    {
        // Convert to the expected error type
        let mut wrapper = DrawTargetWrapper { inner: target };
        (self.draw_fn)(viewport, &mut wrapper)
    }
}

/// Wrapper to convert DrawTarget errors
struct DrawTargetWrapper<'a, DT> {
    inner: &'a mut DT,
}

impl<'a, C, DT> DrawTarget for DrawTargetWrapper<'a, DT>
where
    C: PixelColor,
    DT: DrawTarget<Color = C>,
{
    type Color = C;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        // Ignore any errors from the inner target
        let _ = self.inner.draw_iter(pixels);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::pixelcolor::Rgb565;

    #[test]
    fn test_panel_creation() {
        let draw_fn: DrawFn<Rgb565> = Box::new(|_viewport, _target| Ok(()));
        
        let panel = ChartPanel::new(
            PanelId(0),
            GridPosition::new(0, 0),
            draw_fn,
        );
        
        assert_eq!(panel.id(), PanelId(0));
        assert_eq!(panel.position(), GridPosition::new(0, 0));
    }
}