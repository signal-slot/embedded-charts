# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Interactive capabilities (mouse/touch interaction, zoom/pan, data point selection)
- Advanced animation features (timeline management, chart morphing, physics-based animations)
- Performance optimization tools (advanced profiling, rendering optimization)

### Changed
- TBD

### Deprecated
- TBD

### Removed
- TBD

### Fixed
- TBD

### Security
- TBD

## [0.3.0] - 2025-06-23

### Added
- **Gradient Fills & Advanced Styling** - Linear/radial gradients, pattern fills (lines, dots, checkerboard, cross-hatch), multi-stop gradients with optimized rendering
- **Chart Animations & Transitions** - Smooth transitions with easing functions, real-time streaming animations, multi-state animations, time-based progress tracking
- **Advanced Axis Scales** - Logarithmic scales for wide data ranges, custom scale functions, automatic tick generation, scale transformations with inverse mapping
- **Dashboard Layouts** - Grid-based layouts for multi-chart displays, flexible positioning with span support, pre-built layout presets
- **Data Aggregation & Downsampling** - LTTB algorithm for visually lossless downsampling, statistical aggregations (mean, min/max, median), memory-efficient processing
- **Test Coverage Improvements** - Increased coverage from 44% to 65%+ across critical components

### Changed
- Improved memory efficiency for data series operations
- Enhanced rendering pipeline for better performance with gradients
- Updated window system for proper frame synchronization

### Fixed
- Fixed RGB565 color calculations in theme showcase
- Corrected viewport coordinate calculations for proper rendering
- Fixed window update timing to ensure content displays correctly
- Resolved issues with no_std builds and gradient rendering

## [0.2.0] - 2025-06-15

### Added
- **Smooth Curve Interpolation System** - Comprehensive smooth curve rendering with multiple algorithms:
  - Linear, Cubic Spline, Catmull-Rom, and Bezier interpolation methods
  - Memory-bounded operations with configurable subdivision counts
  - Full integration with CurveChart for professional data visualization
- **Enhanced Testing Infrastructure**:
  - Comprehensive test suites for CurveChart with all interpolation algorithms
  - Comprehensive data series testing framework (Phase 1.3)
  - Significantly improved test coverage infrastructure and organization
- **Improved no_std Support**:
  - Enhanced heapless support for embedded environments
  - Better memory management for resource-constrained systems
  - Fixed no_std floating-point math operations
- **Enhanced Donut Chart Support**:
  - Comprehensive improvements to donut chart rendering
  - Better layout and visual quality
  - Improved CI coverage for donut charts

### Changed
- Updated MSRV to 1.83.0 for Cargo.lock v4 support
- Improved CI/CD pipeline with real-world feature combination scenarios
- Changed screenshot save path from assets/ to docs/assets/ for better organization
- Enhanced documentation with comprehensive examples and showcases

### Fixed
- Resolved all 52 failing documentation tests
- Fixed all clippy warnings (reduced from 19 to 0)
- Resolved rustfmt formatting issues across the entire codebase
- Fixed CI failures for various feature combinations:
  - std,pie,donut combination
  - std,color-support combination
  - no_std floating-point operations
  - Advanced-charts feature dependencies
- Fixed doctest return types for proper error handling
- Resolved benchmark CI failures
- Fixed no-std target build failures
- **Examples cleanup**: Comprehensive cleanup of all examples
  - Resolved all compilation errors across all examples
  - Removed unused imports and dead code warnings
  - Streamlined common utilities module (633 → ~330 lines)
  - Standardized error handling patterns for no_std cases
  - Fixed missing function exports and import issues
  - Verified functionality across all chart types
  - Maintained backward compatibility while improving maintainability

## [0.1.0] - 2025-06-02

### Added
- **Complete Chart Types**
  - Line charts with multi-series support, markers (circle, square, triangle, diamond, cross, x, star), area filling, and smooth curves
  - Bar charts with vertical/horizontal orientation, spacing control, custom colors, and stacked support
  - Pie charts with full circles, donut charts, labels, and professional styling
  - Scatter charts with multiple point shapes, size mapping for bubble charts, color mapping, collision detection, and connection lines
  - Gauge charts with multiple types (semicircle, three-quarter, full circle), animated needles, threshold zones, and value display
  - Stacked charts (both bar and line variants) with animated transitions and multi-layer data management

- **Advanced Rendering System**
  - Linear axes system with customizable ticks, labels, and formatting
  - Grid system with major/minor grids, tick-aligned grids, and custom spacing
  - Legend system with multiple orientations (horizontal/vertical) and positioning (TopLeft, TopRight, BottomLeft, BottomRight, Custom)
  - Triangle filling algorithms for efficient area rendering
  - Professional color palettes and themes optimized for different display types

- **Animation Framework**
  - Core animation system with state management, controllers, and easing functions (Linear, EaseIn, EaseOut, EaseInOut)
  - Data transition system with smooth interpolation for value changes
  - Streaming animation with unified streaming buffer, automatic memory management, and real-time updates
  - Multi-state animation with keyframe-based system and configurable easing
  - Time-based progress control with external timeline management

- **Memory Management**
  - Static allocation patterns with compile-time memory bounds
  - Heapless collections for no_std compatibility
  - Configurable capacity management with sliding window buffers
  - Memory-efficient streaming data handling

- **System Target Support**
  - Memory-constrained system support (<64KB RAM) with integer-only math
  - Standard system support (128-512KB RAM) with fixed-point arithmetic
  - Full-featured system support (>512KB RAM) with full floating-point math and animations
  - Multiple math backends (integer, fixed-point, floating-point, CORDIC, libm)

- **Comprehensive Examples**
  - 24 working examples demonstrating all chart types and features
  - Basic examples: line_chart, bar_chart, pie_chart, scatter_chart, gauge_chart, stacked charts
  - Animation examples: streaming_animation_demo, data_transition_demo
  - Interactive examples: multi_series_chart, real_time_dashboard, unified_streaming_demo
  - Production examples: temperature_monitor, production_ready_demo, theme_showcase

- **Development Infrastructure**
  - Comprehensive integration tests for all chart types and animation systems
  - Performance tests for large datasets (10,000+ points)
  - Memory usage validation and monitoring
  - CI/CD workflows with automated testing, benchmarking, and security scanning
  - Code quality tools (clippy, fmt, deny) with automated enforcement

### Changed
- Project name from "embedded-graphics-graph" to "embedded-charts" for better clarity
- API design to use consistent builder patterns across all chart types
- Memory allocation strategy to prioritize static allocation for embedded compatibility

### Fixed
- Clippy warnings reduced from 57 to 22 through comprehensive code quality improvements
- CORDIC API compatibility issues with embedded math backends
- Animation interpolation type mismatches in streaming data transitions
- Memory bounds validation in sliding window buffers
- Triangle filling algorithm edge cases for complex polygon shapes

### Security
- Added comprehensive security scanning with cargo-audit and cargo-deny
- Implemented dependency vulnerability monitoring through GitHub Dependabot
- Added security-focused CI workflows for continuous vulnerability assessment

[Unreleased]: https://github.com/signal-slot/embedded-charts/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/signal-slot/embedded-charts/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/signal-slot/embedded-charts/releases/tag/v0.1.0