//! Memory management utilities for embedded environments.

use crate::data::DataPoint;
use crate::error::{DataError, DataResult};
use heapless::Vec;

// Memory pool management
pub mod pool;
pub use pool::{AllocationHandle, MemoryPoolManager, MemoryUsage, PoolSize, PoolStats};

/// Fixed-capacity collections wrapper for chart data
pub struct FixedCapacityCollections;

impl FixedCapacityCollections {
    /// Create a fixed-capacity vector for data points
    pub fn data_vec<T: DataPoint, const N: usize>() -> Vec<T, N> {
        Vec::new()
    }

    /// Create a fixed-capacity vector for strings (labels, etc.)
    pub fn string_vec<const N: usize, const S: usize>() -> Vec<heapless::String<S>, N> {
        Vec::new()
    }

    /// Create a fixed-capacity vector for colors
    pub fn color_vec<C: Copy, const N: usize>() -> Vec<C, N> {
        Vec::new()
    }
}

/// Memory usage statistics for monitoring
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryStats {
    /// Total memory allocated for chart data
    pub total_allocated: usize,
    /// Memory currently in use
    pub used: usize,
    /// Available memory
    pub available: usize,
    /// Peak memory usage
    pub peak_usage: usize,
}

impl MemoryStats {
    /// Create new memory statistics
    pub const fn new(total: usize) -> Self {
        Self {
            total_allocated: total,
            used: 0,
            available: total,
            peak_usage: 0,
        }
    }

    /// Update memory usage
    pub fn update_usage(&mut self, used: usize) {
        self.used = used;
        self.available = self.total_allocated.saturating_sub(used);
        if used > self.peak_usage {
            self.peak_usage = used;
        }
    }

    /// Get memory utilization as a percentage
    pub fn utilization_percent(&self) -> f32 {
        if self.total_allocated == 0 {
            0.0
        } else {
            (self.used as f32 / self.total_allocated as f32) * 100.0
        }
    }

    /// Check if memory usage is above a threshold
    pub fn is_above_threshold(&self, threshold_percent: f32) -> bool {
        self.utilization_percent() > threshold_percent
    }
}

/// Memory manager for chart operations
pub struct ChartMemoryManager<const POOL_SIZE: usize> {
    stats: MemoryStats,
    high_water_mark: usize,
}

impl<const POOL_SIZE: usize> ChartMemoryManager<POOL_SIZE> {
    /// Create a new memory manager
    pub fn new() -> Self {
        Self {
            stats: MemoryStats::new(POOL_SIZE),
            high_water_mark: 0,
        }
    }
}

impl<const POOL_SIZE: usize> Default for ChartMemoryManager<POOL_SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const POOL_SIZE: usize> ChartMemoryManager<POOL_SIZE> {
    /// Get current memory statistics
    pub fn stats(&self) -> &MemoryStats {
        &self.stats
    }

    /// Update memory usage statistics
    pub fn update_usage(&mut self, used: usize) {
        self.stats.update_usage(used);
        if used > self.high_water_mark {
            self.high_water_mark = used;
        }
    }

    /// Get the high water mark (peak usage)
    pub fn high_water_mark(&self) -> usize {
        self.high_water_mark
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = MemoryStats::new(POOL_SIZE);
        self.high_water_mark = 0;
    }

    /// Check if memory is critically low
    pub fn is_memory_critical(&self, threshold: f32) -> bool {
        self.stats.is_above_threshold(threshold)
    }
}

/// Sliding window buffer for real-time data with memory management
#[derive(Debug, Clone)]
pub struct ManagedSlidingWindow<T: Copy, const N: usize> {
    buffer: [Option<T>; N],
    head: usize,
    count: usize,
    full: bool,
    memory_stats: MemoryStats,
}

impl<T: Copy, const N: usize> ManagedSlidingWindow<T, N> {
    /// Create a new managed sliding window
    pub fn new() -> Self {
        Self {
            buffer: [None; N],
            head: 0,
            count: 0,
            full: false,
            memory_stats: MemoryStats::new(N * core::mem::size_of::<T>()),
        }
    }

    /// Push a new item into the window
    pub fn push(&mut self, item: T) {
        self.buffer[self.head] = Some(item);
        self.head = (self.head + 1) % N;

        if self.full {
            // Overwriting old data, memory usage stays the same
        } else {
            self.count += 1;
            if self.count == N {
                self.full = true;
            }
            self.update_memory_stats();
        }
    }

    /// Get the current number of items
    pub fn len(&self) -> usize {
        self.count
    }

    /// Check if the window is empty
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Check if the window is full
    pub fn is_full(&self) -> bool {
        self.full
    }

    /// Get memory statistics
    pub fn memory_stats(&self) -> &MemoryStats {
        &self.memory_stats
    }

    /// Clear all data
    pub fn clear(&mut self) {
        self.buffer = [None; N];
        self.head = 0;
        self.count = 0;
        self.full = false;
        self.update_memory_stats();
    }

    /// Get an iterator over the current items
    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        let start_idx = if self.full { self.head } else { 0 };
        let len = if self.full { N } else { self.count };

        (0..len).filter_map(move |i| {
            let idx = (start_idx + i) % N;
            self.buffer[idx]
        })
    }

    /// Update memory statistics
    fn update_memory_stats(&mut self) {
        let used = self.count * core::mem::size_of::<T>();
        self.memory_stats.update_usage(used);
    }
}

impl<T: Copy, const N: usize> Default for ManagedSlidingWindow<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory-efficient string storage for chart labels
#[derive(Debug, Clone)]
pub struct LabelStorage<const MAX_LABELS: usize, const MAX_LENGTH: usize> {
    labels: Vec<heapless::String<MAX_LENGTH>, MAX_LABELS>,
    memory_stats: MemoryStats,
}

impl<const MAX_LABELS: usize, const MAX_LENGTH: usize> LabelStorage<MAX_LABELS, MAX_LENGTH> {
    /// Create new label storage
    pub fn new() -> Self {
        Self {
            labels: Vec::new(),
            memory_stats: MemoryStats::new(MAX_LABELS * MAX_LENGTH),
        }
    }

    /// Add a label to storage
    pub fn add_label(&mut self, label: &str) -> DataResult<usize> {
        let mut string = heapless::String::new();
        if string.push_str(label).is_err() {
            return Err(DataError::INVALID_DATA_POINT);
        }
        let index = self.labels.len();
        self.labels
            .push(string)
            .map_err(|_| DataError::BUFFER_FULL)?;
        self.update_memory_stats();
        Ok(index)
    }

    /// Get a label by index
    pub fn get_label(&self, index: usize) -> Option<&str> {
        self.labels.get(index).map(|s| s.as_str())
    }

    /// Get the number of labels
    pub fn len(&self) -> usize {
        self.labels.len()
    }

    /// Check if storage is empty
    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }

    /// Clear all labels
    pub fn clear(&mut self) {
        self.labels.clear();
        self.update_memory_stats();
    }

    /// Get memory statistics
    pub fn memory_stats(&self) -> &MemoryStats {
        &self.memory_stats
    }

    /// Update memory statistics
    fn update_memory_stats(&mut self) {
        let used = self.labels.iter().map(|s| s.len()).sum::<usize>();
        self.memory_stats.update_usage(used);
    }
}

impl<const MAX_LABELS: usize, const MAX_LENGTH: usize> Default
    for LabelStorage<MAX_LABELS, MAX_LENGTH>
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_stats() {
        let mut stats = MemoryStats::new(1000);
        assert_eq!(stats.total_allocated, 1000);
        assert_eq!(stats.used, 0);
        assert_eq!(stats.available, 1000);

        stats.update_usage(300);
        assert_eq!(stats.used, 300);
        assert_eq!(stats.available, 700);
        assert!((stats.utilization_percent() - 30.0).abs() < 0.001);
    }

    #[test]
    fn test_managed_sliding_window() {
        let mut window: ManagedSlidingWindow<i32, 3> = ManagedSlidingWindow::new();
        assert!(window.is_empty());
        assert!(!window.is_full());

        window.push(1);
        window.push(2);
        window.push(3);

        assert_eq!(window.len(), 3);
        assert!(window.is_full());

        // Test overwriting
        window.push(4);
        assert_eq!(window.len(), 3);

        let values: Vec<i32, 3> = window.iter().collect();
        assert_eq!(values.as_slice(), &[2, 3, 4]);
    }

    #[test]
    fn test_label_storage() {
        let mut storage: LabelStorage<5, 20> = LabelStorage::new();
        assert!(storage.is_empty());

        let index1 = storage.add_label("Label 1").unwrap();
        let index2 = storage.add_label("Label 2").unwrap();

        assert_eq!(storage.len(), 2);
        assert_eq!(storage.get_label(index1), Some("Label 1"));
        assert_eq!(storage.get_label(index2), Some("Label 2"));
    }

    #[test]
    fn test_memory_manager() {
        let mut manager: ChartMemoryManager<1000> = ChartMemoryManager::new();
        assert_eq!(manager.stats().total_allocated, 1000);

        manager.update_usage(500);
        assert_eq!(manager.high_water_mark(), 500);
        assert!(!manager.is_memory_critical(60.0));
        assert!(manager.is_memory_critical(40.0));
    }
}
