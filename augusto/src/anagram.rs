//! Anagram generation module
//!
//! This module provides functionality for generating all possible permutations
//! (anagrams) of a given input string.
//!
//! # Examples
//!
//! ```
//! use augusto::letter_combinations;
//!
//! let result = letter_combinations("cat");
//! assert_eq!(result.len(), 6); // 3! = 6 permutations
//! ```

/// Generates all possible letter combinations (permutations) of the input string.
///
/// This function recursively generates all permutations by:
/// 1. Taking each character in turn
/// 2. Generating all permutations of the remaining characters
/// 3. Prepending the chosen character to each permutation
///
/// # Arguments
///
/// * `input` - A string slice containing the characters to permute
///
/// # Returns
///
/// A `Vec<String>` containing all permutations of the input string.
/// Note: This may contain duplicates if the input has repeated characters.
/// For unique results, collect into a HashSet.
///
/// # Performance
///
/// Time complexity: O(n! * n) where n is the length of the input string
/// Space complexity: O(n! * n) for storing all permutations
///
/// # Examples
///
/// ```
/// use augusto::letter_combinations;
/// use std::collections::HashSet;
///
/// let result = letter_combinations("ab");
/// let unique: HashSet<_> = result.into_iter().collect();
/// assert_eq!(unique.len(), 2); // "ab" and "ba"
/// ```
pub fn letter_combinations(input: &str) -> Vec<String> {
    if input.len() <= 1 {
        return vec![input.to_string()];
    }

    let mut result: Vec<String> = Vec::new();

    for (i, char) in input.chars().enumerate() {
        let rest = input
            .chars()
            .enumerate()
            .filter_map(|(j, c)| if i != j { Some(c) } else { None })
            .collect::<String>();

        for anagram in letter_combinations(&rest) {
            result.push(format!("{}{}", char, anagram));
        }
    }

    result
}
