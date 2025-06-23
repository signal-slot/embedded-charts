#!/bin/bash

# Generate Visual Assets for Documentation
# This script runs all examples with capture enabled to generate screenshots

set -e

echo "🎨 Generating visual assets for embedded-graphics-graph documentation..."

# Create docs assets directory
mkdir -p docs/assets

# Generate theme showcase
echo "📸 Generating theme showcase..."
cargo run --example theme_showcase --features std,capture,line,bar,pie

# Generate basic chart examples
echo "📊 Generating basic chart examples..."
cargo run --example line_chart --features std,capture
cargo run --example bar_chart --features std,capture
cargo run --example pie_chart --features std,capture
cargo run --example donut_chart --features std,capture,pie

# Generate advanced chart examples
echo "🔬 Generating advanced chart examples..."
cargo run --example scatter_chart --features std,capture,scatter
cargo run --example gauge_chart --features std,capture,gauge

# Generate production dashboard
echo "🚀 Generating production dashboard..."
cargo run --example production_ready_demo --features std,capture

# Generate multi-series examples
echo "📈 Generating multi-series examples..."
cargo run --example multi_series_dashboard --features std,capture,line,bar,animations

# Generate real-time examples
echo "⚡ Generating real-time examples..."
cargo run --example real_time_dashboard --features std,capture,line,gauge,animations

# Generate v0.3.0 feature examples
echo "🆕 Generating v0.3.0 feature examples..."
mkdir -p docs/assets/v0.3.0
echo "  - Gradient showcase..."
cargo run --example gradient_showcase --features std,capture,bar,color-support
echo "  - Dashboard layouts..."
cargo run --example dashboard_layouts_showcase --features std,capture
echo "  - Logarithmic scales..."
cargo run --example logarithmic_scale_demo --features std,capture,line,floating-point
echo "  - Data aggregation..."
cargo run --example data_aggregation_demo --features std,capture,line,floating-point

# Generate GIF animations
echo "🎬 Generating GIF animations..."
echo "  - Ring buffer demo..."
CAPTURE_GIF=1 cargo run --example ring_buffer_demo --all-features --release
echo "  - Chart animation demo..."
CAPTURE_GIF=1 cargo run --example chart_animation_demo --features std,capture,line,animations --release

echo "✅ All visual assets generated successfully!"
echo "📁 Assets saved to: docs/assets/"
echo ""
echo "Generated files:"
ls -la docs/assets/
echo ""
echo "v0.3.0 assets:"
ls -la docs/assets/v0.3.0/ 2>/dev/null || echo "No v0.3.0 assets generated yet"