//! Tests for platform-specific optimizations

use embedded_charts::platform::{self, PlatformOptimized};
use embedded_charts::data::Point2D;

#[test]
fn test_generic_platform_sqrt() {
    // Test various values
    let test_values = [0.0, 1.0, 4.0, 9.0, 16.0, 25.0, 100.0];
    
    for &val in &test_values {
        let result = platform::GenericPlatform::fast_sqrt(val);
        let expected = val.sqrt();
        let error = (result - expected).abs();
        
        // Allow up to 5% error for fast approximation
        assert!(error / expected.max(0.1) < 0.05, 
            "sqrt({}) = {}, expected {}, error: {:.2}%", 
            val, result, expected, error / expected * 100.0);
    }
}

#[test]
fn test_generic_platform_trig() {
    // Test sin/cos at key angles
    let angles = [0.0, 0.5, 1.0, 1.57, 3.14, 4.71, 6.28];
    
    for &angle in &angles {
        let sin_result = platform::GenericPlatform::fast_sin(angle);
        let sin_expected = angle.sin();
        let sin_error = (sin_result - sin_expected).abs();
        
        // Allow up to 0.02 absolute error for fast approximation
        assert!(sin_error < 0.02, 
            "sin({}) = {}, expected {}, error: {}", 
            angle, sin_result, sin_expected, sin_error);
        
        let cos_result = platform::GenericPlatform::fast_cos(angle);
        let cos_expected = angle.cos();
        let cos_error = (cos_result - cos_expected).abs();
        
        assert!(cos_error < 0.02, 
            "cos({}) = {}, expected {}, error: {}", 
            angle, cos_result, cos_expected, cos_error);
    }
}

#[test]
fn test_generic_platform_line_drawing() {
    let start = Point2D { x: 0.0, y: 0.0 };
    let end = Point2D { x: 10.0, y: 10.0 };
    
    let mut pixels = Vec::new();
    platform::GenericPlatform::draw_line_optimized(start, end, |x, y| {
        pixels.push((x, y));
    });
    
    // Should draw a diagonal line
    assert!(!pixels.is_empty());
    
    // Check that we have the start and end points
    assert!(pixels.contains(&(0, 0)));
    assert!(pixels.contains(&(10, 10)));
    
    // Check that it's roughly diagonal
    for (x, y) in &pixels {
        let diff = (x - y).abs();
        assert!(diff <= 1, "Line should be roughly diagonal, got ({}, {})", x, y);
    }
}

#[test]
fn test_generic_platform_rect_filling() {
    let top_left = Point2D { x: 0.0, y: 0.0 };
    let width = 5;
    let height = 3;
    
    let mut pixels = Vec::new();
    platform::GenericPlatform::fill_rect_optimized(top_left, width, height, |x, y| {
        pixels.push((x, y));
    });
    
    // Should have exactly width * height pixels
    assert_eq!(pixels.len(), (width * height) as usize);
    
    // Check bounds
    for (x, y) in &pixels {
        assert!(*x >= 0 && *x < width as i32);
        assert!(*y >= 0 && *y < height as i32);
    }
    
    // Check that all pixels are covered
    for y in 0..height {
        for x in 0..width {
            assert!(pixels.contains(&(x as i32, y as i32)), 
                "Missing pixel at ({}, {})", x, y);
        }
    }
}

#[test]
fn test_sqrt_accuracy() {
    // Test accuracy of fast sqrt
    let test_range: Vec<f32> = (1..100).map(|i| i as f32 * 0.1).collect();
    let mut max_error = 0.0f32;
    
    for &val in &test_range {
        let fast = platform::GenericPlatform::fast_sqrt(val);
        let accurate = val.sqrt();
        let error = ((fast - accurate) / accurate).abs();
        max_error = max_error.max(error);
    }
    
    // Fast sqrt should be within 5% of accurate sqrt
    assert!(max_error < 0.05, "Max error: {:.2}%", max_error * 100.0);
}

#[test]
fn test_sin_accuracy() {
    // Test accuracy of fast sin
    let test_range: Vec<f32> = (0..360).map(|i| (i as f32).to_radians()).collect();
    let mut max_error = 0.0f32;
    
    for &angle in &test_range {
        let fast = platform::GenericPlatform::fast_sin(angle);
        let accurate = angle.sin();
        let error = (fast - accurate).abs();
        max_error = max_error.max(error);
    }
    
    // Fast sin should be within 0.02 absolute error
    assert!(max_error < 0.02, "Max error: {}", max_error);
}