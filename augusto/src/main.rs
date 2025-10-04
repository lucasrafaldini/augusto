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
    println!("    bench <operation> <args...>         Benchmark an operation with stats");
    println!("    compare <word1> <word2> ...         Compare anagram performance");
    println!("    help                                Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("    augusto anagram \"cat\"");
    println!("    augusto art \"RUST\" \"code\"");
    println!("    augusto art \"RUST\" \"code\" 2");
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

    // Generate ASCII art with spacing if specified
    let art = if spacing > 0 {
        ascii_art::word_art_with_spacing(main_word, filler_word, spacing)
    } else {
        ascii_art::word_art(main_word, filler_word)
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
                || {
                    ascii_art::word_art(main_word, filler_word)
                },
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
