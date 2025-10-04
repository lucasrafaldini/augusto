# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive README documentation with installation instructions
- Code documentation for all public modules and functions
- Additional unit tests for edge cases
- Improved error handling and user feedback
- GitHub Actions workflow enhancements (caching, clippy, fmt checks)
- CHANGELOG.md file for tracking project changes

### Changed
- Enhanced Cargo.toml with complete metadata
- Improved CLI with usage instructions
- Updated Code of Conduct reference in README

### Fixed
- GitHub Actions workflow directory paths
- Missing error messages for invalid inputs

## [0.1.0] - 2023

### Added
- Initial release
- Basic anagram generation functionality
- CLI interface for word input
- Unit tests for anagram generation
- MIT License
- Basic README and contributing guidelines
- GitHub Actions CI/CD pipeline
- Pre-commit hooks configuration

### Features
- Generate all permutations of a given word
- Deduplicate results using HashSet
- Recursive algorithm for efficient generation

[Unreleased]: https://github.com/lucasrafaldini/augusto/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/lucasrafaldini/augusto/releases/tag/v0.1.0
