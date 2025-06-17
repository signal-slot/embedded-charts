//! Comprehensive test suite for memory.rs
//! Target: Increase coverage from 80.88% to 90%
//!
//! This test suite covers:
//! - FixedCapacityCollections factory methods
//! - MemoryStats edge cases and calculations
//! - ChartMemoryManager functionality and defaults
//! - ManagedSlidingWindow with all states and edge cases
//! - LabelStorage error handling and capacity limits
//! - Iterator implementations and edge cases
//! - Default trait implementations
//! - Memory statistics tracking and updates

use embedded_charts::{
    data::point::Point2D,
    error::DataError,
    memory::{
        ChartMemoryManager, FixedCapacityCollections, LabelStorage, ManagedSlidingWindow,
        MemoryStats,
    },
};
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use heapless::Vec;

#[test]
fn test_fixed_capacity_collections() {
    // Test data_vec creation
    let mut data_vec: Vec<Point2D, 10> = FixedCapacityCollections::data_vec();
    assert_eq!(data_vec.capacity(), 10);
    assert_eq!(data_vec.len(), 0);
    data_vec.push(Point2D::new(1.0, 2.0)).unwrap();
    assert_eq!(data_vec.len(), 1);

    // Test string_vec creation
    let mut string_vec: Vec<heapless::String<32>, 5> = FixedCapacityCollections::string_vec();
    assert_eq!(string_vec.capacity(), 5);
    assert_eq!(string_vec.len(), 0);
    let mut test_string = heapless::String::new();
    test_string.push_str("test").unwrap();
    string_vec.push(test_string).unwrap();
    assert_eq!(string_vec.len(), 1);

    // Test color_vec creation
    let mut color_vec: Vec<Rgb565, 8> = FixedCapacityCollections::color_vec();
    assert_eq!(color_vec.capacity(), 8);
    assert_eq!(color_vec.len(), 0);
    color_vec.push(Rgb565::RED).unwrap();
    assert_eq!(color_vec.len(), 1);
}

#[test]
fn test_memory_stats_edge_cases() {
    // Test with zero total allocation
    let stats = MemoryStats::new(0);
    assert_eq!(stats.total_allocated, 0);
    assert_eq!(stats.utilization_percent(), 0.0);
    assert!(!stats.is_above_threshold(50.0));

    // Test update_usage with various values
    let mut stats = MemoryStats::new(1000);

    // Initial state
    assert_eq!(stats.peak_usage, 0);

    // First update
    stats.update_usage(500);
    assert_eq!(stats.used, 500);
    assert_eq!(stats.available, 500);
    assert_eq!(stats.peak_usage, 500);

    // Update with lower value - peak should remain
    stats.update_usage(300);
    assert_eq!(stats.used, 300);
    assert_eq!(stats.available, 700);
    assert_eq!(stats.peak_usage, 500);

    // Update with higher value - peak should update
    stats.update_usage(800);
    assert_eq!(stats.used, 800);
    assert_eq!(stats.available, 200);
    assert_eq!(stats.peak_usage, 800);

    // Test saturating subtraction (usage > total)
    stats.update_usage(1200);
    assert_eq!(stats.used, 1200);
    assert_eq!(stats.available, 0); // Should saturate at 0
    assert_eq!(stats.peak_usage, 1200);
}

#[test]
fn test_memory_stats_threshold_checks() {
    let mut stats = MemoryStats::new(1000);

    stats.update_usage(0);
    assert!(!stats.is_above_threshold(0.0));

    stats.update_usage(250);
    assert!(!stats.is_above_threshold(25.0));
    assert!(stats.is_above_threshold(24.9));

    stats.update_usage(750);
    assert!(!stats.is_above_threshold(75.0));
    assert!(stats.is_above_threshold(74.9));

    stats.update_usage(1000);
    assert!(!stats.is_above_threshold(100.0));
    assert!(stats.is_above_threshold(99.9));
}

#[test]
fn test_chart_memory_manager_comprehensive() {
    let mut manager: ChartMemoryManager<2000> = ChartMemoryManager::new();

    // Test initial state
    assert_eq!(manager.stats().total_allocated, 2000);
    assert_eq!(manager.stats().used, 0);
    assert_eq!(manager.high_water_mark(), 0);

    // Test usage updates
    manager.update_usage(500);
    assert_eq!(manager.stats().used, 500);
    assert_eq!(manager.high_water_mark(), 500);

    // Update with lower value - high water mark should remain
    manager.update_usage(300);
    assert_eq!(manager.stats().used, 300);
    assert_eq!(manager.high_water_mark(), 500);

    // Update with higher value
    manager.update_usage(1500);
    assert_eq!(manager.stats().used, 1500);
    assert_eq!(manager.high_water_mark(), 1500);

    // Test memory critical checks
    assert!(!manager.is_memory_critical(80.0));
    assert!(manager.is_memory_critical(70.0));

    // Test reset
    manager.reset_stats();
    assert_eq!(manager.stats().used, 0);
    assert_eq!(manager.high_water_mark(), 0);
    assert_eq!(manager.stats().total_allocated, 2000);
}

#[test]
fn test_chart_memory_manager_default() {
    let manager: ChartMemoryManager<1024> = ChartMemoryManager::default();
    assert_eq!(manager.stats().total_allocated, 1024);
    assert_eq!(manager.stats().used, 0);
    assert_eq!(manager.high_water_mark(), 0);
}

#[test]
fn test_managed_sliding_window_comprehensive() {
    let mut window: ManagedSlidingWindow<u32, 5> = ManagedSlidingWindow::new();

    // Test initial state
    assert_eq!(window.len(), 0);
    assert!(window.is_empty());
    assert!(!window.is_full());

    // Test pushing items
    window.push(10);
    assert_eq!(window.len(), 1);
    assert!(!window.is_empty());
    assert!(!window.is_full());

    window.push(20);
    window.push(30);
    window.push(40);
    assert_eq!(window.len(), 4);
    assert!(!window.is_full());

    // Fill the window
    window.push(50);
    assert_eq!(window.len(), 5);
    assert!(window.is_full());

    // Test overwriting when full
    let stats_before = window.memory_stats().used;
    window.push(60);
    assert_eq!(window.len(), 5);
    assert!(window.is_full());
    // Memory usage should remain the same when overwriting
    assert_eq!(window.memory_stats().used, stats_before);

    // Verify the correct items are in the window
    let items: Vec<u32, 5> = window.iter().collect();
    assert_eq!(items.as_slice(), &[20, 30, 40, 50, 60]);

    // Test clear
    window.clear();
    assert_eq!(window.len(), 0);
    assert!(window.is_empty());
    assert!(!window.is_full());
    assert_eq!(window.memory_stats().used, 0);
}

#[test]
fn test_managed_sliding_window_iterator_edge_cases() {
    let mut window: ManagedSlidingWindow<i32, 4> = ManagedSlidingWindow::new();

    // Test iterator on empty window
    let empty_items: Vec<i32, 4> = window.iter().collect();
    assert_eq!(empty_items.len(), 0);

    // Test iterator with partial fill
    window.push(1);
    window.push(2);
    let partial_items: Vec<i32, 4> = window.iter().collect();
    assert_eq!(partial_items.as_slice(), &[1, 2]);

    // Test iterator after wrapping
    window.push(3);
    window.push(4);
    window.push(5); // This wraps around
    window.push(6); // This too
    let wrapped_items: Vec<i32, 4> = window.iter().collect();
    assert_eq!(wrapped_items.as_slice(), &[3, 4, 5, 6]);
}

#[test]
fn test_managed_sliding_window_memory_tracking() {
    let mut window: ManagedSlidingWindow<u64, 10> = ManagedSlidingWindow::new();

    let item_size = core::mem::size_of::<u64>();
    let total_capacity = 10 * item_size;

    assert_eq!(window.memory_stats().total_allocated, total_capacity);
    assert_eq!(window.memory_stats().used, 0);

    // Add items and check memory usage
    for i in 0..5 {
        window.push(i);
        assert_eq!(window.memory_stats().used, (i + 1) as usize * item_size);
    }

    // Fill completely
    for i in 5..10 {
        window.push(i);
    }
    assert_eq!(window.memory_stats().used, 10 * item_size);

    // Overwrite shouldn't increase memory usage
    window.push(100);
    assert_eq!(window.memory_stats().used, 10 * item_size);
}

#[test]
fn test_managed_sliding_window_default() {
    let window: ManagedSlidingWindow<f32, 8> = ManagedSlidingWindow::default();
    assert_eq!(window.len(), 0);
    assert!(window.is_empty());
    assert!(!window.is_full());
}

#[test]
fn test_label_storage_comprehensive() {
    let mut storage: LabelStorage<10, 50> = LabelStorage::new();

    // Test initial state
    assert_eq!(storage.len(), 0);
    assert!(storage.is_empty());

    // Add labels
    let idx1 = storage.add_label("First Label").unwrap();
    let idx2 = storage.add_label("Second Label").unwrap();
    let idx3 = storage.add_label("Third Label").unwrap();

    assert_eq!(storage.len(), 3);
    assert!(!storage.is_empty());

    // Verify labels
    assert_eq!(storage.get_label(idx1), Some("First Label"));
    assert_eq!(storage.get_label(idx2), Some("Second Label"));
    assert_eq!(storage.get_label(idx3), Some("Third Label"));

    // Test invalid index
    assert_eq!(storage.get_label(100), None);

    // Test memory stats
    let total_chars = "First Label".len() + "Second Label".len() + "Third Label".len();
    assert_eq!(storage.memory_stats().used, total_chars);

    // Test clear
    storage.clear();
    assert_eq!(storage.len(), 0);
    assert!(storage.is_empty());
    assert_eq!(storage.memory_stats().used, 0);
}

#[test]
fn test_label_storage_error_cases() {
    let mut storage: LabelStorage<5, 10> = LabelStorage::new();

    // Test label that's too long
    let long_label = "This is a very long label that exceeds the maximum length";
    let result = storage.add_label(long_label);
    assert!(matches!(result, Err(DataError::INVALID_DATA_POINT)));

    // Fill up the storage
    for i in 0..5 {
        storage.add_label(&format!("L{i}")).unwrap();
    }

    // Try to add one more - should fail with buffer full
    let result = storage.add_label("Overflow");
    assert!(matches!(result, Err(DataError::BUFFER_FULL)));
}

#[test]
fn test_label_storage_memory_tracking() {
    let mut storage: LabelStorage<5, 30> = LabelStorage::new();

    assert_eq!(storage.memory_stats().total_allocated, 5 * 30);
    assert_eq!(storage.memory_stats().used, 0);

    // Add labels and track memory
    storage.add_label("Short").unwrap();
    assert_eq!(storage.memory_stats().used, 5);

    storage.add_label("Medium Label").unwrap();
    assert_eq!(storage.memory_stats().used, 5 + 12);

    storage.add_label("Longer Label Here").unwrap();
    assert_eq!(storage.memory_stats().used, 5 + 12 + 17);

    // Clear and verify memory is tracked
    storage.clear();
    assert_eq!(storage.memory_stats().used, 0);
}

#[test]
fn test_label_storage_default() {
    let storage: LabelStorage<20, 100> = LabelStorage::default();
    assert_eq!(storage.len(), 0);
    assert!(storage.is_empty());
    assert_eq!(storage.memory_stats().total_allocated, 20 * 100);
}

#[test]
fn test_memory_stats_utilization_calculations() {
    let mut stats = MemoryStats::new(1000);

    // Test various utilization levels
    stats.update_usage(0);
    assert_eq!(stats.utilization_percent(), 0.0);

    stats.update_usage(250);
    assert!((stats.utilization_percent() - 25.0).abs() < 0.001);

    stats.update_usage(500);
    assert!((stats.utilization_percent() - 50.0).abs() < 0.001);

    stats.update_usage(750);
    assert!((stats.utilization_percent() - 75.0).abs() < 0.001);

    stats.update_usage(1000);
    assert!((stats.utilization_percent() - 100.0).abs() < 0.001);

    // Test over-utilization
    stats.update_usage(1500);
    assert!((stats.utilization_percent() - 150.0).abs() < 0.001);
}

#[test]
fn test_integration_scenario() {
    // Simulate a real-world usage scenario
    let mut manager: ChartMemoryManager<10000> = ChartMemoryManager::new();
    let mut data_window: ManagedSlidingWindow<f32, 100> = ManagedSlidingWindow::new();
    let mut labels: LabelStorage<10, 20> = LabelStorage::new();

    // Simulate data collection
    for i in 0..50 {
        data_window.push(i as f32 * 1.5);
        if i % 10 == 0 {
            labels.add_label(&format!("Checkpoint {}", i / 10)).unwrap();
        }
    }

    // Update manager with total usage
    let total_usage = data_window.memory_stats().used + labels.memory_stats().used;
    manager.update_usage(total_usage);

    assert!(manager.high_water_mark() > 0);
    assert!(!manager.is_memory_critical(90.0));

    // Continue to fill data window
    for i in 50..150 {
        data_window.push(i as f32 * 1.5);
    }

    // Update usage again
    let new_total_usage = data_window.memory_stats().used + labels.memory_stats().used;
    manager.update_usage(new_total_usage);

    assert_eq!(manager.high_water_mark(), new_total_usage);

    // Verify data integrity after wrapping
    let latest_values: Vec<f32, 100> = data_window.iter().collect();
    assert_eq!(latest_values.len(), 100);
    assert!((latest_values[0] - 75.0).abs() < 0.001); // Should start from value 50
    assert!((latest_values[99] - 223.5).abs() < 0.001); // Should end at value 149 * 1.5
}
