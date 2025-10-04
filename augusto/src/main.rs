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
//! ```

use std::{collections::HashSet, env};
mod anagram;
mod ascii_art;

/// Main entry point for the augusto CLI tool
///
/// # Commands
///
/// - `anagram <word>` - Generate all anagrams of a word
/// - `art <main_word> <filler_word>` - Create ASCII art using one word to fill another
///
/// # Examples
///
/// ```bash
/// augusto anagram "cat"
/// augusto art "RUST" "code"
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
                eprintln!("\nUsage: augusto art <main_word> <filler_word>");
                eprintln!("Example: augusto art \"RUST\" \"code\"");
                std::process::exit(1);
            }
            run_ascii_art(&args[2], &args[3]);
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
    println!("    anagram <word>                  Generate all anagrams of a word");
    println!("    art <main> <filler>             Create ASCII art using filler word");
    println!("    help                            Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("    augusto anagram \"cat\"");
    println!("    augusto art \"RUST\" \"code\"");
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
fn run_ascii_art(main_word: &str, filler_word: &str) {
    // Validate input
    if main_word.is_empty() {
        eprintln!("Error: Main word cannot be empty");
        std::process::exit(1);
    }
    if filler_word.is_empty() {
        eprintln!("Error: Filler word cannot be empty");
        std::process::exit(1);
    }

    // Generate ASCII art
    let art = ascii_art::word_art(main_word, filler_word);
    println!("{}", art);
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
