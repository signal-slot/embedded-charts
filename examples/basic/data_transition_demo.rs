//! Data Transition Demo - Smooth Value Interpolation
//!
//! This example demonstrates the new ChartAnimator capabilities for smooth value interpolation
//! between different data sets using various easing functions with the new 0-100 progress API.
//!
//! Features demonstrated:
//! - Smooth transitions between different data values using ChartAnimator
//! - Different easing functions (Linear, EaseIn, EaseOut, EaseInOut)
//! - External timeline control with TimeBasedProgress
//! - Visual comparison of easing effects
//! - Progress-based animation system (0-100)
//!
//! Run with: cargo run --example data_transition_demo --features "std,animations"

use embedded_charts::prelude::*;

// Import the common abstraction
#[path = "../common/mod.rs"]
mod common;

use common::WindowConfig;

#[cfg(feature = "std")]
fn main() -> ChartResult<()> {
    // Define different data sets to transition between
    let dataset_a = [10.0, 20.0, 15.0, 25.0, 30.0, 18.0, 22.0, 28.0];
    let dataset_b = [30.0, 35.0, 40.0, 45.0, 20.0, 25.0, 35.0, 15.0];
    let dataset_c = [25.0, 15.0, 35.0, 20.0, 40.0, 30.0, 10.0, 45.0];

    // Convert arrays to StaticDataSeries for animation
    let mut series_a = StaticDataSeries::new();
    let mut series_b = StaticDataSeries::new();
    let mut series_c = StaticDataSeries::new();

    for (i, ((a, b), c)) in dataset_a
        .iter()
        .zip(dataset_b.iter())
        .zip(dataset_c.iter())
        .enumerate()
    {
        series_a.push(Point2D::new(i as f32, *a))?;
        series_b.push(Point2D::new(i as f32, *b))?;
        series_c.push(Point2D::new(i as f32, *c))?;
    }

    // Create chart animators with different easing functions
    let mut linear_animator =
        ChartAnimator::new(series_a.clone(), series_b.clone(), EasingFunction::Linear);
    let mut ease_in_animator =
        ChartAnimator::new(series_a.clone(), series_b.clone(), EasingFunction::EaseIn);
    let mut ease_out_animator =
        ChartAnimator::new(series_a.clone(), series_b.clone(), EasingFunction::EaseOut);
    let mut ease_in_out_animator = ChartAnimator::new(
        series_a.clone(),
        series_b.clone(),
        EasingFunction::EaseInOut,
    );

    // Create time-based progress calculator (2 second duration)
    let mut time_progress = TimeBasedProgress::new(2000);

    // Create line charts for each transition type
    let linear_chart = LineChart::builder()
        .line_color(Rgb565::RED)
        .line_width(2)
        .build()?;

    let ease_in_chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .line_width(2)
        .build()?;

    let ease_out_chart = LineChart::builder()
        .line_color(Rgb565::GREEN)
        .line_width(2)
        .build()?;

    let ease_in_out_chart = LineChart::builder()
        .line_color(Rgb565::new(31, 16, 0)) // Orange
        .line_width(2)
        .build()?;

    // Pre-create animation timing constants
    let transition_duration = 2.0; // 2 seconds per transition
    let pause_duration = 0.5; // 0.5 seconds pause between transitions
    let total_phases = 3; // Number of transition phases (A->B, B->C, C->A)
    let time_progress_duration_ms = 2000; // Duration in milliseconds for TimeBasedProgress

    // Transition state management
    let mut current_phase = 0; // 0: A->B, 1: B->C, 2: C->A
    let mut phase_start_time = 0.0f32;

    // Create time provider for consistent timing
    let mut time_provider = ManualTimeProvider::new();

    println!("🎬 Starting Data Transition Demo");
    println!("📊 Demonstrating different easing functions:");
    println!("   🔴 Red: Linear");
    println!("   🔵 Blue: Ease In");
    println!("   🟢 Green: Ease Out");
    println!("   🟠 Orange: Ease In-Out");
    println!("⏱️  Each transition takes 2 seconds with 0.5s pause");

    // Pre-create text style and chart configuration outside the loop
    use embedded_graphics::{
        mono_font::{ascii::FONT_6X10, MonoTextStyle},
        prelude::*,
        text::{Baseline, Text},
    };
    let text_style = MonoTextStyle::new(&FONT_6X10, Rgb565::BLACK);

    // Pre-create chart config with space for legend
    let _chart_config = ChartConfig {
        background_color: Some(Rgb565::WHITE),
        margins: Margins {
            top: 5,
            right: 60, // Space for right-side legend in grid layout
            bottom: 5,
            left: 5,
        },
        ..Default::default()
    };

    // Pre-calculate layout constants outside the render loop
    let chart_width_divisor = 2u32;
    let chart_height_divisor = 2u32;
    let margin = 5u32;

    // Use the common visual example runner with animation loop
    common::window::run(
        WindowConfig::new("Data Transition Demo")
            .fps(60)
            .background(Rgb565::WHITE),
        move |display, viewport, elapsed| {
            // Update time provider
            time_provider.set_time_ms((elapsed * 1000.0) as u32);

            // Check if we need to start a new transition phase
            let phase_elapsed = elapsed - phase_start_time;
            if phase_elapsed >= transition_duration + pause_duration {
                // Start next transition phase
                current_phase = (current_phase + 1) % total_phases;
                phase_start_time = elapsed;

                // Update animators with new states for the next phase
                let (from_series, to_series) = match current_phase {
                    0 => (&series_a, &series_b), // A->B
                    1 => (&series_b, &series_c), // B->C
                    2 => (&series_c, &series_a), // C->A
                    _ => (&series_a, &series_b),
                };

                linear_animator.set_states(from_series.clone(), to_series.clone());
                ease_in_animator.set_states(from_series.clone(), to_series.clone());
                ease_out_animator.set_states(from_series.clone(), to_series.clone());
                ease_in_out_animator.set_states(from_series.clone(), to_series.clone());

                // Reset time progress for new phase
                time_progress = TimeBasedProgress::new(time_progress_duration_ms);
            }

            // Get current progress (0-100) during active transition period
            let progress = if phase_elapsed < transition_duration {
                time_progress.progress_from_time(&time_provider)
            } else {
                100 // Animation complete, hold at end state
            };

            // Calculate layout for four charts (2x2 grid) using pre-calculated constants
            let chart_width = viewport.size.width / chart_width_divisor;
            let chart_height = viewport.size.height / chart_height_divisor;

            // Linear chart (top-left)
            let linear_viewport = Rectangle::new(
                viewport.top_left,
                Size::new(chart_width - margin, chart_height - margin),
            );

            // Ease In chart (top-right)
            let ease_in_viewport = Rectangle::new(
                Point::new(
                    viewport.top_left.x + chart_width as i32,
                    viewport.top_left.y,
                ),
                Size::new(chart_width - margin, chart_height - margin),
            );

            // Ease Out chart (bottom-left)
            let ease_out_viewport = Rectangle::new(
                Point::new(
                    viewport.top_left.x,
                    viewport.top_left.y + chart_height as i32,
                ),
                Size::new(chart_width - margin, chart_height - margin),
            );

            // Ease In-Out chart (bottom-right)
            let ease_in_out_viewport = Rectangle::new(
                Point::new(
                    viewport.top_left.x + chart_width as i32,
                    viewport.top_left.y + chart_height as i32,
                ),
                Size::new(chart_width - margin, chart_height - margin),
            );

            // Get current interpolated values using the new animators
            let linear_values = linear_animator
                .value_at(progress)
                .unwrap_or_else(|| series_a.clone());
            let ease_in_values = ease_in_animator
                .value_at(progress)
                .unwrap_or_else(|| series_a.clone());
            let ease_out_values = ease_out_animator
                .value_at(progress)
                .unwrap_or_else(|| series_a.clone());
            let ease_in_out_values = ease_in_out_animator
                .value_at(progress)
                .unwrap_or_else(|| series_a.clone());

            // Optimized render function using pre-created config
            let mut render_transition = |animated_series: &StaticDataSeries<Point2D, 256>,
                                         chart: &LineChart<Rgb565>,
                                         viewport: Rectangle|
             -> ChartResult<()> {
                if !animated_series.is_empty() {
                    chart.draw(animated_series, chart.config(), viewport, display)?;
                }
                Ok(())
            };

            // Render all charts
            render_transition(&linear_values, &linear_chart, linear_viewport)?;
            render_transition(&ease_in_values, &ease_in_chart, ease_in_viewport)?;
            render_transition(&ease_out_values, &ease_out_chart, ease_out_viewport)?;
            render_transition(
                &ease_in_out_values,
                &ease_in_out_chart,
                ease_in_out_viewport,
            )?;

            // Chart labels using pre-created text style
            Text::with_baseline(
                "Linear",
                Point::new(
                    linear_viewport.top_left.x + 5,
                    linear_viewport.top_left.y + 15,
                ),
                text_style,
                Baseline::Top,
            )
            .draw(display)
            .map_err(|_| ChartError::RenderingError)?;

            Text::with_baseline(
                "Ease In",
                Point::new(
                    ease_in_viewport.top_left.x + 5,
                    ease_in_viewport.top_left.y + 15,
                ),
                text_style,
                Baseline::Top,
            )
            .draw(display)
            .map_err(|_| ChartError::RenderingError)?;

            Text::with_baseline(
                "Ease Out",
                Point::new(
                    ease_out_viewport.top_left.x + 5,
                    ease_out_viewport.top_left.y + 15,
                ),
                text_style,
                Baseline::Top,
            )
            .draw(display)
            .map_err(|_| ChartError::RenderingError)?;

            Text::with_baseline(
                "Ease In-Out",
                Point::new(
                    ease_in_out_viewport.top_left.x + 5,
                    ease_in_out_viewport.top_left.y + 15,
                ),
                text_style,
                Baseline::Top,
            )
            .draw(display)
            .map_err(|_| ChartError::RenderingError)?;

            // Phase indicator
            let phase_name = match current_phase {
                0 => "A → B",
                1 => "B → C",
                2 => "C → A",
                _ => "Unknown",
            };

            let mut phase_text = heapless::String::<32>::new();
            let _ = core::fmt::write(&mut phase_text, format_args!("Phase: {}", phase_name));

            Text::with_baseline(
                &phase_text,
                Point::new(
                    viewport.top_left.x + 10,
                    viewport.top_left.y + viewport.size.height as i32 - 20,
                ),
                text_style,
                Baseline::Top,
            )
            .draw(display)
            .map_err(|_| ChartError::RenderingError)?;

            // Progress indicator
            if phase_elapsed < transition_duration {
                let mut progress_text = heapless::String::<32>::new();
                let _ =
                    core::fmt::write(&mut progress_text, format_args!("Progress: {}%", progress));

                Text::with_baseline(
                    &progress_text,
                    Point::new(
                        viewport.top_left.x + 150,
                        viewport.top_left.y + viewport.size.height as i32 - 20,
                    ),
                    text_style,
                    Baseline::Top,
                )
                .draw(display)
                .map_err(|_| ChartError::RenderingError)?;
            } else {
                Text::with_baseline(
                    "Pausing...",
                    Point::new(
                        viewport.top_left.x + 150,
                        viewport.top_left.y + viewport.size.height as i32 - 20,
                    ),
                    text_style,
                    Baseline::Top,
                )
                .draw(display)
                .map_err(|_| ChartError::RenderingError)?;
            }

            Ok(())
        },
    )
}

#[cfg(not(feature = "std"))]
fn main() {
    common::utils::print_feature_requirement("std", "data transition animation");
}
