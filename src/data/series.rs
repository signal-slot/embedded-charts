//! Data series implementations for chart data management.

use crate::data::bounds::DataBounds;
use crate::data::point::DataPoint;
use crate::error::{DataError, DataResult};
use heapless::Vec;

/// Memory-efficient iterator for StaticDataSeries that uses index-based access
pub struct StaticDataSeriesIter<T, const N: usize> {
    data: heapless::Vec<T, N>,
    index: usize,
}

impl<T: Clone, const N: usize> Iterator for StaticDataSeriesIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let item = self.data.get(self.index)?.clone();
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.data.len() - self.index;
        (remaining, Some(remaining))
    }
}

impl<T: Clone, const N: usize> ExactSizeIterator for StaticDataSeriesIter<T, N> {}

/// Reference iterator for StaticDataSeries that yields references to avoid cloning
pub struct StaticDataSeriesRefIter<'a, T> {
    data: &'a [T],
    index: usize,
}

impl<'a, T> Iterator for StaticDataSeriesRefIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let item = &self.data[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.data.len() - self.index;
        (remaining, Some(remaining))
    }
}

impl<'a, T> ExactSizeIterator for StaticDataSeriesRefIter<'a, T> {}

/// Trait for data series that can be used in charts
pub trait DataSeries {
    /// The type of data points in this series
    type Item: DataPoint;
    /// Iterator type for iterating over data points (cloning)
    type Iter: Iterator<Item = Self::Item>;

    /// Get an iterator over the data points (clones items)
    fn iter(&self) -> Self::Iter;

    /// Get the number of data points in the series
    fn len(&self) -> usize;

    /// Check if the series is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Calculate the bounds of this data series
    fn calculate_bounds(&self) -> DataResult<()> {
        // This is a placeholder - bounds calculation will be implemented
        // in concrete implementations where the types are known
        Ok(())
    }

    /// Get a specific data point by index
    fn get(&self, index: usize) -> Option<Self::Item>;
}

/// A static data series with compile-time capacity bounds
#[derive(Debug, Clone)]
pub struct StaticDataSeries<T, const N: usize>
where
    T: DataPoint,
{
    data: Vec<T, N>,
    label: Option<heapless::String<32>>,
}

impl<T, const N: usize> StaticDataSeries<T, N>
where
    T: DataPoint,
{
    /// Create a new empty static data series
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            label: None,
        }
    }

    /// Create a new static data series with a label
    pub fn with_label(label: &str) -> Self {
        let mut series = Self::new();
        series.set_label(label);
        series
    }

    /// Set the label for this series
    pub fn set_label(&mut self, label: &str) {
        let mut string = heapless::String::new();
        if string.push_str(label).is_ok() {
            self.label = Some(string);
        }
    }

    /// Get the label for this series
    pub fn label(&self) -> Option<&str> {
        self.label.as_ref().map(|s| s.as_str())
    }

    /// Add a data point to the series
    pub fn push(&mut self, point: T) -> DataResult<()> {
        self.data
            .push(point)
            .map_err(|_| DataError::buffer_full("push data point", N))
    }

    /// Add multiple data points from an iterator
    pub fn extend<I>(&mut self, points: I) -> DataResult<()>
    where
        I: IntoIterator<Item = T>,
    {
        for point in points {
            self.push(point)?;
        }
        Ok(())
    }

    /// Add data points from a slice of tuples
    pub fn from_tuples(tuples: &[(T::X, T::Y)]) -> DataResult<Self>
    where
        T: DataPoint,
    {
        let mut series = Self::new();
        for &(x, y) in tuples {
            series.push(T::new(x, y))?;
        }
        Ok(series)
    }

    /// Clear all data points
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Get the capacity of this series
    pub fn capacity(&self) -> usize {
        N
    }

    /// Get the remaining capacity
    pub fn remaining_capacity(&self) -> usize {
        N - self.data.len()
    }

    /// Check if the series is at capacity
    pub fn is_full(&self) -> bool {
        self.data.len() == N
    }

    /// Get a slice of all data points
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// Sort the data points by X coordinate
    pub fn sort_by_x(&mut self)
    where
        T::X: Ord,
        T: Clone,
    {
        // Use efficient insertion sort for small arrays, or merge sort for larger ones
        if self.data.len() <= 16 {
            self.insertion_sort_by_x();
        } else {
            self.merge_sort_by_x();
        }
    }

    /// Insertion sort by X coordinate (efficient for small arrays)
    fn insertion_sort_by_x(&mut self)
    where
        T::X: Ord,
        T: Clone,
    {
        for i in 1..self.data.len() {
            let key = self.data[i];
            let mut j = i;

            while j > 0 && self.data[j - 1].x() > key.x() {
                self.data[j] = self.data[j - 1];
                j -= 1;
            }
            self.data[j] = key;
        }
    }

    /// Merge sort by X coordinate (efficient for larger arrays)
    fn merge_sort_by_x(&mut self)
    where
        T::X: Ord,
        T: Clone,
    {
        let len = self.data.len();
        if len <= 1 {
            return;
        }

        // Use a temporary buffer for merging
        let mut temp = heapless::Vec::<T, N>::new();
        for _ in 0..len {
            if temp.push(self.data[0]).is_err() {
                // If we can't allocate temp buffer, fall back to insertion sort
                self.insertion_sort_by_x();
                return;
            }
        }

        self.merge_sort_recursive(0, len, &mut temp);
    }

    /// Recursive merge sort helper
    fn merge_sort_recursive(&mut self, start: usize, end: usize, temp: &mut heapless::Vec<T, N>)
    where
        T::X: Ord,
        T: Clone,
    {
        if end - start <= 1 {
            return;
        }

        let mid = start + (end - start) / 2;
        self.merge_sort_recursive(start, mid, temp);
        self.merge_sort_recursive(mid, end, temp);

        // Merge the two sorted halves
        let mut i = start;
        let mut j = mid;
        let mut k = start;

        // Copy data to temp buffer for merging
        for idx in start..end {
            temp[idx] = self.data[idx];
        }

        while i < mid && j < end {
            if temp[i].x() <= temp[j].x() {
                self.data[k] = temp[i];
                i += 1;
            } else {
                self.data[k] = temp[j];
                j += 1;
            }
            k += 1;
        }

        // Copy remaining elements
        while i < mid {
            self.data[k] = temp[i];
            i += 1;
            k += 1;
        }

        while j < end {
            self.data[k] = temp[j];
            j += 1;
            k += 1;
        }
    }

    /// Sort the data points by Y coordinate
    pub fn sort_by_y(&mut self)
    where
        T::Y: Ord,
        T: Clone,
    {
        // Use efficient insertion sort for small arrays, or merge sort for larger ones
        if self.data.len() <= 16 {
            self.insertion_sort_by_y();
        } else {
            self.merge_sort_by_y();
        }
    }

    /// Insertion sort by Y coordinate (efficient for small arrays)
    fn insertion_sort_by_y(&mut self)
    where
        T::Y: Ord,
        T: Clone,
    {
        for i in 1..self.data.len() {
            let key = self.data[i];
            let mut j = i;

            while j > 0 && self.data[j - 1].y() > key.y() {
                self.data[j] = self.data[j - 1];
                j -= 1;
            }
            self.data[j] = key;
        }
    }

    /// Merge sort by Y coordinate (efficient for larger arrays)
    fn merge_sort_by_y(&mut self)
    where
        T::Y: Ord,
        T: Clone,
    {
        let len = self.data.len();
        if len <= 1 {
            return;
        }

        // Use a temporary buffer for merging
        let mut temp = heapless::Vec::<T, N>::new();
        for _ in 0..len {
            if temp.push(self.data[0]).is_err() {
                // If we can't allocate temp buffer, fall back to insertion sort
                self.insertion_sort_by_y();
                return;
            }
        }

        self.merge_sort_by_y_recursive(0, len, &mut temp);
    }

    /// Recursive merge sort helper for Y coordinate
    fn merge_sort_by_y_recursive(
        &mut self,
        start: usize,
        end: usize,
        temp: &mut heapless::Vec<T, N>,
    ) where
        T::Y: Ord,
        T: Clone,
    {
        if end - start <= 1 {
            return;
        }

        let mid = start + (end - start) / 2;
        self.merge_sort_by_y_recursive(start, mid, temp);
        self.merge_sort_by_y_recursive(mid, end, temp);

        // Merge the two sorted halves
        let mut i = start;
        let mut j = mid;
        let mut k = start;

        // Copy data to temp buffer for merging
        for idx in start..end {
            temp[idx] = self.data[idx];
        }

        while i < mid && j < end {
            if temp[i].y() <= temp[j].y() {
                self.data[k] = temp[i];
                i += 1;
            } else {
                self.data[k] = temp[j];
                j += 1;
            }
            k += 1;
        }

        // Copy remaining elements
        while i < mid {
            self.data[k] = temp[i];
            i += 1;
            k += 1;
        }

        while j < end {
            self.data[k] = temp[j];
            j += 1;
            k += 1;
        }
    }
}

impl<T, const N: usize> Default for StaticDataSeries<T, N>
where
    T: DataPoint,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> StaticDataSeries<T, N>
where
    T: DataPoint + Clone,
{
    /// Get a zero-copy reference iterator (recommended for performance)
    pub fn iter_ref(&self) -> StaticDataSeriesRefIter<'_, T> {
        StaticDataSeriesRefIter {
            data: self.data.as_slice(),
            index: 0,
        }
    }

    /// Get the underlying data as a slice (zero-copy access)
    pub fn data(&self) -> &[T] {
        self.data.as_slice()
    }
}

impl<T, const N: usize> DataSeries for StaticDataSeries<T, N>
where
    T: DataPoint + Clone,
{
    type Item = T;
    type Iter = StaticDataSeriesIter<T, N>;

    fn iter(&self) -> Self::Iter {
        // Note: This clones the data vector for backwards compatibility.
        // For better performance, use iter_ref() or data() methods which provide
        // zero-copy access to the underlying data.
        StaticDataSeriesIter {
            data: self.data.clone(),
            index: 0,
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn get(&self, index: usize) -> Option<Self::Item> {
        self.data.get(index).copied()
    }
}

impl<T, const N: usize> StaticDataSeries<T, N>
where
    T: DataPoint + Clone,
    T::X: PartialOrd + Copy,
    T::Y: PartialOrd + Copy,
{
    /// Get the bounds of this data series
    pub fn bounds(&self) -> DataResult<crate::data::bounds::DataBounds<T::X, T::Y>> {
        use crate::data::bounds::calculate_bounds;
        calculate_bounds(self.iter())
    }
}

/// A multi-series container for holding multiple data series
#[derive(Debug, Clone)]
pub struct MultiSeries<T, const SERIES: usize, const POINTS: usize>
where
    T: DataPoint,
{
    series: Vec<StaticDataSeries<T, POINTS>, SERIES>,
}

impl<T, const SERIES: usize, const POINTS: usize> MultiSeries<T, SERIES, POINTS>
where
    T: DataPoint,
{
    /// Create a new empty multi-series container
    pub fn new() -> Self {
        Self { series: Vec::new() }
    }

    /// Add a new data series
    pub fn add_series(&mut self, series: StaticDataSeries<T, POINTS>) -> DataResult<usize> {
        let index = self.series.len();
        self.series
            .push(series)
            .map_err(|_| DataError::buffer_full("add data series", SERIES))?;
        Ok(index)
    }

    /// Get a reference to a series by index
    pub fn get_series(&self, index: usize) -> Option<&StaticDataSeries<T, POINTS>> {
        self.series.get(index)
    }

    /// Get a mutable reference to a series by index
    pub fn get_series_mut(&mut self, index: usize) -> Option<&mut StaticDataSeries<T, POINTS>> {
        self.series.get_mut(index)
    }

    /// Get the number of series
    pub fn series_count(&self) -> usize {
        self.series.len()
    }

    /// Check if there are no series
    pub fn is_empty(&self) -> bool {
        self.series.is_empty()
    }

    /// Get an iterator over all series
    pub fn iter_series(&self) -> core::slice::Iter<StaticDataSeries<T, POINTS>> {
        self.series.iter()
    }

    /// Calculate combined bounds for all series
    pub fn combined_bounds(&self) -> DataResult<DataBounds<T::X, T::Y>>
    where
        T: DataPoint + Clone,
        T::X: PartialOrd + Copy,
        T::Y: PartialOrd + Copy,
    {
        if self.series.is_empty() {
            return Err(DataError::insufficient_data(
                "calculate combined bounds",
                1,
                0,
            ));
        }

        let mut combined_bounds = self.series[0].bounds()?;

        for series in self.series.iter().skip(1) {
            let series_bounds = series.bounds()?;
            combined_bounds = combined_bounds.merge(&series_bounds);
        }

        Ok(combined_bounds)
    }

    /// Clear all series
    pub fn clear(&mut self) {
        self.series.clear();
    }
}

impl<T, const SERIES: usize, const POINTS: usize> Default for MultiSeries<T, SERIES, POINTS>
where
    T: DataPoint,
{
    fn default() -> Self {
        Self::new()
    }
}

/// A sliding window data series for real-time data
#[cfg(feature = "animations")]
#[derive(Debug, Clone)]
pub struct SlidingWindowSeries<T, const N: usize>
where
    T: DataPoint + Copy,
{
    buffer: [Option<T>; N],
    head: usize,
    count: usize,
    full: bool,
    label: Option<heapless::String<32>>,
}

#[cfg(feature = "animations")]
impl<T, const N: usize> SlidingWindowSeries<T, N>
where
    T: DataPoint + Copy,
{
    /// Create a new sliding window series
    pub fn new() -> Self {
        Self {
            buffer: [None; N],
            head: 0,
            count: 0,
            full: false,
            label: None,
        }
    }

    /// Create a new sliding window series with a label
    pub fn with_label(label: &str) -> Self {
        let mut series = Self::new();
        series.set_label(label);
        series
    }

    /// Set the label for this series
    pub fn set_label(&mut self, label: &str) {
        let mut string = heapless::String::new();
        if string.push_str(label).is_ok() {
            self.label = Some(string);
        }
    }

    /// Get the label for this series
    pub fn label(&self) -> Option<&str> {
        self.label.as_ref().map(|s| s.as_str())
    }

    /// Push a new data point (may overwrite old data)
    pub fn push(&mut self, point: T) {
        self.buffer[self.head] = Some(point);
        self.head = (self.head + 1) % N;

        if self.full {
            // Overwriting old data
        } else {
            self.count += 1;
            if self.count == N {
                self.full = true;
            }
        }
    }

    /// Get the current number of data points
    pub fn current_len(&self) -> usize {
        self.count
    }

    /// Check if the buffer is full
    pub fn is_full(&self) -> bool {
        self.full
    }

    /// Get the capacity of the sliding window
    pub fn capacity(&self) -> usize {
        N
    }

    /// Clear all data
    pub fn clear(&mut self) {
        self.buffer = [None; N];
        self.head = 0;
        self.count = 0;
        self.full = false;
    }

    /// Get an iterator over the current data points in chronological order
    pub fn iter_chronological(&self) -> impl Iterator<Item = T> + '_ {
        let start_idx = if self.full { self.head } else { 0 };
        let len = if self.full { N } else { self.count };

        (0..len).filter_map(move |i| {
            let idx = (start_idx + i) % N;
            self.buffer[idx]
        })
    }
}

#[cfg(feature = "animations")]
impl<T, const N: usize> Default for SlidingWindowSeries<T, N>
where
    T: DataPoint + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "animations")]
impl<T, const N: usize> DataSeries for SlidingWindowSeries<T, N>
where
    T: DataPoint + Copy,
{
    type Item = T;
    type Iter = <heapless::Vec<T, N> as IntoIterator>::IntoIter;

    fn iter(&self) -> Self::Iter {
        let mut vec = heapless::Vec::new();
        for point in self.iter_chronological() {
            let _ = vec.push(point);
        }
        vec.into_iter()
    }

    fn len(&self) -> usize {
        self.current_len()
    }

    fn get(&self, index: usize) -> Option<Self::Item> {
        if index >= self.current_len() {
            return None;
        }

        let start_idx = if self.full { self.head } else { 0 };
        let actual_idx = (start_idx + index) % N;
        self.buffer[actual_idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::point::Point2D;

    #[test]
    fn test_static_series_creation() {
        let series: StaticDataSeries<Point2D, 10> = StaticDataSeries::new();
        assert_eq!(series.len(), 0);
        assert!(series.is_empty());
        assert_eq!(series.capacity(), 10);
    }

    #[test]
    fn test_static_series_push() {
        let mut series: StaticDataSeries<Point2D, 10> = StaticDataSeries::new();
        let point = Point2D::new(1.0, 2.0);

        series.push(point).unwrap();
        assert_eq!(series.len(), 1);
        assert_eq!(series.get(0), Some(point));
    }

    #[test]
    fn test_static_series_from_tuples() {
        let tuples = [(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)];
        let series: StaticDataSeries<Point2D, 10> = StaticDataSeries::from_tuples(&tuples).unwrap();

        assert_eq!(series.len(), 3);
        assert_eq!(series.get(0), Some(Point2D::new(1.0, 2.0)));
        assert_eq!(series.get(1), Some(Point2D::new(3.0, 4.0)));
        assert_eq!(series.get(2), Some(Point2D::new(5.0, 6.0)));
    }

    #[test]
    fn test_multi_series() {
        let mut multi: MultiSeries<Point2D, 5, 10> = MultiSeries::new();
        let mut series1 = StaticDataSeries::with_label("Series 1");
        series1.push(Point2D::new(1.0, 2.0)).unwrap();

        let index = multi.add_series(series1).unwrap();
        assert_eq!(index, 0);
        assert_eq!(multi.series_count(), 1);

        let retrieved_series = multi.get_series(0).unwrap();
        assert_eq!(retrieved_series.label(), Some("Series 1"));
        assert_eq!(retrieved_series.len(), 1);
    }

    #[cfg(feature = "animations")]
    #[test]
    fn test_sliding_window_series() {
        let mut series: SlidingWindowSeries<Point2D, 3> = SlidingWindowSeries::new();

        series.push(Point2D::new(1.0, 1.0));
        series.push(Point2D::new(2.0, 2.0));
        series.push(Point2D::new(3.0, 3.0));

        assert_eq!(series.current_len(), 3);
        assert!(series.is_full());

        // Push one more to test overwriting
        series.push(Point2D::new(4.0, 4.0));
        assert_eq!(series.current_len(), 3);

        // Check that the oldest point was overwritten
        let points: Vec<Point2D, 3> = series.iter().collect();
        assert_eq!(points.len(), 3);
        assert_eq!(points[0], Point2D::new(2.0, 2.0));
        assert_eq!(points[1], Point2D::new(3.0, 3.0));
        assert_eq!(points[2], Point2D::new(4.0, 4.0));
    }
}
