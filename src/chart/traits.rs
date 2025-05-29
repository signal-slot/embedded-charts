//! Core traits for chart implementations.

use crate::data::DataSeries;
use crate::error::ChartResult;
use embedded_graphics::{prelude::*, primitives::Rectangle};

/// Main trait for all chart types
pub trait Chart<C: PixelColor> {
    /// The type of data this chart can render
    type Data: DataSeries;
    /// Configuration type for this chart
    type Config;

    /// Draw the chart to the target display
    ///
    /// # Arguments
    /// * `data` - The data to render
    /// * `config` - Chart configuration
    /// * `viewport` - The area to draw the chart in
    /// * `target` - The display target to draw to
    fn draw<D>(
        &self,
        data: &Self::Data,
        config: &Self::Config,
        viewport: Rectangle,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>;

    /// Get the data bounds for this chart
    fn data_bounds(&self, _data: &Self::Data) -> ChartResult<()> {
        // Default implementation - concrete charts should override this
        // if they need specific bounds calculation
        Ok(())
    }
}

/// Trait for charts that support real-time data streaming
#[cfg(feature = "animations")]
pub trait StreamingChart<C: PixelColor>: Chart<C> {
    /// The type of individual data points
    type DataPoint: Copy + Clone;

    /// Get the streaming animator for this chart
    fn streaming_animator(&mut self) -> &mut crate::animation::StreamingAnimator<Self::DataPoint>;

    /// Push a new data point to the chart
    ///
    /// # Arguments
    /// * `point` - The new data point to add
    fn push_data(&mut self, point: Self::DataPoint) -> ChartResult<()> {
        self.streaming_animator().push_data(point);
        Ok(())
    }

    /// Draw the chart with streaming data and interpolation
    ///
    /// # Arguments
    /// * `config` - Chart configuration
    /// * `viewport` - The area to draw the chart in
    /// * `target` - The display target to draw to
    /// * `interpolation_progress` - Progress for smooth interpolation (0-100)
    fn draw_streaming<D>(
        &self,
        config: &Self::Config,
        viewport: embedded_graphics::primitives::Rectangle,
        target: &mut D,
        interpolation_progress: crate::animation::Progress,
    ) -> ChartResult<()>
    where
        D: embedded_graphics::draw_target::DrawTarget<Color = C>;

    /// Enable or disable smooth interpolation for streaming data
    ///
    /// # Arguments
    /// * `enabled` - Whether to enable smooth interpolation
    fn set_smooth_interpolation(&mut self, enabled: bool) {
        self.streaming_animator().set_smooth_interpolation(enabled);
    }

    /// Check if smooth interpolation is enabled
    fn is_smooth_interpolation_enabled(&self) -> bool;
}

/// Trait for charts that can be animated using the new progress-based system
#[cfg(feature = "animations")]
pub trait AnimatedChart<C: PixelColor>: Chart<C> {
    /// The type of data that can be animated
    type AnimatedData: crate::animation::Interpolatable;

    /// Draw the chart with animation at the specified progress
    ///
    /// # Arguments
    /// * `data` - The data to render
    /// * `config` - Chart configuration
    /// * `viewport` - The area to draw the chart in
    /// * `target` - The display target to draw to
    /// * `progress` - Animation progress (0-100)
    fn draw_animated<D>(
        &self,
        data: &Self::Data,
        config: &Self::Config,
        viewport: embedded_graphics::primitives::Rectangle,
        target: &mut D,
        progress: crate::animation::Progress,
    ) -> ChartResult<()>
    where
        D: embedded_graphics::draw_target::DrawTarget<Color = C>;

    /// Set up a transition animation between two data states
    ///
    /// # Arguments
    /// * `from_data` - Starting data state
    /// * `to_data` - Target data state
    /// * `easing` - Easing function to use
    ///
    /// # Returns
    /// A ChartAnimator that can be used to interpolate between states
    fn create_transition_animator(
        &self,
        from_data: Self::AnimatedData,
        to_data: Self::AnimatedData,
        easing: crate::animation::EasingFunction,
    ) -> crate::animation::ChartAnimator<Self::AnimatedData>;

    /// Extract animatable data from the chart's data
    ///
    /// # Arguments
    /// * `data` - The chart data to extract from
    ///
    /// # Returns
    /// The animatable representation of the data
    fn extract_animated_data(&self, data: &Self::Data) -> ChartResult<Self::AnimatedData>;
}

/// Trait for customizable chart styling
pub trait StylableChart<C: PixelColor> {
    /// Style configuration type
    type Style;

    /// Apply a style to the chart
    ///
    /// # Arguments
    /// * `style` - The style configuration to apply
    fn apply_style(&mut self, style: Self::Style);

    /// Get the current style
    fn style(&self) -> &Self::Style;
}

/// Trait for charts with configurable axes
pub trait AxisChart<C: PixelColor>: Chart<C> {
    /// X-axis type
    type XAxis;
    /// Y-axis type
    type YAxis;

    /// Set the X-axis configuration
    ///
    /// # Arguments
    /// * `axis` - X-axis configuration
    fn set_x_axis(&mut self, axis: Self::XAxis);

    /// Set the Y-axis configuration
    ///
    /// # Arguments
    /// * `axis` - Y-axis configuration
    fn set_y_axis(&mut self, axis: Self::YAxis);

    /// Get the X-axis configuration
    fn x_axis(&self) -> ChartResult<&Self::XAxis>;

    /// Get the Y-axis configuration
    fn y_axis(&self) -> ChartResult<&Self::YAxis>;
}

/// Trait for charts that support legends
pub trait LegendChart<C: PixelColor>: Chart<C> {
    /// Legend configuration type
    type Legend;

    /// Set the legend configuration
    ///
    /// # Arguments
    /// * `legend` - Legend configuration
    fn set_legend(&mut self, legend: Option<Self::Legend>);

    /// Get the legend configuration
    fn legend(&self) -> Option<&Self::Legend>;

    /// Calculate the space required for the legend
    fn legend_size(&self) -> Size;
}

/// Builder trait for fluent chart construction
pub trait ChartBuilder<C: PixelColor> {
    /// The chart type this builder creates
    type Chart: Chart<C>;
    /// Error type for building operations
    type Error;

    /// Build the chart with current configuration
    fn build(self) -> Result<Self::Chart, Self::Error>;
}

/// Trait for charts that can be rendered incrementally
pub trait IncrementalChart<C: PixelColor>: Chart<C> {
    /// Render only the changed portions of the chart
    ///
    /// # Arguments
    /// * `data` - The data to render
    /// * `config` - Chart configuration
    /// * `viewport` - The area to draw the chart in
    /// * `target` - The display target to draw to
    /// * `dirty_region` - The region that needs to be redrawn
    fn draw_incremental<D>(
        &self,
        data: &Self::Data,
        config: &Self::Config,
        viewport: Rectangle,
        target: &mut D,
        dirty_region: Rectangle,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>;

    /// Mark a region as dirty (needing redraw)
    ///
    /// # Arguments
    /// * `region` - The region to mark as dirty
    fn mark_dirty(&mut self, region: Rectangle);

    /// Get all dirty regions
    fn dirty_regions(&self) -> &[Rectangle];

    /// Clear all dirty regions
    fn clear_dirty(&mut self);
}

/// Trait for charts that support interaction
pub trait InteractiveChart<C: PixelColor>: Chart<C> {
    /// Event type for interactions
    type Event;
    /// Response type for interactions
    type Response;

    /// Handle an interaction event
    ///
    /// # Arguments
    /// * `event` - The interaction event
    /// * `viewport` - The chart viewport
    fn handle_event(
        &mut self,
        event: Self::Event,
        viewport: Rectangle,
    ) -> ChartResult<Self::Response>;

    /// Check if a point is within an interactive area
    ///
    /// # Arguments
    /// * `point` - The point to check
    /// * `viewport` - The chart viewport
    fn hit_test(&self, point: Point, viewport: Rectangle) -> Option<Self::Response>;
}

/// Trait for chart renderers that support animation frame rendering
#[cfg(feature = "animations")]
pub trait AnimationRenderer<C: PixelColor> {
    /// Check if the renderer needs frame updates
    fn needs_frame_update(&self) -> bool;

    /// Get the current frame rate (FPS)
    fn frame_rate(&self) -> u32;

    /// Set the target frame rate
    fn set_frame_rate(&mut self, fps: u32);
}

/// Common chart configuration
#[derive(Debug, Clone)]
pub struct ChartConfig<C: PixelColor> {
    /// Chart title
    pub title: Option<heapless::String<64>>,
    /// Background color
    pub background_color: Option<C>,
    /// Chart margins
    pub margins: Margins,
    /// Whether to show grid lines
    pub show_grid: bool,
    /// Grid color
    pub grid_color: Option<C>,
}

/// Chart margins configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Margins {
    /// Top margin in pixels
    pub top: u32,
    /// Right margin in pixels
    pub right: u32,
    /// Bottom margin in pixels
    pub bottom: u32,
    /// Left margin in pixels
    pub left: u32,
}

impl Margins {
    /// Create new margins with the same value for all sides
    pub const fn all(margin: u32) -> Self {
        Self {
            top: margin,
            right: margin,
            bottom: margin,
            left: margin,
        }
    }

    /// Create new margins with different horizontal and vertical values
    pub const fn symmetric(horizontal: u32, vertical: u32) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    /// Create new margins with individual values
    pub const fn new(top: u32, right: u32, bottom: u32, left: u32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    /// Get the total horizontal margin (left + right)
    pub const fn horizontal(&self) -> u32 {
        self.left + self.right
    }

    /// Get the total vertical margin (top + bottom)
    pub const fn vertical(&self) -> u32 {
        self.top + self.bottom
    }

    /// Apply margins to a rectangle, returning the inner area
    pub fn apply_to(&self, rect: Rectangle) -> Rectangle {
        let top_left = Point::new(
            rect.top_left.x + self.left as i32,
            rect.top_left.y + self.top as i32,
        );
        let size = Size::new(
            rect.size.width.saturating_sub(self.horizontal()),
            rect.size.height.saturating_sub(self.vertical()),
        );
        Rectangle::new(top_left, size)
    }
}

impl Default for Margins {
    fn default() -> Self {
        Self::all(10)
    }
}

impl<C: PixelColor> Default for ChartConfig<C> {
    fn default() -> Self {
        Self {
            title: None,
            background_color: None,
            margins: Margins::default(),
            show_grid: false,
            grid_color: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_margins_creation() {
        let margins = Margins::all(5);
        assert_eq!(margins.top, 5);
        assert_eq!(margins.right, 5);
        assert_eq!(margins.bottom, 5);
        assert_eq!(margins.left, 5);
    }

    #[test]
    fn test_margins_symmetric() {
        let margins = Margins::symmetric(10, 20);
        assert_eq!(margins.top, 20);
        assert_eq!(margins.right, 10);
        assert_eq!(margins.bottom, 20);
        assert_eq!(margins.left, 10);
    }

    #[test]
    fn test_margins_totals() {
        let margins = Margins::new(5, 10, 15, 20);
        assert_eq!(margins.horizontal(), 30);
        assert_eq!(margins.vertical(), 20);
    }

    #[test]
    fn test_margins_apply_to() {
        let margins = Margins::all(10);
        let rect = Rectangle::new(Point::new(0, 0), Size::new(100, 80));
        let inner = margins.apply_to(rect);

        assert_eq!(inner.top_left, Point::new(10, 10));
        assert_eq!(inner.size, Size::new(80, 60));
    }
}
