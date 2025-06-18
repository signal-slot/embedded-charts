#!/bin/bash
# Script to capture GIF animations from examples

# Ensure docs/assets directory exists
mkdir -p docs/assets

echo "Capturing GIF animations for README..."

# Ring buffer demo
echo "1. Capturing ring buffer demo..."
CAPTURE_GIF=1 cargo run --example ring_buffer_demo --all-features --release

# Add more examples here as they get GIF capture support
# echo "2. Capturing streaming animation demo..."
# CAPTURE_GIF=1 cargo run --example streaming_animation --all-features --release

echo "GIF capture complete!"
echo "Generated files:"
ls -lh docs/assets/*.gif