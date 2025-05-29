# Contributing to embedded-charts

Thank you for your interest in contributing to embedded-charts! This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Prerequisites

- Rust 1.70.0 or later
- Git
- Basic understanding of embedded systems and graphics programming

### Development Setup

1. **Clone the repository:**
   ```bash
   git clone https://github.com/signal-slot/embedded-charts.git
   cd embedded-charts
   ```

2. **Install dependencies:**
   ```bash
   cargo build
   ```

3. **Run tests:**
   ```bash
   cargo test
   ```

4. **Run examples:**
   ```bash
   cargo run --example line_chart
   cargo run --example bar_chart
   ```

## 📋 Development Guidelines

### Code Quality Standards

- **Formatting:** Use `cargo fmt` before committing
- **Linting:** Ensure `cargo clippy -- -D warnings` passes
- **Testing:** Add tests for new functionality
- **Documentation:** Document all public APIs with examples

### Code Style

- Follow Rust naming conventions
- Use meaningful variable and function names
- Keep functions focused and small
- Prefer explicit error handling over panics
- Use `#[must_use]` for important return values

### Feature Development

- All new features should be behind feature flags when appropriate
- Maintain `no_std` compatibility
- Consider memory usage and performance implications
- Add comprehensive tests and documentation

## 🔧 Project Structure

```
embedded-charts/
├── src/
│   ├── chart/          # Chart implementations
│   ├── data/           # Data structures and series
│   ├── style/          # Styling and themes
│   ├── axes/           # Axis system
│   ├── grid/           # Grid system
│   ├── legend/         # Legend system
│   ├── render/         # Rendering primitives
│   ├── animation/      # Animation system
│   ├── math/           # Math abstraction layer
│   └── prelude.rs      # Convenient re-exports
├── examples/           # Example applications
├── tests/              # Integration tests
└── docs/               # Documentation
```

## 🎯 Areas for Contribution

### High Priority
- Bug fixes and performance improvements
- Documentation improvements
- Additional chart types
- Better error messages
- Platform-specific optimizations

### Medium Priority
- New styling options
- Additional animation effects
- More comprehensive examples
- Accessibility features

### Low Priority
- Code cleanup and refactoring
- Additional utility functions
- Performance benchmarks

## 📝 Pull Request Process

1. **Fork the repository** and create a feature branch
2. **Make your changes** following the coding guidelines
3. **Add tests** for new functionality
4. **Update documentation** as needed
5. **Run the full test suite:**
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   cargo doc --no-deps
   ```
6. **Submit a pull request** with a clear description

### Pull Request Guidelines

- **Title:** Use a clear, descriptive title
- **Description:** Explain what changes you made and why
- **Testing:** Describe how you tested your changes
- **Breaking Changes:** Clearly mark any breaking changes
- **Documentation:** Update relevant documentation

## 🐛 Bug Reports

When reporting bugs, please include:

- **Environment:** Rust version, target platform, feature flags
- **Steps to reproduce:** Clear, minimal reproduction steps
- **Expected behavior:** What you expected to happen
- **Actual behavior:** What actually happened
- **Code sample:** Minimal code that demonstrates the issue

## 💡 Feature Requests

For feature requests, please provide:

- **Use case:** Why is this feature needed?
- **Proposed solution:** How should it work?
- **Alternatives:** What alternatives have you considered?
- **Implementation:** Any thoughts on implementation approach?

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for specific features
cargo test --features "line,bar,pie"

# Run tests without default features
cargo test --no-default-features

# Run integration tests
cargo test --test integration_tests
```

### Writing Tests

- Add unit tests for new functions and methods
- Add integration tests for new chart types
- Test edge cases and error conditions
- Ensure tests work in `no_std` environments

## 📚 Documentation

### API Documentation

- Document all public APIs with rustdoc
- Include code examples in documentation
- Explain parameters and return values
- Document error conditions

### Examples

- Create focused examples for new features
- Ensure examples compile and run correctly
- Include comments explaining key concepts
- Test examples as part of CI

## 🔄 Release Process

Releases follow semantic versioning:

- **Patch (0.1.x):** Bug fixes, documentation updates
- **Minor (0.x.0):** New features, non-breaking changes
- **Major (x.0.0):** Breaking changes

See [RELEASE_CHECKLIST.md](docs/RELEASE_CHECKLIST.md) for the complete release process.

## 📄 License

By contributing to embedded-charts, you agree that your contributions will be licensed under the same terms as the project (MIT OR Apache-2.0).

## 🤝 Code of Conduct

We are committed to providing a welcoming and inclusive environment for all contributors. Please be respectful and professional in all interactions.

## 📞 Getting Help

- **Issues:** Open an issue for bugs or feature requests
- **Discussions:** Use GitHub Discussions for questions
- **Email:** Contact the maintainers directly for sensitive issues

## 🙏 Recognition

All contributors will be recognized in the project's acknowledgments. Significant contributions may be highlighted in release notes.

Thank you for contributing to embedded-charts!