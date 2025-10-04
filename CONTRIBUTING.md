# Contributing to Augusto

Thank you for your interest in contributing to **augusto**! We welcome contributions from everyone, whether you're fixing bugs, adding features, improving documentation, or suggesting new ideas.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Reporting Bugs](#reporting-bugs)
- [Suggesting Features](#suggesting-features)

## Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CONTRIBUTE.md). Please read it to understand the expectations we have for everyone who contributes.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/augusto.git
   cd augusto
   ```
3. **Add the upstream repository**:
   ```bash
   git remote add upstream https://github.com/lucasrafaldini/augusto.git
   ```
4. **Set up your development environment**:
   ```bash
   cd augusto
   cargo build
   cargo test
   ```

## How to Contribute

### Types of Contributions

- 🐛 **Bug fixes**: Fix issues or bugs in the codebase
- ✨ **New features**: Implement new functionality
- 📝 **Documentation**: Improve README, code comments, or examples
- 🎨 **Code quality**: Refactoring, performance improvements
- 🧪 **Tests**: Add or improve test coverage
- 💡 **Ideas**: Suggest new features or improvements

## Development Workflow

### 1. Create a Branch

Create a feature branch for your work:

```bash
git checkout -b feature/your-feature-name
```

Branch naming conventions:
- `feature/` - for new features
- `fix/` - for bug fixes
- `docs/` - for documentation updates
- `refactor/` - for code refactoring
- `test/` - for test additions/improvements

### 2. Make Your Changes

- Write clear, concise commit messages
- Follow the existing code style
- Add tests for new functionality
- Update documentation as needed

### 3. Test Your Changes

Before submitting, ensure all checks pass:

```bash
# Run tests
cargo test

# Check formatting
cargo fmt -- --check

# Run linter
cargo clippy -- -D warnings

# Build release version
cargo build --release
```

### 4. Keep Your Branch Updated

Regularly sync with the upstream repository:

```bash
git fetch upstream
git rebase upstream/main
```

## Coding Standards

### Rust Style Guidelines

- Follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/)
- Use `cargo fmt` to format your code
- Address all `cargo clippy` warnings
- Write idiomatic Rust code

### Code Quality

- **Clarity**: Write self-documenting code with clear variable names
- **Comments**: Add doc comments (`///`) for public APIs
- **Error Handling**: Use proper Result/Option types, avoid unwrap() in library code
- **Performance**: Consider performance implications, but prioritize readability
- **Safety**: Avoid unsafe code unless absolutely necessary (and document why)

### Documentation

- Add doc comments for all public functions, structs, and modules
- Include examples in doc comments when helpful
- Update README.md for user-facing changes
- Update CHANGELOG.md following [Keep a Changelog](https://keepachangelog.com/) format

## Testing

### Writing Tests

- Add unit tests in the same file as the code (`#[cfg(test)]` module)
- Add integration tests in `tests/` directory
- Aim for high test coverage, especially for core functionality
- Test edge cases and error conditions

### Test Examples

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Arrange
        let input = "test";
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected_output);
    }
}
```

## Submitting Changes

### Pull Request Process

1. **Push your changes** to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Open a Pull Request** on GitHub with:
   - Clear title describing the change
   - Detailed description of what was changed and why
   - Reference to any related issues (e.g., "Fixes #123")
   - Screenshots or examples if applicable

3. **PR Template**:
   ```markdown
   ## Description
   Brief description of changes
   
   ## Type of Change
   - [ ] Bug fix
   - [ ] New feature
   - [ ] Documentation update
   - [ ] Code refactoring
   
   ## Testing
   - [ ] All tests pass
   - [ ] Added new tests
   - [ ] Manually tested
   
   ## Checklist
   - [ ] Code follows project style
   - [ ] Self-reviewed code
   - [ ] Commented complex code
   - [ ] Updated documentation
   - [ ] No new warnings
   ```

4. **Code Review**: Maintainers will review your PR and may request changes

5. **Merge**: Once approved, your PR will be merged!

## Reporting Bugs

### Before Submitting a Bug Report

- Check if the bug has already been reported in [Issues](https://github.com/lucasrafaldini/augusto/issues)
- Ensure you're using the latest version
- Collect relevant information about your environment

### Bug Report Template

```markdown
**Describe the bug**
A clear description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Run command '...'
2. With input '...'
3. See error

**Expected behavior**
What you expected to happen.

**Actual behavior**
What actually happened.

**Environment:**
- OS: [e.g., macOS 12.0]
- Rust version: [e.g., 1.65.0]
- Augusto version: [e.g., 0.1.0]

**Additional context**
Any other relevant information.
```

## Suggesting Features

We love new ideas! To suggest a feature:

1. **Check existing issues** to avoid duplicates
2. **Open a new issue** with the `enhancement` label
3. **Describe the feature**:
   - What problem does it solve?
   - How should it work?
   - Example use cases
   - Potential implementation ideas (optional)

### Feature Request Template

```markdown
**Is your feature related to a problem?**
Description of the problem.

**Describe the solution you'd like**
Clear description of what you want to happen.

**Describe alternatives considered**
Other solutions you've thought about.

**Additional context**
Any other relevant information, mockups, or examples.
```

## Questions?

If you have questions about contributing, feel free to:
- Open a discussion on GitHub
- Comment on relevant issues
- Reach out to the maintainers

## Recognition

All contributors will be recognized in our release notes and README. Thank you for helping make augusto better! 🎉

---

**Happy Contributing! 🚀**
