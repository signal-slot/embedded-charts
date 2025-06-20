//! Rendering utilities for chart components.

use crate::error::{RenderError, RenderResult};
use crate::style::{FillStyle, GradientDirection, LineStyle, StrokeStyle};
use embedded_graphics::{
    draw_target::DrawTarget,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
};

/// Main renderer for chart components
pub struct ChartRenderer;

impl ChartRenderer {
    /// Draw a line with the specified style
    pub fn draw_line<C, D>(
        start: Point,
        end: Point,
        style: &LineStyle<C>,
        target: &mut D,
    ) -> RenderResult<()>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
    {
        let primitive_style = PrimitiveStyleBuilder::new()
            .stroke_color(style.color)
            .stroke_width(style.width)
            .build();

        Line::new(start, end)
            .into_styled(primitive_style)
            .draw(target)
            .map_err(|_| RenderError::DrawingFailed)?;

        Ok(())
    }

    /// Draw a series of connected lines (polyline)
    pub fn draw_polyline<C, D>(
        points: &[Point],
        style: &LineStyle<C>,
        target: &mut D,
    ) -> RenderResult<()>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
    {
        if points.len() < 2 {
            return Ok(());
        }

        for window in points.windows(2) {
            if let [p1, p2] = window {
                Self::draw_line(*p1, *p2, style, target)?;
            }
        }

        Ok(())
    }

    /// Draw a filled rectangle
    pub fn draw_filled_rectangle<C, D>(
        rect: Rectangle,
        fill_style: &FillStyle<C>,
        target: &mut D,
    ) -> RenderResult<()>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
    {
        use crate::style::FillPattern;

        match &fill_style.pattern {
            FillPattern::Solid(color) => {
                let primitive_style = PrimitiveStyle::with_fill(*color);
                rect.into_styled(primitive_style)
                    .draw(target)
                    .map_err(|_| RenderError::DrawingFailed)?;
            }
            FillPattern::LinearGradient(gradient) => {
                Self::draw_linear_gradient_rect(rect, gradient, target)?;
            }
            FillPattern::RadialGradient(gradient) => {
                Self::draw_radial_gradient_rect(rect, gradient, target)?;
            }
            FillPattern::Pattern(pattern) => {
                Self::draw_pattern_rect(rect, pattern, target)?;
            }
        }
        Ok(())
    }

    /// Draw a rectangle with stroke and optional fill
    pub fn draw_rectangle<C, D>(
        rect: Rectangle,
        stroke_style: Option<&StrokeStyle<C>>,
        fill_style: Option<&FillStyle<C>>,
        target: &mut D,
    ) -> RenderResult<()>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
    {
        let mut style_builder = PrimitiveStyleBuilder::new();

        if let Some(fill) = fill_style {
            if let Some(color) = fill.solid_color() {
                style_builder = style_builder.fill_color(color);
            }
        }

        if let Some(stroke) = stroke_style {
            style_builder = style_builder
                .stroke_color(stroke.color)
                .stroke_width(stroke.width);
        }

        rect.into_styled(style_builder.build())
            .draw(target)
            .map_err(|_| RenderError::DrawingFailed)?;

        Ok(())
    }

    /// Draw a circle
    pub fn draw_circle<C, D>(
        center: Point,
        radius: u32,
        stroke_style: Option<&StrokeStyle<C>>,
        fill_style: Option<&FillStyle<C>>,
        target: &mut D,
    ) -> RenderResult<()>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
    {
        let mut style_builder = PrimitiveStyleBuilder::new();

        if let Some(fill) = fill_style {
            if let Some(color) = fill.solid_color() {
                style_builder = style_builder.fill_color(color);
            }
        }

        if let Some(stroke) = stroke_style {
            style_builder = style_builder
                .stroke_color(stroke.color)
                .stroke_width(stroke.width);
        }

        let circle = Circle::new(
            Point::new(center.x - radius as i32, center.y - radius as i32),
            radius * 2,
        );

        circle
            .into_styled(style_builder.build())
            .draw(target)
            .map_err(|_| RenderError::DrawingFailed)?;

        Ok(())
    }

    /// Draw a grid
    pub fn draw_grid<C, D>(
        area: Rectangle,
        grid_spacing: Size,
        style: &LineStyle<C>,
        target: &mut D,
    ) -> RenderResult<()>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
    {
        // Draw vertical lines
        let mut x = area.top_left.x;
        while x <= area.top_left.x + area.size.width as i32 {
            let start = Point::new(x, area.top_left.y);
            let end = Point::new(x, area.top_left.y + area.size.height as i32);
            Self::draw_line(start, end, style, target)?;
            x += grid_spacing.width as i32;
        }

        // Draw horizontal lines
        let mut y = area.top_left.y;
        while y <= area.top_left.y + area.size.height as i32 {
            let start = Point::new(area.top_left.x, y);
            let end = Point::new(area.top_left.x + area.size.width as i32, y);
            Self::draw_line(start, end, style, target)?;
            y += grid_spacing.height as i32;
        }

        Ok(())
    }

    /// Clear an area with a background color
    pub fn clear_area<C, D>(area: Rectangle, color: C, target: &mut D) -> RenderResult<()>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
    {
        let fill_style = FillStyle::solid(color);
        Self::draw_filled_rectangle(area, &fill_style, target)
    }

    /// Draw a rectangle filled with a linear gradient
    fn draw_linear_gradient_rect<C, D, const N: usize>(
        rect: Rectangle,
        gradient: &crate::style::LinearGradient<C, N>,
        target: &mut D,
    ) -> RenderResult<()>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
    {
        if !gradient.is_valid() {
            return Ok(());
        }

        match gradient.direction() {
            GradientDirection::Horizontal => {
                // Draw vertical lines with interpolated colors
                for x in 0..rect.size.width {
                    let t = x as f32 / (rect.size.width - 1) as f32;
                    if let Some(color) = gradient.color_at(t) {
                        let line_start = Point::new(rect.top_left.x + x as i32, rect.top_left.y);
                        let line_end = Point::new(
                            rect.top_left.x + x as i32,
                            rect.top_left.y + rect.size.height as i32 - 1,
                        );
                        Line::new(line_start, line_end)
                            .into_styled(PrimitiveStyle::with_stroke(color, 1))
                            .draw(target)
                            .map_err(|_| RenderError::DrawingFailed)?;
                    }
                }
            }
            GradientDirection::Vertical => {
                // Draw horizontal lines with interpolated colors
                for y in 0..rect.size.height {
                    let t = y as f32 / (rect.size.height - 1) as f32;
                    if let Some(color) = gradient.color_at(t) {
                        Self::draw_horizontal_line(
                            Point::new(rect.top_left.x, rect.top_left.y + y as i32),
                            rect.size.width,
                            color,
                            target,
                        )?;
                    }
                }
            }
            GradientDirection::Diagonal | GradientDirection::ReverseDiagonal => {
                // Use the same diagonal line approach as the optimized version
                let total = rect.size.width + rect.size.height - 2;
                let step_size = if total > 100 { 2 } else { 1 }; // Skip pixels for large gradients

                for y in (0..rect.size.height).step_by(step_size as usize) {
                    for x in (0..rect.size.width).step_by(step_size as usize) {
                        let t = if gradient.direction() == GradientDirection::Diagonal {
                            (x + y) as f32 / total as f32
                        } else {
                            (rect.size.width - 1 - x + y) as f32 / total as f32
                        };

                        if let Some(color) = gradient.color_at(t) {
                            // Draw a small filled rectangle instead of single pixel
                            let pixel_rect = Rectangle::new(
                                Point::new(rect.top_left.x + x as i32, rect.top_left.y + y as i32),
                                Size::new(step_size, step_size),
                            );
                            pixel_rect
                                .into_styled(PrimitiveStyle::with_fill(color))
                                .draw(target)
                                .map_err(|_| RenderError::DrawingFailed)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Draw a rectangle filled with a radial gradient
    fn draw_radial_gradient_rect<C, D, const N: usize>(
        rect: Rectangle,
        gradient: &crate::style::RadialGradient<C, N>,
        target: &mut D,
    ) -> RenderResult<()>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
    {
        if !gradient.is_valid() {
            return Ok(());
        }

        let center = gradient.center();
        let center_x = rect.top_left.x + (rect.size.width as i32 * center.x / 100);
        let center_y = rect.top_left.y + (rect.size.height as i32 * center.y / 100);

        // Calculate maximum distance from center to corners
        let max_dist = {
            let dx1 = (rect.top_left.x - center_x).abs();
            let dx2 = (rect.top_left.x + rect.size.width as i32 - center_x).abs();
            let dy1 = (rect.top_left.y - center_y).abs();
            let dy2 = (rect.top_left.y + rect.size.height as i32 - center_y).abs();
            let max_dx = dx1.max(dx2) as f32;
            let max_dy = dy1.max(dy2) as f32;
            (max_dx * max_dx + max_dy * max_dy).sqrt()
        };

        // Draw each pixel with color based on distance from center
        for y in 0..rect.size.height {
            for x in 0..rect.size.width {
                let px = rect.top_left.x + x as i32;
                let py = rect.top_left.y + y as i32;
                let dx = (px - center_x) as f32;
                let dy = (py - center_y) as f32;
                let dist = (dx * dx + dy * dy).sqrt();
                let t = (dist / max_dist).clamp(0.0, 1.0);

                if let Some(color) = gradient.color_at_distance(t) {
                    Pixel(Point::new(px, py), color)
                        .draw(target)
                        .map_err(|_| RenderError::DrawingFailed)?;
                }
            }
        }
        Ok(())
    }

    /// Draw a rectangle filled with a pattern
    fn draw_pattern_rect<C, D>(
        rect: Rectangle,
        pattern: &crate::style::PatternFill<C>,
        target: &mut D,
    ) -> RenderResult<()>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
    {
        // Draw each pixel with pattern color
        for y in 0..rect.size.height {
            for x in 0..rect.size.width {
                let color = pattern.color_at(x as i32, y as i32);
                Pixel(
                    Point::new(rect.top_left.x + x as i32, rect.top_left.y + y as i32),
                    color,
                )
                .draw(target)
                .map_err(|_| RenderError::DrawingFailed)?;
            }
        }
        Ok(())
    }

    /// Draw a horizontal line (optimized for gradient rendering)
    fn draw_horizontal_line<C, D>(
        start: Point,
        width: u32,
        color: C,
        target: &mut D,
    ) -> RenderResult<()>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
    {
        Line::new(start, Point::new(start.x + width as i32 - 1, start.y))
            .into_styled(PrimitiveStyle::with_stroke(color, 1))
            .draw(target)
            .map_err(|_| RenderError::DrawingFailed)?;
        Ok(())
    }

    /// Draw a rectangle filled with a linear gradient (Rgb565 optimized version)
    #[cfg(feature = "color-support")]
    pub fn draw_linear_gradient_rect_rgb565<D, const N: usize>(
        rect: Rectangle,
        gradient: &crate::style::LinearGradient<embedded_graphics::pixelcolor::Rgb565, N>,
        target: &mut D,
    ) -> RenderResult<()>
    where
        D: DrawTarget<Color = embedded_graphics::pixelcolor::Rgb565>,
    {
        use crate::style::GradientInterpolation;

        if !gradient.is_valid() {
            return Ok(());
        }

        match gradient.direction() {
            GradientDirection::Horizontal => {
                // Draw vertical lines with interpolated colors
                for x in 0..rect.size.width {
                    let t = x as f32 / (rect.size.width - 1) as f32;
                    if let Some(color) = gradient.interpolated_color_at(t) {
                        let line_start = Point::new(rect.top_left.x + x as i32, rect.top_left.y);
                        let line_end = Point::new(
                            rect.top_left.x + x as i32,
                            rect.top_left.y + rect.size.height as i32 - 1,
                        );
                        Line::new(line_start, line_end)
                            .into_styled(PrimitiveStyle::with_stroke(color, 1))
                            .draw(target)
                            .map_err(|_| RenderError::DrawingFailed)?;
                    }
                }
            }
            GradientDirection::Vertical => {
                // Draw horizontal lines with interpolated colors
                for y in 0..rect.size.height {
                    let t = y as f32 / (rect.size.height - 1) as f32;
                    if let Some(color) = gradient.interpolated_color_at(t) {
                        Self::draw_horizontal_line(
                            Point::new(rect.top_left.x, rect.top_left.y + y as i32),
                            rect.size.width,
                            color,
                            target,
                        )?;
                    }
                }
            }
            GradientDirection::Diagonal | GradientDirection::ReverseDiagonal => {
                // Draw diagonal gradient using small rectangles
                let step = 3; // Size of each rectangle

                for y in (0..rect.size.height).step_by(step) {
                    for x in (0..rect.size.width).step_by(step) {
                        // Calculate position along diagonal
                        let t = if gradient.direction() == GradientDirection::Diagonal {
                            (x + y) as f32 / (rect.size.width + rect.size.height - 2) as f32
                        } else {
                            (rect.size.width - 1 - x + y) as f32
                                / (rect.size.width + rect.size.height - 2) as f32
                        };

                        if let Some(color) = gradient.interpolated_color_at(t) {
                            Rectangle::new(
                                Point::new(rect.top_left.x + x as i32, rect.top_left.y + y as i32),
                                Size::new(step as u32, step as u32),
                            )
                            .into_styled(PrimitiveStyle::with_fill(color))
                            .draw(target)
                            .map_err(|_| RenderError::DrawingFailed)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Draw a rectangle filled with a radial gradient (Rgb565 optimized version)
    #[cfg(feature = "color-support")]
    pub fn draw_radial_gradient_rect_rgb565<D, const N: usize>(
        rect: Rectangle,
        gradient: &crate::style::RadialGradient<embedded_graphics::pixelcolor::Rgb565, N>,
        target: &mut D,
    ) -> RenderResult<()>
    where
        D: DrawTarget<Color = embedded_graphics::pixelcolor::Rgb565>,
    {
        use crate::style::RadialGradientInterpolation;

        if !gradient.is_valid() {
            return Ok(());
        }

        let center = gradient.center();
        let center_x = rect.top_left.x + (rect.size.width as i32 * center.x / 100);
        let center_y = rect.top_left.y + (rect.size.height as i32 * center.y / 100);

        // Calculate maximum distance from center to corners
        let max_dist = {
            let dx1 = (rect.top_left.x - center_x).abs();
            let dx2 = (rect.top_left.x + rect.size.width as i32 - center_x).abs();
            let dy1 = (rect.top_left.y - center_y).abs();
            let dy2 = (rect.top_left.y + rect.size.height as i32 - center_y).abs();
            let max_dx = dx1.max(dx2) as f32;
            let max_dy = dy1.max(dy2) as f32;
            (max_dx * max_dx + max_dy * max_dy).sqrt()
        };

        // Draw radial gradient using filled rectangles for better performance
        // We'll use a lower resolution for speed
        let step_size = 3;

        for y in (0..rect.size.height).step_by(step_size) {
            for x in (0..rect.size.width).step_by(step_size) {
                let px = rect.top_left.x + x as i32;
                let py = rect.top_left.y + y as i32;
                let dx = (px - center_x) as f32;
                let dy = (py - center_y) as f32;
                let dist = (dx * dx + dy * dy).sqrt();
                let t = (dist / max_dist).clamp(0.0, 1.0);

                if let Some(color) = gradient.interpolated_color_at_distance(t) {
                    Rectangle::new(
                        Point::new(px, py),
                        Size::new(step_size as u32, step_size as u32),
                    )
                    .into_styled(PrimitiveStyle::with_fill(color))
                    .draw(target)
                    .map_err(|_| RenderError::DrawingFailed)?;
                }
            }
        }
        Ok(())
    }
}

/// Clipping utilities for efficient rendering
pub struct ClippingRenderer;

impl ClippingRenderer {
    /// Check if a point is within the clipping bounds
    pub fn is_point_visible(point: Point, bounds: Rectangle) -> bool {
        point.x >= bounds.top_left.x
            && point.x < bounds.top_left.x + bounds.size.width as i32
            && point.y >= bounds.top_left.y
            && point.y < bounds.top_left.y + bounds.size.height as i32
    }

    /// Check if a rectangle intersects with the clipping bounds
    pub fn is_rectangle_visible(rect: Rectangle, bounds: Rectangle) -> bool {
        !(rect.top_left.x >= bounds.top_left.x + bounds.size.width as i32
            || rect.top_left.x + rect.size.width as i32 <= bounds.top_left.x
            || rect.top_left.y >= bounds.top_left.y + bounds.size.height as i32
            || rect.top_left.y + rect.size.height as i32 <= bounds.top_left.y)
    }

    /// Clip a line to the bounds (simplified Cohen-Sutherland algorithm)
    pub fn clip_line(start: Point, end: Point, bounds: Rectangle) -> Option<(Point, Point)> {
        let mut x1 = start.x;
        let mut y1 = start.y;
        let mut x2 = end.x;
        let mut y2 = end.y;

        let xmin = bounds.top_left.x;
        let ymin = bounds.top_left.y;
        let xmax = bounds.top_left.x + bounds.size.width as i32;
        let ymax = bounds.top_left.y + bounds.size.height as i32;

        // Outcodes for the endpoints
        let mut outcode1 = Self::compute_outcode(x1, y1, xmin, ymin, xmax, ymax);
        let mut outcode2 = Self::compute_outcode(x2, y2, xmin, ymin, xmax, ymax);

        loop {
            if (outcode1 | outcode2) == 0 {
                // Both points inside
                return Some((Point::new(x1, y1), Point::new(x2, y2)));
            } else if (outcode1 & outcode2) != 0 {
                // Both points outside same region
                return None;
            } else {
                // Line needs clipping
                let outcode_out = if outcode1 != 0 { outcode1 } else { outcode2 };

                let (x, y) = if (outcode_out & 8) != 0 {
                    // Point is above
                    let x = x1 + (x2 - x1) * (ymax - y1) / (y2 - y1);
                    (x, ymax)
                } else if (outcode_out & 4) != 0 {
                    // Point is below
                    let x = x1 + (x2 - x1) * (ymin - y1) / (y2 - y1);
                    (x, ymin)
                } else if (outcode_out & 2) != 0 {
                    // Point is to the right
                    let y = y1 + (y2 - y1) * (xmax - x1) / (x2 - x1);
                    (xmax, y)
                } else {
                    // Point is to the left
                    let y = y1 + (y2 - y1) * (xmin - x1) / (x2 - x1);
                    (xmin, y)
                };

                if outcode_out == outcode1 {
                    x1 = x;
                    y1 = y;
                    outcode1 = Self::compute_outcode(x1, y1, xmin, ymin, xmax, ymax);
                } else {
                    x2 = x;
                    y2 = y;
                    outcode2 = Self::compute_outcode(x2, y2, xmin, ymin, xmax, ymax);
                }
            }
        }
    }

    /// Compute outcode for Cohen-Sutherland clipping
    fn compute_outcode(x: i32, y: i32, xmin: i32, ymin: i32, xmax: i32, ymax: i32) -> u8 {
        let mut code = 0;

        if x < xmin {
            code |= 1; // Left
        } else if x > xmax {
            code |= 2; // Right
        }

        if y < ymin {
            code |= 4; // Below
        } else if y > ymax {
            code |= 8; // Above
        }

        code
    }
}

/// Text rendering utilities (when fonts feature is enabled)
pub mod text {
    use super::*;
    use embedded_graphics::mono_font::{MonoFont, MonoTextStyle};
    use embedded_graphics::text::{Baseline, Text};

    /// Text renderer for chart labels and titles
    pub struct TextRenderer;

    impl TextRenderer {
        /// Draw text at the specified position
        pub fn draw_text<C, D>(
            text: &str,
            position: Point,
            style: &MonoTextStyle<C>,
            target: &mut D,
        ) -> RenderResult<()>
        where
            C: PixelColor,
            D: DrawTarget<Color = C>,
        {
            Text::with_baseline(text, position, *style, Baseline::Top)
                .draw(target)
                .map_err(|_| RenderError::TextRenderingFailed)?;

            Ok(())
        }

        /// Calculate the size of text when rendered
        pub fn text_size<C>(text: &str, font: &MonoFont) -> Size {
            let char_size = font.character_size;
            Size::new(char_size.width * text.len() as u32, char_size.height)
        }

        /// Draw centered text within a rectangle
        pub fn draw_centered_text<C, D>(
            text: &str,
            container: Rectangle,
            style: &MonoTextStyle<C>,
            font: &MonoFont,
            target: &mut D,
        ) -> RenderResult<()>
        where
            C: PixelColor,
            D: DrawTarget<Color = C>,
        {
            let text_size = Self::text_size::<C>(text, font);
            let x =
                container.top_left.x + (container.size.width as i32 - text_size.width as i32) / 2;
            let y =
                container.top_left.y + (container.size.height as i32 - text_size.height as i32) / 2;

            Self::draw_text(text, Point::new(x, y), style, target)
        }
    }
}

/// Primitive drawing utilities for custom shapes
pub struct PrimitiveRenderer;

impl PrimitiveRenderer {
    /// Draw a triangle
    pub fn draw_triangle<C, D>(
        p1: Point,
        p2: Point,
        p3: Point,
        stroke_style: Option<&StrokeStyle<C>>,
        fill_style: Option<&FillStyle<C>>,
        target: &mut D,
    ) -> RenderResult<()>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
    {
        // For simplicity, draw triangle as three lines
        // A full implementation would use a proper triangle primitive
        if let Some(stroke) = stroke_style {
            let line_style = LineStyle::solid(stroke.color).width(stroke.width);
            ChartRenderer::draw_line(p1, p2, &line_style, target)?;
            ChartRenderer::draw_line(p2, p3, &line_style, target)?;
            ChartRenderer::draw_line(p3, p1, &line_style, target)?;
        }

        // Fill triangle using scanline algorithm
        if let Some(fill) = fill_style {
            Self::fill_triangle(p1, p2, p3, fill, target)?;
        }

        Ok(())
    }

    /// Fill a triangle using scanline algorithm
    fn fill_triangle<C, D>(
        p1: Point,
        p2: Point,
        p3: Point,
        fill_style: &FillStyle<C>,
        target: &mut D,
    ) -> RenderResult<()>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
    {
        // Sort points by Y coordinate (p1.y <= p2.y <= p3.y)
        let mut points = [p1, p2, p3];
        // Manual sorting for no_std compatibility
        if points[0].y > points[1].y {
            points.swap(0, 1);
        }
        if points[1].y > points[2].y {
            points.swap(1, 2);
        }
        if points[0].y > points[1].y {
            points.swap(0, 1);
        }
        let [top, mid, bottom] = points;

        // Handle degenerate cases
        if top.y == bottom.y {
            // All points on same horizontal line
            let min_x = top.x.min(mid.x).min(bottom.x);
            let max_x = top.x.max(mid.x).max(bottom.x);
            if let Some(color) = fill_style.solid_color() {
                Self::draw_horizontal_line(min_x, max_x, top.y, color, target)?;
            }
            return Ok(());
        }

        // Calculate slopes for the three edges
        let total_height = bottom.y - top.y;

        // Draw upper part of triangle (from top to mid)
        if mid.y > top.y {
            let segment_height = mid.y - top.y;
            for y in top.y..mid.y {
                let alpha = (y - top.y) as f32 / total_height as f32;
                let beta = (y - top.y) as f32 / segment_height as f32;

                let x1 = top.x + ((bottom.x - top.x) as f32 * alpha) as i32;
                let x2 = top.x + ((mid.x - top.x) as f32 * beta) as i32;

                let (min_x, max_x) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
                if let Some(color) = fill_style.solid_color() {
                    Self::draw_horizontal_line(min_x, max_x, y, color, target)?;
                }
            }
        }

        // Draw lower part of triangle (from mid to bottom)
        if bottom.y > mid.y {
            let segment_height = bottom.y - mid.y;
            for y in mid.y..=bottom.y {
                let alpha = (y - top.y) as f32 / total_height as f32;
                let beta = (y - mid.y) as f32 / segment_height as f32;

                let x1 = top.x + ((bottom.x - top.x) as f32 * alpha) as i32;
                let x2 = mid.x + ((bottom.x - mid.x) as f32 * beta) as i32;

                let (min_x, max_x) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
                if let Some(color) = fill_style.solid_color() {
                    Self::draw_horizontal_line(min_x, max_x, y, color, target)?;
                }
            }
        }

        Ok(())
    }

    /// Draw a horizontal line from x1 to x2 at y coordinate
    fn draw_horizontal_line<C, D>(
        x1: i32,
        x2: i32,
        y: i32,
        color: C,
        target: &mut D,
    ) -> RenderResult<()>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
    {
        if x1 == x2 {
            // Single pixel
            target
                .draw_iter(core::iter::once(Pixel(Point::new(x1, y), color)))
                .map_err(|_| RenderError::DrawingFailed)?;
        } else {
            // Horizontal line
            let line_style = PrimitiveStyle::with_stroke(color, 1);
            Line::new(Point::new(x1, y), Point::new(x2, y))
                .into_styled(line_style)
                .draw(target)
                .map_err(|_| RenderError::DrawingFailed)?;
        }
        Ok(())
    }

    /// Draw a diamond shape
    pub fn draw_diamond<C, D>(
        center: Point,
        size: u32,
        stroke_style: Option<&StrokeStyle<C>>,
        fill_style: Option<&FillStyle<C>>,
        target: &mut D,
    ) -> RenderResult<()>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
    {
        let half_size = size as i32 / 2;
        let top = Point::new(center.x, center.y - half_size);
        let right = Point::new(center.x + half_size, center.y);
        let bottom = Point::new(center.x, center.y + half_size);
        let left = Point::new(center.x - half_size, center.y);

        Self::draw_triangle(top, right, bottom, stroke_style, fill_style, target)?;
        Self::draw_triangle(top, bottom, left, stroke_style, fill_style, target)?;

        Ok(())
    }
}

/// Animation frame renderer for coordinating animated chart rendering
#[cfg(feature = "animations")]
pub struct AnimationFrameRenderer {
    /// Target frame rate
    frame_rate: u32,
    /// Time accumulator for frame timing
    time_accumulator: crate::time::Milliseconds,
    /// Last frame timestamp
    last_frame_time: Option<crate::time::Milliseconds>,
}

#[cfg(feature = "animations")]
impl AnimationFrameRenderer {
    /// Create a new animation frame renderer
    pub fn new(frame_rate: u32) -> Self {
        Self {
            frame_rate: frame_rate.clamp(1, 120),
            time_accumulator: 0,
            last_frame_time: None,
        }
    }

    /// Update the frame renderer with elapsed time
    pub fn update(&mut self, current_time: crate::time::Milliseconds) -> bool {
        let frame_duration = 1000 / self.frame_rate;

        if let Some(last_time) = self.last_frame_time {
            let delta = current_time.saturating_sub(last_time);
            self.time_accumulator += delta;
        }

        self.last_frame_time = Some(current_time);

        if self.time_accumulator >= frame_duration {
            self.time_accumulator = self.time_accumulator.saturating_sub(frame_duration);
            true // Frame should be rendered
        } else {
            false
        }
    }

    /// Get the current frame rate
    pub fn frame_rate(&self) -> u32 {
        self.frame_rate
    }

    /// Set the target frame rate
    pub fn set_frame_rate(&mut self, fps: u32) {
        self.frame_rate = fps.clamp(1, 120);
    }

    /// Reset the frame timing
    pub fn reset(&mut self) {
        self.time_accumulator = 0;
        self.last_frame_time = None;
    }

    /// Render an animated chart frame
    pub fn render_animated_chart<C, D, T>(
        &self,
        chart: &T,
        data: &T::Data,
        config: &T::Config,
        viewport: embedded_graphics::primitives::Rectangle,
        target: &mut D,
    ) -> crate::error::RenderResult<()>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
        T: crate::chart::traits::AnimatedChart<C> + crate::chart::traits::AnimationRenderer<C>,
    {
        if chart.needs_frame_update() {
            chart
                .draw_animated(data, config, viewport, target, 0)
                .map_err(|_| crate::error::RenderError::DrawingFailed)?;
        }
        Ok(())
    }

    /// Check if any charts need frame updates
    pub fn charts_need_update<C>(
        &self,
        charts: &[&dyn crate::chart::traits::AnimationRenderer<C>],
    ) -> bool
    where
        C: PixelColor,
    {
        charts.iter().any(|chart| chart.needs_frame_update())
    }
}

#[cfg(feature = "animations")]
impl Default for AnimationFrameRenderer {
    fn default() -> Self {
        Self::new(60) // Default 60 FPS
    }
}

/// Enhanced chart renderer with animation support
pub struct EnhancedChartRenderer;

impl EnhancedChartRenderer {
    /// Render a chart with optional animation support
    pub fn render_chart<C, D, T>(
        chart: &T,
        data: &T::Data,
        config: &T::Config,
        viewport: embedded_graphics::primitives::Rectangle,
        target: &mut D,
    ) -> crate::error::RenderResult<()>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
        T: crate::chart::traits::Chart<C>,
        T::Data: crate::data::DataSeries,
        <T::Data as crate::data::DataSeries>::Item: crate::data::DataPoint,
        <<T::Data as crate::data::DataSeries>::Item as crate::data::DataPoint>::X:
            Into<f32> + Copy + PartialOrd,
        <<T::Data as crate::data::DataSeries>::Item as crate::data::DataPoint>::Y:
            Into<f32> + Copy + PartialOrd,
    {
        chart
            .draw(data, config, viewport, target)
            .map_err(|_| crate::error::RenderError::DrawingFailed)
    }

    /// Render an animated chart
    #[cfg(feature = "animations")]
    pub fn render_animated_chart<C, D, T>(
        chart: &T,
        data: &T::Data,
        config: &T::Config,
        viewport: embedded_graphics::primitives::Rectangle,
        target: &mut D,
    ) -> crate::error::RenderResult<()>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
        T: crate::chart::traits::AnimatedChart<C>,
    {
        chart
            .draw_animated(data, config, viewport, target, 0)
            .map_err(|_| crate::error::RenderError::DrawingFailed)
    }

    /// Update and render an animated chart with timing
    #[cfg(feature = "animations")]
    pub fn update_and_render<C, D, T>(
        chart: &mut T,
        data: &T::Data,
        _delta_time: crate::time::Milliseconds,
        config: &T::Config,
        viewport: embedded_graphics::primitives::Rectangle,
        target: &mut D,
    ) -> crate::error::RenderResult<()>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
        T: crate::chart::traits::AnimatedChart<C>,
    {
        // Render the animated frame (animation state is controlled externally)
        Self::render_animated_chart(chart, data, config, viewport, target)
    }

    /// Clear a rectangular area with a background color
    pub fn clear_viewport<C, D>(
        viewport: embedded_graphics::primitives::Rectangle,
        color: C,
        target: &mut D,
    ) -> crate::error::RenderResult<()>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
    {
        ChartRenderer::clear_area(viewport, color, target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::mock_display::MockDisplay;
    use embedded_graphics::pixelcolor::Rgb565;

    #[test]
    fn test_clipping_point_visibility() {
        let bounds = Rectangle::new(Point::new(10, 10), Size::new(100, 80));

        assert!(ClippingRenderer::is_point_visible(
            Point::new(50, 50),
            bounds
        ));
        assert!(!ClippingRenderer::is_point_visible(
            Point::new(5, 50),
            bounds
        ));
        assert!(!ClippingRenderer::is_point_visible(
            Point::new(150, 50),
            bounds
        ));
    }

    #[test]
    fn test_clipping_rectangle_visibility() {
        let bounds = Rectangle::new(Point::new(0, 0), Size::new(100, 100));

        // Completely inside
        let inside = Rectangle::new(Point::new(10, 10), Size::new(20, 20));
        assert!(ClippingRenderer::is_rectangle_visible(inside, bounds));

        // Completely outside
        let outside = Rectangle::new(Point::new(150, 150), Size::new(20, 20));
        assert!(!ClippingRenderer::is_rectangle_visible(outside, bounds));

        // Partially overlapping
        let overlapping = Rectangle::new(Point::new(90, 90), Size::new(20, 20));
        assert!(ClippingRenderer::is_rectangle_visible(overlapping, bounds));
    }

    #[test]
    fn test_line_clipping() {
        let bounds = Rectangle::new(Point::new(0, 0), Size::new(100, 100));

        // Line completely inside
        let inside_line =
            ClippingRenderer::clip_line(Point::new(10, 10), Point::new(50, 50), bounds);
        assert!(inside_line.is_some());

        // Line completely outside
        let outside_line =
            ClippingRenderer::clip_line(Point::new(150, 150), Point::new(200, 200), bounds);
        assert!(outside_line.is_none());
    }

    #[test]
    fn test_chart_renderer_line() {
        let mut display = MockDisplay::<Rgb565>::new();
        let style = LineStyle::solid(Rgb565::RED).width(1);

        let result =
            ChartRenderer::draw_line(Point::new(0, 0), Point::new(10, 10), &style, &mut display);

        assert!(result.is_ok());
    }

    #[test]
    fn test_chart_renderer_rectangle() {
        let mut display = MockDisplay::<Rgb565>::new();
        let stroke = StrokeStyle::new(Rgb565::BLUE, 1);
        let fill = FillStyle::solid(Rgb565::GREEN);

        let rect = Rectangle::new(Point::new(5, 5), Size::new(20, 15));
        let result = ChartRenderer::draw_rectangle(rect, Some(&stroke), Some(&fill), &mut display);

        assert!(result.is_ok());
    }
}
