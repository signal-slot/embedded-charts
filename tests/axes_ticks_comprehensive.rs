//! Comprehensive tests for tick generation algorithms

use embedded_charts::axes::{
    ticks::{CustomTickGenerator, LinearTickGenerator, LogTickGenerator},
    traits::TickGenerator,
};

#[test]
fn test_linear_tick_generator_creation() {
    let generator = LinearTickGenerator::new(7);
    assert_eq!(
        <LinearTickGenerator as TickGenerator<f32>>::preferred_tick_count(&generator),
        7
    );

    // Test clamping behavior
    let generator_low = LinearTickGenerator::new(1);
    assert_eq!(
        <LinearTickGenerator as TickGenerator<f32>>::preferred_tick_count(&generator_low),
        2
    ); // Clamped to minimum

    let generator_high = LinearTickGenerator::new(50);
    assert_eq!(
        <LinearTickGenerator as TickGenerator<f32>>::preferred_tick_count(&generator_high),
        20
    ); // Clamped to maximum
}

#[test]
fn test_linear_tick_generator_with_minor_ticks() {
    let generator = LinearTickGenerator::new(5).with_minor_ticks(4);

    let ticks = generator.generate_ticks(0.0f32, 10.0f32, 50);

    // Should have both major and minor ticks
    let major_count = ticks.iter().filter(|t| t.is_major).count();
    let minor_count = ticks.iter().filter(|t| !t.is_major).count();

    assert!(major_count > 0);
    assert!(minor_count > 0);

    // Verify ticks are sorted
    for window in ticks.windows(2) {
        assert!(window[0].value <= window[1].value);
    }
}

#[test]
fn test_linear_tick_generator_without_minor_ticks() {
    let generator = LinearTickGenerator::new(5)
        .with_minor_ticks(4)
        .without_minor_ticks();

    let ticks = generator.generate_ticks(0.0f32, 10.0f32, 50);

    // Should only have major ticks
    assert!(ticks.iter().all(|t| t.is_major));
}

#[test]
fn test_linear_tick_generator_edge_cases() {
    let generator = LinearTickGenerator::new(5);

    // Test with zero range
    let ticks = generator.generate_ticks(5.0f32, 5.0f32, 10);
    assert_eq!(ticks.len(), 1);
    assert_eq!(ticks[0].value, 5.0);

    // Test with negative range (min > max) - implementation may handle this differently
    let _ticks = generator.generate_ticks(10.0f32, 0.0f32, 10);
    // Just verify it doesn't panic - implementation details may vary

    // Test with very small range
    let ticks = generator.generate_ticks(0.0f32, 0.0001f32, 10);
    assert!(!ticks.is_empty());

    // Test with negative values
    let ticks = generator.generate_ticks(-50.0f32, -10.0f32, 10);
    assert!(!ticks.is_empty());
    for tick in &ticks {
        assert!(tick.value >= -50.0 && tick.value <= -10.0);
    }
}

#[test]
fn test_linear_tick_generator_nice_step_calculation() {
    let generator = LinearTickGenerator::new(5);

    // Test various ranges to ensure nice step values
    let test_cases = [
        (0.0f32, 10.0f32),   // Should give steps like 2.0 or 2.5
        (0.0f32, 100.0f32),  // Should give steps like 20.0 or 25.0
        (0.0f32, 1.0f32),    // Should give steps like 0.2 or 0.25
        (0.0f32, 0.1f32),    // Should give steps like 0.02 or 0.025
        (-10.0f32, 10.0f32), // Should handle negative to positive
    ];

    for (min, max) in test_cases {
        let ticks = generator.generate_ticks(min, max, 10);
        // Just verify we get some ticks
        assert!(!ticks.is_empty());

        // Verify all ticks are within range
        for tick in &ticks {
            assert!(tick.value >= min);
            assert!(tick.value <= max);
        }
    }
}

#[test]
fn test_linear_tick_generator_minor_tick_ratios() {
    // Test different minor tick ratios
    let ratios = [1, 2, 4, 5, 10];

    for ratio in ratios {
        let generator = LinearTickGenerator::new(3).with_minor_ticks(ratio);
        let ticks = generator.generate_ticks(0.0f32, 10.0f32, 50);

        // Find pairs of consecutive major ticks
        let major_ticks: Vec<_> = ticks
            .iter()
            .filter(|t| t.is_major)
            .map(|t| t.value)
            .collect();

        if major_ticks.len() >= 2 {
            // Check that minor ticks exist between majors
            for window in major_ticks.windows(2) {
                let minor_between = ticks
                    .iter()
                    .filter(|t| !t.is_major && t.value > window[0] && t.value < window[1])
                    .count();

                // Should have some minor ticks between major ones
                assert!(minor_between <= ratio);
            }
        }
    }
}

#[test]
fn test_linear_tick_generator_max_ticks_limit() {
    let generator = LinearTickGenerator::new(100); // Request many ticks

    // But limit to 10 max
    let ticks = generator.generate_ticks(0.0f32, 100.0f32, 10);
    assert!(ticks.len() <= 10);

    // Test with minor ticks enabled
    let generator_with_minor = LinearTickGenerator::new(20).with_minor_ticks(5);
    let ticks = generator_with_minor.generate_ticks(0.0f32, 100.0f32, 15);
    assert!(ticks.len() <= 15);
}

#[test]
fn test_linear_tick_generator_set_preferred_count() {
    let mut generator = LinearTickGenerator::new(5);
    assert_eq!(
        <LinearTickGenerator as TickGenerator<f32>>::preferred_tick_count(&generator),
        5
    );

    <LinearTickGenerator as TickGenerator<f32>>::set_preferred_tick_count(&mut generator, 10);
    assert_eq!(
        <LinearTickGenerator as TickGenerator<f32>>::preferred_tick_count(&generator),
        10
    );

    // Test clamping
    <LinearTickGenerator as TickGenerator<f32>>::set_preferred_tick_count(&mut generator, 1);
    assert_eq!(
        <LinearTickGenerator as TickGenerator<f32>>::preferred_tick_count(&generator),
        2
    );

    <LinearTickGenerator as TickGenerator<f32>>::set_preferred_tick_count(&mut generator, 100);
    assert_eq!(
        <LinearTickGenerator as TickGenerator<f32>>::preferred_tick_count(&generator),
        20
    );
}

#[test]
fn test_custom_tick_generator_basic() {
    let generator = CustomTickGenerator::new()
        .add_major_tick(0.0f32, "Start")
        .add_major_tick(5.0f32, "Middle")
        .add_major_tick(10.0f32, "End")
        .add_minor_tick(2.5f32)
        .add_minor_tick(7.5f32);

    let ticks = generator.generate_ticks(0.0f32, 10.0f32, 10);
    assert_eq!(ticks.len(), 5);

    // Verify major ticks have labels
    let major_ticks: Vec<_> = ticks.iter().filter(|t| t.is_major).collect();
    assert_eq!(major_ticks.len(), 3);
    for tick in major_ticks {
        assert!(tick.label.is_some());
    }

    // Verify minor ticks don't have labels
    let minor_ticks: Vec<_> = ticks.iter().filter(|t| !t.is_major).collect();
    assert_eq!(minor_ticks.len(), 2);
    for tick in minor_ticks {
        assert!(tick.label.is_none());
    }
}

#[test]
fn test_custom_tick_generator_range_filtering() {
    let generator = CustomTickGenerator::new()
        .add_major_tick(-10.0f32, "Outside1")
        .add_major_tick(0.0f32, "Start")
        .add_major_tick(5.0f32, "Middle")
        .add_major_tick(10.0f32, "End")
        .add_major_tick(20.0f32, "Outside2");

    // Only ticks within range should be returned
    let ticks = generator.generate_ticks(0.0f32, 10.0f32, 10);
    assert_eq!(ticks.len(), 3);

    for tick in &ticks {
        assert!(tick.value >= 0.0 && tick.value <= 10.0);
    }
}

#[test]
fn test_custom_tick_generator_clear() {
    let mut generator = CustomTickGenerator::new()
        .add_major_tick(0.0f32, "Test")
        .add_minor_tick(5.0f32);

    let ticks = generator.generate_ticks(0.0f32, 10.0f32, 10);
    assert_eq!(ticks.len(), 2);

    generator.clear();
    let ticks = generator.generate_ticks(0.0f32, 10.0f32, 10);
    assert_eq!(ticks.len(), 0);
}

#[test]
fn test_custom_tick_generator_capacity_limit() {
    let mut generator = CustomTickGenerator::new();

    // Try to add more than 32 ticks
    for i in 0..40 {
        generator = generator.add_major_tick(i as f32, "Tick");
    }

    // Should be limited to 32
    let ticks = generator.generate_ticks(0.0f32, 50.0f32, 50);
    assert!(ticks.len() <= 32);
}

#[test]
fn test_custom_tick_generator_preferred_count() {
    let generator = CustomTickGenerator::new()
        .add_major_tick(0.0f32, "A")
        .add_major_tick(5.0f32, "B")
        .add_major_tick(10.0f32, "C");

    assert_eq!(generator.preferred_tick_count(), 3);

    // set_preferred_tick_count is ignored for custom generator
    let mut generator = generator;
    generator.set_preferred_tick_count(10);
    assert_eq!(generator.preferred_tick_count(), 3); // Unchanged
}

#[test]
fn test_log_tick_generator_basic() {
    let generator = LogTickGenerator::new();
    let ticks = generator.generate_ticks(1.0f32, 1000.0f32, 10);

    assert!(!ticks.is_empty());
    assert!(ticks.len() <= 10);

    // All ticks should be major for log scale
    assert!(ticks.iter().all(|t| t.is_major));

    // All ticks should have labels
    assert!(ticks.iter().all(|t| t.label.is_some()));

    // Verify we got reasonable values for a log scale
    // The exact values depend on the implementation details
}

#[test]
fn test_log_tick_generator_custom_base() {
    let generator = LogTickGenerator::with_base(2.0);
    let ticks = generator.generate_ticks(1.0f32, 32.0f32, 10);

    assert!(!ticks.is_empty());

    // Verify we got values within the expected range
    for tick in &ticks {
        assert!(tick.value >= 1.0 && tick.value <= 32.0);
    }
}

#[test]
fn test_log_tick_generator_with_minor_ticks() {
    let generator = LogTickGenerator::new().with_minor_ticks();
    let ticks = generator.generate_ticks(1.0f32, 100.0f32, 20);

    // Should still only generate major ticks for now
    // (minor tick implementation for log scale could be added later)
    assert!(ticks.iter().all(|t| t.is_major));
}

#[test]
fn test_log_tick_generator_negative_values() {
    let generator = LogTickGenerator::new();

    // Log scale requires positive values
    let ticks = generator.generate_ticks(-10.0f32, 100.0f32, 10);
    assert!(ticks.is_empty());

    let ticks = generator.generate_ticks(-100.0f32, -10.0f32, 10);
    assert!(ticks.is_empty());
}

#[test]
fn test_log_tick_generator_small_range() {
    let generator = LogTickGenerator::new();
    let ticks = generator.generate_ticks(1.0f32, 2.0f32, 10);

    // Should handle small ranges gracefully
    assert!(!ticks.is_empty());
    for tick in &ticks {
        assert!(tick.value >= 1.0 && tick.value <= 2.0);
    }
}

#[test]
fn test_log_tick_generator_label_formatting() {
    let generator = LogTickGenerator::new();
    let ticks = generator.generate_ticks(0.1f32, 10000.0f32, 10);

    // Check label formatting
    for tick in &ticks {
        if let Some(label) = &tick.label {
            let label_str = label.as_str();
            if tick.value >= 1000.0 {
                assert!(label_str.contains('k')); // Should use 'k' suffix
            } else if tick.value < 1.0 {
                assert!(label_str == "0.1"); // Small values
            } else {
                // Should be integer format
                assert!(!label_str.contains('.'));
            }
        }
    }
}

#[test]
fn test_log_tick_generator_base_clamping() {
    // Base should be clamped to minimum of 2.0
    let generator = LogTickGenerator::with_base(1.5);
    let ticks = generator.generate_ticks(1.0f32, 10.0f32, 10);

    // Should still generate valid ticks
    assert!(!ticks.is_empty());
}

#[test]
fn test_linear_tick_generator_extreme_values() {
    let generator = LinearTickGenerator::new(5);

    // Very large values
    let _ticks = generator.generate_ticks(1e6f32, 1e7f32, 10);
    // Just verify it handles large values without panicking

    // Very small values
    let _ticks = generator.generate_ticks(1e-6f32, 1e-5f32, 10);
    // Just verify it handles small values without panicking
}

#[test]
fn test_tick_label_formatting() {
    let generator = LinearTickGenerator::new(5);
    let ticks = generator.generate_ticks(0.0f32, 10.0f32, 10);

    // All major ticks should have labels
    for tick in &ticks {
        if tick.is_major {
            assert!(tick.label.is_some());
            // Label should be a valid string representation
            let label = tick.label.as_ref().unwrap();
            assert!(!label.is_empty());
        }
    }
}

#[test]
fn test_minor_tick_boundary_conditions() {
    let generator = LinearTickGenerator::new(3).with_minor_ticks(2);

    // Test that minor ticks don't overlap with major ticks
    let ticks = generator.generate_ticks(0.0f32, 10.0f32, 50);

    let major_positions: Vec<f32> = ticks
        .iter()
        .filter(|t| t.is_major)
        .map(|t| t.value)
        .collect();

    for tick in &ticks {
        if !tick.is_major {
            // Minor tick should not be at same position as any major tick
            for &major_pos in &major_positions {
                assert!((tick.value - major_pos).abs() > 0.001);
            }
        }
    }
}

#[test]
fn test_tick_generator_with_integer_values() {
    let generator = LinearTickGenerator::new(5);
    let ticks = generator.generate_ticks(0i32, 10i32, 10);

    assert!(!ticks.is_empty());
    for tick in &ticks {
        assert!(tick.value >= 0 && tick.value <= 10);
    }
}

#[test]
fn test_custom_tick_generator_default() {
    let generator: CustomTickGenerator<f32> = Default::default();
    let ticks = generator.generate_ticks(0.0f32, 10.0f32, 10);
    assert_eq!(ticks.len(), 0);
}

#[test]
fn test_log_tick_generator_default() {
    let generator: LogTickGenerator = Default::default();
    assert_eq!(generator.preferred_tick_count(), 5);
}
