//! Memory pool management for fragmentation prevention
//!
//! Provides fixed-size memory pools to minimize fragmentation in embedded systems.
//! Supports multiple pool sizes for different allocation needs.

/// Size categories for memory pools
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoolSize {
    /// Small allocations (16 bytes)
    Small,
    /// Medium allocations (64 bytes)
    Medium,
    /// Large allocations (256 bytes)
    Large,
    /// Extra large allocations (1024 bytes)
    ExtraLarge,
}

impl PoolSize {
    /// Get the actual size in bytes for this pool category
    pub const fn size_bytes(&self) -> usize {
        match self {
            PoolSize::Small => 16,
            PoolSize::Medium => 64,
            PoolSize::Large => 256,
            PoolSize::ExtraLarge => 1024,
        }
    }

    /// Determine the appropriate pool size for a given allocation size
    pub const fn from_size(size: usize) -> Option<Self> {
        if size <= 16 {
            Some(PoolSize::Small)
        } else if size <= 64 {
            Some(PoolSize::Medium)
        } else if size <= 256 {
            Some(PoolSize::Large)
        } else if size <= 1024 {
            Some(PoolSize::ExtraLarge)
        } else {
            None
        }
    }
}

/// Handle to an allocated memory block
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocationHandle {
    pool_size: PoolSize,
    index: usize,
}

/// A memory pool for a specific size category using safe allocation tracking
pub struct MemoryPool<const SIZE: usize, const COUNT: usize> {
    /// Tracks which blocks are allocated
    allocated: [bool; COUNT],
    /// Count of allocated blocks
    allocated_count: usize,
    /// Total allocations since creation
    total_allocations: usize,
    /// Total deallocations since creation
    total_deallocations: usize,
    /// Failed allocation attempts
    failed_allocations: usize,
}

impl<const SIZE: usize, const COUNT: usize> MemoryPool<SIZE, COUNT> {
    /// Create a new memory pool
    pub const fn new() -> Self {
        Self {
            allocated: [false; COUNT],
            allocated_count: 0,
            total_allocations: 0,
            total_deallocations: 0,
            failed_allocations: 0,
        }
    }

    /// Allocate a block from this pool
    pub fn allocate(&mut self) -> Option<usize> {
        for (idx, allocated) in self.allocated.iter_mut().enumerate() {
            if !*allocated {
                *allocated = true;
                self.allocated_count += 1;
                self.total_allocations += 1;
                return Some(idx);
            }
        }
        self.failed_allocations += 1;
        None
    }

    /// Deallocate a block back to this pool
    pub fn deallocate(&mut self, index: usize) -> bool {
        if index < COUNT && self.allocated[index] {
            self.allocated[index] = false;
            self.allocated_count = self.allocated_count.saturating_sub(1);
            self.total_deallocations += 1;
            true
        } else {
            false
        }
    }

    /// Get the number of currently allocated blocks
    pub const fn allocated_count(&self) -> usize {
        self.allocated_count
    }

    /// Get the number of available blocks
    pub const fn available_count(&self) -> usize {
        COUNT - self.allocated_count
    }

    /// Get the total capacity of this pool
    pub const fn capacity(&self) -> usize {
        COUNT
    }

    /// Get the fragmentation percentage (0.0 to 100.0)
    pub fn fragmentation_percent(&self) -> f32 {
        if COUNT == 0 {
            0.0
        } else {
            let fragmented = self.count_fragmented_blocks();
            (fragmented as f32 / COUNT as f32) * 100.0
        }
    }

    /// Count fragmented blocks (non-contiguous free blocks)
    fn count_fragmented_blocks(&self) -> usize {
        let mut fragmented = 0;
        let mut prev_was_free = false;
        let mut found_used = false;

        for &is_allocated in &self.allocated {
            if is_allocated {
                found_used = true;
                prev_was_free = false;
            } else {
                if found_used && !prev_was_free {
                    fragmented += 1;
                }
                prev_was_free = true;
            }
        }

        fragmented
    }

    /// Get statistics about this pool
    pub fn stats(&self) -> PoolStats {
        PoolStats {
            size: SIZE,
            capacity: COUNT,
            allocated: self.allocated_count,
            available: COUNT - self.allocated_count,
            total_allocations: self.total_allocations,
            total_deallocations: self.total_deallocations,
            failed_allocations: self.failed_allocations,
            fragmentation_percent: self.fragmentation_percent(),
        }
    }

    /// Reset the pool, deallocating all blocks
    pub fn reset(&mut self) {
        self.allocated = [false; COUNT];
        self.allocated_count = 0;
        // Keep statistics for analysis
    }
}

impl<const SIZE: usize, const COUNT: usize> Default for MemoryPool<SIZE, COUNT> {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics for a memory pool
#[derive(Debug, Clone, Copy)]
pub struct PoolStats {
    /// Size of each block in bytes
    pub size: usize,
    /// Total capacity (number of blocks)
    pub capacity: usize,
    /// Currently allocated blocks
    pub allocated: usize,
    /// Currently available blocks
    pub available: usize,
    /// Total allocations since creation
    pub total_allocations: usize,
    /// Total deallocations since creation
    pub total_deallocations: usize,
    /// Failed allocation attempts
    pub failed_allocations: usize,
    /// Fragmentation percentage
    pub fragmentation_percent: f32,
}

/// Memory pool manager with multiple pool sizes
pub struct MemoryPoolManager<
    const SMALL_COUNT: usize,
    const MEDIUM_COUNT: usize,
    const LARGE_COUNT: usize,
    const XL_COUNT: usize,
> {
    small_pool: MemoryPool<16, SMALL_COUNT>,
    medium_pool: MemoryPool<64, MEDIUM_COUNT>,
    large_pool: MemoryPool<256, LARGE_COUNT>,
    xl_pool: MemoryPool<1024, XL_COUNT>,
    fragmentation_threshold: f32,
}

impl<
        const SMALL_COUNT: usize,
        const MEDIUM_COUNT: usize,
        const LARGE_COUNT: usize,
        const XL_COUNT: usize,
    > MemoryPoolManager<SMALL_COUNT, MEDIUM_COUNT, LARGE_COUNT, XL_COUNT>
{
    /// Create a new memory pool manager
    pub const fn new() -> Self {
        Self {
            small_pool: MemoryPool::new(),
            medium_pool: MemoryPool::new(),
            large_pool: MemoryPool::new(),
            xl_pool: MemoryPool::new(),
            fragmentation_threshold: 50.0,
        }
    }

    /// Set the fragmentation threshold percentage
    pub fn set_fragmentation_threshold(&mut self, threshold: f32) {
        self.fragmentation_threshold = threshold.clamp(0.0, 100.0);
    }

    /// Allocate memory from the appropriate pool
    pub fn allocate(&mut self, size: usize) -> Option<AllocationHandle> {
        match PoolSize::from_size(size) {
            Some(PoolSize::Small) => self.small_pool.allocate().map(|index| AllocationHandle {
                pool_size: PoolSize::Small,
                index,
            }),
            Some(PoolSize::Medium) => self.medium_pool.allocate().map(|index| AllocationHandle {
                pool_size: PoolSize::Medium,
                index,
            }),
            Some(PoolSize::Large) => self.large_pool.allocate().map(|index| AllocationHandle {
                pool_size: PoolSize::Large,
                index,
            }),
            Some(PoolSize::ExtraLarge) => self.xl_pool.allocate().map(|index| AllocationHandle {
                pool_size: PoolSize::ExtraLarge,
                index,
            }),
            None => None,
        }
    }

    /// Deallocate memory back to the appropriate pool
    pub fn deallocate(&mut self, handle: AllocationHandle) -> bool {
        match handle.pool_size {
            PoolSize::Small => self.small_pool.deallocate(handle.index),
            PoolSize::Medium => self.medium_pool.deallocate(handle.index),
            PoolSize::Large => self.large_pool.deallocate(handle.index),
            PoolSize::ExtraLarge => self.xl_pool.deallocate(handle.index),
        }
    }

    /// Check if any pool exceeds the fragmentation threshold
    pub fn is_fragmentation_critical(&self) -> bool {
        self.small_pool.fragmentation_percent() > self.fragmentation_threshold
            || self.medium_pool.fragmentation_percent() > self.fragmentation_threshold
            || self.large_pool.fragmentation_percent() > self.fragmentation_threshold
            || self.xl_pool.fragmentation_percent() > self.fragmentation_threshold
    }

    /// Get statistics for all pools
    pub fn all_stats(&self) -> [PoolStats; 4] {
        [
            self.small_pool.stats(),
            self.medium_pool.stats(),
            self.large_pool.stats(),
            self.xl_pool.stats(),
        ]
    }

    /// Get statistics for a specific pool size
    pub fn pool_stats(&self, pool_size: PoolSize) -> PoolStats {
        match pool_size {
            PoolSize::Small => self.small_pool.stats(),
            PoolSize::Medium => self.medium_pool.stats(),
            PoolSize::Large => self.large_pool.stats(),
            PoolSize::ExtraLarge => self.xl_pool.stats(),
        }
    }

    /// Get total memory usage across all pools
    pub fn total_memory_usage(&self) -> MemoryUsage {
        let stats = self.all_stats();

        let total_capacity = stats.iter().map(|s| s.size * s.capacity).sum();

        let total_allocated = stats.iter().map(|s| s.size * s.allocated).sum();

        let total_available = stats.iter().map(|s| s.size * s.available).sum();

        MemoryUsage {
            total_capacity,
            total_allocated,
            total_available,
            utilization_percent: if total_capacity > 0 {
                (total_allocated as f32 / total_capacity as f32) * 100.0
            } else {
                0.0
            },
        }
    }

    /// Defragment pools by compacting allocations
    /// Returns the number of blocks that were moved
    pub fn defragment(&mut self) -> usize {
        // In this safe implementation, defragmentation would require
        // a different approach, such as allocation migration callbacks
        // For now, return 0 as defragmentation requires application cooperation
        0
    }

    /// Reset all pools
    pub fn reset_all(&mut self) {
        self.small_pool.reset();
        self.medium_pool.reset();
        self.large_pool.reset();
        self.xl_pool.reset();
    }
}

impl<
        const SMALL_COUNT: usize,
        const MEDIUM_COUNT: usize,
        const LARGE_COUNT: usize,
        const XL_COUNT: usize,
    > Default for MemoryPoolManager<SMALL_COUNT, MEDIUM_COUNT, LARGE_COUNT, XL_COUNT>
{
    fn default() -> Self {
        Self::new()
    }
}

/// Total memory usage across all pools
#[derive(Debug, Clone, Copy)]
pub struct MemoryUsage {
    /// Total capacity in bytes
    pub total_capacity: usize,
    /// Total allocated in bytes
    pub total_allocated: usize,
    /// Total available in bytes
    pub total_available: usize,
    /// Utilization percentage
    pub utilization_percent: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_size_from_size() {
        assert_eq!(PoolSize::from_size(8), Some(PoolSize::Small));
        assert_eq!(PoolSize::from_size(16), Some(PoolSize::Small));
        assert_eq!(PoolSize::from_size(17), Some(PoolSize::Medium));
        assert_eq!(PoolSize::from_size(64), Some(PoolSize::Medium));
        assert_eq!(PoolSize::from_size(65), Some(PoolSize::Large));
        assert_eq!(PoolSize::from_size(256), Some(PoolSize::Large));
        assert_eq!(PoolSize::from_size(257), Some(PoolSize::ExtraLarge));
        assert_eq!(PoolSize::from_size(1024), Some(PoolSize::ExtraLarge));
        assert_eq!(PoolSize::from_size(1025), None);
    }

    #[test]
    fn test_memory_pool_allocation() {
        let mut pool: MemoryPool<16, 4> = MemoryPool::new();

        assert_eq!(pool.allocated_count(), 0);
        assert_eq!(pool.available_count(), 4);

        // Allocate blocks
        let idx1 = pool.allocate().unwrap();
        assert_eq!(pool.allocated_count(), 1);
        assert_eq!(idx1, 0);

        let _idx2 = pool.allocate().unwrap();
        let _idx3 = pool.allocate().unwrap();
        let _idx4 = pool.allocate().unwrap();

        assert_eq!(pool.allocated_count(), 4);
        assert_eq!(pool.available_count(), 0);

        // Pool is full, allocation should fail
        assert!(pool.allocate().is_none());
        assert_eq!(pool.stats().failed_allocations, 1);
    }

    #[test]
    fn test_memory_pool_deallocation() {
        let mut pool: MemoryPool<16, 2> = MemoryPool::new();

        let idx1 = pool.allocate().unwrap();
        assert_eq!(pool.allocated_count(), 1);

        // Deallocate
        assert!(pool.deallocate(idx1));
        assert_eq!(pool.allocated_count(), 0);
        assert_eq!(pool.available_count(), 2);

        // Can allocate again
        let new_idx = pool.allocate().unwrap();
        assert_eq!(new_idx, 0); // Reuses the same slot
    }

    #[test]
    fn test_fragmentation_detection() {
        let mut pool: MemoryPool<16, 4> = MemoryPool::new();

        // Allocate all blocks
        let indices: heapless::Vec<usize, 4> = (0..4).map(|_| pool.allocate().unwrap()).collect();

        // Deallocate blocks 1 and 3, creating fragmentation
        pool.deallocate(indices[1]);
        pool.deallocate(indices[3]);

        // Should have fragmented blocks
        assert!(pool.fragmentation_percent() > 0.0);
    }

    #[test]
    fn test_memory_pool_manager() {
        let mut manager: MemoryPoolManager<4, 4, 2, 1> = MemoryPoolManager::new();

        // Allocate different sizes
        let small_handle = manager.allocate(10).unwrap();
        assert_eq!(small_handle.pool_size, PoolSize::Small);

        let medium_handle = manager.allocate(50).unwrap();
        assert_eq!(medium_handle.pool_size, PoolSize::Medium);

        let large_handle = manager.allocate(200).unwrap();
        assert_eq!(large_handle.pool_size, PoolSize::Large);

        let xl_handle = manager.allocate(800).unwrap();
        assert_eq!(xl_handle.pool_size, PoolSize::ExtraLarge);

        // Allocation too large should fail
        assert!(manager.allocate(2000).is_none());

        // Check usage
        let usage = manager.total_memory_usage();
        assert!(usage.total_allocated > 0);
        assert!(usage.utilization_percent > 0.0);

        // Deallocate
        assert!(manager.deallocate(small_handle));
        assert!(manager.deallocate(medium_handle));
        assert!(manager.deallocate(large_handle));
        assert!(manager.deallocate(xl_handle));

        let usage_after = manager.total_memory_usage();
        assert_eq!(usage_after.total_allocated, 0);
    }

    #[test]
    fn test_pool_reset() {
        let mut pool: MemoryPool<16, 4> = MemoryPool::new();

        // Allocate all blocks
        for _ in 0..4 {
            pool.allocate().unwrap();
        }

        assert_eq!(pool.allocated_count(), 4);

        // Reset
        pool.reset();

        assert_eq!(pool.allocated_count(), 0);
        assert_eq!(pool.available_count(), 4);

        // Should be able to allocate again
        assert!(pool.allocate().is_some());
    }
}
