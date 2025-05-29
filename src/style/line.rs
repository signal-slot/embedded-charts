//! Line styling utilities for charts.

use embedded_graphics::prelude::*;

/// Line style configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineStyle<C: PixelColor> {
    /// Color of the line
    pub color: C,
    /// Width of the line in pixels
    pub width: u32,
    /// Line pattern
    pub pattern: LinePattern,
    /// Line cap style
    pub cap: LineCap,
    /// Line join style
    pub join: LineJoin,
}

/// Line pattern types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinePattern {
    /// Solid line
    Solid,
    /// Dashed line
    Dashed,
    /// Dotted line
    Dotted,
    /// Dash-dot pattern
    DashDot,
    /// Custom pattern (not implemented in basic version)
    Custom,
}

/// Line cap styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineCap {
    /// Flat cap
    Butt,
    /// Rounded cap
    Round,
    /// Square cap
    Square,
}

/// Line join styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineJoin {
    /// Miter join
    Miter,
    /// Round join
    Round,
    /// Bevel join
    Bevel,
}

impl<C: PixelColor> LineStyle<C> {
    /// Create a new solid line style
    pub const fn solid(color: C) -> Self {
        Self {
            color,
            width: 1,
            pattern: LinePattern::Solid,
            cap: LineCap::Butt,
            join: LineJoin::Miter,
        }
    }

    /// Create a new dashed line style
    pub const fn dashed(color: C) -> Self {
        Self {
            color,
            width: 1,
            pattern: LinePattern::Dashed,
            cap: LineCap::Butt,
            join: LineJoin::Miter,
        }
    }

    /// Create a new dotted line style
    pub const fn dotted(color: C) -> Self {
        Self {
            color,
            width: 1,
            pattern: LinePattern::Dotted,
            cap: LineCap::Round,
            join: LineJoin::Round,
        }
    }

    /// Set the line width
    pub const fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    /// Set the line color
    pub const fn color(mut self, color: C) -> Self {
        self.color = color;
        self
    }

    /// Set the line pattern
    pub const fn pattern(mut self, pattern: LinePattern) -> Self {
        self.pattern = pattern;
        self
    }

    /// Set the line cap style
    pub const fn cap(mut self, cap: LineCap) -> Self {
        self.cap = cap;
        self
    }

    /// Set the line join style
    pub const fn join(mut self, join: LineJoin) -> Self {
        self.join = join;
        self
    }
}

impl<C: PixelColor> Default for LineStyle<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::solid(embedded_graphics::pixelcolor::Rgb565::WHITE.into())
    }
}

/// Border style for chart elements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BorderStyle<C: PixelColor> {
    /// Line style for the border
    pub line: LineStyle<C>,
    /// Border radius (for rounded corners)
    pub radius: u32,
    /// Whether the border is visible
    pub visible: bool,
}

impl<C: PixelColor> BorderStyle<C> {
    /// Create a new border style
    pub const fn new(line: LineStyle<C>) -> Self {
        Self {
            line,
            radius: 0,
            visible: true,
        }
    }

    /// Create a border with rounded corners
    pub const fn rounded(line: LineStyle<C>, radius: u32) -> Self {
        Self {
            line,
            radius,
            visible: true,
        }
    }

    /// Set the border radius
    pub const fn radius(mut self, radius: u32) -> Self {
        self.radius = radius;
        self
    }

    /// Set border visibility
    pub const fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
}

impl<C: PixelColor> Default for BorderStyle<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::new(LineStyle::default())
    }
}

/// Stroke style for drawing operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StrokeStyle<C: PixelColor> {
    /// Color of the stroke
    pub color: C,
    /// Width of the stroke
    pub width: u32,
}

impl<C: PixelColor> StrokeStyle<C> {
    /// Create a new stroke style
    pub const fn new(color: C, width: u32) -> Self {
        Self { color, width }
    }
}

impl<C: PixelColor> From<LineStyle<C>> for StrokeStyle<C> {
    fn from(line_style: LineStyle<C>) -> Self {
        Self {
            color: line_style.color,
            width: line_style.width,
        }
    }
}

/// Fill style for drawing operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FillStyle<C: PixelColor> {
    /// Fill color
    pub color: C,
    /// Fill pattern (for future use)
    pub pattern: FillPattern,
}

/// Fill pattern types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FillPattern {
    /// Solid fill
    Solid,
    /// Gradient fill (not implemented in basic version)
    Gradient,
    /// Pattern fill (not implemented in basic version)
    Pattern,
}

impl<C: PixelColor> FillStyle<C> {
    /// Create a solid fill style
    pub const fn solid(color: C) -> Self {
        Self {
            color,
            pattern: FillPattern::Solid,
        }
    }
}

impl<C: PixelColor> Default for FillStyle<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::solid(embedded_graphics::pixelcolor::Rgb565::WHITE.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::pixelcolor::Rgb565;

    #[test]
    fn test_line_style_creation() {
        let style = LineStyle::solid(Rgb565::RED);
        assert_eq!(style.color, Rgb565::RED);
        assert_eq!(style.width, 1);
        assert_eq!(style.pattern, LinePattern::Solid);
    }

    #[test]
    fn test_line_style_builder() {
        let style = LineStyle::solid(Rgb565::BLUE)
            .width(3)
            .pattern(LinePattern::Dashed)
            .cap(LineCap::Round);

        assert_eq!(style.color, Rgb565::BLUE);
        assert_eq!(style.width, 3);
        assert_eq!(style.pattern, LinePattern::Dashed);
        assert_eq!(style.cap, LineCap::Round);
    }

    #[test]
    fn test_border_style() {
        let line = LineStyle::solid(Rgb565::BLACK);
        let border = BorderStyle::rounded(line, 5);

        assert_eq!(border.radius, 5);
        assert!(border.visible);
        assert_eq!(border.line.color, Rgb565::BLACK);
    }

    #[test]
    fn test_stroke_style_from_line_style() {
        let line_style = LineStyle::solid(Rgb565::GREEN).width(2);
        let stroke_style: StrokeStyle<Rgb565> = line_style.into();

        assert_eq!(stroke_style.color, Rgb565::GREEN);
        assert_eq!(stroke_style.width, 2);
    }

    #[test]
    fn test_fill_style() {
        let fill = FillStyle::solid(Rgb565::YELLOW);
        assert_eq!(fill.color, Rgb565::YELLOW);
        assert_eq!(fill.pattern, FillPattern::Solid);
    }
}
