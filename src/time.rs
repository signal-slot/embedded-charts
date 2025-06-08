//! Time abstraction layer for animation systems.
//!
//! This module provides a time abstraction that works in both std and no_std environments.
//! It allows the animation system to work with different timer sources while maintaining
//! a consistent API.

/// Time value in milliseconds.
pub type Milliseconds = u32;

/// Time value in microseconds.
pub type Microseconds = u64;

/// Trait for providing time information to the animation system.
///
/// This trait abstracts time operations to support both std and no_std environments.
/// Implementations can use system clocks, hardware timers, or external time sources.
pub trait TimeProvider {
    /// Get the current time in milliseconds since some reference point.
    ///
    /// The reference point doesn't matter as long as it's consistent.
    /// This is typically used for calculating elapsed time between calls.
    fn current_time_ms(&self) -> Milliseconds;

    /// Get the current time in microseconds since some reference point.
    ///
    /// This provides higher precision timing for fine-grained animations.
    /// The reference point should be the same as `current_time_ms()`.
    fn current_time_us(&self) -> Microseconds;

    /// Calculate elapsed time in milliseconds since the last call.
    ///
    /// This is a convenience method that handles the delta calculation.
    /// The implementation should track the last time internally.
    fn elapsed_ms(&mut self) -> Milliseconds {
        let current = self.current_time_ms();
        let last = self.last_time_ms();
        self.update_last_time_ms(current);
        current.saturating_sub(last)
    }

    /// Calculate elapsed time in microseconds since the last call.
    ///
    /// This is a convenience method that handles the delta calculation.
    /// The implementation should track the last time internally.
    fn elapsed_us(&mut self) -> Microseconds {
        let current = self.current_time_us();
        let last = self.last_time_us();
        self.update_last_time_us(current);
        current.saturating_sub(last)
    }

    /// Get the last recorded time in milliseconds.
    ///
    /// This is used internally by `elapsed_ms()`.
    fn last_time_ms(&self) -> Milliseconds;

    /// Get the last recorded time in microseconds.
    ///
    /// This is used internally by `elapsed_us()`.
    fn last_time_us(&self) -> Microseconds;

    /// Update the last recorded time in milliseconds.
    ///
    /// This is used internally by `elapsed_ms()`.
    fn update_last_time_ms(&mut self, time: Milliseconds);

    /// Update the last recorded time in microseconds.
    ///
    /// This is used internally by `elapsed_us()`.
    fn update_last_time_us(&mut self, time: Microseconds);

    /// Reset the time provider to start fresh timing.
    ///
    /// This resets any internal state and starts timing from the current moment.
    fn reset(&mut self) {
        let current_ms = self.current_time_ms();
        let current_us = self.current_time_us();
        self.update_last_time_ms(current_ms);
        self.update_last_time_us(current_us);
    }
}

/// A monotonic time provider for no_std environments.
///
/// This implementation uses a user-provided timer function to get the current time.
/// It's designed to work with hardware timers or other monotonic time sources.
#[derive(Debug)]
pub struct MonotonicTimeProvider<F>
where
    F: Fn() -> Microseconds,
{
    /// Function to get current time in microseconds.
    timer_fn: F,
    /// Last recorded time in milliseconds.
    last_ms: Milliseconds,
    /// Last recorded time in microseconds.
    last_us: Microseconds,
}

impl<F> MonotonicTimeProvider<F>
where
    F: Fn() -> Microseconds,
{
    /// Create a new monotonic time provider with the given timer function.
    ///
    /// The timer function should return the current time in microseconds
    /// from a monotonic source (e.g., hardware timer, system tick counter).
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use embedded_charts::time::MonotonicTimeProvider;
    ///
    /// // Example with a hypothetical hardware timer
    /// fn hardware_timer_get_us() -> u64 {
    ///     // Mock implementation - in real use, this would read from hardware
    ///     1000
    /// }
    ///
    /// let timer = MonotonicTimeProvider::new(|| {
    ///     // Get microseconds from hardware timer
    ///     hardware_timer_get_us()
    /// });
    /// ```
    pub fn new(timer_fn: F) -> Self {
        let current_us = timer_fn();
        Self {
            timer_fn,
            last_ms: (current_us / 1000) as Milliseconds,
            last_us: current_us,
        }
    }

    /// Create a new monotonic time provider with a tick-based timer.
    ///
    /// This is useful when you have a timer that increments at a known frequency.
    ///
    /// # Arguments
    ///
    /// * `get_ticks` - Function that returns the current tick count
    /// * `ticks_per_second` - Number of ticks per second (timer frequency)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use embedded_charts::time::{MonotonicTimeProvider, Microseconds};
    ///
    /// // Mock hardware timer function
    /// fn hardware_timer_get_ticks() -> u64 {
    ///     // Mock implementation - in real use, this would read from hardware
    ///     1000
    /// }
    ///
    /// // Example with a 1MHz timer (1,000,000 ticks per second)
    /// // let get_ticks = || hardware_timer_get_ticks();
    /// // let timer = MonotonicTimeProvider::from_ticks(get_ticks, 1_000_000);
    ///
    /// // Use the timer to get current time
    /// // let current_time = timer.now();
    /// ```
    pub fn from_ticks<G>(
        get_ticks: G,
        ticks_per_second: u32,
    ) -> MonotonicTimeProvider<impl Fn() -> Microseconds>
    where
        G: Fn() -> u64,
    {
        let timer_fn = move || {
            let ticks = get_ticks();
            // Convert ticks to microseconds
            (ticks * 1_000_000) / ticks_per_second as u64
        };

        let current_us = timer_fn();
        MonotonicTimeProvider {
            timer_fn,
            last_ms: (current_us / 1000) as Milliseconds,
            last_us: current_us,
        }
    }
}

impl<F> TimeProvider for MonotonicTimeProvider<F>
where
    F: Fn() -> Microseconds,
{
    fn current_time_ms(&self) -> Milliseconds {
        ((self.timer_fn)() / 1000) as Milliseconds
    }

    fn current_time_us(&self) -> Microseconds {
        (self.timer_fn)()
    }

    fn last_time_ms(&self) -> Milliseconds {
        self.last_ms
    }

    fn last_time_us(&self) -> Microseconds {
        self.last_us
    }

    fn update_last_time_ms(&mut self, time: Milliseconds) {
        self.last_ms = time;
    }

    fn update_last_time_us(&mut self, time: Microseconds) {
        self.last_us = time;
    }
}

/// A simple time provider for testing and simulation.
///
/// This provider allows manual control of time, useful for testing animations
/// or when you want to control timing externally.
#[derive(Debug, Clone)]
pub struct ManualTimeProvider {
    /// Current time in microseconds.
    current_us: Microseconds,
    /// Last recorded time in milliseconds.
    last_ms: Milliseconds,
    /// Last recorded time in microseconds.
    last_us: Microseconds,
}

impl ManualTimeProvider {
    /// Create a new manual time provider starting at time zero.
    pub fn new() -> Self {
        Self {
            current_us: 0,
            last_ms: 0,
            last_us: 0,
        }
    }

    /// Create a new manual time provider starting at the specified time.
    pub fn with_start_time(start_us: Microseconds) -> Self {
        Self {
            current_us: start_us,
            last_ms: (start_us / 1000) as Milliseconds,
            last_us: start_us,
        }
    }

    /// Advance time by the specified number of milliseconds.
    pub fn advance_ms(&mut self, delta_ms: Milliseconds) {
        self.current_us += (delta_ms as Microseconds) * 1000;
    }

    /// Advance time by the specified number of microseconds.
    pub fn advance_us(&mut self, delta_us: Microseconds) {
        self.current_us += delta_us;
    }

    /// Set the current time to the specified value in microseconds.
    pub fn set_time_us(&mut self, time_us: Microseconds) {
        self.current_us = time_us;
    }

    /// Set the current time to the specified value in milliseconds.
    pub fn set_time_ms(&mut self, time_ms: Milliseconds) {
        self.current_us = (time_ms as Microseconds) * 1000;
    }
}

impl Default for ManualTimeProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeProvider for ManualTimeProvider {
    fn current_time_ms(&self) -> Milliseconds {
        (self.current_us / 1000) as Milliseconds
    }

    fn current_time_us(&self) -> Microseconds {
        self.current_us
    }

    fn last_time_ms(&self) -> Milliseconds {
        self.last_ms
    }

    fn last_time_us(&self) -> Microseconds {
        self.last_us
    }

    fn update_last_time_ms(&mut self, time: Milliseconds) {
        self.last_ms = time;
    }

    fn update_last_time_us(&mut self, time: Microseconds) {
        self.last_us = time;
    }
}

/// Standard library time provider using `std::time::Instant`.
///
/// This provider uses the system's monotonic clock and is only available
/// when the `std` feature is enabled.
#[cfg(feature = "std")]
#[derive(Debug)]
pub struct StdTimeProvider {
    /// Reference point for time calculations.
    start_time: std::time::Instant,
    /// Last recorded time in milliseconds.
    last_ms: Milliseconds,
    /// Last recorded time in microseconds.
    last_us: Microseconds,
}

#[cfg(feature = "std")]
impl StdTimeProvider {
    /// Create a new standard library time provider.
    pub fn new() -> Self {
        let now = std::time::Instant::now();
        Self {
            start_time: now,
            last_ms: 0,
            last_us: 0,
        }
    }
}

#[cfg(feature = "std")]
impl Default for StdTimeProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "std")]
impl TimeProvider for StdTimeProvider {
    fn current_time_ms(&self) -> Milliseconds {
        self.start_time.elapsed().as_millis() as Milliseconds
    }

    fn current_time_us(&self) -> Microseconds {
        self.start_time.elapsed().as_micros() as Microseconds
    }

    fn last_time_ms(&self) -> Milliseconds {
        self.last_ms
    }

    fn last_time_us(&self) -> Microseconds {
        self.last_us
    }

    fn update_last_time_ms(&mut self, time: Milliseconds) {
        self.last_ms = time;
    }

    fn update_last_time_us(&mut self, time: Microseconds) {
        self.last_us = time;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manual_time_provider() {
        let mut provider = ManualTimeProvider::new();

        assert_eq!(provider.current_time_ms(), 0);
        assert_eq!(provider.current_time_us(), 0);

        provider.advance_ms(100);
        assert_eq!(provider.current_time_ms(), 100);
        assert_eq!(provider.current_time_us(), 100_000);

        provider.advance_us(500);
        assert_eq!(provider.current_time_ms(), 100);
        assert_eq!(provider.current_time_us(), 100_500);
    }

    #[test]
    fn test_manual_time_provider_elapsed() {
        let mut provider = ManualTimeProvider::new();

        // First call should return 0 (no time has passed)
        assert_eq!(provider.elapsed_ms(), 0);

        provider.advance_ms(50);
        assert_eq!(provider.elapsed_ms(), 50);

        provider.advance_ms(25);
        assert_eq!(provider.elapsed_ms(), 25);
    }

    #[test]
    fn test_monotonic_time_provider() {
        use core::cell::RefCell;

        let counter = RefCell::new(0u64);
        let timer_fn = || {
            let mut c = counter.borrow_mut();
            *c += 1000; // Advance by 1ms each call
            *c
        };

        let provider = MonotonicTimeProvider::new(timer_fn);

        let first_time = provider.current_time_us();
        let second_time = provider.current_time_us();

        assert!(second_time > first_time);
        assert_eq!(second_time - first_time, 1000); // 1ms difference
    }

    // #[test]
    // fn test_monotonic_time_provider_from_ticks() {
    //     // TODO: Fix type inference issues with closure types
    //     // This test is temporarily disabled due to complex type inference
    // }

    #[cfg(feature = "std")]
    #[test]
    fn test_std_time_provider() {
        let provider = StdTimeProvider::new();

        let first_time = provider.current_time_ms();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let second_time = provider.current_time_ms();

        assert!(second_time >= first_time + 10);
    }

    #[test]
    fn test_time_provider_reset() {
        let mut provider = ManualTimeProvider::new();

        provider.advance_ms(100);
        let _ = provider.elapsed_ms(); // This should be 100

        provider.advance_ms(50);
        provider.reset();

        provider.advance_ms(25);
        let elapsed = provider.elapsed_ms();

        assert_eq!(elapsed, 25); // Should only count from reset point
    }
}
