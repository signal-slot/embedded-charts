//! Gradient fills and advanced styling for no_std environments
//!
//! This module provides gradient rendering capabilities that work efficiently
//! on embedded systems without heap allocation.

use crate::error::ChartError;
use embedded_graphics::prelude::*;
use heapless::Vec;

/// Maximum number of gradient stops supported
pub const MAX_GRADIENT_STOPS: usize = 8;

/// A color stop in a gradient
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GradientStop<C: PixelColor> {
    /// Position along the gradient (0.0 to 1.0)
    pub position: f32,
    /// Color at this position
    pub color: C,
}

impl<C: PixelColor> GradientStop<C> {
    /// Create a new gradient stop
    pub const fn new(position: f32, color: C) -> Self {
        Self { position, color }
    }
}

/// Direction of a linear gradient
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GradientDirection {
    /// Horizontal gradient (left to right)
    Horizontal,
    /// Vertical gradient (top to bottom)
    Vertical,
    /// Diagonal gradient (top-left to bottom-right)
    Diagonal,
    /// Reverse diagonal gradient (top-right to bottom-left)
    ReverseDiagonal,
}

/// Linear gradient definition
#[derive(Debug, Clone)]
pub struct LinearGradient<C: PixelColor, const N: usize = MAX_GRADIENT_STOPS> {
    /// Gradient stops (must have at least 2)
    stops: Vec<GradientStop<C>, N>,
    /// Direction of the gradient
    direction: GradientDirection,
}

impl<C: PixelColor, const N: usize> LinearGradient<C, N> {
    /// Create a new linear gradient
    pub fn new(direction: GradientDirection) -> Self {
        Self {
            stops: Vec::new(),
            direction,
        }
    }

    /// Create a simple two-color gradient
    pub fn simple(start: C, end: C, direction: GradientDirection) -> Result<Self, ChartError> {
        let mut gradient = Self::new(direction);
        gradient.add_stop(0.0, start)?;
        gradient.add_stop(1.0, end)?;
        Ok(gradient)
    }

    /// Add a color stop to the gradient
    pub fn add_stop(&mut self, position: f32, color: C) -> Result<(), ChartError> {
        if !(0.0..=1.0).contains(&position) {
            return Err(ChartError::InvalidConfiguration);
        }

        let stop = GradientStop::new(position, color);

        // Insert in sorted order by position
        let insert_pos = self
            .stops
            .iter()
            .position(|s| s.position > position)
            .unwrap_or(self.stops.len());

        self.stops
            .insert(insert_pos, stop)
            .map_err(|_| ChartError::MemoryFull)?;

        Ok(())
    }

    /// Get the color at a specific position (0.0 to 1.0)
    pub fn color_at(&self, position: f32) -> Option<C> {
        if self.stops.len() < 2 {
            return None;
        }

        let position = position.clamp(0.0, 1.0);

        // Find the two stops to interpolate between
        let mut lower_stop = &self.stops[0];
        let mut upper_stop = &self.stops[self.stops.len() - 1];

        for i in 0..self.stops.len() - 1 {
            if position >= self.stops[i].position && position <= self.stops[i + 1].position {
                lower_stop = &self.stops[i];
                upper_stop = &self.stops[i + 1];
                break;
            }
        }

        if lower_stop.position == upper_stop.position {
            Some(lower_stop.color)
        } else {
            // Simple linear interpolation for now
            // More sophisticated color interpolation requires the color-support feature
            let t = (position - lower_stop.position) / (upper_stop.position - lower_stop.position);
            if t < 0.5 {
                Some(lower_stop.color)
            } else {
                Some(upper_stop.color)
            }
        }
    }

    /// Get the gradient direction
    pub fn direction(&self) -> GradientDirection {
        self.direction
    }

    /// Get the number of stops
    pub fn stop_count(&self) -> usize {
        self.stops.len()
    }

    /// Check if the gradient is valid (has at least 2 stops)
    pub fn is_valid(&self) -> bool {
        self.stops.len() >= 2
    }
}

/// Extension trait for color interpolation with gradients
#[cfg(feature = "color-support")]
pub trait GradientInterpolation<C: PixelColor> {
    /// Get interpolated color at position
    fn interpolated_color_at(&self, position: f32) -> Option<C>;
}

#[cfg(feature = "color-support")]
impl<const N: usize> GradientInterpolation<embedded_graphics::pixelcolor::Rgb565>
    for LinearGradient<embedded_graphics::pixelcolor::Rgb565, N>
{
    fn interpolated_color_at(
        &self,
        position: f32,
    ) -> Option<embedded_graphics::pixelcolor::Rgb565> {
        use crate::style::ColorInterpolation;
        use embedded_graphics::pixelcolor::Rgb565;

        if self.stops.len() < 2 {
            return None;
        }

        let position = position.clamp(0.0, 1.0);

        // Find the two stops to interpolate between
        let mut lower_stop = &self.stops[0];
        let mut upper_stop = &self.stops[self.stops.len() - 1];

        for i in 0..self.stops.len() - 1 {
            if position >= self.stops[i].position && position <= self.stops[i + 1].position {
                lower_stop = &self.stops[i];
                upper_stop = &self.stops[i + 1];
                break;
            }
        }

        if lower_stop.position == upper_stop.position {
            Some(lower_stop.color)
        } else {
            let t = (position - lower_stop.position) / (upper_stop.position - lower_stop.position);
            Some(Rgb565::interpolate(lower_stop.color, upper_stop.color, t))
        }
    }
}

/// Extension trait for radial gradient interpolation
#[cfg(feature = "color-support")]
pub trait RadialGradientInterpolation<C: PixelColor> {
    /// Get interpolated color at distance
    fn interpolated_color_at_distance(&self, distance: f32) -> Option<C>;
}

#[cfg(feature = "color-support")]
impl<const N: usize> RadialGradientInterpolation<embedded_graphics::pixelcolor::Rgb565>
    for RadialGradient<embedded_graphics::pixelcolor::Rgb565, N>
{
    fn interpolated_color_at_distance(
        &self,
        distance: f32,
    ) -> Option<embedded_graphics::pixelcolor::Rgb565> {
        use crate::style::ColorInterpolation;
        use embedded_graphics::pixelcolor::Rgb565;

        if self.stops.len() < 2 {
            return None;
        }

        let distance = distance.clamp(0.0, 1.0);

        // Find stops to interpolate between
        let mut lower_stop = &self.stops[0];
        let mut upper_stop = &self.stops[self.stops.len() - 1];

        for i in 0..self.stops.len() - 1 {
            if distance >= self.stops[i].position && distance <= self.stops[i + 1].position {
                lower_stop = &self.stops[i];
                upper_stop = &self.stops[i + 1];
                break;
            }
        }

        if lower_stop.position == upper_stop.position {
            Some(lower_stop.color)
        } else {
            let t = (distance - lower_stop.position) / (upper_stop.position - lower_stop.position);
            Some(Rgb565::interpolate(lower_stop.color, upper_stop.color, t))
        }
    }
}

/// Radial gradient definition
#[derive(Debug, Clone)]
pub struct RadialGradient<C: PixelColor, const N: usize = MAX_GRADIENT_STOPS> {
    /// Center point of the gradient (relative to bounds, 0.0 to 1.0)
    center: Point,
    /// Gradient stops
    stops: Vec<GradientStop<C>, N>,
}

impl<C: PixelColor, const N: usize> RadialGradient<C, N> {
    /// Create a new radial gradient
    pub fn new(center: Point) -> Self {
        Self {
            center,
            stops: Vec::new(),
        }
    }

    /// Create a simple two-color radial gradient
    pub fn simple(inner: C, outer: C, center: Point) -> Result<Self, ChartError> {
        let mut gradient = Self::new(center);
        gradient.add_stop(0.0, inner)?;
        gradient.add_stop(1.0, outer)?;
        Ok(gradient)
    }

    /// Add a color stop
    pub fn add_stop(&mut self, position: f32, color: C) -> Result<(), ChartError> {
        if !(0.0..=1.0).contains(&position) {
            return Err(ChartError::InvalidConfiguration);
        }

        let stop = GradientStop::new(position, color);

        // Insert in sorted order
        let insert_pos = self
            .stops
            .iter()
            .position(|s| s.position > position)
            .unwrap_or(self.stops.len());

        self.stops
            .insert(insert_pos, stop)
            .map_err(|_| ChartError::MemoryFull)?;

        Ok(())
    }

    /// Get color at a specific distance from center (0.0 to 1.0)
    pub fn color_at_distance(&self, distance: f32) -> Option<C> {
        if self.stops.len() < 2 {
            return None;
        }

        let distance = distance.clamp(0.0, 1.0);

        // Find stops to interpolate between
        let mut lower_stop = &self.stops[0];
        let mut upper_stop = &self.stops[self.stops.len() - 1];

        for i in 0..self.stops.len() - 1 {
            if distance >= self.stops[i].position && distance <= self.stops[i + 1].position {
                lower_stop = &self.stops[i];
                upper_stop = &self.stops[i + 1];
                break;
            }
        }

        #[cfg(feature = "color-support")]
        {
            if lower_stop.position == upper_stop.position {
                Some(lower_stop.color)
            } else {
                // Simple nearest-neighbor interpolation for generic colors
                let t =
                    (distance - lower_stop.position) / (upper_stop.position - lower_stop.position);
                if t < 0.5 {
                    Some(lower_stop.color)
                } else {
                    Some(upper_stop.color)
                }
            }
        }

        #[cfg(not(feature = "color-support"))]
        {
            let mid = (lower_stop.position + upper_stop.position) / 2.0;
            if distance <= mid {
                Some(lower_stop.color)
            } else {
                Some(upper_stop.color)
            }
        }
    }

    /// Get the center point
    pub fn center(&self) -> Point {
        self.center
    }

    /// Check if the gradient is valid
    pub fn is_valid(&self) -> bool {
        self.stops.len() >= 2
    }
}

/// Pattern fill types for advanced styling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternType {
    /// Horizontal lines
    HorizontalLines {
        /// Spacing between lines in pixels
        spacing: u32,
        /// Width of each line in pixels
        width: u32,
    },
    /// Vertical lines
    VerticalLines {
        /// Spacing between lines in pixels
        spacing: u32,
        /// Width of each line in pixels
        width: u32,
    },
    /// Diagonal lines
    DiagonalLines {
        /// Spacing between lines in pixels
        spacing: u32,
        /// Width of each line in pixels
        width: u32,
    },
    /// Dots
    Dots {
        /// Spacing between dot centers in pixels
        spacing: u32,
        /// Radius of each dot in pixels
        radius: u32,
    },
    /// Checkerboard
    Checkerboard {
        /// Size of each square in pixels
        size: u32,
    },
    /// Cross hatch
    CrossHatch {
        /// Spacing between lines in pixels
        spacing: u32,
        /// Width of each line in pixels
        width: u32,
    },
}

/// Pattern fill definition
#[derive(Debug, Clone, Copy)]
pub struct PatternFill<C: PixelColor> {
    /// Foreground color (pattern color)
    pub foreground: C,
    /// Background color
    pub background: C,
    /// Pattern type
    pub pattern: PatternType,
}

impl<C: PixelColor> PatternFill<C> {
    /// Create a new pattern fill
    pub const fn new(foreground: C, background: C, pattern: PatternType) -> Self {
        Self {
            foreground,
            background,
            pattern,
        }
    }

    /// Check if a pixel at the given position should use foreground color
    pub fn is_foreground(&self, x: i32, y: i32) -> bool {
        match self.pattern {
            PatternType::HorizontalLines { spacing, width } => (y as u32 % spacing) < width,
            PatternType::VerticalLines { spacing, width } => (x as u32 % spacing) < width,
            PatternType::DiagonalLines { spacing, width } => ((x + y) as u32 % spacing) < width,
            PatternType::Dots { spacing, radius } => {
                let px = x as u32 % spacing;
                let py = y as u32 % spacing;
                let center = spacing / 2;
                let dx = px.abs_diff(center);
                let dy = py.abs_diff(center);
                (dx * dx + dy * dy) <= (radius * radius)
            }
            PatternType::Checkerboard { size } => ((x as u32 / size) + (y as u32 / size)) % 2 == 0,
            PatternType::CrossHatch { spacing, width } => {
                let h = (y as u32 % spacing) < width;
                let v = (x as u32 % spacing) < width;
                h || v
            }
        }
    }

    /// Get the color at a specific position
    pub fn color_at(&self, x: i32, y: i32) -> C {
        if self.is_foreground(x, y) {
            self.foreground
        } else {
            self.background
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::pixelcolor::Rgb565;

    #[test]
    fn test_linear_gradient_simple() {
        let gradient: LinearGradient<Rgb565, 8> =
            LinearGradient::simple(Rgb565::RED, Rgb565::BLUE, GradientDirection::Horizontal)
                .unwrap();

        assert!(gradient.is_valid());
        assert_eq!(gradient.stop_count(), 2);
    }

    #[test]
    fn test_gradient_color_at() {
        let mut gradient: LinearGradient<Rgb565, 4> =
            LinearGradient::new(GradientDirection::Horizontal);
        gradient.add_stop(0.0, Rgb565::RED).unwrap();
        gradient.add_stop(1.0, Rgb565::BLUE).unwrap();

        // Test edge colors
        assert_eq!(gradient.color_at(0.0), Some(Rgb565::RED));
        assert_eq!(gradient.color_at(1.0), Some(Rgb565::BLUE));
    }

    #[test]
    fn test_pattern_fill() {
        let pattern = PatternFill::new(
            Rgb565::BLACK,
            Rgb565::WHITE,
            PatternType::Checkerboard { size: 10 },
        );

        assert_eq!(pattern.color_at(0, 0), Rgb565::BLACK);
        assert_eq!(pattern.color_at(10, 0), Rgb565::WHITE);
        assert_eq!(pattern.color_at(0, 10), Rgb565::WHITE);
        assert_eq!(pattern.color_at(10, 10), Rgb565::BLACK);
    }
}
