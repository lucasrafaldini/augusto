# augusto 🎭

[![Rust](https://github.com/lucasrafaldini/augusto/actions/workflows/rust.yml/badge.svg)](https://github.com/lucasrafaldini/augusto/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.56%2B-blue.svg)](https://www.rust-lang.org)
[![Crates.io](https://img.shields.io/crates/v/augusto.svg)](https://crates.io/crates/augusto)
[![Crates.io Downloads](https://img.shields.io/crates/d/augusto.svg)](https://crates.io/crates/augusto)
[![Hacktoberfest 2026](https://img.shields.io/badge/Hacktoberfest-2026-orange.svg)](https://hacktoberfest.digitalocean.com/)
[![Open Issues](https://img.shields.io/github/issues/lucasrafaldini/augusto)](https://github.com/lucasrafaldini/augusto/issues)
[![Good First Issues](https://img.shields.io/github/issues/lucasrafaldini/augusto/good%20first%20issue)](https://github.com/lucasrafaldini/augusto/issues?q=is%3Aissue+is%3Aopen+label%3A"good+first+issue")
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/lucasrafaldini/augusto/pulls)
[![Code Style: Rust](https://img.shields.io/badge/code_style-rustfmt-blue.svg)](https://github.com/rust-lang/rustfmt)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)

augusto is a Rust command-line suite that allows you to interact with words in creative and insightful ways directly from your terminal.

## Introduction

Inspired by the Brazilian concrete poet Augusto de Campos, who explored the visual and sonic dimensions of language, **augusto** empowers you to deconstruct and recombine words through various operations. Currently featuring anagram generation and ASCII art creation, with plans to expand into analogic interpolations and other word transformations.

## Features

- 🔄 **Anagram Generation**: Generate all possible letter combinations of a word
- 🎨 **ASCII Art**: Write one word using another word as filler, creating concrete poetry
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
- [Hacktoberfest 2026](#hacktoberfest-2026-)
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

#### ASCII Animations (NEW in 0.2.0)

```bash
augusto animate <shape> <word> [options]
```

**Arguments:**
- `<shape>`: The 3D shape to animate (`donut`, `cube`, `cube5d`, `sphere`, `mandala`, `pyramid`)
- `<word>`: The word to display within the animation

**Options:**
- `--speed <ms>`: Frame delay in milliseconds (default: 100)
- `--frames <n>`: Number of frames to render (default: infinite, use Ctrl+C to stop)
- `--color`: Enable ANSI color output
- `--filler <word>`: Word to use as filler characters (default: the main word)

**Output:**
Animated ASCII art of the specified 3D shape with the word embedded.

**Examples:**
```bash
# Rotating donut with word "RUST"
augusto animate donut "RUST"

# Rotating cube with custom speed
augusto animate cube "CODE" --speed 50

# 5D cube projection with color
augusto animate cube5d "HACKTOBERFEST" --color

# Sphere with custom filler word
augusto animate sphere "RUST" --filler "code"

# Mandala pattern
augusto animate mandala "PEACE" --speed 80

# Pyramid with limited frames
augusto animate pyramid "RUST" --frames 50
```

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

### ASCII Animation Examples

#### Rotating Donut
```bash
augusto animate donut "RUST" --frames 1 --width 70 --height 25
```
```
                        RRTSSTSUURTTSURTSTSURRT                       
                     USSUURTSSURTSUURTUSUSURSURSUT                    
                   SRTSURSUTSUTSUURTSRTSSRTRTRURURRT                  
                 RRSUTTUR RTUTSURURTSRURTTSUU TRSSURST                
               USTT RUTSTURSRTRTSTSURUTUURRTSTSURST UTTR              
              RSURUSTTUURSSSTUSUURRSRSTSSUSUTRSTUUTTURRSS             
             RUTSTSURTSURTRUTRRT       RRSUSUUTRSSRRSSSTSU            
             SUURSRRSSRSUTTUS             USSRRUSUTTRUURUR            
             TSUUTRUSTRRUSTSU             RTTTSRTURSUTRTRT            
             URTSUSUTTRUUSSTSTTT       STRTRTSSURRSTRSUURR            
              TSRT RTSURTTRRURUSRUUTUTSTURSUSUURTSUU STRT             
               URUS USRURSSUTRUSTTUSTRURSSTUURSTTUR SSUT              
                TSTRRTUSRSTRSTRUUSTRUSTSRURSTUTRRSTSRRS               
                  SRUTSSRUSTTSTRUSTRUSSTRSRURUTRUSTTS                 
                     URUSRURUSTSTRUSSTRRSTRUSSRUUU                    
                        TRTUSTRUSSTRRUSSTRRUSTT                       
```
*Run without `--frames` for infinite animation. Add `--color` for ANSI colors.*

#### Rotating Cube
```bash
augusto animate cube "CODE" --frames 1 --width 70 --height 25
```
```
                    ELOLLEHOLLEHOLLEHOLLEHOLLEHOLEE                   
                    L LO                       LL H                   
                    L   HE                   HO   O                   
                    O     LOLLEHOLLEHOLLEHOLE     L                   
                    H      H               L      L                   
                    E      E               E      E                   
                    L      L               H      H                   
                    L      L               O      O                   
                    O     OLLOHELLOHELLOHELLH     L                   
                    H   LL                   OL   L                   
                    E HE                       LE E                   
                    HOLLOHELLOHELLOHELLOHELLOHELLHH                   
```
*Wireframe cube rotating on X, Y, and Z axes. Use `--speed 50` for faster rotation.*

#### 5D Cube (Hypercube/Tesseract Projection)
```bash
augusto animate cube5d "HACK" --frames 1 --width 70 --height 25 --color
```
```
         CKCKHACKHACKHACKHACKHACKHACKHACKHACKHACKHACKHACKHACCK        
         C HAC                                            HK K        
         A   CKH                                       KCA   C        
         H      ACK                                  AH      A        
         K         HAC                            HKC        H        
         C           CKH                       ACA           K        
         A              AAHACKHACKHACKHACKHACKKK             C        
         H               A KCHACKHACKHACKHKHA K              A        
         K               H  KAKHACKHACKHAC A  C              H        
         C               K  C C         A  H  A              K        
         A               C  A A         H  K  H              C        
         H               A  HKCHACKHACKHAC C  K              A        
         K               H AHACKHACKHACKHAKHA C              H        
         C              AKHACKHACKHACKHACKHACKKA             K        
         A            KH                       AHK           C        
         H         HAC                            CAH        A        
         K      ACK                                 CKC      H        
         C    KH                                       AHK   K        
         A HAC                                            CA C        
         AKHACKHACKHACKHACKHACKHACKHACKHACKHACKHACKHACKHACKHHK        
```
*4D hypercube (tesseract) projected to 3D then 2D. `--color` enables rainbow ANSI colors.*

#### Rotating Sphere
```bash
augusto animate sphere "RUST" --filler "code" --frames 1 --width 60 --height 20
```
```
                          USSTRURSU                         
                    TRTUURTSURRSTUTRURSTR                   
                URSTURRTSUURTSURTSUSTUTSTRRTU               
              TSTURSTSRRTSRURUTTSRRTSURRSSUTSST             
              RTSUTSSUTSTUUTSSRRSUUTTSUUTTSRUSUS            
             RTRTRUTUSRRTSURTSURTSURTSSRRSUUTTUR            
             TSSUSRSTRUSTRUSTRUSTRUSTRUSTRSRUSTS            
             URURRSTUTTSUUTRUTTSUUTRUTTUUTSSRRSU            
              RTSSTURRTSUUTRTRSSURTTUSURTTURTURS            
              USUTSUURRTSRSSUUTRURSSRTTSUUTSTUT             
                TRRRUTUSTRTSURTSURTURSUTRRTRT               
                   USTTUSTSURSTUTUURSUTRTT                  
                         SRTRRUTUUUS                        
```
*Wireframe sphere with latitude/longitude lines. `--filler` sets the surface characters.*

#### Mandala Pattern
```bash
augusto animate mandala "PEACE" --frames 1 --width 80 --height 40 --speed 80
```
```
         EEEEEEEE        EEEEEEEE        EEEEEEEE        EEEEEEEE        
       EE      EE      EE      EE      EE      EE      EE      EE      
     EE          EE  EE          EE  EE          EE  EE          EE    
    E              EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE    
   E              EE                                                         
  E              EE      EE      EE      EE      EE      EE      EE      
  E             EE        EE  EE        EE  EE        EE  EE        EE   
  E            EE          EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE     
  E           EE                                                         
  E           EE      EE      EE      EE      EE      EE      EE      EE 
  E          EE        EE  EE        EE  EE        EE  EE        EE  EE 
  E         EE          EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE 
  E        EE                                                         
  E        EE      EE      EE      EE      EE      EE      EE      EE   
  E       EE        EE  EE        EE  EE        EE  EE        EE  EE    
  E      EE          EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE
  E     EE                                                         
  E     EE      EE      EE      EE      EE      EE      EE      EE      EE
  E    EE        EE  EE        EE  EE        EE  EE        EE  EE  EE   
  E   EE          EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE 
  E  EE                                                         
  E  EE      EE      EE      EE      EE      EE      EE      EE      EE  
  E EE        EE  EE        EE  EE        EE  EE        EE  EE  EE      
  EEE          EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE  
 EE                                                         
 EE      EE      EE      EE      EE      EE      EE      EE      EE      EE
EE        EE  EE        EE  EE        EE  EE        EE  EE        EE  EE   
EE         EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE
```
*Hypnotic rotational symmetry pattern with 12-fold symmetry. Best viewed animated with `--speed 80`.*

#### Pyramid
```bash
augusto animate pyramid "RUST" --frames 1 --width 60 --height 25
```
```
                     TSURTSURTSURTSURTTU                    
                     R TR           UR R                    
                     U   US       TS   T                    
                     S     TR   UR     S                    
                     T       USS       U                    
                     R    STR   URT    R                    
                     U TRU         SUR T                    
                     RSSTRUSTRUSTRUSTRTS                    
```
*3D pyramid rotating with text on its 4 faces and base.*

## Development

### Project Structure

```
augusto/
├── augusto/              # Main Rust project
│   ├── src/
│   │   ├── main.rs       # Entry point and CLI handling
│   │   ├── anagram.rs    # Anagram generation logic
│   │   ├── ascii_art.rs  # ASCII art generation logic
│   │   ├── animation.rs  # ASCII animation logic (3D shapes, donuts, etc.)
│   │   └── benchmark.rs  # Performance benchmarking utilities
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

## Hacktoberfest 2026 🎃

augusto is participating in **Hacktoberfest 2026**! We welcome contributors of all skill levels to help improve this project.

### How to Participate

1. **Check out our [Hacktoberfest issues](https://github.com/lucasrafaldini/augusto/issues?q=is%3Aissue+is%3Aopen+label%3Ahacktoberfest)** - Look for issues labeled `hacktoberfest` and `good first issue`
2. **Comment on an issue** to claim it and ask questions
3. **Fork the repository** and create a branch for your work
4. **Make your changes** following our [contribution guidelines](#contributing)
5. **Submit a Pull Request** with the `hacktoberfest` label

### Hacktoberfest-Specific Issues for 2026

We've prepared the following issues perfect for Hacktoberfest contributions:

| Issue | Difficulty | Description |
|-------|------------|-------------|
| [Add new ASCII animation shapes](https://github.com/lucasrafaldini/augusto/issues/1) | 🟢 Beginner | Add more 3D shapes like torus, klein bottle, mobius strip |
| [Implement color support for animations](https://github.com/lucasrafaldini/augusto/issues/2) | 🟡 Intermediate | Add ANSI color codes to ASCII animations |
| [Add export to GIF/Video feature](https://github.com/lucasrafaldini/augusto/issues/3) | 🔴 Advanced | Export animations as animated GIF or MP4 |
| [Create interactive animation mode](https://github.com/lucasrafaldini/augusto/issues/4) | 🟡 Intermediate | Real-time keyboard controls for animation parameters |
| [Add word-based particle effects](https://github.com/lucasrafaldini/augusto/issues/5) | 🟢 Beginner | Make letters explode, float, or swarm based on word meaning |

### Quick Start for Hacktoberfest Contributors

```bash
# 1. Fork and clone
git clone https://github.com/YOUR_USERNAME/augusto.git
cd augusto

# 2. Build and test
cd augusto
cargo build --release
cargo test

# 3. Try the new animation feature!
cargo run -- animate donut "RUST"
cargo run -- animate cube "CODE"
cargo run -- animate sphere "HACKTOBERFEST"
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
- [x] ASCII art generation
- [x] CLI interface
- [x] Unit tests
- [x] Performance benchmarks
- [x] Documentation improvements

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
