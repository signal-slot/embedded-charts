//! Data bounds calculation and management for chart scaling.

use crate::data::point::DataPoint;
use crate::error::{DataError, DataResult};
use crate::math::{Math, NumericConversion};

/// Represents the bounds of a dataset in 2D space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DataBounds<X, Y>
where
    X: PartialOrd + Copy,
    Y: PartialOrd + Copy,
{
    /// Minimum X value
    pub min_x: X,
    /// Maximum X value
    pub max_x: X,
    /// Minimum Y value
    pub min_y: Y,
    /// Maximum Y value
    pub max_y: Y,
}

impl<X, Y> DataBounds<X, Y>
where
    X: PartialOrd + Copy,
    Y: PartialOrd + Copy,
{
    /// Create new data bounds
    pub fn new(min_x: X, max_x: X, min_y: Y, max_y: Y) -> DataResult<Self> {
        if min_x > max_x || min_y > max_y {
            return Err(DataError::INVALID_DATA_POINT);
        }

        Ok(Self {
            min_x,
            max_x,
            min_y,
            max_y,
        })
    }

    /// Get the width of the X range
    pub fn width(&self) -> X
    where
        X: core::ops::Sub<Output = X>,
    {
        self.max_x - self.min_x
    }

    /// Get the height of the Y range
    pub fn height(&self) -> Y
    where
        Y: core::ops::Sub<Output = Y>,
    {
        self.max_y - self.min_y
    }

    /// Check if a point is within these bounds
    pub fn contains<P>(&self, point: &P) -> bool
    where
        P: DataPoint<X = X, Y = Y>,
    {
        point.x() >= self.min_x
            && point.x() <= self.max_x
            && point.y() >= self.min_y
            && point.y() <= self.max_y
    }

    /// Expand bounds to include a new point
    pub fn expand_to_include<P>(&mut self, point: &P)
    where
        P: DataPoint<X = X, Y = Y>,
    {
        if point.x() < self.min_x {
            self.min_x = point.x();
        }
        if point.x() > self.max_x {
            self.max_x = point.x();
        }
        if point.y() < self.min_y {
            self.min_y = point.y();
        }
        if point.y() > self.max_y {
            self.max_y = point.y();
        }
    }

    /// Merge with another bounds, creating a bounds that contains both
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            min_x: if self.min_x < other.min_x {
                self.min_x
            } else {
                other.min_x
            },
            max_x: if self.max_x > other.max_x {
                self.max_x
            } else {
                other.max_x
            },
            min_y: if self.min_y < other.min_y {
                self.min_y
            } else {
                other.min_y
            },
            max_y: if self.max_y > other.max_y {
                self.max_y
            } else {
                other.max_y
            },
        }
    }
}

/// Specialized bounds for floating point data
pub type FloatBounds = DataBounds<f32, f32>;

/// Specialized bounds for integer data
pub type IntBounds = DataBounds<i32, i32>;

impl FloatBounds {
    /// Create bounds with some padding around the data
    pub fn with_padding(&self, padding_percent: f32) -> Self {
        let x_padding = self.width() * padding_percent / 100.0;
        let y_padding = self.height() * padding_percent / 100.0;

        Self {
            min_x: self.min_x - x_padding,
            max_x: self.max_x + x_padding,
            min_y: self.min_y - y_padding,
            max_y: self.max_y + y_padding,
        }
    }

    /// Create bounds that are nice for display (rounded to nice numbers)
    pub fn nice_bounds(&self) -> Self {
        fn nice_number(value: f32, round: bool) -> f32 {
            if value == 0.0 {
                return 0.0;
            }

            let value_num = value.to_number();
            let abs_val = Math::abs(value_num);
            let exp = Math::floor(Math::log10(abs_val));
            let ten = 10.0f32.to_number();
            let divisor = Math::pow(ten, exp);
            let divisor_f32 = f32::from_number(divisor);
            let f = value / divisor_f32;

            let nice_f = if round {
                if f < 1.5 {
                    1.0
                } else if f < 3.0 {
                    2.0
                } else if f < 7.0 {
                    5.0
                } else {
                    10.0
                }
            } else if f <= 1.0 {
                1.0
            } else if f <= 2.0 {
                2.0
            } else if f <= 5.0 {
                5.0
            } else {
                10.0
            };

            let _exp_f32 = f32::from_number(exp);
            let ten_pow_exp = f32::from_number(Math::pow(10.0f32.to_number(), exp));
            nice_f * ten_pow_exp
        }

        let x_range = self.width();
        let y_range = self.height();

        let nice_x_range = nice_number(x_range, false);
        let nice_y_range = nice_number(y_range, false);

        let x_center = (self.min_x + self.max_x) / 2.0;
        let y_center = (self.min_y + self.max_y) / 2.0;

        Self {
            min_x: x_center - nice_x_range / 2.0,
            max_x: x_center + nice_x_range / 2.0,
            min_y: y_center - nice_y_range / 2.0,
            max_y: y_center + nice_y_range / 2.0,
        }
    }
}

/// Calculate bounds for a collection of data points
pub fn calculate_bounds<P, I>(points: I) -> DataResult<DataBounds<P::X, P::Y>>
where
    P: DataPoint,
    P::X: PartialOrd + Copy,
    P::Y: PartialOrd + Copy,
    I: Iterator<Item = P>,
{
    let mut points_iter = points;

    // Get the first point to initialize bounds
    let first_point = points_iter.next().ok_or(DataError::INSUFFICIENT_DATA)?;

    let mut bounds = DataBounds {
        min_x: first_point.x(),
        max_x: first_point.x(),
        min_y: first_point.y(),
        max_y: first_point.y(),
    };

    // Expand bounds to include all other points
    for point in points_iter {
        bounds.expand_to_include(&point);
    }

    Ok(bounds)
}

/// Calculate bounds for multiple data series
pub fn calculate_multi_series_bounds<P, I, S>(series: S) -> DataResult<DataBounds<P::X, P::Y>>
where
    P: DataPoint,
    P::X: PartialOrd + Copy,
    P::Y: PartialOrd + Copy,
    I: Iterator<Item = P>,
    S: Iterator<Item = I>,
{
    let mut series_iter = series;

    // Calculate bounds for the first series
    let first_series = series_iter.next().ok_or(DataError::INSUFFICIENT_DATA)?;
    let mut combined_bounds = calculate_bounds(first_series)?;

    // Merge bounds from all other series
    for series_data in series_iter {
        let series_bounds = calculate_bounds(series_data)?;
        combined_bounds = combined_bounds.merge(&series_bounds);
    }

    Ok(combined_bounds)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::point::Point2D;

    #[test]
    fn test_bounds_creation() {
        let bounds = DataBounds::new(0.0, 10.0, 0.0, 20.0).unwrap();
        assert_eq!(bounds.min_x, 0.0);
        assert_eq!(bounds.max_x, 10.0);
        assert_eq!(bounds.min_y, 0.0);
        assert_eq!(bounds.max_y, 20.0);
    }

    #[test]
    fn test_invalid_bounds() {
        let result = DataBounds::new(10.0, 0.0, 0.0, 20.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_bounds_contains() {
        let bounds = DataBounds::new(0.0, 10.0, 0.0, 20.0).unwrap();
        let point = Point2D::new(5.0, 10.0);
        assert!(bounds.contains(&point));

        let outside_point = Point2D::new(15.0, 10.0);
        assert!(!bounds.contains(&outside_point));
    }

    #[test]
    fn test_bounds_expansion() {
        let mut bounds = DataBounds::new(0.0, 10.0, 0.0, 20.0).unwrap();
        let point = Point2D::new(15.0, 25.0);
        bounds.expand_to_include(&point);

        assert_eq!(bounds.max_x, 15.0);
        assert_eq!(bounds.max_y, 25.0);
    }

    #[test]
    fn test_calculate_bounds() {
        let mut points = heapless::Vec::<Point2D, 8>::new();
        points.push(Point2D::new(1.0, 2.0)).unwrap();
        points.push(Point2D::new(5.0, 8.0)).unwrap();
        points.push(Point2D::new(3.0, 4.0)).unwrap();

        let bounds = calculate_bounds(points.into_iter()).unwrap();
        assert_eq!(bounds.min_x, 1.0);
        assert_eq!(bounds.max_x, 5.0);
        assert_eq!(bounds.min_y, 2.0);
        assert_eq!(bounds.max_y, 8.0);
    }

    #[test]
    fn test_bounds_merge() {
        let bounds1 = DataBounds::new(0.0, 5.0, 0.0, 10.0).unwrap();
        let bounds2 = DataBounds::new(3.0, 8.0, 5.0, 15.0).unwrap();

        let merged = bounds1.merge(&bounds2);
        assert_eq!(merged.min_x, 0.0);
        assert_eq!(merged.max_x, 8.0);
        assert_eq!(merged.min_y, 0.0);
        assert_eq!(merged.max_y, 15.0);
    }
}
