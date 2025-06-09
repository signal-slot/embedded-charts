//! Tick generation algorithms for axes.

use crate::axes::traits::{AxisValue, Tick, TickGenerator};
use crate::math::{Math, NumericConversion};
use heapless::Vec;

/// Linear tick generator that creates evenly spaced ticks
#[derive(Debug, Clone)]
pub struct LinearTickGenerator {
    /// Preferred number of ticks
    preferred_count: usize,
    /// Whether to include minor ticks
    include_minor_ticks: bool,
    /// Ratio of minor ticks to major ticks
    minor_tick_ratio: usize,
}

impl LinearTickGenerator {
    /// Create a new linear tick generator
    pub fn new(preferred_count: usize) -> Self {
        Self {
            preferred_count: preferred_count.clamp(2, 20),
            include_minor_ticks: false,
            minor_tick_ratio: 4,
        }
    }

    /// Enable minor ticks with the specified ratio
    ///
    /// # Arguments
    /// * `ratio` - Number of minor ticks between major ticks
    pub fn with_minor_ticks(mut self, ratio: usize) -> Self {
        self.include_minor_ticks = true;
        self.minor_tick_ratio = ratio.clamp(1, 10);
        self
    }

    /// Disable minor ticks
    pub fn without_minor_ticks(mut self) -> Self {
        self.include_minor_ticks = false;
        self
    }

    /// Calculate nice tick spacing for the given range
    fn calculate_nice_step<T: AxisValue>(min: T, max: T, target_count: usize) -> T {
        let min_f32 = min.to_f32();
        let max_f32 = max.to_f32();
        let range = max_f32 - min_f32;

        // Safety checks for edge cases
        if target_count <= 1 {
            return T::from_f32(range.max(1.0)); // Fallback to range or 1.0
        }

        if range <= 0.0 || !range.is_finite() {
            return T::from_f32(1.0); // Fallback to 1.0
        }

        let rough_step = range / (target_count - 1) as f32;

        // Minimum step threshold to prevent infinite loops
        if rough_step < 1e-10 {
            return T::from_f32(1e-6); // Safe minimum step
        }

        // Find the magnitude of the step using Math abstraction
        let rough_step_num = rough_step.to_number();
        let magnitude = Math::floor(Math::log10(rough_step_num));
        let ten = 10.0f32.to_number();
        let normalized_step = rough_step_num / Math::pow(ten, magnitude);

        // Choose a nice normalized step
        let one = 1.0f32.to_number();
        let two = 2.0f32.to_number();
        let five = 5.0f32.to_number();
        let ten_norm = 10.0f32.to_number();

        let nice_normalized = if normalized_step <= one {
            one
        } else if normalized_step <= two {
            two
        } else if normalized_step <= five {
            five
        } else {
            ten_norm
        };

        let result = if magnitude >= 0.0.to_number() && magnitude <= 10.0.to_number() {
            nice_normalized * Math::pow(ten, magnitude)
        } else {
            // Fallback for extreme magnitudes to prevent overflow
            nice_normalized
        };
        let step_f32 = f32::from_number(result);

        // Final safety check
        if step_f32 <= 0.0 || !step_f32.is_finite() {
            return T::from_f32(range / target_count as f32); // Simple fallback
        }

        T::from_f32(step_f32)
    }

    /// Generate major ticks for the range
    fn generate_major_ticks<T: AxisValue>(&self, min: T, max: T) -> Vec<Tick<T>, 32> {
        let mut ticks = Vec::new();

        let step = Self::calculate_nice_step(min, max, self.preferred_count);
        let step_f32 = step.to_f32();

        // Safety check: prevent infinite loops from zero or very small steps
        if step_f32 <= 0.0 || step_f32 < 1e-10 || !step_f32.is_finite() {
            // Fallback: create simple ticks at min and max
            let label_min = min.format();
            let label_max = max.format();
            let _ = ticks.push(Tick::major(min, label_min.as_str()));
            if min.to_f32() != max.to_f32() {
                let _ = ticks.push(Tick::major(max, label_max.as_str()));
            }
            return ticks;
        }

        // Find the first tick position (rounded down to nearest step)
        let first_tick_value = {
            let min_f32 = min.to_f32();
            let min_num = min_f32.to_number();
            let step_num = step_f32.to_number();
            let first_tick_num = Math::floor(min_num / step_num) * step_num;
            let first_tick_f32 = f32::from_number(first_tick_num);
            T::from_f32(first_tick_f32)
        };

        // Generate ticks with additional safety checks
        let mut current = first_tick_value;
        let mut iteration_count = 0;
        let max_iterations = 100; // Safety limit

        while current.to_f32() <= max.to_f32()
            && ticks.len() < 32
            && iteration_count < max_iterations
        {
            if current.to_f32() >= min.to_f32() {
                let label = current.format();

                let _ = ticks.push(Tick::major(current, label.as_str()));
            }

            let prev_value = current.to_f32();
            current = T::from_f32(current.to_f32() + step_f32);
            iteration_count += 1;

            // Safety check: ensure we're actually making progress
            if current.to_f32() <= prev_value {
                break; // Step is too small, causing no progress
            }
        }

        ticks
    }

    /// Generate minor ticks for the given range
    fn generate_minor_ticks_for_range<T: AxisValue>(
        &self,
        min: T,
        max: T,
        major_ticks: &[Tick<T>],
    ) -> Vec<Tick<T>, 32> {
        let mut minor_ticks = Vec::new();

        if major_ticks.len() < 2 {
            return minor_ticks;
        }

        // Calculate the step size between major ticks
        let major_step = major_ticks[1].value.to_f32() - major_ticks[0].value.to_f32();
        let minor_step = major_step / (self.minor_tick_ratio + 1) as f32;

        // Generate minor ticks between ALL consecutive pairs of major ticks
        for window in major_ticks.windows(2) {
            if let [tick1, tick2] = window {
                for i in 1..=self.minor_tick_ratio {
                    let minor_value_f32 = tick1.value.to_f32() + minor_step * i as f32;

                    // Only add if within range and not equal to the next major tick
                    if minor_value_f32 >= min.to_f32() && minor_value_f32 <= max.to_f32() {
                        // Make sure we don't add a minor tick exactly at a major tick position
                        let distance_to_next_major = (tick2.value.to_f32() - minor_value_f32).abs();
                        if distance_to_next_major > 0.001 {
                            // Small tolerance for floating point comparison
                            let minor_value = T::from_f32(minor_value_f32);
                            if minor_ticks.len() < 32 {
                                let _ = minor_ticks.push(Tick::minor(minor_value));
                            }
                        }
                    }
                }
            }
        }

        minor_ticks
    }

    /// Generate minor ticks between major ticks (legacy method for compatibility)
    #[allow(dead_code)]
    fn generate_minor_ticks<T: AxisValue>(&self, major_ticks: &[Tick<T>]) -> Vec<Tick<T>, 32> {
        let mut minor_ticks = Vec::new();

        if major_ticks.len() < 2 {
            return minor_ticks;
        }

        // Calculate the step size between major ticks
        let major_step = major_ticks[1].value.to_f32() - major_ticks[0].value.to_f32();
        let minor_step = major_step / (self.minor_tick_ratio + 1) as f32;

        // Generate minor ticks between each pair of major ticks
        for window in major_ticks.windows(2) {
            if let [tick1, _tick2] = window {
                for i in 1..=self.minor_tick_ratio {
                    let minor_value = T::from_f32(tick1.value.to_f32() + minor_step * i as f32);
                    if minor_ticks.len() < 32 {
                        let _ = minor_ticks.push(Tick::minor(minor_value));
                    }
                }
            }
        }

        minor_ticks
    }
}

impl<T: AxisValue> TickGenerator<T> for LinearTickGenerator {
    fn generate_ticks(&self, min: T, max: T, max_ticks: usize) -> Vec<Tick<T>, 32> {
        let mut all_ticks = Vec::new();

        // Generate major ticks
        let major_ticks = self.generate_major_ticks(min, max);

        // Add major ticks to the result
        for tick in &major_ticks {
            if all_ticks.len() < max_ticks.min(32) {
                let _ = all_ticks.push(tick.clone());
            }
        }

        // Generate and add minor ticks if enabled
        if self.include_minor_ticks {
            let minor_ticks = self.generate_minor_ticks_for_range(min, max, &major_ticks);

            for tick in minor_ticks {
                if all_ticks.len() < max_ticks.min(32) {
                    let _ = all_ticks.push(tick);
                }
            }

            // Sort ticks by value (manual implementation for heapless::Vec)
            let len = all_ticks.len();
            for i in 0..len {
                for j in 0..len - 1 - i {
                    let a_val = all_ticks[j].value.to_f32();
                    let b_val = all_ticks[j + 1].value.to_f32();
                    if a_val > b_val {
                        all_ticks.swap(j, j + 1);
                    }
                }
            }
        }

        all_ticks
    }

    fn preferred_tick_count(&self) -> usize {
        self.preferred_count
    }

    fn set_preferred_tick_count(&mut self, count: usize) {
        self.preferred_count = count.clamp(2, 20);
    }
}

/// Custom tick generator that allows manual specification of tick positions
#[derive(Debug, Clone)]
pub struct CustomTickGenerator<T> {
    /// Manually specified tick positions
    ticks: Vec<Tick<T>, 32>,
}

impl<T: Copy> CustomTickGenerator<T> {
    /// Create a new custom tick generator
    pub fn new() -> Self {
        Self { ticks: Vec::new() }
    }

    /// Add a major tick at the specified value with a label
    pub fn add_major_tick(mut self, value: T, label: &str) -> Self {
        if self.ticks.len() < 32 {
            let _ = self.ticks.push(Tick::major(value, label));
        }
        self
    }

    /// Add a minor tick at the specified value
    pub fn add_minor_tick(mut self, value: T) -> Self {
        if self.ticks.len() < 32 {
            let _ = self.ticks.push(Tick::minor(value));
        }
        self
    }

    /// Clear all ticks
    pub fn clear(&mut self) {
        self.ticks.clear();
    }
}

impl<T: Copy + PartialOrd> TickGenerator<T> for CustomTickGenerator<T> {
    fn generate_ticks(&self, min: T, max: T, max_ticks: usize) -> Vec<Tick<T>, 32> {
        let mut result = Vec::new();

        for tick in &self.ticks {
            if tick.value >= min && tick.value <= max && result.len() < max_ticks.min(32) {
                let _ = result.push(tick.clone());
            }
        }

        result
    }

    fn preferred_tick_count(&self) -> usize {
        self.ticks.len()
    }

    fn set_preferred_tick_count(&mut self, _count: usize) {
        // Custom tick generator ignores preferred count
    }
}

impl<T: Copy> Default for CustomTickGenerator<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Logarithmic tick generator for logarithmic scales
#[derive(Debug, Clone)]
pub struct LogTickGenerator {
    /// Base of the logarithm (typically 10)
    base: f32,
    /// Whether to include minor ticks
    include_minor_ticks: bool,
}

impl LogTickGenerator {
    /// Create a new logarithmic tick generator with base 10
    pub fn new() -> Self {
        Self {
            base: 10.0,
            include_minor_ticks: false,
        }
    }

    /// Create a logarithmic tick generator with a custom base
    pub fn with_base(base: f32) -> Self {
        Self {
            base: base.max(2.0),
            include_minor_ticks: false,
        }
    }

    /// Enable minor ticks
    pub fn with_minor_ticks(mut self) -> Self {
        self.include_minor_ticks = true;
        self
    }
}

impl TickGenerator<f32> for LogTickGenerator {
    fn generate_ticks(&self, min: f32, max: f32, max_ticks: usize) -> Vec<Tick<f32>, 32> {
        let mut ticks = Vec::new();

        if min <= 0.0 || max <= 0.0 {
            return ticks; // Logarithmic scale requires positive values
        }

        let min_num = min.to_number();
        let max_num = max.to_number();
        let base_num = self.base.to_number();

        let log_min = Math::ln(min_num) / Math::ln(base_num);
        let log_max = Math::ln(max_num) / Math::ln(base_num);

        let start_power = f32::from_number(Math::floor(log_min)) as i32;
        let end_power = f32::from_number(Math::ceil(log_max)) as i32;

        for power in start_power..=end_power {
            if ticks.len() >= max_ticks.min(32) {
                break;
            }

            let power_num = (power as f32).to_number();
            let value_num = Math::pow(base_num, power_num);
            let value = f32::from_number(value_num);
            if value >= min && value <= max {
                // Simple no_std formatting
                let mut label = heapless::String::new();
                if value >= 1000.0 {
                    let k_val = (value / 1000.0) as i32;
                    // Simple integer to string conversion
                    let mut val = k_val;
                    let mut digits = heapless::Vec::<u8, 8>::new();
                    if val == 0 {
                        let _ = digits.push(b'0');
                    } else {
                        while val > 0 {
                            let _ = digits.push((val % 10) as u8 + b'0');
                            val /= 10;
                        }
                    }
                    for &digit in digits.iter().rev() {
                        let _ = label.push(digit as char);
                    }
                    let _ = label.push('k');
                } else if value >= 1.0 {
                    let int_val = value as i32;
                    let mut val = int_val;
                    let mut digits = heapless::Vec::<u8, 8>::new();
                    if val == 0 {
                        let _ = digits.push(b'0');
                    } else {
                        while val > 0 {
                            let _ = digits.push((val % 10) as u8 + b'0');
                            val /= 10;
                        }
                    }
                    for &digit in digits.iter().rev() {
                        let _ = label.push(digit as char);
                    }
                } else {
                    // For small values, just show "0.x"
                    let _ = label.push_str("0.1");
                }

                let _ = ticks.push(Tick {
                    value,
                    is_major: true,
                    label: Some(label),
                });
            }
        }

        ticks
    }

    fn preferred_tick_count(&self) -> usize {
        5
    }

    fn set_preferred_tick_count(&mut self, _count: usize) {
        // Log tick generator doesn't use preferred count in the same way
    }
}

impl Default for LogTickGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(not(feature = "integer-math"))] // Skip for integer-math to avoid overflow
    fn test_linear_tick_generator() {
        let generator = LinearTickGenerator::new(5);
        let ticks = generator.generate_ticks(0.0f32, 10.0f32, 10);

        assert!(!ticks.is_empty());
        assert!(ticks.len() <= 10);

        // Check that ticks are in ascending order
        for window in ticks.windows(2) {
            if let [tick1, tick2] = window {
                assert!(tick1.value <= tick2.value);
            }
        }
    }

    #[test]
    #[cfg(not(any(feature = "fixed-point", feature = "integer-math")))] // Skip for fixed-point and integer-math to avoid overflow
    fn test_linear_tick_generator_with_minor_ticks() {
        let generator = LinearTickGenerator::new(3).with_minor_ticks(2);
        let ticks = generator.generate_ticks(0.0f32, 10.0f32, 20);

        assert!(!ticks.is_empty());

        let major_count = ticks.iter().filter(|t| t.is_major).count();
        let minor_count = ticks.iter().filter(|t| !t.is_major).count();

        assert!(major_count > 0);
        assert!(minor_count > 0);
    }

    #[test]
    fn test_custom_tick_generator() {
        let generator = CustomTickGenerator::new()
            .add_major_tick(0.0, "Start")
            .add_major_tick(5.0, "Middle")
            .add_major_tick(10.0, "End")
            .add_minor_tick(2.5);

        let ticks = generator.generate_ticks(0.0f32, 10.0f32, 10);
        assert_eq!(ticks.len(), 4);

        let major_count = ticks.iter().filter(|t| t.is_major).count();
        assert_eq!(major_count, 3);
    }

    #[test]
    #[cfg(not(any(feature = "fixed-point", feature = "integer-math")))] // Skip for fixed-point and integer-math to avoid overflow
    fn test_log_tick_generator() {
        let generator = LogTickGenerator::new();
        let ticks = generator.generate_ticks(1.0f32, 1000.0f32, 10);

        assert!(!ticks.is_empty());

        // All ticks should be major for log scale
        assert!(ticks.iter().all(|t| t.is_major));

        // Values should be powers of 10
        for tick in &ticks {
            let value_num = tick.value.to_number();
            let log_value = f32::from_number(Math::log10(value_num));
            assert!((log_value.round() - log_value).abs() < 0.001);
        }
    }
}
