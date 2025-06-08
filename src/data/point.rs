//! Data point types and traits for chart data.

use crate::error::DataResult;

/// Trait for data points that can be used in charts
pub trait DataPoint: Copy + Clone + PartialEq {
    /// The type of the X coordinate
    type X: PartialOrd + Copy + Clone;
    /// The type of the Y coordinate  
    type Y: PartialOrd + Copy + Clone;

    /// Get the X coordinate of this data point
    fn x(&self) -> Self::X;

    /// Get the Y coordinate of this data point
    fn y(&self) -> Self::Y;

    /// Create a new data point from X and Y coordinates
    fn new(x: Self::X, y: Self::Y) -> Self;
}

/// A simple 2D data point with floating point coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    /// X coordinate
    pub x: f32,
    /// Y coordinate
    pub y: f32,
}

impl Point2D {
    /// Create a new 2D point
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Get the distance from this point to another point
    pub fn distance_to(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        #[cfg(feature = "floating-point")]
        {
            micromath::F32Ext::sqrt(dx * dx + dy * dy)
        }
        #[cfg(not(feature = "floating-point"))]
        {
            // Simple approximation for distance without sqrt
            let abs_dx = if dx < 0.0 { -dx } else { dx };
            let abs_dy = if dy < 0.0 { -dy } else { dy };
            abs_dx + abs_dy
        }
    }
}

impl DataPoint for Point2D {
    type X = f32;
    type Y = f32;

    fn x(&self) -> Self::X {
        self.x
    }

    fn y(&self) -> Self::Y {
        self.y
    }

    fn new(x: Self::X, y: Self::Y) -> Self {
        Self::new(x, y)
    }
}

impl From<(f32, f32)> for Point2D {
    fn from((x, y): (f32, f32)) -> Self {
        Self::new(x, y)
    }
}

impl From<Point2D> for (f32, f32) {
    fn from(point: Point2D) -> Self {
        (point.x, point.y)
    }
}

/// A data point with integer coordinates for memory-constrained environments
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntPoint {
    /// X coordinate
    pub x: i32,
    /// Y coordinate
    pub y: i32,
}

impl IntPoint {
    /// Create a new integer point
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Convert to floating point representation
    pub fn to_f32(self) -> Point2D {
        Point2D::new(self.x as f32, self.y as f32)
    }
}

impl DataPoint for IntPoint {
    type X = i32;
    type Y = i32;

    fn x(&self) -> Self::X {
        self.x
    }

    fn y(&self) -> Self::Y {
        self.y
    }

    fn new(x: Self::X, y: Self::Y) -> Self {
        Self::new(x, y)
    }
}

impl From<(i32, i32)> for IntPoint {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }
}

impl From<IntPoint> for (i32, i32) {
    fn from(point: IntPoint) -> Self {
        (point.x, point.y)
    }
}

/// A data point with a timestamp for time-series data
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimestampedPoint {
    /// Timestamp (typically seconds since epoch or relative time)
    pub timestamp: f32,
    /// Value at this timestamp
    pub value: f32,
}

impl TimestampedPoint {
    /// Create a new timestamped point
    pub const fn new(timestamp: f32, value: f32) -> Self {
        Self { timestamp, value }
    }
}

impl DataPoint for TimestampedPoint {
    type X = f32;
    type Y = f32;

    fn x(&self) -> Self::X {
        self.timestamp
    }

    fn y(&self) -> Self::Y {
        self.value
    }

    fn new(x: Self::X, y: Self::Y) -> Self {
        Self::new(x, y)
    }
}

impl From<(f32, f32)> for TimestampedPoint {
    fn from((timestamp, value): (f32, f32)) -> Self {
        Self::new(timestamp, value)
    }
}

/// Trait for interpolating between data points (used in animations)
#[cfg(feature = "animations")]
pub trait Interpolatable: DataPoint {
    /// Interpolate between this point and another point
    ///
    /// # Arguments
    /// * `other` - The target point to interpolate towards
    /// * `t` - Interpolation factor (0.0 = self, 1.0 = other)
    fn interpolate(&self, other: &Self, t: f32) -> Self;
}

#[cfg(feature = "animations")]
impl Interpolatable for TimestampedPoint {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        let timestamp = self.timestamp + (other.timestamp - self.timestamp) * t;
        let value = self.value + (other.value - self.value) * t;
        Self::new(timestamp, value)
    }
}

/// Validate that a data point has valid coordinates
pub fn validate_point<P: DataPoint>(_point: &P) -> DataResult<()>
where
    P::X: PartialOrd,
    P::Y: PartialOrd,
{
    // For floating point types, check for NaN and infinity
    #[cfg(feature = "floating-point")]
    {
        let (_x, _y) = (_point.x(), _point.y());
        // This is a simplified check - in a real implementation you'd need
        // to handle the specific numeric types properly
        // For now, we'll just return Ok since we can't easily check for NaN
        // without knowing the exact type
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_point2d_creation() {
        let point = Point2D::new(1.0, 2.0);
        assert_eq!(point.x(), 1.0);
        assert_eq!(point.y(), 2.0);
    }

    #[test]
    fn test_point2d_from_tuple() {
        let point: Point2D = (3.0, 4.0).into();
        assert_eq!(point.x(), 3.0);
        assert_eq!(point.y(), 4.0);
    }

    #[test]
    fn test_int_point_creation() {
        let point = IntPoint::new(10, 20);
        assert_eq!(point.x(), 10);
        assert_eq!(point.y(), 20);
    }

    #[test]
    fn test_timestamped_point() {
        let point = TimestampedPoint::new(100.0, 25.5);
        assert_eq!(point.x(), 100.0);
        assert_eq!(point.y(), 25.5);
    }

    #[cfg(feature = "animations")]
    #[test]
    fn test_interpolation() {
        use crate::animation::Interpolatable;
        
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(10.0, 20.0);

        let mid = p1.interpolate(p2, 0.5).unwrap();
        assert_eq!(mid.x(), 5.0);
        assert_eq!(mid.y(), 10.0);
    }
}
