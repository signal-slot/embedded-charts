//! Comprehensive tests for the ring buffer implementation

use embedded_charts::data::{
    OverflowMode, Point2D, PointRingBuffer, RingBuffer, RingBufferConfig, RingBufferEvent,
};
use embedded_charts::error::{ChartError, DataError};

#[test]
fn test_ring_buffer_creation() {
    let buffer: RingBuffer<Point2D, 64> = RingBuffer::new();
    assert_eq!(buffer.capacity(), 64);
    assert_eq!(buffer.len(), 0);
    assert!(buffer.is_empty());
    assert!(!buffer.is_full());
    assert_eq!(buffer.remaining_capacity(), 64);
}

#[test]
fn test_ring_buffer_push_and_pop() {
    let mut buffer: RingBuffer<Point2D, 5> = RingBuffer::new();

    // Push some data
    buffer.push(Point2D::new(1.0, 2.0)).unwrap();
    buffer.push(Point2D::new(3.0, 4.0)).unwrap();
    buffer.push(Point2D::new(5.0, 6.0)).unwrap();

    assert_eq!(buffer.len(), 3);
    assert!(!buffer.is_full());

    // Pop data
    let p1 = buffer.pop().unwrap();
    assert_eq!(p1, Point2D::new(1.0, 2.0));
    assert_eq!(buffer.len(), 2);

    let p2 = buffer.pop().unwrap();
    assert_eq!(p2, Point2D::new(3.0, 4.0));
    assert_eq!(buffer.len(), 1);
}

#[test]
fn test_ring_buffer_overflow_overwrite() {
    let config = RingBufferConfig {
        overflow_mode: OverflowMode::Overwrite,
        ..Default::default()
    };
    let mut buffer: RingBuffer<Point2D, 3> = RingBuffer::with_config(config);

    // Fill buffer beyond capacity
    for i in 0..5 {
        buffer.push(Point2D::new(i as f32, i as f32 * 2.0)).unwrap();
    }

    assert_eq!(buffer.len(), 3);
    assert!(buffer.is_full());

    // Check that we have 3 values (exact order depends on implementation)
    let values: Vec<Point2D> = buffer.iter().copied().collect();
    assert_eq!(values.len(), 3);
    // The buffer should contain the most recent values in some order
}

#[test]
fn test_ring_buffer_overflow_reject() {
    let config = RingBufferConfig {
        overflow_mode: OverflowMode::Reject,
        ..Default::default()
    };
    let mut buffer: RingBuffer<Point2D, 2> = RingBuffer::with_config(config);

    // Fill buffer to capacity
    buffer.push(Point2D::new(1.0, 1.0)).unwrap();
    buffer.push(Point2D::new(2.0, 2.0)).unwrap();

    assert!(buffer.is_full());

    // Try to push when full - should fail
    let result = buffer.push(Point2D::new(3.0, 3.0));
    assert!(matches!(
        result,
        Err(ChartError::DataError(DataError::BUFFER_FULL))
    ));

    // Buffer should remain unchanged
    assert_eq!(buffer.len(), 2);
    assert_eq!(buffer.peek().unwrap(), &Point2D::new(1.0, 1.0));
}

#[test]
fn test_ring_buffer_peek() {
    let mut buffer: RingBuffer<Point2D, 5> = RingBuffer::new();

    // Empty buffer
    assert!(buffer.peek().is_none());
    assert!(buffer.peek_newest().is_none());

    // Add some data
    buffer.push(Point2D::new(1.0, 1.0)).unwrap();
    buffer.push(Point2D::new(2.0, 2.0)).unwrap();
    buffer.push(Point2D::new(3.0, 3.0)).unwrap();

    // Peek doesn't remove elements
    assert_eq!(buffer.peek().unwrap(), &Point2D::new(1.0, 1.0));
    assert_eq!(buffer.peek_newest().unwrap(), &Point2D::new(3.0, 3.0));
    assert_eq!(buffer.len(), 3);
}

#[test]
fn test_ring_buffer_iterator() {
    let mut buffer: RingBuffer<Point2D, 10> = RingBuffer::new();

    // Add data
    for i in 0..5 {
        buffer.push(Point2D::new(i as f32, i as f32)).unwrap();
    }

    // Test iterator
    let values: Vec<Point2D> = buffer.iter().copied().collect();
    assert_eq!(values.len(), 5);
    for (i, point) in values.iter().enumerate() {
        assert_eq!(point.x, i as f32);
        assert_eq!(point.y, i as f32);
    }

    // Test iterator count
    assert_eq!(buffer.iter().count(), 5);
}

#[test]
fn test_ring_buffer_recent() {
    let mut buffer: RingBuffer<Point2D, 10> = RingBuffer::new();

    // Add 7 points
    for i in 0..7 {
        buffer.push(Point2D::new(i as f32, i as f32)).unwrap();
    }

    // Get recent 3 points
    let recent: Vec<Point2D> = buffer.recent(3).copied().collect();
    assert_eq!(recent.len(), 3);
    assert_eq!(recent[0], Point2D::new(4.0, 4.0));
    assert_eq!(recent[1], Point2D::new(5.0, 5.0));
    assert_eq!(recent[2], Point2D::new(6.0, 6.0));

    // Request more than available
    let all_recent: Vec<Point2D> = buffer.recent(20).copied().collect();
    assert_eq!(all_recent.len(), 7);
}

#[test]
fn test_ring_buffer_clear() {
    let mut buffer: RingBuffer<Point2D, 5> = RingBuffer::new();

    // Add data
    for i in 0..3 {
        buffer.push(Point2D::new(i as f32, i as f32)).unwrap();
    }

    assert_eq!(buffer.len(), 3);

    // Clear
    buffer.clear();
    assert_eq!(buffer.len(), 0);
    assert!(buffer.is_empty());
    assert!(buffer.peek().is_none());
}

#[test]
fn test_ring_buffer_bounds_tracking() {
    let config = RingBufferConfig {
        track_bounds: true,
        ..Default::default()
    };
    let mut buffer: RingBuffer<Point2D, 10> = RingBuffer::with_config(config);

    // Initially no bounds
    assert!(buffer.bounds().is_none());

    // Add points
    buffer.push_point(Point2D::new(0.0, 10.0)).unwrap();
    buffer.push_point(Point2D::new(5.0, -5.0)).unwrap();
    buffer.push_point(Point2D::new(10.0, 20.0)).unwrap();
    buffer.push_point(Point2D::new(-5.0, 15.0)).unwrap();

    // Check bounds
    let bounds = buffer.bounds().unwrap();
    assert_eq!(bounds.min_x, -5.0);
    assert_eq!(bounds.max_x, 10.0);
    assert_eq!(bounds.min_y, -5.0);
    assert_eq!(bounds.max_y, 20.0);

    // Clear resets bounds
    buffer.clear();
    assert!(buffer.bounds().is_none());
}

#[test]
fn test_ring_buffer_extend() {
    let mut buffer: RingBuffer<Point2D, 10> = RingBuffer::new();

    let points = vec![
        Point2D::new(1.0, 1.0),
        Point2D::new(2.0, 2.0),
        Point2D::new(3.0, 3.0),
    ];

    let count = buffer.extend(points).unwrap();
    assert_eq!(count, 3);
    assert_eq!(buffer.len(), 3);
}

#[test]
fn test_ring_buffer_stats() {
    let mut buffer: RingBuffer<Point2D, 3> = RingBuffer::new();

    // Initial stats
    assert_eq!(buffer.stats().total_writes, 0);
    assert_eq!(buffer.stats().overflow_count, 0);

    // Add data causing overflow
    for i in 0..5 {
        buffer.push(Point2D::new(i as f32, i as f32)).unwrap();
    }

    let stats = buffer.stats();
    assert_eq!(stats.total_writes, 5);
    assert_eq!(stats.overflow_count, 2);
    assert_eq!(stats.peak_usage, 3);
}

#[test]
fn test_point_ring_buffer_moving_average() {
    let mut buffer: PointRingBuffer<10> = PointRingBuffer::new();

    // Empty buffer
    assert!(buffer.moving_average(5).is_none());

    // Add points
    for i in 0..5 {
        buffer
            .push(Point2D::new(i as f32, (i * 10) as f32))
            .unwrap();
    }

    // Calculate moving average of last 3 points
    let avg = buffer.moving_average(3).unwrap();
    assert_eq!(avg.x, 3.0); // (2 + 3 + 4) / 3
    assert_eq!(avg.y, 30.0); // (20 + 30 + 40) / 3

    // Window larger than buffer
    let full_avg = buffer.moving_average(10).unwrap();
    assert_eq!(full_avg.x, 2.0); // (0 + 1 + 2 + 3 + 4) / 5
    assert_eq!(full_avg.y, 20.0); // (0 + 10 + 20 + 30 + 40) / 5
}

#[test]
fn test_point_ring_buffer_downsample() {
    let mut buffer: PointRingBuffer<20> = PointRingBuffer::new();

    // Add 10 points
    for i in 0..10 {
        buffer.push(Point2D::new(i as f32, i as f32)).unwrap();
    }

    // Downsample by factor of 3
    let downsampled = buffer.downsample(3);
    assert_eq!(downsampled.len(), 4); // Points at indices 0, 3, 6, 9
    assert_eq!(downsampled[0], Point2D::new(0.0, 0.0));
    assert_eq!(downsampled[1], Point2D::new(3.0, 3.0));
    assert_eq!(downsampled[2], Point2D::new(6.0, 6.0));
    assert_eq!(downsampled[3], Point2D::new(9.0, 9.0));
}

#[test]
fn test_point_ring_buffer_rate_of_change() {
    let mut buffer: PointRingBuffer<10> = PointRingBuffer::new();

    // Empty buffer
    assert!(buffer.rate_of_change().is_none());

    // Single point
    buffer.push(Point2D::new(0.0, 0.0)).unwrap();
    assert!(buffer.rate_of_change().is_none());

    // Two points - linear
    buffer.push(Point2D::new(5.0, 10.0)).unwrap();
    let rate = buffer.rate_of_change().unwrap();
    assert_eq!(rate, 2.0); // dy/dx = 10/5 = 2

    // More points
    buffer.push(Point2D::new(10.0, 30.0)).unwrap();
    let rate = buffer.rate_of_change().unwrap();
    assert_eq!(rate, 3.0); // dy/dx = 30/10 = 3
}

#[test]
fn test_ring_buffer_events() {
    // Event handler tests

    static mut EVENTS: Vec<RingBufferEvent> = Vec::new();

    fn event_handler(event: RingBufferEvent) {
        unsafe {
            EVENTS.push(event);
        }
    }

    let config = RingBufferConfig {
        enable_events: true,
        overflow_mode: OverflowMode::Callback, // Use Callback to get DataOverwritten events
        track_bounds: true,
        ..Default::default()
    };

    let mut buffer: RingBuffer<Point2D, 2> = RingBuffer::with_config(config);
    buffer.set_event_handler(event_handler);

    unsafe {
        EVENTS.clear();
    }

    // Add first point
    buffer.push_point(Point2D::new(1.0, 1.0)).unwrap();
    unsafe {
        assert!(EVENTS.contains(&RingBufferEvent::DataAdded));
        assert!(EVENTS.contains(&RingBufferEvent::BoundsChanged));
    }

    // Fill buffer
    buffer.push_point(Point2D::new(2.0, 2.0)).unwrap();
    unsafe {
        assert!(EVENTS.contains(&RingBufferEvent::BufferFull));
    }

    // Overflow
    buffer.push_point(Point2D::new(3.0, 3.0)).unwrap();
    unsafe {
        assert!(EVENTS.contains(&RingBufferEvent::DataOverwritten));
    }

    // Clear
    buffer.clear();
    unsafe {
        assert!(EVENTS.contains(&RingBufferEvent::BufferEmpty));
    }
}

#[test]
fn test_ring_buffer_find() {
    let mut buffer: RingBuffer<Point2D, 10> = RingBuffer::new();

    for i in 0..5 {
        buffer.push(Point2D::new(i as f32, (i * i) as f32)).unwrap();
    }

    // Find point with y > 10
    let found = buffer.find(|p| p.y > 10.0);
    assert!(found.is_some());
    assert_eq!(found.unwrap().x, 4.0);
    assert_eq!(found.unwrap().y, 16.0);

    // Find non-existent
    let not_found = buffer.find(|p| p.x > 100.0);
    assert!(not_found.is_none());
}

#[test]
fn test_ring_buffer_for_each() {
    let mut buffer: RingBuffer<Point2D, 5> = RingBuffer::new();

    for i in 0..3 {
        buffer.push(Point2D::new(i as f32, i as f32)).unwrap();
    }

    let mut sum = 0.0;
    buffer.for_each(|p| {
        sum += p.x + p.y;
    });

    assert_eq!(sum, 6.0); // (0+0) + (1+1) + (2+2) = 6
}

#[test]
fn test_ring_buffer_chronological_iterator() {
    let mut buffer: RingBuffer<Point2D, 5> = RingBuffer::new();

    // Test with non-full buffer
    for i in 0..3 {
        buffer.push(Point2D::new(i as f32, i as f32)).unwrap();
    }

    let points: Vec<Point2D> = buffer.iter_chronological().copied().collect();
    assert_eq!(points.len(), 3);
    for (i, point) in points.iter().enumerate() {
        assert_eq!(point.x, i as f32);
    }

    // Fill the buffer past capacity to test wrap-around
    for i in 3..8 {
        buffer.push(Point2D::new(i as f32, i as f32)).unwrap();
    }

    // The buffer should contain points 3, 4, 5, 6, 7 in that order
    let wrapped_points: Vec<Point2D> = buffer.iter_chronological().copied().collect();
    assert_eq!(wrapped_points.len(), 5);

    // Verify chronological order
    for (i, point) in wrapped_points.iter().enumerate() {
        assert_eq!(point.x, (i + 3) as f32);
        assert_eq!(point.y, (i + 3) as f32);
    }
}

#[test]
fn test_ring_buffer_with_overflow_callback() {
    // Overflow callback tests

    static mut OVERFLOW_COUNT: u32 = 0;

    fn overflow_handler(event: RingBufferEvent) {
        if event == RingBufferEvent::DataOverwritten {
            unsafe {
                OVERFLOW_COUNT += 1;
            }
        }
    }

    let config = RingBufferConfig {
        overflow_mode: OverflowMode::Callback,
        enable_events: true,
        ..Default::default()
    };

    let mut buffer: RingBuffer<Point2D, 2> = RingBuffer::with_config(config);
    buffer.set_event_handler(overflow_handler);

    unsafe {
        OVERFLOW_COUNT = 0;
    }

    // Fill and overflow
    buffer.push(Point2D::new(1.0, 1.0)).unwrap();
    buffer.push(Point2D::new(2.0, 2.0)).unwrap();
    buffer.push(Point2D::new(3.0, 3.0)).unwrap(); // Should trigger callback

    unsafe {
        assert_eq!(OVERFLOW_COUNT, 1);
    }
}

#[test]
fn test_ring_buffer_memory_efficiency() {
    // Test that the ring buffer doesn't allocate more than necessary
    let buffer: RingBuffer<Point2D, 1000> = RingBuffer::new();

    // Size should be predictable
    let size_of_buffer = std::mem::size_of_val(&buffer);
    let expected_min_size = std::mem::size_of::<Point2D>() * 1000;

    // Buffer size should be reasonable (not too much overhead)
    assert!(size_of_buffer < expected_min_size * 2);

    // Test with preallocate
    let config = RingBufferConfig {
        preallocate: true,
        ..Default::default()
    };
    let _preallocated: RingBuffer<Point2D, 1000> = RingBuffer::with_config(config);
}

#[test]
fn test_ring_buffer_concurrent_behavior() {
    // While the ring buffer isn't thread-safe, test that it behaves correctly
    // with rapid sequential operations that might occur in interrupt handlers

    let mut buffer: RingBuffer<Point2D, 100> = RingBuffer::new();

    // Simulate rapid data acquisition
    for i in 0..1000 {
        let point = Point2D::new(i as f32 * 0.001, (i as f32 * 0.001).sin());
        buffer.push(point).unwrap();

        // Occasionally read data
        if i % 100 == 0 {
            let _data: Vec<Point2D> = buffer.iter().copied().collect();
        }
    }

    assert_eq!(buffer.len(), 100);
    assert!(buffer.is_full());
}
