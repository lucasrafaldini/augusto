# Quick Command Reference

This document provides quick access to common development commands for the Augusto project.

## Development Commands

### Building

```bash
# Build debug version
cd augusto && cargo build

# Build release version (optimized)
cd augusto && cargo build --release

# Clean build artifacts
cd augusto && cargo clean
```

### Running

```bash
# Run with cargo
cd augusto && cargo run -- "word"

# Run compiled binary (debug)
./augusto/target/debug/augusto "word"

# Run compiled binary (release)
./augusto/target/release/augusto "word"
```

### Testing

```bash
# Run all tests
cd augusto && cargo test

# Run tests with output
cd augusto && cargo test -- --nocapture

# Run specific test
cd augusto && cargo test test_anagram_combinations

# Run tests with verbose output
cd augusto && cargo test --verbose
```

### Code Quality

```bash
# Format code
cd augusto && cargo fmt

# Check if code is formatted
cd augusto && cargo fmt -- --check

# Run clippy (linter)
cd augusto && cargo clippy

# Run clippy with warnings as errors
cd augusto && cargo clippy -- -D warnings

# Check without building
cd augusto && cargo check
```

### Documentation

```bash
# Generate and open documentation
cd augusto && cargo doc --open

# Generate documentation without opening
cd augusto && cargo doc

# Check documentation examples
cd augusto && cargo test --doc
```

### Installation

```bash
# Install locally
cd augusto && cargo install --path .

# Uninstall
cargo uninstall augusto
```

## Git Commands

### Branching

```bash
# Create and switch to new branch
git checkout -b feature/your-feature

# Switch to existing branch
git checkout main

# List all branches
git branch -a

# Delete local branch
git branch -d feature/your-feature
```

### Committing

```bash
# Stage changes
git add .

# Commit with message
git commit -m "feat: add new feature"

# Amend last commit
git commit --amend

# Interactive staging
git add -p
```

### Syncing

```bash
# Fetch upstream changes
git fetch upstream

# Rebase on upstream main
git rebase upstream/main

# Push to your fork
git push origin feature/your-feature

# Force push (after rebase)
git push origin feature/your-feature --force-with-lease
```

### Status and History

```bash
# Check status
git status

# View commit history
git log --oneline

# View changes
git diff

# View staged changes
git diff --cached
```

## GitHub Actions

### Viewing Workflow Runs

```bash
# View workflows (requires GitHub CLI)
gh workflow list

# View recent runs
gh run list

# View specific run
gh run view <run-id>
```

## Project Management

### Creating Issues

```bash
# Create bug report (requires GitHub CLI)
gh issue create --template bug_report.md

# Create feature request
gh issue create --template feature_request.md

# List issues
gh issue list

# View specific issue
gh issue view <issue-number>
```

### Pull Requests

```bash
# Create PR (requires GitHub CLI)
gh pr create

# View PR status
gh pr status

# Checkout a PR
gh pr checkout <pr-number>
```

## Useful Cargo Commands

### Dependency Management

```bash
# Update dependencies
cd augusto && cargo update

# Check for outdated dependencies
cargo outdated

# Add a dependency
cargo add <package-name>

# Remove a dependency
cargo rm <package-name>

# Show dependency tree
cargo tree
```

### Performance

```bash
# Build with optimizations
cd augusto && cargo build --release

# Run benchmarks (if configured)
cd augusto && cargo bench

# Profile binary size
cargo bloat --release

# Time compilation
cargo build --timings
```

### Advanced

```bash
# Check all targets
cd augusto && cargo check --all-targets

# Run with specific features
cargo run --features "feature-name"

# Cross-compile (example for Linux)
cargo build --target x86_64-unknown-linux-gnu

# Generate lockfile
cargo generate-lockfile

# Verify project
cargo verify-project
```

## Pre-commit Hooks

```bash
# Install pre-commit hooks
pre-commit install

# Run pre-commit on all files
pre-commit run --all-files

# Update hooks
pre-commit autoupdate

# Skip hooks for a commit
git commit --no-verify
```

## Troubleshooting

### Common Issues

```bash
# Clear cargo cache
cargo clean

# Remove and rebuild
rm -rf augusto/target && cd augusto && cargo build

# Update Rust toolchain
rustup update

# Check Rust version
rustc --version
cargo --version

# Fix formatting issues
cd augusto && cargo fmt
```

### Performance Profiling

```bash
# Run with timing information
cd augusto && cargo build --timings

# Profile with perf (Linux)
perf record ./augusto/target/release/augusto "test"
perf report

# Profile with Instruments (macOS)
instruments -t "Time Profiler" ./augusto/target/release/augusto "test"
```

## Release Process

```bash
# Update version in Cargo.toml
# Update CHANGELOG.md

# Create git tag
git tag -a v0.2.0 -m "Release version 0.2.0"

# Push tag
git push origin v0.2.0

# Publish to crates.io
cd augusto && cargo publish

# Create GitHub release (requires GitHub CLI)
gh release create v0.2.0 --title "v0.2.0" --notes "Release notes here"
```

## Environment Setup

### First Time Setup

```bash
# Clone repository
git clone https://github.com/lucasrafaldini/augusto.git
cd augusto

# Add upstream remote
git remote add upstream https://github.com/lucasrafaldini/augusto.git

# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install useful tools
cargo install cargo-watch
cargo install cargo-outdated
cargo install cargo-bloat

# Build project
cd augusto && cargo build
```

### Daily Development Workflow

```bash
# 1. Sync with upstream
git fetch upstream
git rebase upstream/main

# 2. Create feature branch
git checkout -b feature/my-feature

# 3. Make changes and test frequently
cd augusto && cargo watch -x test

# 4. Before committing
cd augusto && cargo fmt
cd augusto && cargo clippy
cd augusto && cargo test

# 5. Commit and push
git add .
git commit -m "feat: add my feature"
git push origin feature/my-feature

# 6. Create PR
gh pr create
```

## Quick Tips

- **Fast iteration**: Use `cargo check` instead of `cargo build` for quick syntax checks
- **Watch mode**: Use `cargo watch -x test` to automatically run tests on file changes
- **Faster builds**: Enable incremental compilation in `.cargo/config.toml`
- **Better errors**: Set `RUST_BACKTRACE=1` for detailed error traces
- **Optimize compile time**: Use `cargo build --release --jobs <n>` to control parallelism

## Resources

- [Rust Documentation](https://doc.rust-lang.org/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/)

---

*Last Updated: October 4, 2025*
