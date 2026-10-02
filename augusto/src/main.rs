//! Augusto - A command-line tool for creative word operations
//!
//! Inspired by Brazilian concrete poet Augusto de Campos, this tool provides
//! various word manipulation operations, including anagram generation and ASCII art.
//!
//! # Usage
//!
//! ```bash
//! # Generate anagrams
//! augusto anagram "word"
//!
//! # Create ASCII art
//! augusto art "WORD" "filler"
//!
//! # Benchmark performance
//! augusto bench anagram "word"
//! ```

use std::{collections::HashSet, env};
mod anagram;
mod animation;
mod ascii_art;
mod benchmark;

/// Main entry point for the augusto CLI tool
///
/// # Commands
///
/// - `anagram <word>` - Generate all anagrams of a word
/// - `art <main_word> <filler_word>` - Create ASCII art using one word to fill another
/// - `bench <operation> <args...>` - Benchmark an operation and show performance stats
///
/// # Examples
///
/// ```bash
/// augusto anagram "cat"
/// augusto art "RUST" "code"
/// augusto bench anagram "test"
/// ```
fn main() {
    let args: Vec<String> = env::args().collect();

    // Show usage if no argument provided
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    let command = &args[1].to_lowercase();

    match command.as_str() {
        "anagram" | "ana" => {
            if args.len() < 3 {
                eprintln!("Error: Missing word for anagram generation");
                eprintln!("\nUsage: augusto anagram <word>");
                eprintln!("Example: augusto anagram \"cat\"");
                std::process::exit(1);
            }
            run_anagram(&args[2]);
        }
        "art" | "ascii" => {
            if args.len() < 4 {
                eprintln!("Error: Missing words for ASCII art generation");
                eprintln!("\nUsage: augusto art <main_word> <filler_word> [spacing]");
                eprintln!("Example: augusto art \"RUST\" \"code\"");
                eprintln!("         augusto art \"RUST\" \"code\" 2");
                std::process::exit(1);
            }
            let spacing = if let Some(s) = args.get(4) {
                match s.parse::<usize>() {
                    Ok(val) => val,
                    Err(_) => {
                        eprintln!("Error: Invalid spacing value '{}'. Spacing must be a non-negative integer.", s);
                        eprintln!("\nUsage: augusto art <main_word> <filler_word> [spacing]");
                        eprintln!("Example: augusto art \"RUST\" \"code\"");
                        eprintln!("         augusto art \"RUST\" \"code\" 2");
                        std::process::exit(1);
                    }
                }
            } else {
                0
            };
            run_ascii_art(&args[2], &args[3], spacing);
        }
        "bench" | "benchmark" | "perf" => {
            if args.len() < 3 {
                eprintln!("Error: Missing operation to benchmark");
                eprintln!("\nUsage: augusto bench <operation> <args...>");
                eprintln!("Example: augusto bench anagram \"test\"");
                eprintln!("         augusto bench art \"HI\" \"rust\"");
                std::process::exit(1);
            }
            run_benchmark(&args[2..]);
        }
        "compare" | "comp" => {
            if args.len() < 3 {
                eprintln!("Error: Missing words for benchmark comparison");
                eprintln!("\nUsage: augusto compare <word1> <word2> ...");
                eprintln!("Example: augusto compare \"cat\" \"test\" \"program\"");
                std::process::exit(1);
            }
            run_comparison(&args[2..]);
        }
        "animate" | "anim" => {
            if args.len() < 4 {
                eprintln!("Error: Missing shape and word for animation");
                eprintln!("\nUsage: augusto animate <shape> <word> [options]");
                eprintln!("Shapes: donut, cube, cube5d, sphere, mandala, pyramid");
                eprintln!("Example: augusto animate donut \"RUST\"");
                eprintln!("         augusto animate cube \"CODE\" --speed 50");
                eprintln!("         augusto animate sphere \"HACK\" --color --filler rust");
                std::process::exit(1);
            }
            run_animation(&args[2..]);
        }
        "help" | "--help" | "-h" => {
            print_usage();
        }
        _ => {
            // For backwards compatibility, if no command is recognized, try as anagram
            if args.len() == 2 {
                run_anagram(&args[1]);
            } else {
                eprintln!("Error: Unknown command '{}'", command);
                print_usage();
                std::process::exit(1);
            }
        }
    }
}

/// Display usage information
fn print_usage() {
    println!("Augusto - Creative word operations inspired by concrete poetry");
    println!();
    println!("USAGE:");
    println!("    augusto <command> [arguments]");
    println!();
    println!("COMMANDS:");
    println!("    anagram <word>                      Generate all anagrams of a word");
    println!("    art <main> <filler> [spacing]       Create ASCII art (optional spacing)");
    println!("    animate <shape> <word> [options]    Create ASCII animations (NEW!)");
    println!("    bench <operation> <args...>         Benchmark an operation with stats");
    println!("    compare <word1> <word2> ...         Compare anagram performance");
    println!("    help                                Show this help message");
    println!();
    println!("ANIMATION SHAPES:");
    println!("    donut       Rotating torus/donut");
    println!("    cube        Rotating 3D wireframe cube");
    println!("    cube5d      4D hypercube (tesseract) projection");
    println!("    sphere      Rotating wireframe sphere");
    println!("    mandala     Hypnotic rotational symmetry pattern");
    println!("    pyramid     Rotating 3D pyramid");
    println!();
    println!("ANIMATION OPTIONS:");
    println!("    --speed <ms>      Frame delay in milliseconds (default: 100)");
    println!("    --frames <n>      Number of frames to render (default: infinite)");
    println!("    --color           Enable ANSI color output");
    println!("    --filler <word>   Word to use as filler characters");
    println!("    --width <n>       Terminal width (default: 80)");
    println!("    --height <n>      Terminal height (default: 40)");
    println!();
    println!("EXAMPLES:");
    println!("    augusto anagram \"cat\"");
    println!("    augusto art \"RUST\" \"code\"");
    println!("    augusto art \"RUST\" \"code\" 2");
    println!("    augusto animate donut \"RUST\"");
    println!("    augusto animate cube \"CODE\" --speed 50");
    println!("    augusto animate cube5d \"HACKTOBERFEST\" --color");
    println!("    augusto animate sphere \"RUST\" --filler code");
    println!("    augusto animate mandala \"PEACE\" --speed 80");
    println!("    augusto animate pyramid \"RUST\" --frames 50");
    println!("    augusto bench anagram \"test\"");
    println!("    augusto bench art \"HI\" \"rust\"");
    println!("    augusto compare \"cat\" \"test\" \"program\"");
    println!();
    println!("For backwards compatibility, you can also use:");
    println!("    augusto <word>                  (same as 'anagram' command)");
}

/// Run anagram generation
fn run_anagram(input: &str) {
    // Validate input
    if input.is_empty() {
        eprintln!("Error: Input word cannot be empty");
        std::process::exit(1);
    }

    // Generate anagrams
    let result: Vec<String> = anagram::letter_combinations(input);

    // Remove duplicates by collecting into a HashSet
    let unique_anagrams: HashSet<_> = result.into_iter().collect();

    println!("{:?}", unique_anagrams);
}

/// Run ASCII art generation
fn run_ascii_art(main_word: &str, filler_word: &str, spacing: usize) {
    // Validate input
    if main_word.is_empty() {
        eprintln!("Error: Main word cannot be empty");
        std::process::exit(1);
    }
    if filler_word.is_empty() {
        eprintln!("Error: Filler word cannot be empty");
        std::process::exit(1);
    }

    // Generate ASCII art; use word_art for default spacing (0 or 1)
    let art = if spacing == 0 || spacing == 1 {
        ascii_art::word_art(main_word, filler_word)
    } else {
        ascii_art::word_art_with_spacing(main_word, filler_word, spacing)
    };
    println!("{}", art);
}

/// Run benchmark for an operation
fn run_benchmark(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: No operation specified for benchmark");
        std::process::exit(1);
    }

    let operation = &args[0].to_lowercase();

    match operation.as_str() {
        "anagram" | "ana" => {
            if args.len() < 2 {
                eprintln!("Error: Missing word for anagram benchmark");
                eprintln!("\nUsage: augusto bench anagram <word>");
                std::process::exit(1);
            }

            let input = &args[1];

            // Benchmark anagram generation
            let stats = benchmark::benchmark_with_result("Anagram Generation", input, || {
                anagram::letter_combinations(input)
            });

            println!("{}", stats);
        }
        "art" | "ascii" => {
            if args.len() < 3 {
                eprintln!("Error: Missing words for ASCII art benchmark");
                eprintln!("\nUsage: augusto bench art <main_word> <filler_word>");
                std::process::exit(1);
            }

            let main_word = &args[1];
            let filler_word = &args[2];

            // Benchmark ASCII art generation
            let stats = benchmark::benchmark_operation(
                "ASCII Art Generation",
                &format!("{}+{}", main_word, filler_word),
                || ascii_art::word_art(main_word, filler_word),
            );

            println!("{}", stats);
        }
        _ => {
            eprintln!("Error: Unknown operation '{}' for benchmark", operation);
            eprintln!("\nSupported operations:");
            eprintln!("  - anagram <word>");
            eprintln!("  - art <main_word> <filler_word>");
            std::process::exit(1);
        }
    }
}

/// Run comparison of multiple anagram operations
fn run_comparison(words: &[String]) {
    if words.is_empty() {
        eprintln!("Error: No words provided for comparison");
        std::process::exit(1);
    }

    let mut suite = benchmark::BenchmarkSuite::new();

    for word in words {
        let stats = benchmark::benchmark_with_result("Anagram Generation", word, || {
            anagram::letter_combinations(word)
        });
        suite.add(stats);
    }

    println!("{}", suite.format_comparison());
}

/// Run ASCII animation
fn run_animation(args: &[String]) {
    if args.len() < 2 {
        eprintln!("Error: Missing shape and word for animation");
        eprintln!("\nUsage: augusto animate <shape> <word> [options]");
        eprintln!("Shapes: donut, cube, cube5d, sphere, mandala, pyramid");
        std::process::exit(1);
    }

    let shape_str = &args[0];
    let word = &args[1];

    // Parse options
    let mut speed_ms = 100u64;
    let mut frames: Option<usize> = None;
    let mut color = false;
    let mut filler: Option<String> = None;
    let mut width = 80usize;
    let mut height = 40usize;

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--speed" => {
                if i + 1 < args.len() {
                    if let Ok(val) = args[i + 1].parse::<u64>() {
                        speed_ms = val;
                    } else {
                        eprintln!("Error: Invalid speed value '{}'", args[i + 1]);
                        std::process::exit(1);
                    }
                    i += 2;
                } else {
                    eprintln!("Error: --speed requires a value");
                    std::process::exit(1);
                }
            }
            "--frames" => {
                if i + 1 < args.len() {
                    if let Ok(val) = args[i + 1].parse::<usize>() {
                        frames = Some(val);
                    } else {
                        eprintln!("Error: Invalid frames value '{}'", args[i + 1]);
                        std::process::exit(1);
                    }
                    i += 2;
                } else {
                    eprintln!("Error: --frames requires a value");
                    std::process::exit(1);
                }
            }
            "--color" => {
                color = true;
                i += 1;
            }
            "--filler" => {
                if i + 1 < args.len() {
                    filler = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --filler requires a value");
                    std::process::exit(1);
                }
            }
            "--width" => {
                if i + 1 < args.len() {
                    if let Ok(val) = args[i + 1].parse::<usize>() {
                        width = val;
                    } else {
                        eprintln!("Error: Invalid width value '{}'", args[i + 1]);
                        std::process::exit(1);
                    }
                    i += 2;
                } else {
                    eprintln!("Error: --width requires a value");
                    std::process::exit(1);
                }
            }
            "--height" => {
                if i + 1 < args.len() {
                    if let Ok(val) = args[i + 1].parse::<usize>() {
                        height = val;
                    } else {
                        eprintln!("Error: Invalid height value '{}'", args[i + 1]);
                        std::process::exit(1);
                    }
                    i += 2;
                } else {
                    eprintln!("Error: --height requires a value");
                    std::process::exit(1);
                }
            }
            _ => {
                eprintln!("Error: Unknown option '{}'", args[i]);
                eprintln!(
                    "Supported options: --speed, --frames, --color, --filler, --width, --height"
                );
                std::process::exit(1);
            }
        }
    }

    let shape = match animation::AnimationShape::from_str(shape_str) {
        Some(s) => s,
        None => {
            eprintln!("Error: Unknown shape '{}'", shape_str);
            eprintln!("Available shapes: donut, cube, cube5d, sphere, mandala, pyramid");
            std::process::exit(1);
        }
    };

    let config = animation::AnimationConfig {
        shape,
        word: word.clone(),
        filler,
        speed_ms,
        frames,
        color,
        width,
        height,
    };

    animation::animate(config);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_anagram_combinations() {
        let input = "aba";
        let result = anagram::letter_combinations(input);

        // Define the expected set of combinations as owned String values
        let expected: HashSet<String> =
            vec!["aab".to_string(), "baa".to_string(), "aba".to_string()]
                .into_iter()
                .collect();

        // Convert the result into a HashSet for comparison
        let result_set: HashSet<String> = result.into_iter().collect();

        assert_eq!(result_set, expected);
    }

    #[test]
    fn test_single_char() {
        let result = anagram::letter_combinations("a");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "a");
    }

    #[test]
    fn test_two_chars() {
        let result = anagram::letter_combinations("ab");
        let result_set: HashSet<String> = result.into_iter().collect();
        assert_eq!(result_set.len(), 2);
        assert!(result_set.contains("ab"));
        assert!(result_set.contains("ba"));
    }

    #[test]
    fn test_ascii_art_generation() {
        let art = ascii_art::word_art("A", "x");
        assert!(!art.is_empty());
        assert!(art.contains('x'));
    }

    #[test]
    fn test_ascii_art_with_word() {
        let art = ascii_art::word_art("HI", "rust");
        assert!(!art.is_empty());
        // Should contain characters from filler word
        let has_filler =
            art.contains('r') || art.contains('u') || art.contains('s') || art.contains('t');
        assert!(has_filler);
    }
}
