//! Time Provider Demo
//!
//! This example demonstrates the new time abstraction layer for animations.
//! It shows how to use different time providers (StdTimeProvider, ManualTimeProvider)
//! and how they integrate with the animation system using TimeBasedProgress.
//!
//! Run with: cargo run --example time_provider_demo --features "std,animations,line"

use embedded_charts::prelude::*;

// Import the common abstraction
#[path = "../common/mod.rs"]
mod common;

#[cfg(feature = "std")]
fn main() -> ChartResult<()> {
    println!("🚀 Starting Time Provider Demo");
    println!("📊 Demonstrating different time providers for animations");

    // Demo 1: Using StdTimeProvider (real-time)
    println!("\n=== Demo 1: StdTimeProvider (Real-time) ===");
    demo_std_time_provider()?;

    // Demo 2: Using ManualTimeProvider (controlled time)
    println!("\n=== Demo 2: ManualTimeProvider (Controlled time) ===");
    demo_manual_time_provider()?;

    // Demo 3: Using MonotonicTimeProvider (simulated hardware timer)
    println!("\n=== Demo 3: MonotonicTimeProvider (Simulated hardware timer) ===");
    demo_monotonic_time_provider()?;

    println!("\n✅ All time provider demos completed successfully!");
    Ok(())
}

#[cfg(feature = "std")]
fn demo_std_time_provider() -> ChartResult<()> {
    use std::thread;
    use std::time::Duration as StdDuration;

    // Create a standard time provider
    let time_provider = StdTimeProvider::new();

    // Create TimeBasedProgress helpers for different animations
    let mut progress1 = TimeBasedProgress::new(1000);
    let mut progress2 = TimeBasedProgress::new(1500);

    println!("Created animations with StdTimeProvider");
    println!("Animation 1: 1000ms");
    println!("Animation 2: 1500ms");

    // Run animation loop
    let mut step = 0;
    loop {
        step += 1;

        // Update progress with the time provider
        let current_progress1 = progress1.progress_from_time(&time_provider);
        let current_progress2 = progress2.progress_from_time(&time_provider);

        if step % 10 == 0 {
            println!("Step {step}: Anim1: {current_progress1}%, Anim2: {current_progress2}%");
        }

        // Check if complete
        if current_progress1 >= 100 && current_progress2 >= 100 {
            println!("All animations completed!");
            break;
        }

        // Sleep for ~60 FPS
        thread::sleep(StdDuration::from_millis(16));

        if step > 200 {
            // Safety break
            break;
        }
    }

    Ok(())
}

#[cfg(feature = "std")]
fn demo_manual_time_provider() -> ChartResult<()> {
    // Create a manual time provider
    let mut time_provider = ManualTimeProvider::new();

    // Create sample data for interpolation
    let mut from_data = StaticDataSeries::new();
    from_data.push(Point2D::new(0.0, 0.0))?;
    from_data.push(Point2D::new(1.0, 10.0))?;
    from_data.push(Point2D::new(2.0, 20.0))?;
    from_data.push(Point2D::new(3.0, 30.0))?;

    let mut to_data = StaticDataSeries::new();
    to_data.push(Point2D::new(0.0, 50.0))?;
    to_data.push(Point2D::new(1.0, 60.0))?;
    to_data.push(Point2D::new(2.0, 70.0))?;
    to_data.push(Point2D::new(3.0, 80.0))?;

    // Create a ChartAnimator for data transition
    let animator = ChartAnimator::new(from_data.clone(), to_data.clone(), EasingFunction::EaseOut);
    let mut progress_calc = TimeBasedProgress::new(2000);

    println!("Created data transition animation with ManualTimeProvider");
    println!("Duration: 2000ms, Easing: EaseOut");

    // Manually advance time and show progress
    for _step in 0..=20 {
        // Advance time by 100ms each step
        time_provider.advance_ms(100);

        // Get current progress
        let progress = progress_calc.progress_from_time(&time_provider);

        // Get interpolated data from animator
        if let Some(current_data) = animator.value_at(progress) {
            let current_time = time_provider.current_time_ms();
            let values: heapless::Vec<f32, 256> = current_data.iter().map(|p| p.y()).collect();
            println!(
                "Time: {}ms, Progress: {}%, Values: [{:.1}, {:.1}, {:.1}, {:.1}]",
                current_time,
                progress,
                values.first().unwrap_or(&0.0),
                values.get(1).unwrap_or(&0.0),
                values.get(2).unwrap_or(&0.0),
                values.get(3).unwrap_or(&0.0)
            );
        }

        if progress >= 100 {
            println!("Animation completed!");
            break;
        }
    }

    Ok(())
}

#[cfg(feature = "std")]
fn demo_monotonic_time_provider() -> ChartResult<()> {
    use std::cell::RefCell;
    use std::rc::Rc;

    // Simulate a hardware timer that increments at 1MHz
    let hardware_counter = Rc::new(RefCell::new(0u64));
    let counter_clone = hardware_counter.clone();

    let timer_fn = move || {
        let mut counter = counter_clone.borrow_mut();
        *counter += 16_667; // Simulate ~60 FPS (16.667ms per call)
        *counter
    };

    // Create a monotonic time provider
    let time_provider = MonotonicTimeProvider::new(timer_fn);

    // Create multiple TimeBasedProgress helpers
    let mut linear_progress = TimeBasedProgress::new(1000);
    let mut ease_in_progress = TimeBasedProgress::new(1000);
    let mut ease_out_progress = TimeBasedProgress::new(1000);
    let mut ease_in_out_progress = TimeBasedProgress::new(1000);

    println!("Created multiple animations with MonotonicTimeProvider");
    println!("All animations: 1000ms duration, different easing functions");

    // Run animation loop
    let mut step = 0;
    loop {
        step += 1;

        // Get progress for all animations
        let linear_prog = linear_progress.progress_from_time(&time_provider);
        let ease_in_prog = ease_in_progress.progress_from_time(&time_provider);
        let ease_out_prog = ease_out_progress.progress_from_time(&time_provider);
        let ease_in_out_prog = ease_in_out_progress.progress_from_time(&time_provider);

        if step % 15 == 0 {
            println!(
                "Step {step}: Linear: {linear_prog}%, EaseIn: {ease_in_prog}%, EaseOut: {ease_out_prog}%, EaseInOut: {ease_in_out_prog}%"
            );
        }

        // Check if complete
        if linear_prog >= 100
            && ease_in_prog >= 100
            && ease_out_prog >= 100
            && ease_in_out_prog >= 100
        {
            println!("All animations completed!");
            break;
        }

        if step > 100 {
            // Safety break
            break;
        }
    }

    Ok(())
}

#[cfg(not(feature = "std"))]
fn main() {
    println!("This example requires the 'std' feature to demonstrate time providers.");
    println!("Run with: cargo run --example time_provider_demo --features \"std,animations,line\"");
}
