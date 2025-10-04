# Getting Started with Augusto Development

Welcome to Augusto! This guide will help you get up and running quickly, whether you're fixing a bug, adding a feature, or just exploring the codebase.

## 🚀 Quick Start (5 minutes)

### 1. Prerequisites

- **Rust 1.56+**: [Install Rust](https://rustup.rs/)
- **Git**: [Install Git](https://git-scm.com/downloads)
- **A GitHub account**: [Sign up](https://github.com/join)

### 2. Fork & Clone

```bash
# Fork the repo on GitHub, then:
git clone https://github.com/YOUR_USERNAME/augusto.git
cd augusto

# Add upstream remote
git remote add upstream https://github.com/lucasrafaldini/augusto.git
```

### 3. Build & Test

```bash
# Navigate to the Rust project
cd augusto

# Build the project
cargo build

# Run tests
cargo test

# Try it out!
cargo run -- "cat"
```

**Expected output:**
```
{"tca", "act", "cta", "tac", "atc", "cat"}
```

🎉 **You're all set!** If you see the anagrams, everything is working.

## 📚 Understanding the Codebase

### Project Structure

```
augusto/
├── augusto/          ← The Rust project lives here
│   ├── src/
│   │   ├── main.rs       ← Entry point, CLI handling
│   │   └── anagram.rs    ← Anagram generation logic
│   └── Cargo.toml        ← Dependencies and metadata
└── docs/             ← Documentation files
```

### Key Files to Know

1. **`augusto/src/main.rs`**
   - Entry point for the CLI
   - Parses command-line arguments
   - Calls anagram generation
   - Handles output

2. **`augusto/src/anagram.rs`**
   - Core anagram generation algorithm
   - Recursive permutation logic
   - Main business logic

3. **`augusto/Cargo.toml`**
   - Project metadata
   - Dependencies (currently just `termion`)
   - Build configuration

### How It Works

```rust
// User types: augusto "cat"

// main.rs receives input
let input = "cat";

// Calls anagram generation
let results = anagram::letter_combinations("cat");
// Returns: ["cat", "cta", "act", "atc", "tca", "tac"]

// Deduplicates with HashSet
let unique = results.into_iter().collect::<HashSet<_>>();

// Prints output
println!("{:?}", unique);
```

## 🛠 Common Development Tasks

### Running the Project

```bash
# Run with cargo (development)
cd augusto
cargo run -- "word"

# Run with optimizations
cargo run --release -- "word"

# Run the built binary directly
./target/debug/augusto "word"
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_anagram_combinations
```

### Code Quality

```bash
# Format your code (do this before committing!)
cargo fmt

# Check for common issues
cargo clippy

# Both at once
cargo fmt && cargo clippy
```

## 🐛 Making Your First Contribution

### Find Something to Work On

1. **Good First Issues**: Look for issues labeled `good-first-issue`
2. **Documentation**: Improve comments, examples, or README
3. **Tests**: Add test cases for edge conditions
4. **Small Features**: Check the roadmap in PROJECT_PLAN.md

### Development Workflow

#### 1. Create a Branch

```bash
git checkout -b feature/my-awesome-feature
```

**Branch naming:**
- `feature/` - new features
- `fix/` - bug fixes
- `docs/` - documentation
- `test/` - test improvements

#### 2. Make Changes

Edit the relevant files in `augusto/src/`.

#### 3. Test Your Changes

```bash
cd augusto

# Run tests
cargo test

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy -- -D warnings

# Try it manually
cargo run -- "test"
```

#### 4. Commit Your Changes

```bash
git add .
git commit -m "feat: add support for emoji in anagrams"
```

**Commit message format:**
- `feat:` - new features
- `fix:` - bug fixes
- `docs:` - documentation changes
- `test:` - test additions
- `refactor:` - code refactoring

#### 5. Push and Create PR

```bash
git push origin feature/my-awesome-feature
```

Then go to GitHub and create a Pull Request!

## 🎯 Example: Adding a Feature

Let's walk through adding a simple feature: a `--reverse` flag that reverses words.

### 1. Add the Function

Edit `augusto/src/anagram.rs`:

```rust
/// Reverses a string
pub fn reverse_word(input: &str) -> String {
    input.chars().rev().collect()
}
```

### 2. Add a Test

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_word() {
        assert_eq!(reverse_word("cat"), "tac");
        assert_eq!(reverse_word("rust"), "tsur");
    }
}
```

### 3. Test It

```bash
cargo test
```

### 4. Add CLI Support (later)

For now, you can use it in code. CLI argument parsing will be added in v0.2.0!

## 🤔 Common Questions

### Q: Where should I start?

**A:** Start by reading the code in `main.rs` and `anagram.rs`. They're short and well-commented. Then try modifying the output format or adding a simple test.

### Q: I found a bug. What should I do?

**A:** 
1. Check if it's already reported in [Issues](https://github.com/lucasrafaldini/augusto/issues)
2. If not, create a new issue using the bug report template
3. If you want to fix it yourself, create a branch and submit a PR!

### Q: My tests are failing. Help!

**A:** 
```bash
# Clean and rebuild
cargo clean
cargo build

# Run tests with verbose output
cargo test -- --nocapture

# Check if it's a formatting issue
cargo fmt
```

### Q: How do I add a new dependency?

**A:**
```bash
# Edit Cargo.toml and add under [dependencies]
# Then run:
cargo build
```

### Q: The CI is failing on my PR. What do I do?

**A:** Check which step failed and run it locally:
```bash
cargo fmt -- --check  # Formatting
cargo clippy -- -D warnings  # Linting
cargo test  # Tests
```

## 📖 Learning Resources

### Rust Basics
- [The Rust Book](https://doc.rust-lang.org/book/) - Start here if you're new to Rust
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by doing
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises

### Project-Specific
- [README.md](README.md) - Project overview and usage
- [CONTRIBUTING.md](CONTRIBUTING.md) - Detailed contribution guidelines
- [PROJECT_PLAN.md](PROJECT_PLAN.md) - Roadmap and future plans
- [COMMANDS.md](COMMANDS.md) - Quick command reference

### Tools
- [Cargo Book](https://doc.rust-lang.org/cargo/) - Everything about Cargo
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/) - What Clippy checks for
- [Rust Analyzer](https://rust-analyzer.github.io/) - IDE support (highly recommended!)

## 💡 Pro Tips

1. **Use Rust Analyzer**: It provides incredible IDE support with inline type hints and error messages
2. **Run `cargo check` frequently**: It's faster than `cargo build` for catching errors
3. **Enable format-on-save**: Configure your editor to run `cargo fmt` automatically
4. **Read error messages carefully**: Rust's compiler errors are very helpful!
5. **Don't be afraid to ask**: Open a discussion or comment on an issue if you're stuck

## 🎓 Next Steps

Once you're comfortable with the basics:

1. **Add more tests**: Improve test coverage
2. **Improve documentation**: Add more examples in doc comments
3. **Optimize performance**: Profile and optimize the anagram algorithm
4. **Add features**: Check PROJECT_PLAN.md for ideas
5. **Help others**: Review PRs and answer questions

## 🆘 Getting Help

- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Code Comments**: Many answers are in the code documentation
- **This Guide**: You're reading it! Come back anytime

## 🎉 Your First Contribution Checklist

- [ ] Forked and cloned the repository
- [ ] Built the project successfully
- [ ] All tests pass locally
- [ ] Ran `cargo fmt` and `cargo clippy`
- [ ] Made your changes in a feature branch
- [ ] Added/updated tests for your changes
- [ ] Committed with a clear message
- [ ] Pushed to your fork
- [ ] Created a PR with a good description

**Welcome to the Augusto community!** 🚀

We're excited to have you here. Every contribution, no matter how small, makes a difference. Happy coding!

---

*Need help? Open an issue or reach out to the maintainers!*
