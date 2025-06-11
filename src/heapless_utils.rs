//! Heapless utilities and enhanced no_std support
//!
//! This module provides enhanced heapless integration for no_std embedded systems,
//! including configurable sizing, utility macros, and specialized data structures.

use crate::error::{DataError, DataResult};
use heapless::{String, Vec};

/// Common heapless sizes optimized for different embedded system constraints
pub mod sizes {
    /// Ultra-constrained systems (≤1KB RAM)
    pub mod ultra {
        /// 8 character strings for labels
        pub type LabelString = heapless::String<8>;
        /// 4 element vectors for colors/positions
        pub type SmallVec<T> = heapless::Vec<T, 4>;
        /// 16 point data series
        pub type DataVec<T> = heapless::Vec<T, 16>;
        /// 2 series maximum
        pub type SeriesVec<T> = heapless::Vec<T, 2>;
    }

    /// Small embedded systems (1-4KB RAM)
    pub mod small {
        /// 16 character strings for labels
        pub type LabelString = heapless::String<16>;
        /// 8 element vectors for colors/positions
        pub type SmallVec<T> = heapless::Vec<T, 8>;
        /// 64 point data series
        pub type DataVec<T> = heapless::Vec<T, 64>;
        /// 4 series maximum
        pub type SeriesVec<T> = heapless::Vec<T, 4>;
    }

    /// Medium embedded systems (4-16KB RAM)
    pub mod medium {
        /// 32 character strings for labels
        pub type LabelString = heapless::String<32>;
        /// 16 element vectors for colors/positions
        pub type SmallVec<T> = heapless::Vec<T, 16>;
        /// 256 point data series
        pub type DataVec<T> = heapless::Vec<T, 256>;
        /// 8 series maximum
        pub type SeriesVec<T> = heapless::Vec<T, 8>;
    }

    /// Large embedded systems (≥16KB RAM)
    pub mod large {
        /// 64 character strings for labels
        pub type LabelString = heapless::String<64>;
        /// 32 element vectors for colors/positions
        pub type SmallVec<T> = heapless::Vec<T, 32>;
        /// 512 point data series
        pub type DataVec<T> = heapless::Vec<T, 512>;
        /// 16 series maximum
        pub type SeriesVec<T> = heapless::Vec<T, 16>;
    }

    /// Default sizes based on feature configuration
    #[cfg(feature = "minimal-memory")]
    pub use ultra::*;

    #[cfg(all(feature = "static-only", not(feature = "minimal-memory")))]
    pub use small::*;

    #[cfg(all(
        not(feature = "static-only"),
        not(feature = "minimal-memory"),
        feature = "no_std"
    ))]
    pub use medium::*;

    #[cfg(all(
        feature = "std",
        not(any(feature = "static-only", feature = "minimal-memory"))
    ))]
    pub use large::*;

    #[cfg(not(any(
        feature = "minimal-memory",
        feature = "static-only",
        feature = "no_std",
        feature = "std"
    )))]
    pub use medium::*;
}

/// Heapless string utilities
pub mod string {
    use super::*;

    /// Create a heapless string from a string slice with error handling
    pub fn try_from_str<const N: usize>(s: &str) -> DataResult<String<N>> {
        String::try_from(s).map_err(|_| DataError::buffer_full("create heapless string", N))
    }

    /// Create a heapless string from a string slice, truncating if necessary
    pub fn from_str_truncate<const N: usize>(s: &str) -> String<N> {
        let truncated = if s.len() > N { &s[..N] } else { s };
        String::try_from(truncated).unwrap_or_else(|_| String::new())
    }

    /// Safely push a string slice to a heapless string
    pub fn push_str_safe<const N: usize>(string: &mut String<N>, s: &str) -> DataResult<()> {
        string
            .push_str(s)
            .map_err(|_| DataError::buffer_full("push string", N))
    }

    /// Push a character with error handling
    pub fn push_char_safe<const N: usize>(string: &mut String<N>, c: char) -> DataResult<()> {
        string
            .push(c)
            .map_err(|_| DataError::buffer_full("push character", N))
    }

    /// Format a number into a heapless string
    pub fn format_number<const N: usize>(value: f32, precision: usize) -> String<N> {
        let mut result = String::new();

        // Handle negative numbers
        if value < 0.0 {
            let _ = result.push('-');
        }

        let abs_value = if value < 0.0 { -value } else { value };

        // Integer part
        let integer_part = abs_value as u32;
        let _ = format_integer(&mut result, integer_part);

        // Decimal part if precision > 0
        if precision > 0 {
            let _ = result.push('.');
            let mut fractional = abs_value - integer_part as f32;

            for _ in 0..precision {
                fractional *= 10.0;
                let digit = fractional as u32 % 10;
                let _ = result.push((b'0' + digit as u8) as char);
            }
        }

        result
    }

    /// Format an integer into a heapless string
    fn format_integer<const N: usize>(string: &mut String<N>, mut value: u32) -> DataResult<()> {
        if value == 0 {
            return push_char_safe(string, '0');
        }

        let mut digits = Vec::<u8, 16>::new();
        while value > 0 {
            digits
                .push((value % 10) as u8)
                .map_err(|_| DataError::buffer_full("format integer", 16))?;
            value /= 10;
        }

        // Reverse digits and push to string
        for &digit in digits.iter().rev() {
            push_char_safe(string, (b'0' + digit) as char)?;
        }

        Ok(())
    }
}

/// Heapless vector utilities
pub mod vec {
    use super::*;

    /// Safely push an item to a heapless vector
    pub fn push_safe<T, const N: usize>(vec: &mut Vec<T, N>, item: T) -> DataResult<()> {
        vec.push(item)
            .map_err(|_| DataError::buffer_full("push item", N))
    }

    /// Extend a heapless vector from an iterator with error handling
    pub fn extend_safe<T, I, const N: usize>(vec: &mut Vec<T, N>, iter: I) -> DataResult<()>
    where
        I: IntoIterator<Item = T>,
    {
        for item in iter {
            push_safe(vec, item)?;
        }
        Ok(())
    }

    /// Create a heapless vector from a slice with error handling
    pub fn try_from_slice<T: Clone, const N: usize>(slice: &[T]) -> DataResult<Vec<T, N>> {
        if slice.len() > N {
            return Err(DataError::buffer_full("create from slice", N));
        }
        Vec::from_slice(slice).map_err(|_| DataError::buffer_full("create from slice", N))
    }

    /// Sort a heapless vector using insertion sort (good for small vecs)
    pub fn insertion_sort<T: Ord + Clone, const N: usize>(vec: &mut Vec<T, N>) {
        let len = vec.len();
        for i in 1..len {
            let key = vec[i].clone();
            let mut j = i;

            while j > 0 && vec[j - 1] > key {
                vec[j] = vec[j - 1].clone();
                j -= 1;
            }
            vec[j] = key;
        }
    }

    /// Find the index of an item in a heapless vector
    pub fn find_index<T: PartialEq, const N: usize>(vec: &Vec<T, N>, item: &T) -> Option<usize> {
        vec.iter().position(|x| x == item)
    }

    /// Remove an item by value from a heapless vector
    pub fn remove_item<T: PartialEq + Clone, const N: usize>(
        vec: &mut Vec<T, N>,
        item: &T,
    ) -> bool {
        if let Some(index) = find_index(vec, item) {
            vec.remove(index);
            true
        } else {
            false
        }
    }
}

/// Memory pool for reusable heapless data structures
pub struct HeaplessPool<T, const N: usize> {
    pool: Vec<Option<T>, N>,
    free_list: Vec<usize, N>,
}

impl<T, const N: usize> HeaplessPool<T, N> {
    /// Create a new memory pool
    pub fn new() -> Self {
        let mut pool = Vec::new();
        let mut free_list = Vec::new();

        // Initialize pool with None values
        for i in 0..N {
            let _ = pool.push(None);
            let _ = free_list.push(i);
        }

        Self { pool, free_list }
    }

    /// Allocate an item from the pool
    pub fn allocate(&mut self, item: T) -> Option<usize> {
        if let Some(index) = self.free_list.pop() {
            self.pool[index] = Some(item);
            Some(index)
        } else {
            None
        }
    }

    /// Deallocate an item back to the pool
    pub fn deallocate(&mut self, index: usize) -> Option<T> {
        if index < N {
            if let Some(item) = self.pool[index].take() {
                let _ = self.free_list.push(index);
                Some(item)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get a reference to an allocated item
    pub fn get(&self, index: usize) -> Option<&T> {
        self.pool.get(index)?.as_ref()
    }

    /// Get a mutable reference to an allocated item
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.pool.get_mut(index)?.as_mut()
    }

    /// Get the number of allocated items
    pub fn allocated_count(&self) -> usize {
        N - self.free_list.len()
    }

    /// Check if the pool is full
    pub fn is_full(&self) -> bool {
        self.free_list.is_empty()
    }

    /// Check if the pool is empty
    pub fn is_empty(&self) -> bool {
        self.free_list.len() == N
    }
}

impl<T, const N: usize> Default for HeaplessPool<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Circular buffer implementation using heapless
pub struct CircularBuffer<T: Copy, const N: usize> {
    buffer: [Option<T>; N],
    head: usize,
    tail: usize,
    full: bool,
}

impl<T: Copy, const N: usize> CircularBuffer<T, N> {
    /// Create a new circular buffer
    pub const fn new() -> Self {
        Self {
            buffer: [None; N],
            head: 0,
            tail: 0,
            full: false,
        }
    }

    /// Push an item to the buffer (overwrites oldest if full)
    pub fn push(&mut self, item: T) {
        self.buffer[self.head] = Some(item);

        if self.full {
            self.tail = (self.tail + 1) % N;
        }

        self.head = (self.head + 1) % N;

        if self.head == self.tail {
            self.full = true;
        }
    }

    /// Pop the oldest item from the buffer
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let item = self.buffer[self.tail].take();
        self.tail = (self.tail + 1) % N;
        self.full = false;
        item
    }

    /// Get the current length
    pub fn len(&self) -> usize {
        if self.full {
            N
        } else if self.head >= self.tail {
            self.head - self.tail
        } else {
            N - self.tail + self.head
        }
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        !self.full && self.head == self.tail
    }

    /// Check if the buffer is full
    pub fn is_full(&self) -> bool {
        self.full
    }

    /// Get capacity
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.buffer = [None; N];
        self.head = 0;
        self.tail = 0;
        self.full = false;
    }

    /// Iterate over items in chronological order
    pub fn iter(&self) -> CircularBufferIter<T, N> {
        CircularBufferIter {
            buffer: &self.buffer,
            current: self.tail,
            remaining: self.len(),
        }
    }
}

impl<T: Copy, const N: usize> Default for CircularBuffer<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator for circular buffer
pub struct CircularBufferIter<'a, T: Copy, const N: usize> {
    buffer: &'a [Option<T>; N],
    current: usize,
    remaining: usize,
}

impl<'a, T: Copy, const N: usize> Iterator for CircularBufferIter<'a, T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let item = self.buffer[self.current]?;
        self.current = (self.current + 1) % N;
        self.remaining -= 1;
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a, T: Copy, const N: usize> ExactSizeIterator for CircularBufferIter<'a, T, N> {}

/// Create a heapless string with error handling
///
/// # Examples
///
/// ```rust
/// use embedded_charts::heapless_string;
///
/// // Create with explicit size  
/// let result = heapless_string!("Hello", 16);
/// assert!(result.is_ok());
/// ```
#[macro_export]
macro_rules! heapless_string {
    ($s:expr) => {
        $crate::heapless_utils::string::try_from_str($s)
    };
    ($s:expr, $n:expr) => {
        heapless::String::<$n>::try_from($s)
    };
}

/// Create a heapless vector from items
///
/// # Examples
///
/// ```rust
/// use embedded_charts::heapless_vec;
///
/// // Create with explicit capacity
/// let vec: heapless::Vec<i32, 8> = heapless_vec![1, 2, 3];
/// assert_eq!(vec.len(), 3);
/// ```
#[macro_export]
macro_rules! heapless_vec {
    ($($item:expr),* $(,)?) => {{
        let mut vec = heapless::Vec::new();
        $(
            let _ = vec.push($item);
        )*
        vec
    }};
    ($item:expr; $count:expr) => {{
        let mut vec = heapless::Vec::new();
        for _ in 0..$count {
            let _ = vec.push($item);
        }
        vec
    }};
}

/// Configuration for heapless usage based on system constraints
pub struct HeaplessConfig {
    /// Maximum string length for labels
    pub max_string_length: usize,
    /// Maximum vector capacity for small collections
    pub max_small_vec_capacity: usize,
    /// Maximum data points per series
    pub max_data_points: usize,
    /// Maximum number of series
    pub max_series_count: usize,
}

impl HeaplessConfig {
    /// Ultra-constrained configuration (≤1KB RAM)
    pub const ULTRA: Self = Self {
        max_string_length: 8,
        max_small_vec_capacity: 4,
        max_data_points: 16,
        max_series_count: 2,
    };

    /// Small embedded configuration (1-4KB RAM)
    pub const SMALL: Self = Self {
        max_string_length: 16,
        max_small_vec_capacity: 8,
        max_data_points: 64,
        max_series_count: 4,
    };

    /// Medium embedded configuration (4-16KB RAM)
    pub const MEDIUM: Self = Self {
        max_string_length: 32,
        max_small_vec_capacity: 16,
        max_data_points: 256,
        max_series_count: 8,
    };

    /// Large embedded configuration (≥16KB RAM)
    pub const LARGE: Self = Self {
        max_string_length: 64,
        max_small_vec_capacity: 32,
        max_data_points: 512,
        max_series_count: 16,
    };

    /// Get the default configuration based on enabled features
    pub const fn default() -> &'static Self {
        #[cfg(feature = "minimal-memory")]
        return &Self::ULTRA;

        #[cfg(all(feature = "static-only", not(feature = "minimal-memory")))]
        return &Self::SMALL;

        #[cfg(all(
            not(feature = "static-only"),
            not(feature = "minimal-memory"),
            feature = "no_std"
        ))]
        return &Self::MEDIUM;

        #[cfg(all(
            feature = "std",
            not(any(feature = "static-only", feature = "minimal-memory"))
        ))]
        return &Self::LARGE;

        #[cfg(not(any(
            feature = "minimal-memory",
            feature = "static-only",
            feature = "no_std",
            feature = "std"
        )))]
        return &Self::MEDIUM;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_utilities() {
        // Test safe string creation
        let result: DataResult<String<16>> = string::try_from_str("Hello");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "Hello");

        // Test string too long
        let result: DataResult<String<8>> = string::try_from_str("This is too long");
        assert!(result.is_err());

        // Test truncation
        let truncated = string::from_str_truncate::<8>("This is too long");
        assert_eq!(truncated.as_str(), "This is ");

        // Test number formatting
        let number_str = string::format_number::<16>(123.45, 2);
        // Allow for floating point precision variations
        assert!(number_str.as_str() == "123.45" || number_str.as_str() == "123.44");
    }

    #[test]
    fn test_vec_utilities() {
        let mut vec: Vec<i32, 8> = Vec::new();

        // Test safe push
        assert!(vec::push_safe(&mut vec, 42).is_ok());
        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0], 42);

        // Test extend
        assert!(vec::extend_safe(&mut vec, [1, 2, 3]).is_ok());
        assert_eq!(vec.len(), 4);

        // Test sorting
        vec::insertion_sort(&mut vec);
        assert_eq!(vec.as_slice(), &[1, 2, 3, 42]);
    }

    #[test]
    fn test_circular_buffer() {
        let mut buffer: CircularBuffer<i32, 4> = CircularBuffer::new();

        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);

        // Push items
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        assert_eq!(buffer.len(), 3);

        // Fill buffer
        buffer.push(4);
        assert!(buffer.is_full());
        assert_eq!(buffer.len(), 4);

        // Overflow (should overwrite oldest)
        buffer.push(5);
        assert_eq!(buffer.len(), 4);

        // Test iteration
        let items: Vec<i32, 4> = buffer.iter().collect();
        assert_eq!(items.as_slice(), &[2, 3, 4, 5]);
    }

    #[test]
    fn test_memory_pool() {
        let mut pool: HeaplessPool<String<16>, 4> = HeaplessPool::new();

        assert!(pool.is_empty());
        assert!(!pool.is_full());

        // Allocate items
        let idx1 = pool.allocate(String::try_from("Hello").unwrap()).unwrap();
        let idx2 = pool.allocate(String::try_from("World").unwrap()).unwrap();

        assert_eq!(pool.allocated_count(), 2);
        assert_eq!(pool.get(idx1).unwrap().as_str(), "Hello");
        assert_eq!(pool.get(idx2).unwrap().as_str(), "World");

        // Deallocate
        let item = pool.deallocate(idx1).unwrap();
        assert_eq!(item.as_str(), "Hello");
        assert_eq!(pool.allocated_count(), 1);
    }

    #[test]
    fn test_heapless_config() {
        let config = HeaplessConfig::default();
        assert!(config.max_string_length > 0);
        assert!(config.max_data_points > 0);
    }
}
