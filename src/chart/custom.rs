//! Custom chart implementation.
//!
//! This module provides functionality for creating custom chart types.

use crate::error::ChartResult;
use embedded_graphics::pixelcolor::PixelColor;

/// A custom chart for user-defined chart types.
pub struct CustomChart<C: PixelColor> {
    _phantom: core::marker::PhantomData<C>,
}

impl<C: PixelColor> CustomChart<C> {
    /// Create a new custom chart builder.
    pub fn builder() -> CustomChartBuilder<C> {
        CustomChartBuilder::new()
    }
}

/// Builder for creating custom charts.
pub struct CustomChartBuilder<C: PixelColor> {
    _phantom: core::marker::PhantomData<C>,
}

impl<C: PixelColor> CustomChartBuilder<C> {
    /// Create a new custom chart builder.
    pub fn new() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }

    /// Build the custom chart.
    pub fn build(self) -> ChartResult<CustomChart<C>> {
        Ok(CustomChart {
            _phantom: core::marker::PhantomData,
        })
    }
}

impl<C: PixelColor> Default for CustomChartBuilder<C> {
    fn default() -> Self {
        Self::new()
    }
}
