//! Stacked chart implementations with animation support.
//!
//! This module provides animated stacked bar and line (area) charts that support
//! smooth transitions between data states with proper cumulative value interpolation.

#[cfg(feature = "animations")]
use crate::animation::Interpolatable;
use crate::chart::traits::{Chart, ChartConfig, Margins};
#[cfg(feature = "animations")]
use crate::chart::traits::AnimatedChart;
use crate::data::{DataPoint, DataSeries};
use crate::error::{ChartError, ChartResult};
use crate::math::{Math, NumericConversion};
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Line, PrimitiveStyle, Rectangle},
};

/// Multi-layer data structure for stacked charts
#[derive(Debug, Clone)]
pub struct StackedData<T: Copy + Clone + DataPoint, const N: usize> {
    /// Multiple data series, one for each stack layer
    layers: heapless::Vec<crate::data::series::StaticDataSeries<T, N>, 8>,
    /// Layer labels for legend
    labels: heapless::Vec<heapless::String<32>, 8>,
    /// Colors for each layer
    colors: heapless::Vec<Rgb565, 8>,
}

impl<T: Copy + Clone + DataPoint, const N: usize> StackedData<T, N> {
    /// Create a new stacked data structure
    pub fn new() -> Self {
        Self {
            layers: heapless::Vec::new(),
            labels: heapless::Vec::new(),
            colors: heapless::Vec::new(),
        }
    }

    /// Add a new layer to the stack
    pub fn add_layer(
        &mut self,
        data: crate::data::series::StaticDataSeries<T, N>,
        label: &str,
        color: Rgb565,
    ) -> ChartResult<()> {
        self.layers.push(data).map_err(|_| ChartError::MemoryFull)?;
        self.labels
            .push(heapless::String::try_from(label).map_err(|_| ChartError::MemoryFull)?)
            .map_err(|_| ChartError::MemoryFull)?;
        self.colors
            .push(color)
            .map_err(|_| ChartError::MemoryFull)?;
        Ok(())
    }

    /// Get the number of layers
    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }

    /// Get a specific layer
    pub fn layer(&self, index: usize) -> Option<&crate::data::series::StaticDataSeries<T, N>> {
        self.layers.get(index)
    }

    /// Get layer label
    pub fn label(&self, index: usize) -> Option<&str> {
        self.labels.get(index).map(|s| s.as_str())
    }

    /// Get layer color
    pub fn color(&self, index: usize) -> Option<Rgb565> {
        self.colors.get(index).copied()
    }

    /// Calculate cumulative values for stacking
    pub fn calculate_cumulative(&self) -> ChartResult<heapless::Vec<heapless::Vec<T::Y, N>, 8>>
    where
        T::Y: Copy + Clone + core::ops::Add<Output = T::Y> + Default,
    {
        let mut cumulative_layers = heapless::Vec::new();

        if self.layers.is_empty() {
            return Ok(cumulative_layers);
        }

        let data_length = self.layers[0].len();

        // Verify all layers have the same length
        for layer in &self.layers {
            if layer.len() != data_length {
                return Err(ChartError::DataError(crate::error::DataError::BUFFER_FULL));
            }
        }

        // Calculate cumulative values for each layer
        for layer_idx in 0..self.layers.len() {
            let mut cumulative_values = heapless::Vec::new();

            for point_idx in 0..data_length {
                let mut cumulative_y = T::Y::default();

                // Sum up all values from bottom to current layer
                for bottom_layer_idx in 0..=layer_idx {
                    if let Some(point) = self.layers[bottom_layer_idx].get(point_idx) {
                        cumulative_y = cumulative_y + point.y();
                    }
                }

                cumulative_values
                    .push(cumulative_y)
                    .map_err(|_| ChartError::MemoryFull)?;
            }

            cumulative_layers
                .push(cumulative_values)
                .map_err(|_| ChartError::MemoryFull)?;
        }

        Ok(cumulative_layers)
    }
}

impl<T: Copy + Clone + DataPoint, const N: usize> Default for StackedData<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "animations")]
impl<T: Copy + Clone + DataPoint, const N: usize> Interpolatable for StackedData<T, N>
where
    T: Interpolatable,
{
    fn interpolate(self, other: Self, progress: f32) -> Option<Self> {
        let mut result = StackedData::new();

        // Interpolate each layer
        let layer_count = self.layer_count().min(other.layer_count());
        for layer_idx in 0..layer_count {
            if let (Some(from_layer), Some(to_layer)) =
                (self.layer(layer_idx), other.layer(layer_idx))
            {
                let mut interpolated_layer = crate::data::series::StaticDataSeries::new();

                // Interpolate each point in the layer
                let point_count = from_layer.len().min(to_layer.len());
                for point_idx in 0..point_count {
                    if let (Some(from_point), Some(to_point)) =
                        (from_layer.get(point_idx), to_layer.get(point_idx))
                    {
                        if let Some(interpolated) = from_point.interpolate(to_point, progress) {
                            let _ = interpolated_layer.push(interpolated);
                        }
                    }
                }

                // Copy layer metadata
                let label = self.label(layer_idx).unwrap_or("Layer");
                let color = self.color(layer_idx).unwrap_or(Rgb565::BLUE);
                let _ = result.add_layer(interpolated_layer, label, color);
            }
        }

        Some(result)
    }
}

/// Implement DataSeries for StackedData to make it compatible with Chart trait
impl<T: Copy + Clone + DataPoint, const N: usize> DataSeries for StackedData<T, N> {
    type Item = T;
    type Iter = core::iter::Flatten<
        core::option::IntoIter<crate::data::series::StaticDataSeriesIter<T, N>>,
    >;

    fn len(&self) -> usize {
        // Return the length of the first layer, or 0 if no layers
        self.layers.first().map(|layer| layer.len()).unwrap_or(0)
    }

    fn is_empty(&self) -> bool {
        self.layers.is_empty() || self.len() == 0
    }

    fn get(&self, index: usize) -> Option<Self::Item> {
        // For DataSeries compatibility, return the first layer's item
        // This is mainly used for bounds calculation
        self.layers.first()?.get(index)
    }

    fn iter(&self) -> Self::Iter {
        // Return iterator over the first layer for compatibility
        self.layers
            .first()
            .map(|layer| layer.iter())
            .into_iter()
            .flatten()
    }
}

/// Animated stacked bar chart implementation
#[derive(Debug)]
pub struct AnimatedStackedBarChart<C: PixelColor> {
    /// Current animated data (interpolated cumulative values)
    current_data: Option<StackedData<crate::data::point::Point2D, 256>>,
    /// Chart configuration
    config: ChartConfig<C>,
    /// Bar width configuration
    bar_width: StackedBarWidth,
    /// Spacing between bars
    spacing: u32,
    /// Frame rate for animations
    frame_rate: u32,
}

/// Bar width configuration for stacked charts
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StackedBarWidth {
    /// Automatic width based on available space
    Auto,
    /// Fixed width in pixels
    Fixed(u32),
    /// Percentage of available space (0.0 to 1.0)
    Percentage(f32),
}

impl<C: PixelColor> AnimatedStackedBarChart<C>
where
    C: From<Rgb565>,
{
    /// Create a new animated stacked bar chart
    pub fn new() -> Self {
        Self {
            current_data: None,
            config: ChartConfig::default(),
            bar_width: StackedBarWidth::Auto,
            spacing: 5,
            frame_rate: 60,
        }
    }

    /// Create a builder for configuring the animated stacked bar chart
    pub fn builder() -> AnimatedStackedBarChartBuilder<C> {
        AnimatedStackedBarChartBuilder::new()
    }

    /// Set the bar width configuration
    pub fn set_bar_width(&mut self, width: StackedBarWidth) {
        self.bar_width = width;
    }

    /// Set the spacing between bars
    pub fn set_spacing(&mut self, spacing: u32) {
        self.spacing = spacing;
    }

    /// Set the frame rate for animations
    pub fn set_frame_rate(&mut self, fps: u32) {
        self.frame_rate = fps.clamp(1, 120);
    }

    /// Calculate the actual bar width based on configuration and available space
    fn calculate_bar_width(&self, available_width: u32, bar_count: usize) -> u32 {
        match self.bar_width {
            StackedBarWidth::Auto => {
                if bar_count == 0 {
                    return 0;
                }
                // Simple auto-sizing without spacing
                available_width / (bar_count as u32).max(1)
            }
            StackedBarWidth::Fixed(width) => width,
            StackedBarWidth::Percentage(pct) => {
                ((available_width as f32) * pct.clamp(0.0, 1.0)) as u32
            }
        }
    }

    /// Interpolate between two stacked data sets based on animation progress
    #[allow(dead_code)]
    fn interpolate_stacked_data(
        &self,
        from_data: &StackedData<crate::data::point::Point2D, 256>,
        to_data: &StackedData<crate::data::point::Point2D, 256>,
        progress: f32,
    ) -> ChartResult<StackedData<crate::data::point::Point2D, 256>> {
        let mut result = StackedData::new();

        // Ensure both datasets have the same number of layers
        let layer_count = from_data.layer_count().min(to_data.layer_count());

        for layer_idx in 0..layer_count {
            if let (Some(from_layer), Some(to_layer)) =
                (from_data.layer(layer_idx), to_data.layer(layer_idx))
            {
                let mut interpolated_layer = crate::data::series::StaticDataSeries::new();

                // Interpolate each point in the layer
                let point_count = from_layer.len().min(to_layer.len());
                for point_idx in 0..point_count {
                    if let (Some(from_point), Some(to_point)) =
                        (from_layer.get(point_idx), to_layer.get(point_idx))
                    {
                        let interpolated_x =
                            from_point.x() + (to_point.x() - from_point.x()) * progress;
                        let interpolated_y =
                            from_point.y() + (to_point.y() - from_point.y()) * progress;

                        interpolated_layer
                            .push(crate::data::point::Point2D::new(
                                interpolated_x,
                                interpolated_y,
                            ))
                            .map_err(|_| ChartError::MemoryFull)?;
                    }
                }

                // Copy layer metadata
                let label = from_data.label(layer_idx).unwrap_or("Layer");
                let color = from_data.color(layer_idx).unwrap_or(Rgb565::BLUE);
                result.add_layer(interpolated_layer, label, color)?;
            }
        }

        Ok(result)
    }

    /// Get the current render data
    fn get_render_data(&self) -> StackedData<crate::data::point::Point2D, 256> {
        self.current_data.clone().unwrap_or_default()
    }
}

impl<C: PixelColor> Default for AnimatedStackedBarChart<C>
where
    C: From<Rgb565>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<C: PixelColor> Chart<C> for AnimatedStackedBarChart<C>
where
    C: From<Rgb565>,
{
    type Data = StackedData<crate::data::point::Point2D, 256>;
    type Config = ChartConfig<C>;

    fn draw<D>(
        &self,
        data: &Self::Data,
        config: &Self::Config,
        viewport: Rectangle,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        // Use animated data if available, otherwise use provided data
        let render_data = if self.current_data.is_some() {
            self.get_render_data()
        } else {
            data.clone()
        };

        self.draw_stacked_bars(&render_data, config, viewport, target)
    }
}

impl<C: PixelColor> AnimatedStackedBarChart<C>
where
    C: From<Rgb565>,
{
    /// Draw the stacked bars
    fn draw_stacked_bars<D>(
        &self,
        data: &StackedData<crate::data::point::Point2D, 256>,
        config: &ChartConfig<C>,
        viewport: Rectangle,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        if data.layer_count() == 0 {
            return Ok(());
        }

        // Calculate drawing area with margins
        let draw_area = config.margins.apply_to(viewport);

        // Get the first layer to determine the number of data points
        let first_layer = data.layer(0).unwrap();
        let data_point_count = first_layer.len();

        if data_point_count == 0 {
            return Ok(());
        }

        // Calculate cumulative values for stacking
        let cumulative_values = data.calculate_cumulative()?;

        // Find the maximum total value for scaling
        let max_total = cumulative_values
            .last()
            .map(|last_layer| last_layer.iter().fold(0.0f32, |acc, &val| acc.max(val)))
            .unwrap_or(1.0);

        // Calculate bar dimensions
        let bar_width = self.calculate_bar_width(draw_area.size.width, data_point_count);
        let total_bar_space = bar_width * data_point_count as u32;
        let total_spacing = self.spacing * (data_point_count.saturating_sub(1) as u32);
        let start_x = draw_area.top_left.x
            + ((draw_area
                .size
                .width
                .saturating_sub(total_bar_space + total_spacing))
                / 2) as i32;

        // Draw stacked bars for each data point
        for point_idx in 0..data_point_count {
            let bar_x = start_x + (point_idx as u32 * (bar_width + self.spacing)) as i32;
            let base_y = draw_area.top_left.y + draw_area.size.height as i32;

            // Draw segments from bottom to top
            let mut current_bottom = base_y;

            for layer_idx in 0..data.layer_count() {
                if let Some(cumulative_layer) = cumulative_values.get(layer_idx) {
                    if let Some(&cumulative_value) = cumulative_layer.get(point_idx) {
                        let cumulative_f32: f32 = cumulative_value;

                        // Calculate segment height
                        let segment_top_y = base_y
                            - ((cumulative_f32 / max_total) * (draw_area.size.height as f32 - 1.0))
                                as i32;

                        // Only draw if there's a visible height
                        if current_bottom > segment_top_y {
                            let segment_rect = Rectangle::new(
                                Point::new(bar_x, segment_top_y),
                                Size::new(bar_width, (current_bottom - segment_top_y) as u32),
                            );

                            let color = data.color(layer_idx).unwrap_or(Rgb565::BLUE);
                            segment_rect
                                .into_styled(PrimitiveStyle::with_fill(C::from(color)))
                                .draw(target)
                                .map_err(|_| {
                                    ChartError::RenderError(
                                        crate::error::RenderError::DrawingFailed,
                                    )
                                })?;

                            current_bottom = segment_top_y;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(feature = "animations")]
impl<C: PixelColor> AnimatedChart<C> for AnimatedStackedBarChart<C>
where
    C: From<Rgb565>,
{
    type AnimatedData = StackedData<crate::data::point::Point2D, 256>;

    fn draw_animated<D>(
        &self,
        data: &Self::Data,
        config: &Self::Config,
        viewport: embedded_graphics::primitives::Rectangle,
        target: &mut D,
        _progress: crate::animation::Progress,
    ) -> ChartResult<()>
    where
        D: embedded_graphics::draw_target::DrawTarget<Color = C>,
    {
        // Use the provided data which should already be interpolated by the caller
        self.draw_stacked_bars(data, config, viewport, target)
    }

    fn create_transition_animator(
        &self,
        from_data: Self::AnimatedData,
        to_data: Self::AnimatedData,
        easing: crate::animation::EasingFunction,
    ) -> crate::animation::ChartAnimator<Self::AnimatedData> {
        crate::animation::ChartAnimator::new(from_data, to_data, easing)
    }

    fn extract_animated_data(&self, data: &Self::Data) -> ChartResult<Self::AnimatedData> {
        // Clone the data for animation
        Ok(data.clone())
    }
}

/// Builder for animated stacked bar charts
#[derive(Debug)]
pub struct AnimatedStackedBarChartBuilder<C: PixelColor> {
    bar_width: StackedBarWidth,
    spacing: u32,
    frame_rate: u32,
    config: ChartConfig<C>,
}

impl<C: PixelColor> AnimatedStackedBarChartBuilder<C>
where
    C: From<Rgb565>,
{
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            bar_width: StackedBarWidth::Auto,
            spacing: 5,
            frame_rate: 60,
            config: ChartConfig::default(),
        }
    }

    /// Set the bar width
    pub fn bar_width(mut self, width: StackedBarWidth) -> Self {
        self.bar_width = width;
        self
    }

    /// Set the spacing between bars
    pub fn spacing(mut self, spacing: u32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set the frame rate
    pub fn frame_rate(mut self, fps: u32) -> Self {
        self.frame_rate = fps;
        self
    }

    /// Set the chart title
    pub fn with_title(mut self, title: &str) -> Self {
        self.config.title = heapless::String::try_from(title).ok();
        self
    }

    /// Set the background color
    pub fn background_color(mut self, color: C) -> Self {
        self.config.background_color = Some(color);
        self
    }

    /// Set the margins
    pub fn margins(mut self, margins: Margins) -> Self {
        self.config.margins = margins;
        self
    }

    /// Build the animated stacked bar chart
    pub fn build(self) -> ChartResult<AnimatedStackedBarChart<C>> {
        let mut chart = AnimatedStackedBarChart::new();
        chart.set_bar_width(self.bar_width);
        chart.set_spacing(self.spacing);
        chart.set_frame_rate(self.frame_rate);
        chart.config = self.config;
        Ok(chart)
    }
}

impl<C: PixelColor> Default for AnimatedStackedBarChartBuilder<C>
where
    C: From<Rgb565>,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Animated stacked line chart (area chart) implementation
#[derive(Debug)]
pub struct AnimatedStackedLineChart<C: PixelColor> {
    /// Current animated data (interpolated cumulative values)
    current_data: Option<StackedData<crate::data::point::Point2D, 256>>,
    /// Chart configuration
    config: ChartConfig<C>,
    /// Whether to smooth the lines (bezier curves)
    smooth_lines: bool,
    /// Line width for area boundaries
    line_width: u32,
    /// Frame rate for animations
    frame_rate: u32,
}

impl<C: PixelColor> AnimatedStackedLineChart<C>
where
    C: From<Rgb565>,
{
    /// Create a new animated stacked line chart
    pub fn new() -> Self {
        Self {
            current_data: None,
            config: ChartConfig::default(),
            smooth_lines: false,
            line_width: 2,
            frame_rate: 60,
        }
    }

    /// Create a builder for configuring the animated stacked line chart
    pub fn builder() -> AnimatedStackedLineChartBuilder<C> {
        AnimatedStackedLineChartBuilder::new()
    }

    /// Set whether to smooth the lines
    pub fn set_smooth_lines(&mut self, smooth: bool) {
        self.smooth_lines = smooth;
    }

    /// Set the line width
    pub fn set_line_width(&mut self, width: u32) {
        self.line_width = width;
    }

    /// Set the frame rate for animations
    pub fn set_frame_rate(&mut self, fps: u32) {
        self.frame_rate = fps.clamp(1, 120);
    }

    /// Get the current frame rate
    pub fn frame_rate(&self) -> u32 {
        self.frame_rate
    }

    /// Get the current render data
    fn get_render_data(&self) -> StackedData<crate::data::point::Point2D, 256> {
        self.current_data.clone().unwrap_or_default()
    }
}

impl<C: PixelColor> Default for AnimatedStackedLineChart<C>
where
    C: From<Rgb565>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<C: PixelColor> Chart<C> for AnimatedStackedLineChart<C>
where
    C: From<Rgb565>,
{
    type Data = StackedData<crate::data::point::Point2D, 256>;
    type Config = ChartConfig<C>;

    fn draw<D>(
        &self,
        data: &Self::Data,
        config: &Self::Config,
        viewport: Rectangle,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        // Use animated data if available, otherwise use provided data
        let render_data = if self.current_data.is_some() {
            self.get_render_data()
        } else {
            data.clone()
        };

        self.draw_stacked_areas(&render_data, config, viewport, target)
    }
}

impl<C: PixelColor> AnimatedStackedLineChart<C>
where
    C: From<Rgb565>,
{
    /// Draw the stacked areas
    fn draw_stacked_areas<D>(
        &self,
        data: &StackedData<crate::data::point::Point2D, 256>,
        config: &ChartConfig<C>,
        viewport: Rectangle,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        if data.layer_count() == 0 {
            return Ok(());
        }

        // Calculate drawing area with margins
        let draw_area = config.margins.apply_to(viewport);

        // Get the first layer to determine the number of data points
        let first_layer = data.layer(0).unwrap();
        let data_point_count = first_layer.len();

        if data_point_count == 0 {
            return Ok(());
        }

        // Calculate cumulative values for stacking
        let cumulative_values = data.calculate_cumulative()?;

        // Find the maximum total value for scaling
        let max_total = cumulative_values
            .last()
            .map(|last_layer| last_layer.iter().fold(0.0f32, |acc, &val| acc.max(val)))
            .unwrap_or(1.0);

        // Convert cumulative data to screen coordinates for each layer
        let mut screen_points = heapless::Vec::<heapless::Vec<Point, 256>, 8>::new();

        for layer_idx in 0..data.layer_count() {
            if let Some(cumulative_layer) = cumulative_values.get(layer_idx) {
                let mut layer_points = heapless::Vec::new();

                for (point_idx, &cumulative_value) in cumulative_layer.iter().enumerate() {
                    let cumulative_f32: f32 = cumulative_value;

                    // Calculate screen coordinates
                    let x = draw_area.top_left.x
                        + ((point_idx as f32 / (data_point_count - 1).max(1) as f32)
                            * (draw_area.size.width as f32 - 1.0)) as i32;
                    let y = draw_area.top_left.y + draw_area.size.height as i32
                        - 1
                        - ((cumulative_f32 / max_total) * (draw_area.size.height as f32 - 1.0))
                            as i32;

                    layer_points
                        .push(Point::new(x, y))
                        .map_err(|_| ChartError::MemoryFull)?;
                }

                screen_points
                    .push(layer_points)
                    .map_err(|_| ChartError::MemoryFull)?;
            }
        }

        // Draw stacked areas from top to bottom
        for layer_idx in (0..data.layer_count()).rev() {
            if let Some(current_layer_points) = screen_points.get(layer_idx) {
                let color = data.color(layer_idx).unwrap_or(Rgb565::BLUE);

                // Get the bottom boundary (previous layer or baseline)
                if layer_idx > 0 {
                    if let Some(bottom_layer_points) = screen_points.get(layer_idx - 1) {
                        // Draw the area between current layer and bottom layer
                        self.draw_area_between_curves(
                            current_layer_points,
                            bottom_layer_points,
                            C::from(color),
                            target,
                        )?;

                        // Draw the outline for this layer
                        self.draw_layer_outline(current_layer_points, C::from(color), target)?;
                    }
                } else {
                    // Create baseline points at the bottom of the chart
                    let mut baseline: heapless::Vec<Point, 256> = heapless::Vec::new();
                    for point in current_layer_points {
                        baseline
                            .push(Point::new(
                                point.x,
                                draw_area.top_left.y + draw_area.size.height as i32,
                            ))
                            .map_err(|_| ChartError::MemoryFull)?;
                    }

                    // Draw the area between current layer and baseline
                    self.draw_area_between_curves(
                        current_layer_points,
                        &baseline,
                        C::from(color),
                        target,
                    )?;

                    // Draw the outline for this layer
                    self.draw_layer_outline(current_layer_points, C::from(color), target)?;
                }
            }
        }

        Ok(())
    }

    /// Draw area between two curves using scan-line filling
    fn draw_area_between_curves<D>(
        &self,
        top_curve: &[Point],
        bottom_curve: &[Point],
        color: C,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        if top_curve.len() != bottom_curve.len() || top_curve.is_empty() {
            return Ok(());
        }

        // For each segment between consecutive points
        for i in 0..top_curve.len() - 1 {
            let top_start = top_curve[i];
            let top_end = top_curve[i + 1];
            let bottom_start = bottom_curve[i];
            let bottom_end = bottom_curve[i + 1];

            // Draw filled quadrilateral using scan lines
            self.draw_filled_quad(top_start, top_end, bottom_end, bottom_start, color, target)?;
        }

        Ok(())
    }

    /// Draw a filled quadrilateral using horizontal scan lines
    fn draw_filled_quad<D>(
        &self,
        p1: Point,
        p2: Point,
        p3: Point,
        p4: Point,
        color: C,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        // Split the quadrilateral into two triangles and fill them
        // Triangle 1: p1, p2, p3
        self.draw_filled_triangle(p1, p2, p3, color, target)?;
        // Triangle 2: p1, p3, p4
        self.draw_filled_triangle(p1, p3, p4, color, target)?;

        Ok(())
    }

    /// Draw a filled triangle using scan line algorithm
    fn draw_filled_triangle<D>(
        &self,
        p1: Point,
        p2: Point,
        p3: Point,
        color: C,
        target: &mut D,
    ) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        // Find bounding box
        let min_x = p1.x.min(p2.x).min(p3.x);
        let max_x = p1.x.max(p2.x).max(p3.x);
        let min_y = p1.y.min(p2.y).min(p3.y);
        let max_y = p1.y.max(p2.y).max(p3.y);

        // For each horizontal scan line
        for y in min_y..=max_y {
            let mut intersections = heapless::Vec::<i32, 8>::new();

            // Check intersection with each edge of the triangle
            let edges = [(p1, p2), (p2, p3), (p3, p1)];
            for (start, end) in edges.iter() {
                if let Some(x) = self.line_intersection_x(*start, *end, y) {
                    if x >= min_x && x <= max_x {
                        intersections.push(x).ok(); // Ignore if buffer is full
                    }
                }
            }

            // Remove duplicates and sort
            intersections.sort();

            // Manual deduplication for heapless::Vec
            let mut unique_intersections = heapless::Vec::<i32, 8>::new();
            let mut last_value: Option<i32> = None;
            for &value in &intersections {
                if last_value != Some(value) {
                    unique_intersections.push(value).ok(); // Ignore if buffer is full
                    last_value = Some(value);
                }
            }
            let intersections = unique_intersections;

            // Draw horizontal line between the two intersection points
            if intersections.len() >= 2 {
                let start_x = intersections[0];
                let end_x = intersections[intersections.len() - 1];
                if start_x != end_x {
                    let rect = Rectangle::new(
                        Point::new(start_x, y),
                        Size::new((end_x - start_x) as u32, 1),
                    );
                    rect.into_styled(PrimitiveStyle::with_fill(color))
                        .draw(target)
                        .map_err(|_| {
                            ChartError::RenderError(crate::error::RenderError::DrawingFailed)
                        })?;
                }
            }
        }

        Ok(())
    }

    /// Find x-coordinate where a line segment intersects a horizontal line at y
    fn line_intersection_x(&self, start: Point, end: Point, y: i32) -> Option<i32> {
        if start.y == end.y {
            // Horizontal line - no single intersection point
            return None;
        }

        if (start.y <= y && y <= end.y) || (end.y <= y && y <= start.y) {
            // Linear interpolation
            let t = (y - start.y) as f32 / (end.y - start.y) as f32;
            let x = start.x as f32 + t * (end.x - start.x) as f32;
            let x_num = x.to_number();
            let half = 0.5f32.to_number();
            let rounded = Math::floor(x_num + half);
            Some(f32::from_number(rounded) as i32)
        } else {
            None
        }
    }

    /// Draw the outline for a layer
    fn draw_layer_outline<D>(&self, points: &[Point], color: C, target: &mut D) -> ChartResult<()>
    where
        D: DrawTarget<Color = C>,
    {
        let line_style = PrimitiveStyle::with_stroke(color, self.line_width);

        for i in 0..points.len() - 1 {
            let line = Line::new(points[i], points[i + 1]);
            line.into_styled(line_style)
                .draw(target)
                .map_err(|_| ChartError::RenderError(crate::error::RenderError::DrawingFailed))?;
        }

        Ok(())
    }
}

#[cfg(feature = "animations")]
impl<C: PixelColor> AnimatedChart<C> for AnimatedStackedLineChart<C>
where
    C: From<Rgb565>,
{
    type AnimatedData = StackedData<crate::data::point::Point2D, 256>;

    fn draw_animated<D>(
        &self,
        data: &Self::Data,
        config: &Self::Config,
        viewport: embedded_graphics::primitives::Rectangle,
        target: &mut D,
        _progress: crate::animation::Progress,
    ) -> ChartResult<()>
    where
        D: embedded_graphics::draw_target::DrawTarget<Color = C>,
    {
        // Use the provided data which should already be interpolated by the caller
        self.draw_stacked_areas(data, config, viewport, target)
    }

    fn create_transition_animator(
        &self,
        from_data: Self::AnimatedData,
        to_data: Self::AnimatedData,
        easing: crate::animation::EasingFunction,
    ) -> crate::animation::ChartAnimator<Self::AnimatedData> {
        crate::animation::ChartAnimator::new(from_data, to_data, easing)
    }

    fn extract_animated_data(&self, data: &Self::Data) -> ChartResult<Self::AnimatedData> {
        // Clone the data for animation
        Ok(data.clone())
    }
}

/// Builder for animated stacked line charts
#[derive(Debug)]
pub struct AnimatedStackedLineChartBuilder<C: PixelColor> {
    smooth_lines: bool,
    line_width: u32,
    frame_rate: u32,
    config: ChartConfig<C>,
}

impl<C: PixelColor> AnimatedStackedLineChartBuilder<C>
where
    C: From<Rgb565>,
{
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            smooth_lines: false,
            line_width: 2,
            frame_rate: 60,
            config: ChartConfig::default(),
        }
    }

    /// Set whether to smooth the lines
    pub fn smooth_lines(mut self, smooth: bool) -> Self {
        self.smooth_lines = smooth;
        self
    }

    /// Set the line width
    pub fn line_width(mut self, width: u32) -> Self {
        self.line_width = width;
        self
    }

    /// Set the frame rate
    pub fn frame_rate(mut self, fps: u32) -> Self {
        self.frame_rate = fps;
        self
    }

    /// Set the chart title
    pub fn with_title(mut self, title: &str) -> Self {
        self.config.title = heapless::String::try_from(title).ok();
        self
    }

    /// Set the background color
    pub fn background_color(mut self, color: C) -> Self {
        self.config.background_color = Some(color);
        self
    }

    /// Set the margins
    pub fn margins(mut self, margins: Margins) -> Self {
        self.config.margins = margins;
        self
    }

    /// Build the animated stacked line chart
    pub fn build(self) -> ChartResult<AnimatedStackedLineChart<C>> {
        let mut chart = AnimatedStackedLineChart::new();
        chart.set_smooth_lines(self.smooth_lines);
        chart.set_line_width(self.line_width);
        chart.set_frame_rate(self.frame_rate);
        chart.config = self.config;
        Ok(chart)
    }
}

impl<C: PixelColor> Default for AnimatedStackedLineChartBuilder<C>
where
    C: From<Rgb565>,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::point::Point2D;
    use crate::data::series::StaticDataSeries;

    #[test]
    fn test_stacked_data_creation() {
        let mut stacked_data = StackedData::<Point2D, 10>::new();
        assert_eq!(stacked_data.layer_count(), 0);

        let mut layer1 = StaticDataSeries::new();
        layer1.push(Point2D::new(0.0, 10.0)).unwrap();
        layer1.push(Point2D::new(1.0, 15.0)).unwrap();

        stacked_data
            .add_layer(layer1, "Layer 1", Rgb565::BLUE)
            .unwrap();
        assert_eq!(stacked_data.layer_count(), 1);
        assert_eq!(stacked_data.label(0), Some("Layer 1"));
        assert_eq!(stacked_data.color(0), Some(Rgb565::BLUE));
    }

    #[test]
    fn test_cumulative_calculation() {
        let mut stacked_data = StackedData::<Point2D, 10>::new();

        let mut layer1 = StaticDataSeries::new();
        layer1.push(Point2D::new(0.0, 10.0)).unwrap();
        layer1.push(Point2D::new(1.0, 15.0)).unwrap();

        let mut layer2 = StaticDataSeries::new();
        layer2.push(Point2D::new(0.0, 5.0)).unwrap();
        layer2.push(Point2D::new(1.0, 8.0)).unwrap();

        stacked_data
            .add_layer(layer1, "Layer 1", Rgb565::BLUE)
            .unwrap();
        stacked_data
            .add_layer(layer2, "Layer 2", Rgb565::RED)
            .unwrap();

        let cumulative = stacked_data.calculate_cumulative().unwrap();
        assert_eq!(cumulative.len(), 2);

        // First layer should be just the original values
        assert_eq!(cumulative[0][0], 10.0);
        assert_eq!(cumulative[0][1], 15.0);

        // Second layer should be cumulative
        assert_eq!(cumulative[1][0], 15.0); // 10 + 5
        assert_eq!(cumulative[1][1], 23.0); // 15 + 8
    }

    #[test]
    fn test_animated_stacked_bar_chart_creation() {
        let chart = AnimatedStackedBarChart::<Rgb565>::new();
        assert_eq!(chart.frame_rate, 60);
    }

    #[test]
    fn test_animated_stacked_line_chart_creation() {
        let chart = AnimatedStackedLineChart::<Rgb565>::new();
        assert_eq!(chart.frame_rate, 60);
    }

    #[test]
    fn test_bar_width_calculation() {
        let chart = AnimatedStackedBarChart::<Rgb565>::new();

        // Test auto width (simple division, no spacing considered)
        let width = chart.calculate_bar_width(400, 4);
        assert_eq!(width, 100); // 400 / 4 = 100

        // Test with spacing (spacing doesn't affect auto calculation in current implementation)
        let mut chart_with_spacing = AnimatedStackedBarChart::<Rgb565>::new();
        chart_with_spacing.set_spacing(10);
        let width = chart_with_spacing.calculate_bar_width(400, 4);
        assert_eq!(width, 100); // 400 / 4 = 100 (spacing not considered in auto mode)
    }

    #[test]
    fn test_builder_pattern() {
        let chart = AnimatedStackedBarChart::<Rgb565>::builder()
            .bar_width(StackedBarWidth::Fixed(50))
            .spacing(10)
            .frame_rate(30)
            .with_title("Test Chart")
            .build()
            .unwrap();

        assert_eq!(chart.frame_rate, 30);
        assert_eq!(
            chart.config.title.as_ref().map(|s| s.as_str()),
            Some("Test Chart")
        );
    }

    #[test]
    fn test_line_chart_builder_pattern() {
        let chart = AnimatedStackedLineChart::<Rgb565>::builder()
            .smooth_lines(true)
            .line_width(3)
            .frame_rate(30)
            .with_title("Test Line Chart")
            .build()
            .unwrap();

        assert_eq!(chart.frame_rate(), 30);
        assert_eq!(
            chart.config.title.as_ref().map(|s| s.as_str()),
            Some("Test Line Chart")
        );
    }

    #[test]
    fn test_line_intersection() {
        let chart = AnimatedStackedLineChart::<Rgb565>::new();

        // Test horizontal line intersection
        let start = Point::new(0, 0);
        let end = Point::new(10, 10);
        let y = 5;

        let intersection = chart.line_intersection_x(start, end, y);
        assert_eq!(intersection, Some(5));

        // Test no intersection
        let y_outside = 15;
        let no_intersection = chart.line_intersection_x(start, end, y_outside);
        assert_eq!(no_intersection, None);
    }
}
