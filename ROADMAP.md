# embedded-charts Roadmap

## Overview

This document outlines the development roadmap for embedded-charts, providing a high-level view of planned releases and their major features. For detailed planning, see the specific release plan documents in the `docs/` directory.

## Release Timeline

### ✅ v0.1.0 (Released: June 2, 2025)
**Theme:** "Foundation Release"
- Complete chart type implementations (line, bar, pie, scatter, gauge, stacked)
- Advanced rendering system with axes, grids, and legends
- Animation framework with streaming support
- Full no_std compatibility
- 24 working examples

### ✅ v0.2.0 (Released: June 15, 2025)
**Theme:** "Smooth Curves & Quality"
- Smooth curve interpolation system (Linear, Cubic Spline, Catmull-Rom, Bezier)
- Enhanced testing infrastructure (Phase 1.1 complete)
- Improved no_std support and memory management
- Fixed all major CI/CD issues
- Test coverage baseline: 44.05%

### 🎯 v0.3.0 (Target: December 2025)
**Theme:** "Production Maturity & Performance Excellence"
- Test coverage improvement: 44% → 65%
- Performance optimization: 20-30% faster rendering
- Memory optimization: 15-25% smaller footprint
- Interactive framework foundation
- Enhanced animations (physics-based)
- New charts: Heatmap, Box Plot
- Developer tools suite

[Detailed plan: docs/RELEASE_PLAN_v0.3.0.md](docs/RELEASE_PLAN_v0.3.0.md)

### 🔮 v0.4.0 (Target: Q2 2026)
**Theme:** "Advanced Interactivity"
- Full interactive capabilities (zoom, pan, gestures)
- 3D visualization prototypes
- Hardware acceleration support
- RTOS integration packages
- Advanced chart types (candlestick, contour)
- Accessibility features

### 🔮 v0.5.0 (Target: Q4 2026)
**Theme:** "Ecosystem Expansion"
- Cloud connectivity features
- Machine learning integration
- Industry-specific templates
- Formal verification support
- Platform SDK packages
- Edge computing capabilities

### 🔮 v1.0.0 (Target: 2027)
**Theme:** "Production Certification"
- Safety-critical certification ready
- Complete API stability
- Long-term support commitment
- Enterprise features
- Comprehensive tooling
- Full ecosystem maturity

## Development Principles

1. **Embedded-First**: Every feature must work efficiently on resource-constrained systems
2. **Zero Allocation**: Maintain no_std compatibility and static memory allocation
3. **Performance**: Optimize for speed and memory usage on embedded targets
4. **Quality**: Comprehensive testing and documentation for production use
5. **Compatibility**: Support wide range of embedded platforms and displays

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Versioning

This project follows [Semantic Versioning](https://semver.org/):
- MAJOR version for incompatible API changes
- MINOR version for backwards-compatible functionality additions
- PATCH version for backwards-compatible bug fixes

## Status Updates

For the latest development status:
- Check [GitHub Issues](https://github.com/signal-slot/embedded-charts/issues)
- Review [Pull Requests](https://github.com/signal-slot/embedded-charts/pulls)
- See [Milestones](https://github.com/signal-slot/embedded-charts/milestones)

---
*Last updated: June 15, 2025*