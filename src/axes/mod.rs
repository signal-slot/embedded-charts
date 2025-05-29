//! Axis system for embedded graphics charts.
//!
//! This module provides a comprehensive axis system including linear axes,
//! automatic tick generation, and configurable styling. The axis system
//! integrates seamlessly with existing chart types while maintaining
//! no_std compatibility and memory efficiency.

pub mod builder;
pub mod linear;
pub mod range;
pub mod style;
pub mod ticks;
pub mod traits;

pub use builder::presets;
pub use builder::*;
pub use linear::*;
pub use range::*;
pub use style::*;
pub use ticks::*;
pub use traits::*;

/// Axis orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AxisOrientation {
    /// Horizontal axis (X-axis)
    Horizontal,
    /// Vertical axis (Y-axis)
    Vertical,
}

/// Axis position relative to the chart area
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AxisPosition {
    /// Bottom of the chart (for X-axis)
    Bottom,
    /// Top of the chart (for X-axis)
    Top,
    /// Left of the chart (for Y-axis)
    Left,
    /// Right of the chart (for Y-axis)
    Right,
}

/// Common axis configuration
#[derive(Debug, Clone)]
pub struct AxisConfig<T> {
    /// Minimum value of the axis
    pub min: T,
    /// Maximum value of the axis
    pub max: T,
    /// Axis orientation
    pub orientation: AxisOrientation,
    /// Axis position
    pub position: AxisPosition,
    /// Whether to show the axis line
    pub show_line: bool,
    /// Whether to show tick marks
    pub show_ticks: bool,
    /// Whether to show labels
    pub show_labels: bool,
    /// Whether to show grid lines
    pub show_grid: bool,
}

impl<T> AxisConfig<T>
where
    T: Copy + PartialOrd,
{
    /// Create a new axis configuration
    pub fn new(min: T, max: T, orientation: AxisOrientation, position: AxisPosition) -> Self {
        Self {
            min,
            max,
            orientation,
            position,
            show_line: true,
            show_ticks: true,
            show_labels: true,
            show_grid: false,
        }
    }

    /// Get the range of the axis
    pub fn range(&self) -> (T, T) {
        (self.min, self.max)
    }

    /// Check if the axis is horizontal
    pub fn is_horizontal(&self) -> bool {
        self.orientation == AxisOrientation::Horizontal
    }

    /// Check if the axis is vertical
    pub fn is_vertical(&self) -> bool {
        self.orientation == AxisOrientation::Vertical
    }
}

impl<T> Default for AxisConfig<T>
where
    T: Default + Copy + PartialOrd,
{
    fn default() -> Self {
        Self {
            min: T::default(),
            max: T::default(),
            orientation: AxisOrientation::Horizontal,
            position: AxisPosition::Bottom,
            show_line: true,
            show_ticks: true,
            show_labels: true,
            show_grid: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_axis_config_creation() {
        let config = AxisConfig::new(0.0, 10.0, AxisOrientation::Horizontal, AxisPosition::Bottom);
        assert_eq!(config.min, 0.0);
        assert_eq!(config.max, 10.0);
        assert!(config.is_horizontal());
        assert!(!config.is_vertical());
    }

    #[test]
    fn test_axis_config_range() {
        let config = AxisConfig::new(5, 15, AxisOrientation::Vertical, AxisPosition::Left);
        assert_eq!(config.range(), (5, 15));
    }
}
