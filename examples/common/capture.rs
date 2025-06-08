//! Screenshot and animation capture utilities for generating README assets

use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};
use std::path::Path;

/// Capture a screenshot from a display and save it as PNG
#[allow(dead_code)] // Utility function for capturing screenshots
pub fn capture_screenshot<P: AsRef<Path>>(
    display: &SimulatorDisplay<Rgb565>,
    path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let output_image = display.to_rgb_output_image(&OutputSettingsBuilder::new().build());

    // Convert to image::RgbImage using the raw data
    let width = output_image.size().width;
    let height = output_image.size().height;
    let mut img_buffer = image::RgbImage::new(width, height);

    // Access the raw pixel data
    let raw_data = output_image.as_image_buffer();
    for (x, y, pixel) in raw_data.enumerate_pixels() {
        let rgb = [pixel.0[0], pixel.0[1], pixel.0[2]];
        img_buffer.put_pixel(x, y, image::Rgb(rgb));
    }

    img_buffer.save(path)?;
    Ok(())
}

/// Capture multiple frames for GIF animation
#[allow(dead_code)] // GIF capture utility
pub struct GifCapture {
    frames: Vec<image::RgbImage>,
    delay: u16,
}

#[allow(dead_code)] // GIF capture methods
impl GifCapture {
    pub fn new(delay_ms: u16) -> Self {
        Self {
            frames: Vec::new(),
            delay: delay_ms / 10, // GIF delay is in 1/100th seconds
        }
    }

    pub fn add_frame(&mut self, display: &SimulatorDisplay<Rgb565>) {
        let output_image = display.to_rgb_output_image(&OutputSettingsBuilder::new().build());

        // Convert to image::RgbImage using the raw data
        let width = output_image.size().width;
        let height = output_image.size().height;
        let mut img_buffer = image::RgbImage::new(width, height);

        // Access the raw pixel data
        let raw_data = output_image.as_image_buffer();
        for (x, y, pixel) in raw_data.enumerate_pixels() {
            let rgb = [pixel.0[0], pixel.0[1], pixel.0[2]];
            img_buffer.put_pixel(x, y, image::Rgb(rgb));
        }

        self.frames.push(img_buffer);
    }

    pub fn save_gif<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        use image::codecs::gif::{GifEncoder, Repeat};
        use image::{Delay, DynamicImage, Frame};
        use std::fs::File;

        let file = File::create(path)?;
        let mut encoder = GifEncoder::new(file);
        encoder.set_repeat(Repeat::Infinite)?;

        for frame in &self.frames {
            let delay = Delay::from_numer_denom_ms(self.delay as u32 * 10, 1);
            let dynamic_image = DynamicImage::ImageRgb8(frame.clone());
            let rgba_image = dynamic_image.to_rgba8();
            let gif_frame = Frame::from_parts(rgba_image, 0, 0, delay);
            encoder.encode_frame(gif_frame)?;
        }

        Ok(())
    }
}

/// Standard dimensions for README assets
#[allow(dead_code)] // Standard capture dimensions
pub const HERO_SIZE: (u32, u32) = (800, 600);
#[allow(dead_code)]
pub const FEATURE_SIZE: (u32, u32) = (400, 300);
#[allow(dead_code)]
pub const THUMBNAIL_SIZE: (u32, u32) = (200, 150);
