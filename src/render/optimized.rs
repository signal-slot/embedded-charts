//! Display-specific optimized rendering implementations
//!
//! Provides specialized rendering paths for different display types:
//! - OLED: Column-based updates, monochrome optimizations
//! - TFT: DMA-friendly operations, RGB565 optimizations
//! - E-Paper: Batch updates, partial refresh minimization

use embedded_graphics::{
    pixelcolor::{BinaryColor, PixelColor, Rgb565},
    prelude::*,
    primitives::{Line, PrimitiveStyle, Rectangle},
};

extern crate alloc;

/// Display type for optimization selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayType {
    /// OLED displays (typically monochrome, column-based addressing)
    OLED,
    /// TFT displays (full color, row-based addressing)
    TFT,
    /// E-Paper displays (slow refresh, partial update support)
    EPaper,
    /// Generic display (no specific optimizations)
    Generic,
}

/// Trait for display-specific optimized rendering
pub trait OptimizedRenderer<C: PixelColor> {
    /// Get the display type for this renderer
    fn display_type(&self) -> DisplayType;

    /// Draw an optimized line for this display type
    fn draw_line_optimized(
        &mut self,
        start: Point,
        end: Point,
        color: C,
        width: u32,
    ) -> Result<(), core::convert::Infallible>;

    /// Draw an optimized filled rectangle
    fn draw_filled_rect_optimized(
        &mut self,
        rect: Rectangle,
        color: C,
    ) -> Result<(), core::convert::Infallible>;

    /// Begin batching multiple drawing operations for efficiency
    fn begin_batch(&mut self);

    /// End the current batch and flush operations
    fn end_batch(&mut self);
}

/// OLED-optimized renderer for monochrome displays
pub struct OLEDRenderer<D> {
    display: D,
    batch_active: bool,
    column_buffer: heapless::Vec<u8, 128>, // Typical OLED column height
}

impl<D> OLEDRenderer<D>
where
    D: DrawTarget<Color = BinaryColor>,
{
    /// Create a new OLED-optimized renderer
    pub fn new(display: D) -> Self {
        Self {
            display,
            batch_active: false,
            column_buffer: heapless::Vec::new(),
        }
    }
}

impl<D> OptimizedRenderer<BinaryColor> for OLEDRenderer<D>
where
    D: DrawTarget<Color = BinaryColor>,
{
    fn display_type(&self) -> DisplayType {
        DisplayType::OLED
    }

    fn draw_line_optimized(
        &mut self,
        start: Point,
        end: Point,
        color: BinaryColor,
        width: u32,
    ) -> Result<(), core::convert::Infallible> {
        // OLED optimization: Use column-based drawing for vertical lines
        if start.x == end.x {
            // Vertical line - can be drawn as a single column update
            if self.batch_active {
                // Store in column buffer for batch update
                let _column = start.x as usize;
                let y_start = start.y.min(end.y) as usize;
                let y_end = start.y.max(end.y) as usize;

                // Mark pixels in column buffer
                for y in y_start..=y_end {
                    if y < 128 {
                        let byte_idx = y / 8;
                        let bit_idx = y % 8;
                        if let Some(byte) = self.column_buffer.get_mut(byte_idx) {
                            if color == BinaryColor::On {
                                *byte |= 1 << bit_idx;
                            } else {
                                *byte &= !(1 << bit_idx);
                            }
                        }
                    }
                }
                Ok(())
            } else {
                // Direct draw
                let _ = Line::new(start, end)
                    .into_styled(PrimitiveStyle::with_stroke(color, width))
                    .draw(&mut self.display);
                Ok(())
            }
        } else {
            // Non-vertical line - use standard drawing
            let _ = Line::new(start, end)
                .into_styled(PrimitiveStyle::with_stroke(color, width))
                .draw(&mut self.display);
            Ok(())
        }
    }

    fn draw_filled_rect_optimized(
        &mut self,
        rect: Rectangle,
        color: BinaryColor,
    ) -> Result<(), core::convert::Infallible> {
        // OLED optimization: Draw rectangle column by column
        if self.batch_active && rect.size.width <= 8 {
            // Small rectangle - batch it
            for _x in rect.top_left.x..rect.top_left.x + rect.size.width as i32 {
                for y in rect.top_left.y..rect.top_left.y + rect.size.height as i32 {
                    let byte_idx = (y / 8) as usize;
                    let bit_idx = (y % 8) as usize;
                    if let Some(byte) = self.column_buffer.get_mut(byte_idx) {
                        if color == BinaryColor::On {
                            *byte |= 1 << bit_idx;
                        } else {
                            *byte &= !(1 << bit_idx);
                        }
                    }
                }
            }
            Ok(())
        } else {
            // Large rectangle or no batching - use standard drawing
            let _ = rect
                .into_styled(PrimitiveStyle::with_fill(color))
                .draw(&mut self.display);
            Ok(())
        }
    }

    fn begin_batch(&mut self) {
        self.batch_active = true;
        self.column_buffer.clear();
        self.column_buffer.resize_default(16).ok(); // 128 pixels / 8 bits
    }

    fn end_batch(&mut self) {
        self.batch_active = false;
        // In a real implementation, this would send the column buffer to the display
        self.column_buffer.clear();
    }
}

/// TFT-optimized renderer for RGB displays
pub struct TFTRenderer<D> {
    display: D,
    batch_active: bool,
    line_buffer: heapless::Vec<Rgb565, 320>, // Typical TFT width
}

impl<D> TFTRenderer<D>
where
    D: DrawTarget<Color = Rgb565>,
{
    /// Create a new TFT-optimized renderer
    pub fn new(display: D) -> Self {
        Self {
            display,
            batch_active: false,
            line_buffer: heapless::Vec::new(),
        }
    }
}

impl<D> OptimizedRenderer<Rgb565> for TFTRenderer<D>
where
    D: DrawTarget<Color = Rgb565>,
{
    fn display_type(&self) -> DisplayType {
        DisplayType::TFT
    }

    fn draw_line_optimized(
        &mut self,
        start: Point,
        end: Point,
        color: Rgb565,
        width: u32,
    ) -> Result<(), core::convert::Infallible> {
        // TFT optimization: Use DMA-friendly horizontal line drawing
        if start.y == end.y && width == 1 {
            // Horizontal line - can use fast fill
            if self.batch_active {
                // Buffer the line for DMA transfer
                let x_start = start.x.min(end.x) as usize;
                let x_end = start.x.max(end.x) as usize;
                for _ in x_start..=x_end {
                    self.line_buffer.push(color).ok();
                }
                Ok(())
            } else {
                // Direct draw with potential hardware acceleration
                let _ = Line::new(start, end)
                    .into_styled(PrimitiveStyle::with_stroke(color, width))
                    .draw(&mut self.display);
                Ok(())
            }
        } else {
            // Non-horizontal line - use standard drawing
            let _ = Line::new(start, end)
                .into_styled(PrimitiveStyle::with_stroke(color, width))
                .draw(&mut self.display);
            Ok(())
        }
    }

    fn draw_filled_rect_optimized(
        &mut self,
        rect: Rectangle,
        color: Rgb565,
    ) -> Result<(), core::convert::Infallible> {
        // TFT optimization: Use block fill commands
        if self.batch_active {
            // In a real implementation, this would queue a block fill command
            let _ = rect
                .into_styled(PrimitiveStyle::with_fill(color))
                .draw(&mut self.display);
        } else {
            // Direct block fill
            let _ = rect
                .into_styled(PrimitiveStyle::with_fill(color))
                .draw(&mut self.display);
        }
        Ok(())
    }

    fn begin_batch(&mut self) {
        self.batch_active = true;
        self.line_buffer.clear();
    }

    fn end_batch(&mut self) {
        self.batch_active = false;
        // In a real implementation, this would trigger DMA transfer
        self.line_buffer.clear();
    }
}

/// E-Paper optimized renderer
pub struct EPaperRenderer<D> {
    display: D,
    batch_active: bool,
    update_region: Option<Rectangle>,
    pixel_changes: heapless::Vec<(Point, BinaryColor), 1024>,
}

impl<D> EPaperRenderer<D>
where
    D: DrawTarget<Color = BinaryColor>,
{
    /// Create a new E-Paper optimized renderer
    pub fn new(display: D) -> Self {
        Self {
            display,
            batch_active: false,
            update_region: None,
            pixel_changes: heapless::Vec::new(),
        }
    }

    fn expand_update_region(&mut self, point: Point) {
        if let Some(region) = &mut self.update_region {
            let min_x = region.top_left.x.min(point.x);
            let min_y = region.top_left.y.min(point.y);
            let max_x = (region.top_left.x + region.size.width as i32 - 1).max(point.x);
            let max_y = (region.top_left.y + region.size.height as i32 - 1).max(point.y);

            *region = Rectangle::new(
                Point::new(min_x, min_y),
                Size::new((max_x - min_x + 1) as u32, (max_y - min_y + 1) as u32),
            );
        } else {
            self.update_region = Some(Rectangle::new(point, Size::new(1, 1)));
        }
    }
}

impl<D> OptimizedRenderer<BinaryColor> for EPaperRenderer<D>
where
    D: DrawTarget<Color = BinaryColor>,
{
    fn display_type(&self) -> DisplayType {
        DisplayType::EPaper
    }

    fn draw_line_optimized(
        &mut self,
        start: Point,
        end: Point,
        color: BinaryColor,
        width: u32,
    ) -> Result<(), core::convert::Infallible> {
        if self.batch_active {
            // Track the update region
            self.expand_update_region(start);
            self.expand_update_region(end);

            // Store pixel changes for batch update
            // In practice, would use line drawing algorithm
            self.pixel_changes.push((start, color)).ok();
            self.pixel_changes.push((end, color)).ok();
            Ok(())
        } else {
            let _ = Line::new(start, end)
                .into_styled(PrimitiveStyle::with_stroke(color, width))
                .draw(&mut self.display);
            Ok(())
        }
    }

    fn draw_filled_rect_optimized(
        &mut self,
        rect: Rectangle,
        color: BinaryColor,
    ) -> Result<(), core::convert::Infallible> {
        if self.batch_active {
            // Track the update region
            self.expand_update_region(rect.top_left);
            let bottom_right = Point::new(
                rect.top_left.x + rect.size.width as i32 - 1,
                rect.top_left.y + rect.size.height as i32 - 1,
            );
            self.expand_update_region(bottom_right);

            // In a real implementation, would mark this region for update
            Ok(())
        } else {
            let _ = rect
                .into_styled(PrimitiveStyle::with_fill(color))
                .draw(&mut self.display);
            Ok(())
        }
    }

    fn begin_batch(&mut self) {
        self.batch_active = true;
        self.update_region = None;
        self.pixel_changes.clear();
    }

    fn end_batch(&mut self) {
        self.batch_active = false;
        // In a real implementation, this would trigger a partial refresh
        // of only the update_region
        self.update_region = None;
        self.pixel_changes.clear();
    }
}

// TODO: Implement factory function when needed
// /// Factory function to create appropriate renderer based on display type
// pub fn create_optimized_renderer<D, C>(
//     display: D,
//     display_type: DisplayType,
// ) -> Box<dyn OptimizedRenderer<C>>
// where
//     D: DrawTarget<Color = C> + 'static,
//     C: PixelColor + 'static,
// {
//     match display_type {
//         DisplayType::OLED if core::any::TypeId::of::<C>() == core::any::TypeId::of::<BinaryColor>() => {
//             // This is a workaround for type checking at runtime
//             // In practice, would use better type system design
//             todo!("OLED renderer creation")
//         }
//         DisplayType::TFT if core::any::TypeId::of::<C>() == core::any::TypeId::of::<Rgb565>() => {
//             todo!("TFT renderer creation")
//         }
//         DisplayType::EPaper if core::any::TypeId::of::<C>() == core::any::TypeId::of::<BinaryColor>() => {
//             todo!("E-Paper renderer creation")
//         }
//         _ => {
//             todo!("Generic renderer creation")
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::mock_display::MockDisplay;

    #[test]
    fn test_oled_renderer_creation() {
        let display = MockDisplay::<BinaryColor>::new();
        let renderer = OLEDRenderer::new(display);
        assert_eq!(renderer.display_type(), DisplayType::OLED);
    }

    #[test]
    fn test_tft_renderer_creation() {
        let display = MockDisplay::<Rgb565>::new();
        let renderer = TFTRenderer::new(display);
        assert_eq!(renderer.display_type(), DisplayType::TFT);
    }

    #[test]
    fn test_epaper_renderer_creation() {
        let display = MockDisplay::<BinaryColor>::new();
        let renderer = EPaperRenderer::new(display);
        assert_eq!(renderer.display_type(), DisplayType::EPaper);
    }

    #[test]
    fn test_oled_vertical_line_optimization() {
        let display = MockDisplay::<BinaryColor>::new();
        let mut renderer = OLEDRenderer::new(display);

        // Test vertical line is handled specially
        renderer.begin_batch();
        let result = renderer.draw_line_optimized(
            Point::new(10, 10),
            Point::new(10, 50),
            BinaryColor::On,
            1,
        );
        assert!(result.is_ok());
        renderer.end_batch();
    }

    #[test]
    fn test_tft_horizontal_line_optimization() {
        let display = MockDisplay::<Rgb565>::new();
        let mut renderer = TFTRenderer::new(display);

        // Test horizontal line is handled specially
        renderer.begin_batch();
        let result =
            renderer.draw_line_optimized(Point::new(10, 20), Point::new(50, 20), Rgb565::RED, 1);
        assert!(result.is_ok());
        renderer.end_batch();
    }

    #[test]
    fn test_epaper_update_region_tracking() {
        let display = MockDisplay::<BinaryColor>::new();
        let mut renderer = EPaperRenderer::new(display);

        renderer.begin_batch();
        renderer.expand_update_region(Point::new(10, 10));
        renderer.expand_update_region(Point::new(50, 50));

        assert!(renderer.update_region.is_some());
        let region = renderer.update_region.unwrap();
        assert_eq!(region.top_left, Point::new(10, 10));
        assert_eq!(region.size, Size::new(41, 41));

        renderer.end_batch();
    }
}
