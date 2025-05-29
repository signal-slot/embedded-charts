//! Grid styling configuration.

use crate::style::{LinePattern, LineStyle};
use embedded_graphics::prelude::*;

/// Overall grid style configuration
#[derive(Debug, Clone)]
pub struct GridStyle<C: PixelColor> {
    /// Major grid line style
    pub major: MajorGridStyle<C>,
    /// Minor grid line style
    pub minor: MinorGridStyle<C>,
    /// Grid visibility settings
    pub visibility: GridVisibility,
    /// Grid opacity (0.0 = transparent, 1.0 = opaque)
    pub opacity: f32,
}

/// Style configuration for major grid lines
#[derive(Debug, Clone)]
pub struct MajorGridStyle<C: PixelColor> {
    /// Line style for major grid lines
    pub line: GridLineStyle<C>,
    /// Whether major grid lines are enabled
    pub enabled: bool,
    /// Spacing between major grid lines (in data units)
    pub spacing: f32,
}

/// Style configuration for minor grid lines
#[derive(Debug, Clone)]
pub struct MinorGridStyle<C: PixelColor> {
    /// Line style for minor grid lines
    pub line: GridLineStyle<C>,
    /// Whether minor grid lines are enabled
    pub enabled: bool,
    /// Spacing between minor grid lines (in data units)
    pub spacing: f32,
    /// Number of minor divisions between major grid lines
    pub subdivisions: u32,
}

/// Line style specifically for grid lines
#[derive(Debug, Clone)]
pub struct GridLineStyle<C: PixelColor> {
    /// Base line style
    pub line_style: LineStyle<C>,
    /// Whether to use anti-aliasing (if supported)
    pub anti_alias: bool,
    /// Line opacity (0.0 = transparent, 1.0 = opaque)
    pub opacity: f32,
}

/// Grid visibility configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridVisibility {
    /// Whether horizontal grid lines are visible
    pub horizontal: bool,
    /// Whether vertical grid lines are visible
    pub vertical: bool,
    /// Whether major grid lines are visible
    pub major: bool,
    /// Whether minor grid lines are visible
    pub minor: bool,
}

impl<C: PixelColor> GridStyle<C> {
    /// Create a new grid style with default settings
    pub fn new() -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self {
            major: MajorGridStyle::default(),
            minor: MinorGridStyle::default(),
            visibility: GridVisibility::default(),
            opacity: 1.0,
        }
    }

    /// Create a professional grid style
    pub fn professional() -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        let major_color = embedded_graphics::pixelcolor::Rgb565::new(20, 40, 20).into();
        let minor_color = embedded_graphics::pixelcolor::Rgb565::new(10, 20, 10).into();

        Self {
            major: MajorGridStyle {
                line: GridLineStyle {
                    line_style: LineStyle::solid(major_color).width(1),
                    anti_alias: true,
                    opacity: 0.8,
                },
                enabled: true,
                spacing: 1.0,
            },
            minor: MinorGridStyle {
                line: GridLineStyle {
                    line_style: LineStyle::solid(minor_color).width(1),
                    anti_alias: true,
                    opacity: 0.4,
                },
                enabled: true,
                spacing: 0.2,
                subdivisions: 5,
            },
            visibility: GridVisibility::all(),
            opacity: 1.0,
        }
    }

    /// Create a minimal grid style (major lines only)
    pub fn minimal() -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        let grid_color = embedded_graphics::pixelcolor::Rgb565::new(15, 30, 15).into();

        Self {
            major: MajorGridStyle {
                line: GridLineStyle {
                    line_style: LineStyle::solid(grid_color).width(1),
                    anti_alias: false,
                    opacity: 0.6,
                },
                enabled: true,
                spacing: 1.0,
            },
            minor: MinorGridStyle {
                line: GridLineStyle {
                    line_style: LineStyle::solid(grid_color).width(1),
                    anti_alias: false,
                    opacity: 0.3,
                },
                enabled: false,
                spacing: 0.2,
                subdivisions: 5,
            },
            visibility: GridVisibility {
                horizontal: true,
                vertical: true,
                major: true,
                minor: false,
            },
            opacity: 1.0,
        }
    }

    /// Create a dashed grid style
    pub fn dashed() -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        let major_color = embedded_graphics::pixelcolor::Rgb565::new(25, 50, 25).into();
        let minor_color = embedded_graphics::pixelcolor::Rgb565::new(12, 25, 12).into();

        Self {
            major: MajorGridStyle {
                line: GridLineStyle {
                    line_style: LineStyle::dashed(major_color).width(1),
                    anti_alias: true,
                    opacity: 0.7,
                },
                enabled: true,
                spacing: 1.0,
            },
            minor: MinorGridStyle {
                line: GridLineStyle {
                    line_style: LineStyle::dotted(minor_color).width(1),
                    anti_alias: true,
                    opacity: 0.4,
                },
                enabled: true,
                spacing: 0.25,
                subdivisions: 4,
            },
            visibility: GridVisibility::all(),
            opacity: 1.0,
        }
    }

    /// Set the overall grid opacity
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Set grid visibility
    pub fn with_visibility(mut self, visibility: GridVisibility) -> Self {
        self.visibility = visibility;
        self
    }

    /// Enable or disable major grid lines
    pub fn with_major_enabled(mut self, enabled: bool) -> Self {
        self.major.enabled = enabled;
        self
    }

    /// Enable or disable minor grid lines
    pub fn with_minor_enabled(mut self, enabled: bool) -> Self {
        self.minor.enabled = enabled;
        self
    }

    /// Set major grid spacing
    pub fn with_major_spacing(mut self, spacing: f32) -> Self {
        self.major.spacing = spacing;
        self
    }

    /// Set minor grid spacing
    pub fn with_minor_spacing(mut self, spacing: f32) -> Self {
        self.minor.spacing = spacing;
        self
    }
}

impl<C: PixelColor> Default for GridStyle<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<C: PixelColor> MajorGridStyle<C> {
    /// Create a new major grid style
    pub fn new(line_style: LineStyle<C>) -> Self {
        Self {
            line: GridLineStyle {
                line_style,
                anti_alias: false,
                opacity: 1.0,
            },
            enabled: true,
            spacing: 1.0,
        }
    }

    /// Set the line color
    pub fn with_color(mut self, color: C) -> Self {
        self.line.line_style.color = color;
        self
    }

    /// Set the line width
    pub fn with_width(mut self, width: u32) -> Self {
        self.line.line_style.width = width;
        self
    }

    /// Set the line pattern
    pub fn with_pattern(mut self, pattern: LinePattern) -> Self {
        self.line.line_style.pattern = pattern;
        self
    }

    /// Set the opacity
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.line.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Set the spacing
    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

impl<C: PixelColor> Default for MajorGridStyle<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        let color = embedded_graphics::pixelcolor::Rgb565::new(20, 40, 20).into();
        Self::new(LineStyle::solid(color))
    }
}

impl<C: PixelColor> MinorGridStyle<C> {
    /// Create a new minor grid style
    pub fn new(line_style: LineStyle<C>) -> Self {
        Self {
            line: GridLineStyle {
                line_style,
                anti_alias: false,
                opacity: 0.5,
            },
            enabled: true,
            spacing: 0.2,
            subdivisions: 5,
        }
    }

    /// Set the line color
    pub fn with_color(mut self, color: C) -> Self {
        self.line.line_style.color = color;
        self
    }

    /// Set the line width
    pub fn with_width(mut self, width: u32) -> Self {
        self.line.line_style.width = width;
        self
    }

    /// Set the line pattern
    pub fn with_pattern(mut self, pattern: LinePattern) -> Self {
        self.line.line_style.pattern = pattern;
        self
    }

    /// Set the opacity
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.line.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Set the spacing
    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set the number of subdivisions
    pub fn with_subdivisions(mut self, subdivisions: u32) -> Self {
        self.subdivisions = subdivisions;
        self
    }
}

impl<C: PixelColor> Default for MinorGridStyle<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        let color = embedded_graphics::pixelcolor::Rgb565::new(10, 20, 10).into();
        Self::new(LineStyle::solid(color))
    }
}

impl<C: PixelColor> GridLineStyle<C> {
    /// Create a new grid line style
    pub fn new(line_style: LineStyle<C>) -> Self {
        Self {
            line_style,
            anti_alias: false,
            opacity: 1.0,
        }
    }

    /// Enable or disable anti-aliasing
    pub fn with_anti_alias(mut self, anti_alias: bool) -> Self {
        self.anti_alias = anti_alias;
        self
    }

    /// Set the opacity
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }
}

impl GridVisibility {
    /// Create visibility settings with all grids enabled
    pub const fn all() -> Self {
        Self {
            horizontal: true,
            vertical: true,
            major: true,
            minor: true,
        }
    }

    /// Create visibility settings with no grids enabled
    pub const fn none() -> Self {
        Self {
            horizontal: false,
            vertical: false,
            major: false,
            minor: false,
        }
    }

    /// Create visibility settings with only major grids enabled
    pub const fn major_only() -> Self {
        Self {
            horizontal: true,
            vertical: true,
            major: true,
            minor: false,
        }
    }

    /// Create visibility settings with only horizontal grids enabled
    pub const fn horizontal_only() -> Self {
        Self {
            horizontal: true,
            vertical: false,
            major: true,
            minor: true,
        }
    }

    /// Create visibility settings with only vertical grids enabled
    pub const fn vertical_only() -> Self {
        Self {
            horizontal: false,
            vertical: true,
            major: true,
            minor: true,
        }
    }

    /// Check if any grid lines are visible
    pub const fn any_visible(&self) -> bool {
        (self.horizontal || self.vertical) && (self.major || self.minor)
    }

    /// Check if major grid lines are visible
    pub const fn major_visible(&self) -> bool {
        (self.horizontal || self.vertical) && self.major
    }

    /// Check if minor grid lines are visible
    pub const fn minor_visible(&self) -> bool {
        (self.horizontal || self.vertical) && self.minor
    }
}

impl Default for GridVisibility {
    fn default() -> Self {
        Self::major_only()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::pixelcolor::Rgb565;

    #[test]
    fn test_grid_style_creation() {
        let style: GridStyle<Rgb565> = GridStyle::new();
        assert!(style.major.enabled);
        assert_eq!(style.opacity, 1.0);
    }

    #[test]
    fn test_grid_style_professional() {
        let style: GridStyle<Rgb565> = GridStyle::professional();
        assert!(style.major.enabled);
        assert!(style.minor.enabled);
        assert!(style.visibility.any_visible());
    }

    #[test]
    fn test_grid_style_minimal() {
        let style: GridStyle<Rgb565> = GridStyle::minimal();
        assert!(style.major.enabled);
        assert!(!style.minor.enabled);
    }

    #[test]
    fn test_grid_visibility() {
        let vis = GridVisibility::all();
        assert!(vis.any_visible());
        assert!(vis.major_visible());
        assert!(vis.minor_visible());

        let vis = GridVisibility::none();
        assert!(!vis.any_visible());

        let vis = GridVisibility::major_only();
        assert!(vis.major_visible());
        assert!(!vis.minor_visible());
    }

    #[test]
    fn test_major_grid_style_builder() {
        let style: MajorGridStyle<Rgb565> = MajorGridStyle::default()
            .with_width(2)
            .with_opacity(0.8)
            .with_spacing(2.0);

        assert_eq!(style.line.line_style.width, 2);
        assert_eq!(style.line.opacity, 0.8);
        assert_eq!(style.spacing, 2.0);
    }

    #[test]
    fn test_minor_grid_style_builder() {
        let style: MinorGridStyle<Rgb565> = MinorGridStyle::default()
            .with_subdivisions(10)
            .with_spacing(0.1);

        assert_eq!(style.subdivisions, 10);
        assert_eq!(style.spacing, 0.1);
    }
}
