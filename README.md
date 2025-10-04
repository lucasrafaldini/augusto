# augusto 🎭

[![Rust](https://github.com/lucasrafaldini/augusto/workflows/Rust/badge.svg)](https://github.com/lucasrafaldini/augusto/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.56%2B-blue.svg)](https://www.rust-lang.org)

augusto is a Rust command-line suite that allows you to interact with words in creative and insightful ways directly from your terminal.

## Introduction

Inspired by the Brazilian concrete poet Augusto de Campos, who explored the visual and sonic dimensions of language, **augusto** empowers you to deconstruct and recombine words through various operations. Currently featuring anagram generation and ASCII art creation, with plans to expand into analogic interpolations and other word transformations.

## Features

- 🔄 **Anagram Generation**: Generate all possible letter combinations of a word
- 🎨 **ASCII Art**: Write one word using another word as filler, creating concrete poetry
- 🚀 **Fast & Efficient**: Built with Rust for optimal performance
- 📦 **Minimal Dependencies**: Lightweight footprint (only termion for terminal interactions)
- 🎯 **CLI-First**: Designed for seamless command-line workflows

## Table of Contents

- [Installation](#installation)
- [Getting Started](#getting-started)
- [Usage](#usage)
- [Examples](#examples)
- [Development](#development)
- [Contributing](#contributing)
- [Roadmap](#roadmap)
- [License](#license)

## Installation

### Prerequisites

- Rust 1.56 or higher
- Cargo (comes with Rust)

### From Source

1. Clone the repository:
```bash
git clone https://github.com/lucasrafaldini/augusto.git
cd augusto
```

2. Build the project:
```bash
cd augusto
cargo build --release
```

3. The binary will be available at `target/release/augusto`

### Install Locally

To install augusto to your local Cargo bin directory:

```bash
cd augusto
cargo install --path .
```

This makes `augusto` available system-wide.

## Getting Started

Once installed, you can start using augusto immediately:

### Anagrams
```bash
augusto anagram "cat"
# or for backwards compatibility:
augusto "cat"
```

This will generate all anagrams of the word "cat".

### ASCII Art
```bash
augusto art "RUST" "code"
```

This will create ASCII art of the word "RUST" using the letters from "code" as filler.

## Usage

### Commands

Augusto supports multiple commands for different word operations:

#### Anagram Generation

```bash
augusto anagram <word>
# or simply:
augusto <word>
```

**Arguments:**
- `<word>`: The input word to generate anagrams from (required)

**Output:**
The command outputs a set of unique anagrams generated from the input word.

#### ASCII Art Creation

```bash
augusto art <main_word> <filler_word>
```

**Arguments:**
- `<main_word>`: The word to display in large ASCII letters
- `<filler_word>`: The word whose letters will be used to fill the pattern

**Output:**
ASCII art representation of the main word, filled with characters from the filler word.

#### Help

```bash
augusto help
# or:
augusto --help
```

## Examples

### Anagram Examples

#### Simple Word
```bash
augusto anagram "cat"
# Output: {"tca", "act", "cta", "tac", "atc", "cat"}
```

#### Short Word with Repeated Letters
```bash
augusto anagram "aba"
# Output: {"aab", "baa", "aba"}
```

#### Longer Words
```bash
augusto anagram "rust"
# Output: All 24 permutations of "rust"
```

**Note:** The number of anagrams grows factorially with word length. For a word with n unique letters, expect n! combinations.

### ASCII Art Examples

#### Simple Example
```bash
augusto art "HI" "rust"
# Output:
# r   u strus
# t   r   u  
# strus   t  
# r   u   s  
# t   r ustru
```

#### Word Art
```bash
augusto art "RUST" "code"
# Output:
# code  c   o  deco decod
# e   c o   d e       c  
# odec  o   d  eco    d  
# e  c  o   d     e   c  
# o   d  eco  deco    d
```

#### LUXO/LIXO - Tribute to Augusto de Campos
```bash
augusto art "LUXO" "LIXO"
# Output:
# L     I   X O   L  IXO 
# L     I   X  O L  I   X
# O     L   I   X   O   L
# I     X   O  L I  X   O
# LIXOL  IXO  L   I  XOL
#
# Inspired by Augusto de Campos' iconic concrete poem
# "LUXO" (luxury) written with "LIXO" (trash)
# A powerful commentary on consumerism and social inequality
```

#### Creative Poetry
```bash
augusto art "LOVE" "heart"
# Creates ASCII art spelling "LOVE" filled with letters from "heart"
```

**Tip:** The filler word is repeated cyclically, so experiment with different combinations to create unique visual effects!

## Development

### Project Structure

```
augusto/
├── augusto/              # Main Rust project
│   ├── src/
│   │   ├── main.rs       # Entry point and CLI handling
│   │   ├── anagram.rs    # Anagram generation logic
│   │   └── ascii_art.rs  # ASCII art generation logic
│   ├── Cargo.toml        # Project dependencies
│   └── Cargo.lock        # Locked dependencies
├── .github/
│   ├── workflows/        # CI/CD workflows
│   └── scripts/          # GitHub automation scripts
├── README.md             # This file
├── LICENSE               # MIT License
└── CONTRIBUTE.md         # Contribution guidelines and Code of Conduct
```

### Running Tests

```bash
cd augusto
cargo test
```

### Running with Cargo

```bash
cd augusto
cargo run -- "word"
```

### Building for Release

```bash
cd augusto
cargo build --release
```

## Contributing

We welcome contributions from the community! To contribute to augusto, please follow these guidelines:

1. **Fork the repository**
2. **Create a new branch** for your feature or bug fix:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Make your changes** and commit them with clear, descriptive messages
4. **Test your changes thoroughly**:
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```
5. **Push to your fork** and create a pull request with a clear description
6. **After review and approval**, your changes will be merged into the main branch

Please ensure your contributions adhere to our [Code of Conduct](CONTRIBUTE.md).

### Development Guidelines

- Follow Rust naming conventions and idioms
- Write tests for new functionality
- Document public APIs with doc comments
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings

## Roadmap

### Version 0.1.x (Current)
- [x] Basic anagram generation
- [x] CLI interface
- [x] Unit tests
- [ ] Performance benchmarks
- [ ] Documentation improvements

### Version 0.2.0 (Planned)
- [ ] Word combination operations
- [ ] Pattern matching for anagrams
- [ ] Dictionary filtering (real words only)
- [ ] Output formatting options (JSON, CSV, etc.)
- [ ] Interactive mode

### Version 0.3.0 (Future)
- [ ] Analogic interpolations
- [ ] Visual word transformations
- [ ] Multi-word operations
- [ ] Plugin system for custom operations
- [ ] Web API/service version

### Long-term Vision
- [ ] Natural language processing features
- [ ] Poetic pattern generation inspired by concrete poetry
- [ ] Integration with dictionary APIs
- [ ] Educational mode with linguistic insights

## Acknowledgments

- Inspired by **Augusto de Campos**, Brazilian concrete poet and pioneer of visual poetry
- Built with [Rust](https://www.rust-lang.org/)
- Terminal handling by [termion](https://github.com/redox-os/termion)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Copyright (c) 2023 Lucas Rafaldini

---

**Made with ❤️ and Rust**
