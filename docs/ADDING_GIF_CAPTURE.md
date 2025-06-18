# Adding GIF Capture to Examples

This guide explains how to add automatic GIF capture functionality to examples in the embedded-charts library.

## Overview

The library includes a built-in GIF capture utility that can automatically record animations from examples and save them as GIF files. This is useful for:
- Generating README assets
- Creating documentation
- Showcasing real-time features
- Demonstrating animations

## Implementation Steps

### 1. Import the Capture Module

Add these imports to your example:

```rust
// Import capture utilities for GIF generation
#[path = "../common/capture.rs"]
mod capture;
use capture::GifCapture;
```

### 2. Set Up GIF Capture

In your main function, add the GIF capture setup:

```rust
// GIF capture setup
let mut gif_capture = GifCapture::new(50); // 50ms per frame = 20fps
let mut capture_frame_count = 0;
let max_capture_frames = 60; // Adjust based on desired duration
let capture_enabled = std::env::var("CAPTURE_GIF").is_ok();

if capture_enabled {
    println!("GIF capture enabled! Will save to docs/assets/your_example.gif");
}
```

### 3. Capture Frames in Your Render Loop

Inside your main loop, after rendering to the display:

```rust
// Capture frame for GIF if enabled
if capture_enabled && capture_frame_count < max_capture_frames {
    gif_capture.add_frame(&display);
    capture_frame_count += 1;
    println!("Captured frame {}/{}", capture_frame_count, max_capture_frames);
    
    // Save GIF when we have enough frames
    if capture_frame_count >= max_capture_frames {
        println!("Saving GIF...");
        gif_capture.save_gif("docs/assets/your_example.gif")?;
        println!("GIF saved successfully!");
        // Optionally exit after saving
        break 'main_loop;
    }
}
```

## Configuration Options

### Frame Rate

The `GifCapture::new(delay_ms)` parameter controls the frame rate:
- `50` = 20 FPS (smooth, reasonable file size)
- `33` = 30 FPS (very smooth, larger file)
- `100` = 10 FPS (smaller file, less smooth)

### Duration

The `max_capture_frames` controls the GIF duration:
- At 20 FPS: 60 frames = 3 seconds
- At 20 FPS: 100 frames = 5 seconds
- At 20 FPS: 200 frames = 10 seconds

### File Size Considerations

- Keep GIFs under 1MB for README usage
- Use 20 FPS for a good balance
- Limit duration to 3-5 seconds
- Consider resizing the display for smaller files

## Running with GIF Capture

To generate a GIF:

```bash
CAPTURE_GIF=1 cargo run --example your_example --all-features
```

To run normally without capture:

```bash
cargo run --example your_example --all-features
```

## Example: Ring Buffer Demo

See `examples/basic/ring_buffer_demo.rs` for a complete implementation that:
- Captures 60 frames at 20 FPS (3 seconds)
- Shows real-time data streaming
- Automatically exits after saving
- Generates a ~800KB GIF

## Tips

1. **Test First**: Run without capture to ensure your example works correctly
2. **Frame Selection**: Only capture frames when the display is updated
3. **Progress Feedback**: Print capture progress for user feedback
4. **Auto Exit**: Consider exiting after GIF save to automate asset generation
5. **File Naming**: Use descriptive names like `feature_name_demo.gif`

## Automation

Add your example to `scripts/generate_assets.sh`:

```bash
echo "  - Your feature demo..."
CAPTURE_GIF=1 cargo run --example your_example --all-features --release
```

This allows regenerating all assets with:

```bash
./scripts/generate_assets.sh
```