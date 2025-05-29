//! Simplified animation system with external timeline control.
//!
//! This module provides a streamlined animation system based on a 0-100 progress model
//! where applications control the timeline externally. The design eliminates complex
//! internal state management in favor of stateless, on-demand interpolation.

use crate::data::DataSeries;
use crate::error::ChartResult;
use crate::time::{Milliseconds, TimeProvider};

/// Animation progress value (0-100).
///
/// This represents the completion percentage of an animation, where:
/// - 0 = animation start
/// - 100 = animation complete
/// - Values in between represent partial completion
pub type Progress = u8;

/// Animation easing functions for smooth transitions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EasingFunction {
    /// Linear interpolation (no easing).
    Linear,
    /// Ease in (slow start).
    EaseIn,
    /// Ease out (slow end).
    EaseOut,
    /// Ease in-out (slow start and end).
    EaseInOut,
}

impl EasingFunction {
    /// Apply the easing function to a linear progress value (0.0 to 1.0).
    pub fn apply(self, t: f32) -> f32 {
        match self {
            EasingFunction::Linear => t,
            EasingFunction::EaseIn => t * t,
            EasingFunction::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            EasingFunction::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - 2.0 * (1.0 - t) * (1.0 - t)
                }
            }
        }
    }
}

/// Trait for types that can be interpolated between two values.
pub trait Interpolatable: Clone {
    /// Interpolate between two values with the given progress (0.0 to 1.0).
    fn interpolate(self, other: Self, progress: f32) -> Option<Self>;
}

impl Interpolatable for f32 {
    fn interpolate(self, other: Self, progress: f32) -> Option<Self> {
        Some(self + (other - self) * progress)
    }
}

impl Interpolatable for i32 {
    fn interpolate(self, other: Self, progress: f32) -> Option<Self> {
        Some(self + ((other - self) as f32 * progress) as i32)
    }
}

impl Interpolatable for crate::data::point::Point2D {
    fn interpolate(self, other: Self, progress: f32) -> Option<Self> {
        Some(crate::data::point::Point2D::new(
            self.x + (other.x - self.x) * progress,
            self.y + (other.y - self.y) * progress,
        ))
    }
}

impl Interpolatable for crate::data::point::IntPoint {
    fn interpolate(self, other: Self, progress: f32) -> Option<Self> {
        Some(crate::data::point::IntPoint::new(
            self.x + ((other.x - self.x) as f32 * progress) as i32,
            self.y + ((other.y - self.y) as f32 * progress) as i32,
        ))
    }
}

impl Interpolatable for crate::data::series::StaticDataSeries<crate::data::point::Point2D, 256> {
    fn interpolate(self, other: Self, progress: f32) -> Option<Self> {
        let mut result = crate::data::series::StaticDataSeries::new();

        // Handle different data sizes by taking the minimum
        let min_len = self.len().min(other.len());

        for i in 0..min_len {
            if let (Some(from_point), Some(to_point)) = (self.get(i), other.get(i)) {
                if let Some(interpolated_point) = from_point.interpolate(to_point, progress) {
                    if result.push(interpolated_point).is_err() {
                        return None; // Buffer full
                    }
                }
            }
        }

        Some(result)
    }
}

impl<const N: usize> Interpolatable for heapless::Vec<f32, N> {
    fn interpolate(self, other: Self, progress: f32) -> Option<Self> {
        let mut result = heapless::Vec::new();

        // Handle different vector sizes by taking the minimum
        let min_len = self.len().min(other.len());

        for i in 0..min_len {
            if let (Some(from_val), Some(to_val)) = (self.get(i), other.get(i)) {
                let interpolated = from_val + (to_val - from_val) * progress;
                if result.push(interpolated).is_err() {
                    return None; // Buffer full
                }
            }
        }

        Some(result)
    }
}

/// Simple two-state animator for basic chart transitions.
///
/// This animator handles transitions between two states (from and to) using
/// external progress control. It's stateless and performs interpolation on-demand.
#[derive(Debug, Clone)]
pub struct ChartAnimator<T: Interpolatable> {
    /// Starting state.
    from_state: T,
    /// Target state.
    to_state: T,
    /// Easing function to apply.
    easing: EasingFunction,
}

impl<T: Interpolatable> ChartAnimator<T> {
    /// Create a new chart animator.
    ///
    /// # Arguments
    /// * `from` - Starting state
    /// * `to` - Target state
    /// * `easing` - Easing function to apply
    pub fn new(from: T, to: T, easing: EasingFunction) -> Self {
        Self {
            from_state: from,
            to_state: to,
            easing,
        }
    }

    /// Get the interpolated value at the given progress.
    ///
    /// # Arguments
    /// * `progress` - Animation progress (0-100)
    ///
    /// # Returns
    /// The interpolated value, or None if interpolation fails
    pub fn value_at(&self, progress: Progress) -> Option<T> {
        let linear_progress = (progress as f32) / 100.0;
        let eased_progress = self.easing.apply(linear_progress);
        self.from_state
            .clone()
            .interpolate(self.to_state.clone(), eased_progress)
    }

    /// Update the target state for a new transition.
    ///
    /// # Arguments
    /// * `new_to` - New target state
    pub fn set_target(&mut self, new_to: T) {
        self.to_state = new_to;
    }

    /// Update both states for a new transition.
    ///
    /// # Arguments
    /// * `new_from` - New starting state
    /// * `new_to` - New target state
    pub fn set_states(&mut self, new_from: T, new_to: T) {
        self.from_state = new_from;
        self.to_state = new_to;
    }

    /// Get the starting state.
    pub fn from_state(&self) -> T {
        self.from_state.clone()
    }

    /// Get the target state.
    pub fn to_state(&self) -> T {
        self.to_state.clone()
    }

    /// Get the easing function.
    pub fn easing(&self) -> EasingFunction {
        self.easing
    }

    /// Set the easing function.
    pub fn set_easing(&mut self, easing: EasingFunction) {
        self.easing = easing;
    }
}

/// Multi-state animator for keyframe-based animations.
///
/// This animator supports multiple keyframes with different easing functions
/// between each pair. It automatically determines which keyframe pair to
/// interpolate between based on the current progress.
#[derive(Debug, Clone)]
pub struct MultiStateAnimator<T: Interpolatable, const N: usize> {
    /// Keyframe states and their progress positions.
    keyframes: heapless::Vec<(Progress, T), N>,
    /// Easing functions between keyframes.
    easing_functions: heapless::Vec<EasingFunction, N>,
}

impl<T: Interpolatable, const N: usize> MultiStateAnimator<T, N> {
    /// Create a new multi-state animator.
    pub fn new() -> Self {
        Self {
            keyframes: heapless::Vec::new(),
            easing_functions: heapless::Vec::new(),
        }
    }

    /// Add a keyframe at the specified progress.
    ///
    /// # Arguments
    /// * `progress` - Progress position (0-100)
    /// * `state` - State at this keyframe
    /// * `easing` - Easing function to use when transitioning TO this keyframe
    ///
    /// # Returns
    /// Ok(()) on success, Err if the animator is full
    pub fn add_keyframe(
        &mut self,
        progress: Progress,
        state: T,
        easing: EasingFunction,
    ) -> ChartResult<()> {
        self.keyframes.push((progress, state)).map_err(|_| {
            crate::error::ChartError::DataError(crate::error::DataError::BUFFER_FULL)
        })?;

        self.easing_functions.push(easing).map_err(|_| {
            crate::error::ChartError::DataError(crate::error::DataError::BUFFER_FULL)
        })?;

        // Sort keyframes by progress
        self.keyframes.sort_by_key(|(progress, _)| *progress);

        Ok(())
    }

    /// Get the interpolated value at the given progress.
    ///
    /// # Arguments
    /// * `progress` - Animation progress (0-100)
    ///
    /// # Returns
    /// The interpolated value, or None if no keyframes are set
    pub fn value_at(&self, progress: Progress) -> Option<T> {
        if self.keyframes.is_empty() {
            return None;
        }

        if self.keyframes.len() == 1 {
            return Some(self.keyframes[0].1.clone());
        }

        // Find the keyframe pair to interpolate between
        let mut from_idx = 0;
        let mut to_idx = 0;

        for (i, (keyframe_progress, _)) in self.keyframes.iter().enumerate() {
            if *keyframe_progress <= progress {
                from_idx = i;
            } else {
                to_idx = i;
                break;
            }
        }

        // If we're past the last keyframe, return the last state
        if from_idx == self.keyframes.len() - 1 {
            return Some(self.keyframes[from_idx].1.clone());
        }

        // If we haven't found a 'to' keyframe, use the last one
        if to_idx == 0 && from_idx > 0 {
            to_idx = self.keyframes.len() - 1;
        }

        let (from_progress, from_state) = &self.keyframes[from_idx];
        let (to_progress, to_state) = &self.keyframes[to_idx];

        // Calculate local progress between the two keyframes
        let progress_range = to_progress.saturating_sub(*from_progress);
        if progress_range == 0 {
            return Some(from_state.clone());
        }

        let local_progress =
            (progress.saturating_sub(*from_progress) as f32) / (progress_range as f32);
        let easing = self
            .easing_functions
            .get(to_idx)
            .copied()
            .unwrap_or(EasingFunction::Linear);
        let eased_progress = easing.apply(local_progress);

        from_state
            .clone()
            .interpolate(to_state.clone(), eased_progress)
    }

    /// Get the number of keyframes.
    pub fn keyframe_count(&self) -> usize {
        self.keyframes.len()
    }

    /// Clear all keyframes.
    pub fn clear(&mut self) {
        self.keyframes.clear();
        self.easing_functions.clear();
    }
}

impl<T: Interpolatable, const N: usize> Default for MultiStateAnimator<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Streaming animator for continuous data updates.
///
/// This animator manages a sliding window of data points and provides
/// smooth interpolation for real-time data visualization. It's designed
/// for scenarios where new data arrives continuously.
#[derive(Debug)]
pub struct StreamingAnimator<T: Copy + Clone> {
    /// Sliding window buffer for data points.
    buffer: crate::memory::ManagedSlidingWindow<T, 100>,
    /// Interpolation progress for smooth transitions.
    interpolation_progress: Progress,
    /// Whether to enable smooth interpolation between updates.
    smooth_interpolation: bool,
}

impl<T: Copy + Clone> StreamingAnimator<T> {
    /// Create a new streaming animator.
    pub fn new() -> Self {
        Self {
            buffer: crate::memory::ManagedSlidingWindow::new(),
            interpolation_progress: 0,
            smooth_interpolation: true,
        }
    }

    /// Add a new data point to the stream.
    ///
    /// # Arguments
    /// * `point` - The new data point to add
    pub fn push_data(&mut self, point: T) {
        self.buffer.push(point);
        // Reset interpolation progress when new data arrives
        self.interpolation_progress = 0;
    }

    /// Get the current data window.
    pub fn current_data(&self) -> impl Iterator<Item = T> + '_ {
        self.buffer.iter()
    }

    /// Set the interpolation progress for smooth transitions.
    ///
    /// # Arguments
    /// * `progress` - Interpolation progress (0-100)
    pub fn set_interpolation_progress(&mut self, progress: Progress) {
        self.interpolation_progress = progress;
    }

    /// Get the current interpolation progress.
    pub fn interpolation_progress(&self) -> Progress {
        self.interpolation_progress
    }

    /// Enable or disable smooth interpolation.
    ///
    /// # Arguments
    /// * `enabled` - Whether to enable smooth interpolation
    pub fn set_smooth_interpolation(&mut self, enabled: bool) {
        self.smooth_interpolation = enabled;
    }

    /// Check if smooth interpolation is enabled.
    pub fn is_smooth_interpolation_enabled(&self) -> bool {
        self.smooth_interpolation
    }

    /// Get the buffer capacity.
    pub fn capacity(&self) -> usize {
        100 // Fixed capacity for the sliding window
    }

    /// Get the current buffer size.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Check if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Clear all data from the buffer.
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.interpolation_progress = 0;
    }

    /// Update animation with delta time (compatibility method).
    ///
    /// # Arguments
    /// * `_delta_time` - Time elapsed since last update (currently unused)
    ///
    /// # Returns
    /// Always returns Ok(false) as streaming animations don't have completion state
    pub fn update_with_delta(
        &mut self,
        _delta_time: Milliseconds,
    ) -> crate::error::AnimationResult<bool> {
        // For streaming animations, we don't need to track time-based updates
        // The animation state is controlled externally via set_interpolation_progress
        Ok(false)
    }
}

impl<T: Copy + Clone> Default for StreamingAnimator<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Time-based progress calculator for converting time to progress values.
///
/// This helper struct provides utilities for calculating progress based on
/// elapsed time, making it easier to integrate with time-based animations
/// while maintaining the external timeline control design.
#[derive(Debug, Clone)]
pub struct TimeBasedProgress {
    /// Animation duration in milliseconds.
    duration_ms: Milliseconds,
    /// Start time in milliseconds.
    start_time_ms: Option<Milliseconds>,
    /// Whether the animation should loop.
    looping: bool,
}

impl TimeBasedProgress {
    /// Create a new time-based progress calculator.
    ///
    /// # Arguments
    /// * `duration_ms` - Animation duration in milliseconds
    pub fn new(duration_ms: Milliseconds) -> Self {
        Self {
            duration_ms,
            start_time_ms: None,
            looping: false,
        }
    }

    /// Create a new looping time-based progress calculator.
    ///
    /// # Arguments
    /// * `duration_ms` - Animation duration in milliseconds
    pub fn new_looping(duration_ms: Milliseconds) -> Self {
        Self {
            duration_ms,
            start_time_ms: None,
            looping: true,
        }
    }

    /// Calculate progress based on the current time from a time provider.
    ///
    /// # Arguments
    /// * `time_provider` - Time provider to get current time
    ///
    /// # Returns
    /// Current progress (0-100), or 100 if animation is complete (non-looping)
    pub fn progress_from_time<T: TimeProvider>(&mut self, time_provider: &T) -> Progress {
        let current_time = time_provider.current_time_ms();

        // Initialize start time on first call
        if self.start_time_ms.is_none() {
            self.start_time_ms = Some(current_time);
            return 0;
        }

        let start_time = self.start_time_ms.unwrap();
        let elapsed = current_time.saturating_sub(start_time);

        if self.looping {
            // For looping animations, wrap around
            let cycle_progress = elapsed % self.duration_ms;
            ((cycle_progress as f32 / self.duration_ms as f32) * 100.0) as Progress
        } else {
            // For non-looping animations, clamp to 100
            if elapsed >= self.duration_ms {
                100
            } else {
                ((elapsed as f32 / self.duration_ms as f32) * 100.0) as Progress
            }
        }
    }

    /// Calculate progress based on elapsed time.
    ///
    /// # Arguments
    /// * `elapsed_ms` - Elapsed time in milliseconds
    ///
    /// # Returns
    /// Current progress (0-100)
    pub fn progress_from_elapsed(&self, elapsed_ms: Milliseconds) -> Progress {
        if self.looping {
            let cycle_progress = elapsed_ms % self.duration_ms;
            ((cycle_progress as f32 / self.duration_ms as f32) * 100.0) as Progress
        } else if elapsed_ms >= self.duration_ms {
            100
        } else {
            ((elapsed_ms as f32 / self.duration_ms as f32) * 100.0) as Progress
        }
    }

    /// Reset the animation to start from the current time.
    pub fn reset(&mut self) {
        self.start_time_ms = None;
    }

    /// Check if the animation is complete (non-looping only).
    ///
    /// # Arguments
    /// * `time_provider` - Time provider to get current time
    pub fn is_complete<T: TimeProvider>(&self, time_provider: &T) -> bool {
        if self.looping {
            return false; // Looping animations never complete
        }

        if let Some(start_time) = self.start_time_ms {
            let current_time = time_provider.current_time_ms();
            let elapsed = current_time.saturating_sub(start_time);
            elapsed >= self.duration_ms
        } else {
            false // Not started yet
        }
    }

    /// Get the animation duration.
    pub fn duration_ms(&self) -> Milliseconds {
        self.duration_ms
    }

    /// Set the animation duration.
    pub fn set_duration_ms(&mut self, duration_ms: Milliseconds) {
        self.duration_ms = duration_ms;
    }

    /// Check if the animation is set to loop.
    pub fn is_looping(&self) -> bool {
        self.looping
    }

    /// Set whether the animation should loop.
    pub fn set_looping(&mut self, looping: bool) {
        self.looping = looping;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::ManualTimeProvider;

    #[test]
    fn test_easing_functions() {
        assert_eq!(EasingFunction::Linear.apply(0.5), 0.5);
        assert_eq!(EasingFunction::EaseIn.apply(0.5), 0.25);
        assert_eq!(EasingFunction::EaseOut.apply(0.5), 0.75);
        assert_eq!(EasingFunction::EaseInOut.apply(0.5), 0.5);
    }

    #[test]
    fn test_interpolatable_f32() {
        let result = 10.0f32.interpolate(20.0, 0.5);
        assert_eq!(result, Some(15.0));
    }

    #[test]
    fn test_interpolatable_i32() {
        let result = 10i32.interpolate(20, 0.5);
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_chart_animator() {
        let animator = ChartAnimator::new(0.0f32, 100.0, EasingFunction::Linear);

        assert_eq!(animator.value_at(0), Some(0.0));
        assert_eq!(animator.value_at(50), Some(50.0));
        assert_eq!(animator.value_at(100), Some(100.0));
    }

    #[test]
    fn test_chart_animator_easing() {
        let animator = ChartAnimator::new(0.0f32, 100.0, EasingFunction::EaseIn);

        let value_at_50 = animator.value_at(50).unwrap();
        assert!(value_at_50 < 50.0); // EaseIn should be slower at the start
    }

    #[test]
    fn test_multi_state_animator() {
        let mut animator: MultiStateAnimator<f32, 4> = MultiStateAnimator::new();

        animator
            .add_keyframe(0, 0.0, EasingFunction::Linear)
            .unwrap();
        animator
            .add_keyframe(50, 25.0, EasingFunction::Linear)
            .unwrap();
        animator
            .add_keyframe(100, 100.0, EasingFunction::Linear)
            .unwrap();

        assert_eq!(animator.value_at(0), Some(0.0));
        assert_eq!(animator.value_at(25), Some(12.5));
        assert_eq!(animator.value_at(50), Some(25.0));
        assert_eq!(animator.value_at(75), Some(62.5));
        assert_eq!(animator.value_at(100), Some(100.0));
    }

    #[test]
    fn test_streaming_animator() {
        let mut animator = StreamingAnimator::new();

        assert!(animator.is_empty());

        animator.push_data(1.0f32);
        animator.push_data(2.0f32);

        assert_eq!(animator.len(), 2);
        assert!(!animator.is_empty());

        let data: heapless::Vec<f32, 100> = animator.current_data().collect();
        let expected: heapless::Vec<f32, 100> = heapless::Vec::from_slice(&[1.0, 2.0]).unwrap();
        assert_eq!(data, expected);
    }

    #[test]
    fn test_time_based_progress() {
        let mut progress_calc = TimeBasedProgress::new(1000); // 1 second
        let mut time_provider = ManualTimeProvider::new();

        // First call should return 0 and initialize start time
        assert_eq!(progress_calc.progress_from_time(&time_provider), 0);

        // Advance time by 500ms (50% of duration)
        time_provider.advance_ms(500);
        assert_eq!(progress_calc.progress_from_time(&time_provider), 50);

        // Advance to completion
        time_provider.advance_ms(500);
        assert_eq!(progress_calc.progress_from_time(&time_provider), 100);

        // Beyond completion should still return 100
        time_provider.advance_ms(500);
        assert_eq!(progress_calc.progress_from_time(&time_provider), 100);
    }

    #[test]
    fn test_time_based_progress_looping() {
        let mut progress_calc = TimeBasedProgress::new_looping(1000); // 1 second loop
        let mut time_provider = ManualTimeProvider::new();

        // First call should return 0
        assert_eq!(progress_calc.progress_from_time(&time_provider), 0);

        // Complete one cycle
        time_provider.advance_ms(1000);
        assert_eq!(progress_calc.progress_from_time(&time_provider), 0);

        // Half way through second cycle
        time_provider.advance_ms(500);
        assert_eq!(progress_calc.progress_from_time(&time_provider), 50);
    }

    #[test]
    fn test_progress_from_elapsed() {
        let progress_calc = TimeBasedProgress::new(2000); // 2 seconds

        assert_eq!(progress_calc.progress_from_elapsed(0), 0);
        assert_eq!(progress_calc.progress_from_elapsed(1000), 50);
        assert_eq!(progress_calc.progress_from_elapsed(2000), 100);
        assert_eq!(progress_calc.progress_from_elapsed(3000), 100); // Clamped
    }
}
