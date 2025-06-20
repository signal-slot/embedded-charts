//! Chart panels for dashboard composition

use super::GridPosition;
use crate::{
    chart::{Chart, ChartConfig},
    error::ChartResult,
};
use core::any::Any;
use embedded_graphics::{
    pixelcolor::PixelColor,
    prelude::*,
    primitives::Rectangle,
};

/// Unique identifier for a panel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PanelId(pub u16);

/// Type-erased chart storage
trait AnyChart<C: PixelColor>: Any {
    fn draw_erased(&self, viewport: Rectangle, target: &mut dyn DrawTarget<Color = C>) -> ChartResult<()>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Concrete storage for a chart with its data
struct ChartStorage<CH, D, C>
where
    CH: Chart<C>,
    D: Clone + 'static,
    C: PixelColor,
{
    chart: CH,
    data: D,
    config: ChartConfig,
}

impl<CH, D, C> AnyChart<C> for ChartStorage<CH, D, C>
where
    CH: Chart<C, Data = D> + 'static,
    D: Clone + 'static,
    C: PixelColor,
{
    fn draw_erased(&self, viewport: Rectangle, target: &mut dyn DrawTarget<Color = C>) -> ChartResult<()> {
        self.chart.draw(&self.data, &self.config, viewport, target)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// A panel containing a chart in the dashboard
pub struct ChartPanel<C: PixelColor> {
    /// Unique identifier
    id: PanelId,
    /// Position in the grid
    position: GridPosition,
    /// Type-erased chart storage
    chart: Box<dyn AnyChart<C>>,
}

impl<C: PixelColor> ChartPanel<C> {
    /// Create a new chart panel
    pub fn new<CH, D>(
        id: PanelId,
        position: GridPosition,
        chart: CH,
        data: D,
        config: ChartConfig,
    ) -> ChartResult<Self>
    where
        CH: Chart<C, Data = D> + 'static,
        D: Clone + 'static,
    {
        let storage = ChartStorage {
            chart,
            data,
            config,
        };

        Ok(Self {
            id,
            position,
            chart: Box::new(storage),
        })
    }

    /// Get the panel ID
    pub fn id(&self) -> PanelId {
        self.id
    }

    /// Get the grid position
    pub fn position(&self) -> GridPosition {
        self.position
    }

    /// Update the data for this panel (requires exact type match)
    /// For now, this is a placeholder - proper implementation would require type information
    pub fn update_data<D>(&mut self, _data: D) -> ChartResult<()>
    where
        D: 'static,
    {
        // Type-safe updates require knowing the exact chart type
        // This would be better handled by keeping type information
        // or using a visitor pattern
        Err(crate::error::ChartError::NotImplemented)
    }

    /// Draw the panel
    pub fn draw<DT>(&self, viewport: Rectangle, target: &mut DT) -> ChartResult<()>
    where
        DT: DrawTarget<Color = C>,
    {
        self.chart.draw_erased(viewport, target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chart::LineChart;
    use crate::data::{Point2D, StaticDataSeries};
    use embedded_graphics::pixelcolor::Rgb565;

    #[test]
    fn test_panel_creation() {
        let chart = LineChart::<Rgb565>::new();
        let data: StaticDataSeries<Point2D, 100> = StaticDataSeries::new();
        let config = ChartConfig::default();
        
        let panel = ChartPanel::new(
            PanelId(0),
            GridPosition::new(0, 0),
            chart,
            data,
            config,
        ).unwrap();
        
        assert_eq!(panel.id(), PanelId(0));
        assert_eq!(panel.position(), GridPosition::new(0, 0));
    }
}