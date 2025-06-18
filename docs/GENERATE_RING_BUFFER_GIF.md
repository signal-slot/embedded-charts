# Generating Ring Buffer Demo GIF

To generate the ring buffer demo GIF for the README:

## Method 1: Using Screen Recording

1. Run the ring buffer demo:
   ```bash
   cargo run --example ring_buffer_demo --all-features
   ```

2. Use a screen recording tool to capture the window:
   - **Linux**: Use `peek`, `simplescreenrecorder`, or `obs-studio`
   - **macOS**: Use QuickTime Player or `gifcapture`
   - **Windows**: Use ScreenToGif or ShareX

3. Record for about 10-15 seconds to show:
   - Initial buffer filling
   - Buffer wrap-around behavior
   - Real-time statistics updates
   - Moving average calculation
   - Buffer visualization at the bottom

4. Save as `docs/assets/ring_buffer_demo.gif`

## Method 2: Using ffmpeg (Linux/macOS)

```bash
# Start the demo
cargo run --example ring_buffer_demo --all-features &
DEMO_PID=$!

# Wait for window to appear
sleep 2

# Get window ID (Linux with X11)
WINDOW_ID=$(xwininfo -name "Ring Buffer Real-Time Demo" | grep "Window id" | awk '{print $4}')

# Record using ffmpeg
ffmpeg -f x11grab -r 30 -video_size 800x480 -i $WINDOW_ID -t 15 \
    -vf "fps=15,scale=600:-1:flags=lanczos" \
    -c:v gif docs/assets/ring_buffer_demo.gif

# Stop the demo
kill $DEMO_PID
```

## Method 3: Using asciinema + svg-term (Terminal Alternative)

If you want to create a terminal-based visualization instead:

```bash
# Install tools
npm install -g svg-term-cli
pip install asciinema

# Record terminal session
asciinema rec ring_buffer_demo.cast

# Run the demo
cargo run --example ring_buffer_demo --all-features

# Stop recording (Ctrl+D)

# Convert to GIF
svg-term --in ring_buffer_demo.cast --out ring_buffer_demo.svg --window
# Then convert SVG to GIF using online tools or imagemagick
```

## Optimal GIF Settings

- **Resolution**: 600x400 pixels (to fit well in README)
- **Frame Rate**: 15-30 fps
- **Duration**: 10-15 seconds
- **File Size**: Try to keep under 1MB
- **Content to Show**:
  - Initial data streaming
  - Buffer filling up to capacity
  - Wrap-around behavior (oldest data being replaced)
  - Statistics panel updates
  - Moving average line
  - Buffer fill visualization

## Post-Processing

To optimize the GIF size:

```bash
# Using gifsicle
gifsicle -O3 --colors 256 --lossy=30 -o optimized.gif ring_buffer_demo.gif

# Using imagemagick
convert ring_buffer_demo.gif -coalesce -layers Optimize -colors 128 optimized.gif
```