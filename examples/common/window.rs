//! Window Management System for Visual Examples
//!
//! This module provides common window setup and event handling for SimulatorDisplay,
//! with configurable display sizes, themes, and automatic frame rate management.

#[cfg(feature = "std")]
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

#[cfg(all(feature = "std", feature = "capture"))]
use super::capture::{self, capture_screenshot, GifCapture};

use embedded_graphics::{pixelcolor::Rgb565, prelude::Size, primitives::Rectangle};

use embedded_charts::prelude::*;

/// Window theme configuration
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)] // All variants should be available as API
pub enum WindowTheme {
    /// Default theme with white background
    Default,
    /// Dark theme with black background
    Dark,
    /// OLED blue theme
    OledBlue,
    /// OLED white theme
    OledWhite,
    /// Custom theme
    Custom { pixel_spacing: u32, scale: u32 },
}

#[allow(dead_code)] // Utility methods for theme system
impl WindowTheme {
    #[cfg(feature = "std")]
    fn to_binary_color_theme(self) -> BinaryColorTheme {
        match self {
            WindowTheme::Default => BinaryColorTheme::Default,
            WindowTheme::Dark => BinaryColorTheme::OledBlue,
            WindowTheme::OledBlue => BinaryColorTheme::OledBlue,
            WindowTheme::OledWhite => BinaryColorTheme::OledWhite,
            WindowTheme::Custom { .. } => BinaryColorTheme::Default,
        }
    }

    pub fn pixel_spacing(self) -> u32 {
        match self {
            WindowTheme::Custom { pixel_spacing, .. } => pixel_spacing,
            _ => 0, // No spacing between pixels for solid appearance
        }
    }

    pub fn scale(self) -> u32 {
        match self {
            WindowTheme::Custom { scale, .. } => scale,
            WindowTheme::Default | WindowTheme::Dark => 1,
            WindowTheme::OledBlue | WindowTheme::OledWhite => 2,
        }
    }
}

/// Window configuration - merged with DisplayConfig functionality
#[derive(Debug, Clone)]
#[allow(dead_code)] // Utility struct for examples - not all fields used in every example
pub struct WindowConfig {
    pub title: &'static str,
    pub theme: WindowTheme,
    pub target_fps: u32,
    pub auto_close: bool,
    pub background_color: Rgb565,
    pub size: Size,
    #[cfg(feature = "capture")]
    pub capture_screenshot: Option<std::string::String>,
    #[cfg(feature = "capture")]
    pub capture_gif: Option<(std::string::String, u16)>, // (filename, delay_ms)
}

#[cfg(feature = "capture")]
impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Chart Example",
            theme: WindowTheme::Default,
            target_fps: 60,
            auto_close: false,
            background_color: Rgb565::WHITE,
            size: Size::new(640, 480),
            capture_screenshot: None,
            capture_gif: None,
        }
    }
}

#[cfg(not(feature = "capture"))]
impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Chart Example",
            theme: WindowTheme::Default,
            target_fps: 60,
            auto_close: false,
            background_color: Rgb565::WHITE,
            size: Size::new(640, 480),
        }
    }
}

#[allow(dead_code)] // Utility methods for examples
impl WindowConfig {
    /// Create a new window configuration
    pub fn new(title: &'static str) -> Self {
        Self {
            title,
            ..Default::default()
        }
    }

    /// Set the window theme
    pub fn theme(mut self, theme: WindowTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Set the target frame rate
    pub fn fps(mut self, fps: u32) -> Self {
        self.target_fps = fps;
        self
    }

    /// Enable auto-close after a delay (useful for automated testing)
    pub fn auto_close(mut self) -> Self {
        self.auto_close = true;
        self
    }

    /// Set the background color
    pub fn background(mut self, color: Rgb565) -> Self {
        self.background_color = color;
        self
    }

    /// Set the window/display size
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Get the viewport for this configuration
    pub fn viewport(&self) -> Rectangle {
        Rectangle::new(
            Point::new(20, 20),
            Size::new(self.size.width - 40, self.size.height - 40),
        )
    }

    /// Get frame delay in milliseconds
    pub fn frame_delay_ms(&self) -> u64 {
        1000 / self.target_fps as u64
    }
}

/// Window manager for handling visual examples
#[cfg(feature = "std")]
#[allow(dead_code)] // Utility struct for window management
pub struct WindowManager {
    window: Window,
    pub config: WindowConfig,
    start_time: std::time::Instant,
    #[cfg(feature = "capture")]
    gif_capture: Option<GifCapture>,
}

#[cfg(all(feature = "std", feature = "capture"))]
#[allow(dead_code)]
impl WindowManager {
    /// Create a new window manager
    pub fn new(window_config: &WindowConfig) -> Self {
        let title = window_config.title;

        let output_settings = OutputSettingsBuilder::new()
            .theme(window_config.theme.to_binary_color_theme())
            .pixel_spacing(window_config.theme.pixel_spacing())
            .scale(window_config.theme.scale())
            .build();

        let window = Window::new(title, &output_settings);

        Self {
            window,
            config: window_config.clone(),
            start_time: std::time::Instant::now(),
            gif_capture: None,
        }
    }
}

#[cfg(all(feature = "std", not(feature = "capture")))]
#[allow(dead_code)]
impl WindowManager {
    /// Create a new window manager
    pub fn new(window_config: &WindowConfig) -> Self {
        let title = window_config.title;

        let output_settings = OutputSettingsBuilder::new()
            .theme(window_config.theme.to_binary_color_theme())
            .pixel_spacing(window_config.theme.pixel_spacing())
            .scale(window_config.theme.scale())
            .build();

        let window = Window::new(title, &output_settings);

        Self {
            window,
            config: window_config.clone(),
            start_time: std::time::Instant::now(),
        }
    }
}

#[cfg(feature = "std")]
#[allow(dead_code)]
impl WindowManager {
    /// Update the window with the current display content
    pub fn update(&mut self, display: &SimulatorDisplay<Rgb565>) {
        self.window.update(display);
    }

    /// Check if the window should close
    pub fn should_close(&mut self) -> bool {
        // Check for quit events
        if self.window.events().any(|e| e == SimulatorEvent::Quit) {
            return true;
        }

        // Check auto-close timeout (5 seconds for demos)
        if self.config.auto_close && self.start_time.elapsed().as_secs() > 5 {
            return true;
        }

        false
    }
}

/// Capture functionality for WindowManager
#[cfg(all(feature = "std", feature = "capture"))]
#[allow(dead_code)]
impl WindowManager {
    /// Capture a screenshot of the current display
    pub fn capture_screenshot<P: AsRef<std::path::Path>>(
        &self,
        display: &SimulatorDisplay<Rgb565>,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        capture_screenshot(display, path)
    }

    /// Start GIF capture
    pub fn start_gif_capture(&mut self, delay_ms: u16) {
        self.gif_capture = Some(GifCapture::new(delay_ms));
    }

    /// Add a frame to the GIF capture
    pub fn add_gif_frame(&mut self, display: &SimulatorDisplay<Rgb565>) {
        if let Some(ref mut gif_capture) = self.gif_capture {
            gif_capture.add_frame(display);
        }
    }

    /// Save the captured GIF
    pub fn save_gif<P: AsRef<std::path::Path>>(
        &mut self,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(gif_capture) = self.gif_capture.take() {
            gif_capture.save_gif(path)?;
        }
        Ok(())
    }
}

/// Convenience function to run an animated visual example
#[cfg(feature = "std")]
#[allow(dead_code)] // Main window runner for examples
pub fn run<F>(window_config: WindowConfig, mut animation_fn: F) -> ChartResult<()>
where
    F: FnMut(&mut SimulatorDisplay<Rgb565>, Rectangle, f32) -> ChartResult<()>, // time in seconds, returns continue
{
    // Create display using config from WindowConfig
    let mut display = SimulatorDisplay::new(window_config.size);
    let viewport = window_config.viewport();
    let background_color = window_config.background_color;

    // Create window manager
    let mut window_manager = WindowManager::new(&window_config);
    let start_time = std::time::Instant::now();

    #[cfg(feature = "capture")]
    let mut captured_screenshot = false;
    #[cfg(feature = "capture")]
    let mut gif_capture_started = false;

    // Run animation loop
    loop {
        // Update display
        window_manager.update(&display);

        // Check for exit conditions
        if window_manager.should_close() {
            #[cfg(feature = "capture")]
            if gif_capture_started {
                std::fs::create_dir_all("assets").ok();
                let gif_filename = format!(
                    "assets/{}.gif",
                    window_manager.config.title.replace(" ", "_").to_lowercase()
                );
                if window_manager.save_gif(&gif_filename).is_ok() {
                    println!("✅ Animation saved to {gif_filename}");
                }
            }
            break;
        }

        // Clear display
        display
            .clear(background_color)
            .map_err(|_| ChartError::RenderingError)?;

        // Calculate elapsed time
        let elapsed = start_time.elapsed().as_secs_f32();

        // Call animation function
        animation_fn(&mut display, viewport, elapsed)?;

        // Auto-capture functionality
        #[cfg(feature = "capture")]
        {
            // Detect if this is an animated example
            let is_animated = window_manager
                .config
                .title
                .to_lowercase()
                .contains("animation")
                || window_manager
                    .config
                    .title
                    .to_lowercase()
                    .contains("animated")
                || window_manager
                    .config
                    .title
                    .to_lowercase()
                    .contains("streaming")
                || window_manager
                    .config
                    .title
                    .to_lowercase()
                    .contains("real-time")
                || window_manager
                    .config
                    .title
                    .to_lowercase()
                    .contains("dashboard")
                || window_manager.config.title.to_lowercase().contains("demo");

            // Start GIF capture for animated examples
            if !gif_capture_started && elapsed > 0.5 && is_animated {
                window_manager.start_gif_capture(100); // 100ms delay between frames
                gif_capture_started = true;
                println!("🎬 Starting GIF capture for animated example...");
            }

            // Add frames to GIF for animated examples
            if gif_capture_started {
                window_manager.add_gif_frame(&display);
            }

            // For animated examples, capture GIF instead of PNG
            if !captured_screenshot && elapsed > 1.0 && !is_animated {
                // Only capture PNG for non-animated examples
                std::fs::create_dir_all("assets").ok();
                let filename = format!(
                    "assets/{}.png",
                    window_manager.config.title.replace(" ", "_").to_lowercase()
                );
                if window_manager.capture_screenshot(&display, &filename).is_ok() {
                    println!("✅ Screenshot saved to {filename}");
                    captured_screenshot = true;
                }
            }
        }

        // Frame rate limiting
        std::thread::sleep(std::time::Duration::from_millis(
            window_manager.config.frame_delay_ms(),
        ));
    }

    Ok(())
}

/// Convenience function to run a static visual example (single frame)
#[cfg(feature = "std")]
#[allow(dead_code)] // Alternative static runner for examples
pub fn run_static<F>(window_config: WindowConfig, mut render_fn: F) -> ChartResult<()>
where
    F: FnMut(&mut SimulatorDisplay<Rgb565>, Rectangle) -> ChartResult<()>,
{
    // Create display using config from WindowConfig
    let mut display = SimulatorDisplay::new(window_config.size);
    let viewport = window_config.viewport();
    let background_color = window_config.background_color;

    // Clear display
    display
        .clear(background_color)
        .map_err(|_| ChartError::RenderingError)?;

    // Call render function
    render_fn(&mut display, viewport)?;

    // Auto-capture functionality
    #[cfg(feature = "capture")]
    {
        std::fs::create_dir_all("assets").ok();
        let filename = format!(
            "assets/{}.png",
            window_config.title.replace(" ", "_").to_lowercase()
        );
        if capture::capture_screenshot(&display, &filename).is_ok() {
            println!("✅ Screenshot saved to {filename}");
        }
    }

    // Create window manager for display
    let mut window_manager = WindowManager::new(&window_config);

    // Simple display loop
    loop {
        // Update display
        window_manager.update(&display);

        // Check for exit conditions
        if window_manager.should_close() {
            break;
        }

        // Frame rate limiting
        std::thread::sleep(std::time::Duration::from_millis(
            window_manager.config.frame_delay_ms(),
        ));
    }

    Ok(())
}

/// Stub implementations for no_std environments
#[cfg(not(feature = "std"))]
pub struct WindowManager;

#[cfg(not(feature = "std"))]
#[allow(dead_code)]
impl WindowManager {
    pub fn new(_window_config: &WindowConfig) -> Self {
        Self
    }
}

#[cfg(not(feature = "std"))]
pub fn run<F>(_window_config: WindowConfig, _animation_fn: F) -> ChartResult<()>
where
    F: FnMut(
        &mut embedded_graphics::mock_display::MockDisplay<Rgb565>,
        Rectangle,
        f32,
    ) -> ChartResult<()>,
{
    println!("Animated examples require the 'std' feature to run with the simulator");
    Ok(())
}

/// Preset window configurations
#[allow(dead_code)] // Preset utility functions for examples
pub mod presets {
    use super::*;

    /// Default window configuration
    pub fn default(title: &'static str) -> WindowConfig {
        WindowConfig::new(title)
    }

    /// Dark theme window configuration
    pub fn dark_theme(title: &'static str) -> WindowConfig {
        WindowConfig::new(title).theme(WindowTheme::Dark)
    }

    /// OLED theme window configuration
    pub fn oled_theme(title: &'static str) -> WindowConfig {
        WindowConfig::new(title).theme(WindowTheme::OledBlue)
    }

    /// High performance window configuration (30 FPS)
    pub fn performance(title: &'static str) -> WindowConfig {
        WindowConfig::new(title).fps(30)
    }

    /// Demo window configuration (auto-closes after 5 seconds)
    pub fn demo(title: &'static str) -> WindowConfig {
        WindowConfig::new(title).auto_close()
    }

    /// Custom scaled window configuration
    pub fn scaled(title: &'static str, scale: u32) -> WindowConfig {
        WindowConfig::new(title).theme(WindowTheme::Custom {
            pixel_spacing: 1,
            scale,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_config() {
        let config = WindowConfig::new("Test Window")
            .theme(WindowTheme::Dark)
            .fps(30)
            .background(Rgb565::BLACK);

        assert_eq!(config.title, "Test Window");
        assert_eq!(config.target_fps, 30);
        assert_eq!(config.background_color, Rgb565::BLACK);
        assert_eq!(config.frame_delay_ms(), 33); // 1000/30 ≈ 33ms
    }

    #[test]
    fn test_window_theme() {
        let theme = WindowTheme::Custom {
            pixel_spacing: 2,
            scale: 3,
        };

        assert_eq!(theme.pixel_spacing(), 2);
        assert_eq!(theme.scale(), 3);
    }

    #[test]
    fn test_preset_configs() {
        let config = presets::dark_theme("Dark Chart");
        assert_eq!(config.title, "Dark Chart");

        let demo_config = presets::demo("Demo Chart");
        assert!(demo_config.auto_close);
    }
}
