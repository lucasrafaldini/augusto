# augusto 🎭

[![Rust](https://github.com/lucasrafaldini/augusto/workflows/Rust/badge.svg)](https://github.com/lucasrafaldini/augusto/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.56%2B-blue.svg)](https://www.rust-lang.org)

augusto is a Rust command-line suite that allows you to interact with words in creative and insightful ways directly from your terminal.

## Introduction

Inspired by the Brazilian concrete poet Augusto de Campos, who explored the visual and sonic dimensions of language, **augusto** empowers you to deconstruct and recombine words through various operations. Featuring anagram generation, ASCII art, phonetic pattern analysis, palindrome detection, syllable splitting, word blending (portmanteau), and etymological root discovery.

## Features

- 🔄 **Anagram Generation**: Generate all possible letter combinations of a word
- 🎨 **ASCII Art**: Write one word using another word as filler, creating concrete poetry
- 🔤 **Phonetic Pattern**: Visualise the vowel/consonant (V/C) rhythm of any word
- 🔁 **Palindrome Analysis**: Detect palindromes, mirror words, and find the longest palindromic substring
- 🔡 **Syllable Splitting**: Heuristic syllable decomposition useful for poetic rhythm
- 🧬 **Word Blending**: Generate portmanteau fusions of two words
- 📚 **Etymological Roots**: Identify Latin and Greek prefixes and suffixes embedded in a word
- ⚡ **Performance Benchmarks**: Measure and analyze operation performance with detailed statistics
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

### Phonetic Pattern
```bash
augusto pattern "rust"
# Compare two words:
augusto pattern "rust" "poesia"
```

### Palindrome Analysis
```bash
augusto palindrome "racecar"
```

### Syllable Splitting
```bash
augusto syllable "beautiful"
```

### Word Blending
```bash
augusto blend "smoke" "fog"
```

### Etymological Roots
```bash
augusto roots "biology"
```

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

#### Performance Benchmarking

```bash
augusto bench <operation> <arguments...>
```

**Arguments:**
- `<operation>`: The operation to benchmark (`anagram` or `art`)
- `<arguments>`: Arguments for the operation

**Output:**
Detailed performance statistics including execution time, throughput, and iterations.

**Examples:**
```bash
# Benchmark anagram generation
augusto bench anagram "test"

# Benchmark ASCII art
augusto bench art "RUST" "code"
```

#### Phonetic Pattern

```bash
augusto pattern <word> [word2]
```

**Arguments:**
- `<word>`: The word to analyse (required)
- `[word2]`: A second word for side-by-side comparison (optional)

**Output:**
The V/C pattern, vowel count, and consonant count of each word.

#### Palindrome Analysis

```bash
augusto palindrome <word>
```

**Arguments:**
- `<word>`: The word to analyse (required)

**Output:**
Whether the word is a palindrome, its mirror (reverse), and the longest
palindromic substring found within it.

#### Syllable Splitting

```bash
augusto syllable <word>
```

**Arguments:**
- `<word>`: The word to split (required)

**Output:**
The word broken into syllables with a hyphen separator and total syllable count.
Uses a heuristic onset-maximization algorithm (works well for English and
Portuguese; results may vary for other languages).

#### Word Blending (Portmanteau)

```bash
augusto blend <word1> <word2>
```

**Arguments:**
- `<word1>`: First word (required)
- `<word2>`: Second word (required)

**Output:**
A list of portmanteau blends, sorted by how "balanced" the blend is (closest
to the average length of both input words appears first).

#### Etymological Roots

```bash
augusto roots <word>
```

**Arguments:**
- `<word>`: The word to analyse (required)

**Output:**
All Latin and Greek prefixes and suffixes identified in the word, along with
their position, origin, and meaning. The database contains ~100 entries.

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

### Performance Benchmark Examples

#### Benchmark Anagram Generation
```bash
augusto bench anagram "cat"
# Output:
# ╔════════════════════════════════════════════════════════════╗
# ║              PERFORMANCE BENCHMARK RESULTS                 ║
# ╚════════════════════════════════════════════════════════════╝
# 
# Operation:        Anagram Generation
# Input:            "cat"
# Input length:     3 character(s)
# Output size:      6 item(s)
# 
# Total time:       48.29ms
# Iterations:       10000
# Avg per run:      4μs
# Throughput:       207087 ops/sec
```

#### Benchmark ASCII Art
```bash
augusto bench art "LUXO" "LIXO"
# Output:
# ╔════════════════════════════════════════════════════════════╗
# ║              PERFORMANCE BENCHMARK RESULTS                 ║
# ╚════════════════════════════════════════════════════════════╝
# 
# Operation:        ASCII Art Generation
# Input:            "LUXO+LIXO"
# Input length:     9 character(s)
# 
# Total time:       358μs
# Iterations:       10
# Avg per run:      35μs
# Throughput:       27929 ops/sec
```

**Note:** Benchmark iterations automatically adjust based on input complexity. Shorter inputs run more iterations for accurate measurements.

### Phonetic Pattern Examples

```bash
augusto pattern "rust"
# Output:
# Word:       rust
# Pattern:    CVCC
# Vowels:     1
# Consonants: 3

augusto pattern "rust" "poesia"
# Output (both words):
# Word:       rust
# Pattern:    CVCC
# Vowels:     1
# Consonants: 3
#
# Word:       poesia
# Pattern:    CVVCVV
# Vowels:     4
# Consonants: 2
```

### Palindrome Examples

```bash
augusto palindrome "racecar"
# Output:
# Word:             racecar
# Is palindrome:    Yes ✓
# Mirror (reverse): racecar
# Longest palindromic substring: "racecar"

augusto palindrome "hello"
# Output:
# Word:             hello
# Is palindrome:    No
# Mirror (reverse): olleh
# Longest palindromic substring: "ll"
```

### Syllable Examples

```bash
augusto syllable "beautiful"
# Output:
# Word:      beautiful
# Syllables: beau-ti-ful (3 syllables)

augusto syllable "rust"
# Output:
# Word:      rust
# Syllables: rust (1 syllable)
```

### Word Blending Examples

```bash
augusto blend "smoke" "fog"
# Output includes "smog" (sm + og) near the top

augusto blend "breakfast" "lunch"
# Output: portmanteau words like "brunch" and many other combinations
```

### Etymological Roots Examples

```bash
augusto roots "biology"
# Output:
# Word: biology
#
# Roots found:
#   logy       (suffix, Greek) — study of
#   bio        (prefix, Greek) — life
#   bi         (prefix, Latin) — two

augusto roots "telescope"
# Output:
# Word: telescope
#
# Roots found:
#   scope      (suffix, Greek) — viewing instrument
#   tele       (prefix, Greek) — far, distant
```

## Development

### Project Structure

```
augusto/
├── augusto/              # Main Rust project
│   ├── src/
│   │   ├── main.rs       # Entry point and CLI handling
│   │   ├── anagram.rs    # Anagram generation logic
│   │   ├── ascii_art.rs  # ASCII art generation logic
│   │   ├── benchmark.rs  # Performance benchmarking utilities
│   │   ├── pattern.rs    # Phonetic pattern (V/C) analysis
│   │   ├── palindrome.rs # Palindrome detection, mirror, longest substring
│   │   ├── syllable.rs   # Heuristic syllable splitting
│   │   ├── blend.rs      # Word blending / portmanteau generation
│   │   └── roots.rs      # Latin/Greek etymological roots database
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

### Version 0.1.x
- [x] Basic anagram generation
- [x] ASCII art generation
- [x] CLI interface
- [x] Unit tests
- [x] Performance benchmarks
- [x] Documentation improvements

### Version 0.2.0 (Current)
- [x] Phonetic pattern analysis (V/C patterns)
- [x] Palindrome detection, mirror, and longest palindromic substring
- [x] Heuristic syllable splitting
- [x] Word blending / portmanteau generation
- [x] Etymological roots database (Latin & Greek, ~100 entries)

### Version 0.3.0 (Planned)
- [ ] Dictionary filtering (real words only)
- [ ] Output formatting options (JSON, CSV, etc.)
- [ ] Interactive mode
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
