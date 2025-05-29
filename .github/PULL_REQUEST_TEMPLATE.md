# Pull Request

## Description

<!-- Provide a brief description of the changes in this PR -->

## Type of Change

<!-- Mark the relevant option with an "x" -->

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Code refactoring
- [ ] Test improvements
- [ ] CI/CD improvements

## Chart Types Affected

<!-- Mark all that apply -->

- [ ] Line Chart
- [ ] Bar Chart
- [ ] Pie Chart
- [ ] Scatter Chart
- [ ] Gauge Chart
- [ ] Stacked Charts
- [ ] Custom Charts
- [ ] Not chart-specific

## Target Environment

<!-- Mark all that apply -->

- [ ] std (desktop/server)
- [ ] no_std (embedded)
- [ ] small-mcu (<64KB RAM)
- [ ] medium-mcu (128-512KB RAM)
- [ ] large-mcu (>512KB RAM)

## Testing

<!-- Describe the tests you ran to verify your changes -->

- [ ] All existing tests pass
- [ ] Added new tests for the changes
- [ ] Tested with different feature combinations
- [ ] Tested on target hardware (if applicable)
- [ ] Examples still compile and run correctly

### Test Commands Run

```bash
# List the commands you used to test your changes
cargo test
cargo test --all-features
cargo build --examples
# Add any specific test commands here
```

## Performance Impact

<!-- Describe any performance implications -->

- [ ] No performance impact
- [ ] Performance improvement
- [ ] Minor performance regression (justified)
- [ ] Significant performance change (please explain)

**Performance notes:**
<!-- Add details about performance changes if applicable -->

## Memory Impact

<!-- Describe any memory usage implications -->

- [ ] No memory impact
- [ ] Reduced memory usage
- [ ] Increased memory usage (please justify)

**Memory notes:**
<!-- Add details about memory changes if applicable -->

## Breaking Changes

<!-- If this is a breaking change, describe what breaks and how to migrate -->

- [ ] This PR introduces breaking changes

**Migration guide:**
<!-- Provide migration instructions if there are breaking changes -->

## Documentation

- [ ] Updated inline documentation (doc comments)
- [ ] Updated README.md (if applicable)
- [ ] Updated examples (if applicable)
- [ ] Updated CHANGELOG.md (if applicable)

## Code Quality

- [ ] Code follows the project's style guidelines
- [ ] Self-review of the code has been performed
- [ ] Code is properly commented, particularly in hard-to-understand areas
- [ ] No new compiler warnings introduced
- [ ] `cargo fmt` has been run
- [ ] `cargo clippy` passes without warnings

## Examples

<!-- If you added new functionality, please provide usage examples -->

```rust
// Example usage of new functionality
use embedded_charts::prelude::*;

// Your example code here
```

## Related Issues

<!-- Link any related issues -->

Fixes #(issue number)
Closes #(issue number)
Related to #(issue number)

## Additional Notes

<!-- Add any additional notes, concerns, or context for reviewers -->

## Checklist

- [ ] I have read the [CONTRIBUTING](CONTRIBUTING.md) guidelines
- [ ] My code follows the code style of this project
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] All new and existing tests pass
- [ ] I have updated the documentation accordingly
- [ ] I have added an entry to CHANGELOG.md (if applicable)

## Screenshots (if applicable)

<!-- Add screenshots to help explain your changes -->

## Reviewer Notes

<!-- Add any specific notes for reviewers -->

---

**For Maintainers:**

- [ ] Reviewed for security implications
- [ ] Reviewed for API consistency
- [ ] Reviewed for documentation completeness
- [ ] Considered impact on different MCU targets
- [ ] Verified examples still work