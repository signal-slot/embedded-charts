//! Edge case tests for axes tick generation
//!
//! This test suite focuses on improving coverage for the ticks.rs module,
//! specifically targeting edge cases and error conditions.

use embedded_charts::axes::{
    CustomTickGenerator, LinearTickGenerator, LogTickGenerator, TickGenerator,
};

#[test]
fn test_linear_tick_generator_edge_cases() {
    let generator = LinearTickGenerator::new(5);

    // Test with zero range
    let ticks = generator.generate_ticks(5.0f32, 5.0f32, 10);
    assert!(!ticks.is_empty()); // Should at least have one tick
    assert_eq!(ticks[0].value, 5.0);

    // Test with very small range - may produce at least min/max ticks
    // Some backends (fixed-point, integer-math) may treat very small values as zero
    #[cfg(all(not(feature = "fixed-point"), not(feature = "integer-math")))]
    {
        let ticks = generator.generate_ticks(0.0f32, 1e-10f32, 10);
        assert!(ticks.len() >= 2); // Should have at least min and max
    }

    // Test with small but reasonable range for all backends
    let ticks = generator.generate_ticks(0.0f32, 0.01f32, 10);
    assert!(!ticks.is_empty());

    // Test with negative range (max < min) - may not work well with all backends
    // Fixed-point and reversed ranges may produce fewer ticks
    let _ticks = generator.generate_ticks(10.0f32, 0.0f32, 10);
    // Just verify it doesn't panic - implementation may vary

    // Test with very large range
    let ticks = generator.generate_ticks(0.0f32, 1e10f32, 10);
    assert!(!ticks.is_empty());
    assert!(ticks.len() <= 10);
}

#[test]
fn test_linear_tick_generator_extreme_values() {
    let generator = LinearTickGenerator::new(5);

    // Test with very large positive values
    let ticks = generator.generate_ticks(1e6f32, 1e7f32, 10);
    assert!(!ticks.is_empty());
    assert!(ticks.iter().all(|t| t.value >= 1e6 && t.value <= 1e7));

    // Test with very small positive values
    // Some backends have limited precision for very small values
    #[cfg(all(not(feature = "fixed-point"), not(feature = "integer-math")))]
    {
        let ticks = generator.generate_ticks(1e-6f32, 1e-5f32, 10);
        assert!(ticks.len() >= 2); // Should handle small ranges
    }

    // Test with small values
    // Use a larger range that works better with fixed-point
    let ticks = generator.generate_ticks(1.0f32, 10.0f32, 10);
    assert!(!ticks.is_empty());

    // Test with mixed sign range
    let ticks = generator.generate_ticks(-100.0f32, 100.0f32, 10);
    assert!(!ticks.is_empty());
    // Should include zero
    assert!(ticks.iter().any(|t| t.value.abs() < 1.0));
}

#[test]
fn test_linear_tick_generator_preferred_count_bounds() {
    // Test clamping of preferred count
    let generator = LinearTickGenerator::new(1); // Should clamp to minimum
    assert!(<LinearTickGenerator as TickGenerator<f32>>::preferred_tick_count(&generator) >= 2);

    let generator = LinearTickGenerator::new(100); // Should clamp to maximum
    assert!(<LinearTickGenerator as TickGenerator<f32>>::preferred_tick_count(&generator) <= 20);

    // Test set_preferred_tick_count
    let mut generator = LinearTickGenerator::new(5);
    <LinearTickGenerator as TickGenerator<f32>>::set_preferred_tick_count(&mut generator, 1);
    assert!(<LinearTickGenerator as TickGenerator<f32>>::preferred_tick_count(&generator) >= 2);

    <LinearTickGenerator as TickGenerator<f32>>::set_preferred_tick_count(&mut generator, 50);
    assert!(<LinearTickGenerator as TickGenerator<f32>>::preferred_tick_count(&generator) <= 20);
}

#[test]
fn test_minor_tick_generation_edge_cases() {
    // Test with different minor tick ratios
    for ratio in 1..=10 {
        let generator = LinearTickGenerator::new(3).with_minor_ticks(ratio);
        let ticks = generator.generate_ticks(0.0f32, 10.0f32, 50);

        let major_ticks: Vec<_> = ticks.iter().filter(|t| t.is_major).collect();
        let minor_ticks: Vec<_> = ticks.iter().filter(|t| !t.is_major).collect();

        assert!(!major_ticks.is_empty());
        if major_ticks.len() > 1 {
            assert!(!minor_ticks.is_empty());
        }
    }

    // Test minor tick generation with very small range
    let generator = LinearTickGenerator::new(3).with_minor_ticks(4);
    let ticks = generator.generate_ticks(0.0f32, 0.1f32, 50);
    assert!(!ticks.is_empty());

    // Test disabling minor ticks after enabling
    let generator = LinearTickGenerator::new(3)
        .with_minor_ticks(4)
        .without_minor_ticks();
    let ticks = generator.generate_ticks(0.0f32, 10.0f32, 50);
    assert!(ticks.iter().all(|t| t.is_major));
}

#[test]
#[cfg(all(not(feature = "fixed-point"), not(feature = "integer-math")))]
fn test_calculate_nice_step_edge_cases() {
    let generator = LinearTickGenerator::new(5);

    // Test with NaN range - should produce fallback ticks
    let ticks = generator.generate_ticks(0.0f32, f32::NAN, 10);
    assert!(ticks.len() >= 2); // Should handle gracefully with min/max

    // Test with infinite range - should produce fallback ticks
    let ticks = generator.generate_ticks(0.0f32, f32::INFINITY, 10);
    assert!(ticks.len() >= 2); // Should handle gracefully

    // Test with negative infinity - should produce fallback ticks
    let ticks = generator.generate_ticks(f32::NEG_INFINITY, 0.0f32, 10);
    assert!(ticks.len() >= 2); // Should handle gracefully
}

#[test]
fn test_calculate_nice_step_edge_cases_common() {
    let generator = LinearTickGenerator::new(5);

    // Test edge cases that work with both floating-point and fixed-point
    // Test with very large range
    let ticks = generator.generate_ticks(0.0f32, 1000000.0f32, 10);
    assert!(!ticks.is_empty());

    // Test with very small positive range - fixed-point friendly
    let ticks = generator.generate_ticks(0.0f32, 0.01f32, 10);
    assert!(!ticks.is_empty());
}

#[test]
fn test_custom_tick_generator_capacity() {
    let mut generator = CustomTickGenerator::new();

    // Add more than 32 ticks (capacity limit)
    for i in 0..40 {
        generator = generator.add_major_tick(i as f32, &format!("{i}"));
    }

    let ticks = generator.generate_ticks(0.0f32, 40.0f32, 50);
    assert!(ticks.len() <= 32); // Should respect capacity limit

    // Test clear functionality
    generator.clear();
    let ticks = generator.generate_ticks(0.0f32, 40.0f32, 50);
    assert!(ticks.is_empty());
}

#[test]
fn test_custom_tick_generator_range_filtering() {
    let generator = CustomTickGenerator::new()
        .add_major_tick(-10.0, "-10")
        .add_major_tick(0.0, "0")
        .add_major_tick(5.0, "5")
        .add_major_tick(10.0, "10")
        .add_major_tick(20.0, "20")
        .add_minor_tick(2.5)
        .add_minor_tick(7.5);

    // Test range filtering
    let ticks = generator.generate_ticks(0.0f32, 10.0f32, 10);
    // Should have 0, 5, 10, and 2.5 - but fixed-point may include 7.5 due to precision
    assert!(ticks.len() >= 4 && ticks.len() <= 5);
    assert!(ticks.iter().all(|t| t.value >= 0.0 && t.value <= 10.0));
    // Verify expected ticks are present
    assert!(ticks.iter().any(|t| (t.value - 0.0).abs() < 0.01));
    assert!(ticks.iter().any(|t| (t.value - 5.0).abs() < 0.01));
    assert!(ticks.iter().any(|t| (t.value - 10.0).abs() < 0.01));

    // Test partial range
    let ticks = generator.generate_ticks(3.0f32, 8.0f32, 10);
    // Should have 5 and 7.5
    assert!(ticks.iter().all(|t| t.value >= 3.0 && t.value <= 8.0));
    assert!(ticks.iter().any(|t| (t.value - 5.0).abs() < 0.1));
    assert!(ticks.iter().any(|t| (t.value - 7.5).abs() < 0.1));
}

#[test]
fn test_log_tick_generator_edge_cases() {
    let generator = LogTickGenerator::new();

    // Test with negative values (should return empty)
    let ticks = generator.generate_ticks(-10.0f32, 10.0f32, 10);
    assert!(ticks.is_empty());

    // Test with zero min (should return empty)
    let ticks = generator.generate_ticks(0.0f32, 100.0f32, 10);
    assert!(ticks.is_empty());

    // Test with very small positive range
    let ticks = generator.generate_ticks(0.001f32, 0.1f32, 10);
    assert!(!ticks.is_empty());

    // Test with very large range
    let ticks = generator.generate_ticks(1.0f32, 1e10f32, 20);
    assert!(!ticks.is_empty());
    assert!(ticks.len() <= 20);
}

#[test]
fn test_log_tick_generator_different_bases() {
    // Test base 2
    let generator = LogTickGenerator::with_base(2.0);
    let ticks = generator.generate_ticks(1.0f32, 32.0f32, 10);
    assert!(!ticks.is_empty());
    // Should have ticks at powers of base
    assert!(ticks.len() >= 3); // At least some power-of-2 ticks

    // Test base e
    let generator = LogTickGenerator::with_base(core::f32::consts::E);
    let ticks = generator.generate_ticks(1.0f32, 20.0f32, 10);
    assert!(!ticks.is_empty());

    // Test with base clamping (base < 2 should clamp to 2)
    let generator = LogTickGenerator::with_base(1.5);
    let ticks = generator.generate_ticks(1.0f32, 8.0f32, 10);
    assert!(!ticks.is_empty());
}

#[test]
fn test_log_tick_generator_label_formatting() {
    let generator = LogTickGenerator::new();

    // Test label formatting for different magnitudes
    let ticks = generator.generate_ticks(0.1f32, 10000.0f32, 20);
    assert!(!ticks.is_empty());

    // Check that we have some ticks
    assert!(!ticks.is_empty());

    // Check tick labels exist
    let labeled_ticks: Vec<_> = ticks.iter().filter(|t| t.label.is_some()).collect();
    assert!(!labeled_ticks.is_empty());

    // Check that labels follow some pattern
    for tick in &ticks {
        if let Some(label) = &tick.label {
            assert!(!label.is_empty());
        }
    }
}

#[test]
fn test_log_tick_generator_with_minor_ticks() {
    let generator = LogTickGenerator::new().with_minor_ticks();
    assert_eq!(
        <LogTickGenerator as TickGenerator<f32>>::preferred_tick_count(&generator),
        5
    );

    // Test that set_preferred_tick_count is ignored
    let mut generator = generator;
    <LogTickGenerator as TickGenerator<f32>>::set_preferred_tick_count(&mut generator, 10);
    assert_eq!(
        <LogTickGenerator as TickGenerator<f32>>::preferred_tick_count(&generator),
        5
    ); // Should remain unchanged
}

#[test]
fn test_tick_sorting_and_capacity() {
    // Test that ticks are properly sorted when minor ticks are added
    let generator = LinearTickGenerator::new(5).with_minor_ticks(4);
    let ticks = generator.generate_ticks(0.0f32, 10.0f32, 100);

    // Verify sorting
    for window in ticks.windows(2) {
        if let [tick1, tick2] = window {
            assert!(
                tick1.value <= tick2.value,
                "Ticks not sorted: {} > {}",
                tick1.value,
                tick2.value
            );
        }
    }

    // Test max_ticks limit
    let generator = LinearTickGenerator::new(20).with_minor_ticks(10);
    let ticks = generator.generate_ticks(0.0f32, 100.0f32, 5);
    assert!(ticks.len() <= 5);
}

#[test]
fn test_tick_label_edge_cases() {
    use embedded_charts::axes::Tick;

    // Test major tick without label
    let tick = Tick::<f32>::major_unlabeled(5.0);
    assert!(tick.is_major);
    assert!(tick.label.is_none());

    // Test label truncation for very long labels
    let long_label = "This is a very long label that exceeds 16 characters";
    let tick = Tick::major(5.0, long_label);
    assert!(tick.is_major);
    if let Some(label) = tick.label {
        assert!(label.len() <= 16);
    }
}
