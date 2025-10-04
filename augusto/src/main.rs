//! Augusto - A command-line tool for creative word operations
//!
//! Inspired by Brazilian concrete poet Augusto de Campos, this tool provides
//! various word manipulation operations, starting with anagram generation.
//!
//! # Usage
//!
//! ```bash
//! augusto "word"
//! ```

use std::{collections::HashSet, env};
mod anagram;

/// Main entry point for the augusto CLI tool
///
/// # Arguments
///
/// Expects a single command-line argument: the word to generate anagrams for
///
/// # Examples
///
/// ```bash
/// augusto "cat"
/// ```
fn main() {
    let args: Vec<String> = env::args().collect();

    // Show usage if no argument provided
    if args.len() < 2 {
        eprintln!("Usage: augusto <word>");
        eprintln!("Generate all anagrams of the provided word.");
        eprintln!("\nExample: augusto \"cat\"");
        std::process::exit(1);
    }

    let input: String = args[1].clone();

    // Validate input
    if input.is_empty() {
        eprintln!("Error: Input word cannot be empty");
        std::process::exit(1);
    }

    // Generate anagrams
    let result: Vec<String> = anagram::letter_combinations(&input);

    // Remove duplicates by collecting into a HashSet
    let unique_anagrams: HashSet<_> = result.into_iter().collect();

    println!("{:?}", unique_anagrams);
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
        let expected: HashSet<String> = vec![
            "aab".to_string(),
            "baa".to_string(),
            "aba".to_string(),
            // ... (add more expected combinations as needed)
        ]
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
}
