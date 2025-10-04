//! ASCII art generation module
//!
//! This module provides functionality for creating ASCII art where one word
//! is written using another word as filler characters, inspired by concrete poetry.
//!
//! # Examples
//!
//! ```
//! use augusto::ascii_art::word_art;
//!
//! let result = word_art("HI", "rust");
//! println!("{}", result);
//! ```

use std::collections::HashMap;

/// ASCII art letter patterns using a 5x5 grid
/// Each letter is represented as a vector of strings
fn get_letter_pattern(letter: char) -> Vec<String> {
    let patterns: HashMap<char, Vec<&str>> = [
        ('A', vec![" ### ", "#   #", "#####", "#   #", "#   #"]),
        ('B', vec!["#### ", "#   #", "#### ", "#   #", "#### "]),
        ('C', vec![" ####", "#    ", "#    ", "#    ", " ####"]),
        ('D', vec!["#### ", "#   #", "#   #", "#   #", "#### "]),
        ('E', vec!["#####", "#    ", "#### ", "#    ", "#####"]),
        ('F', vec!["#####", "#    ", "#### ", "#    ", "#    "]),
        ('G', vec![" ####", "#    ", "#  ##", "#   #", " ####"]),
        ('H', vec!["#   #", "#   #", "#####", "#   #", "#   #"]),
        ('I', vec!["#####", "  #  ", "  #  ", "  #  ", "#####"]),
        ('J', vec!["#####", "   # ", "   # ", "#  # ", " ##  "]),
        ('K', vec!["#   #", "#  # ", "###  ", "#  # ", "#   #"]),
        ('L', vec!["#    ", "#    ", "#    ", "#    ", "#####"]),
        ('M', vec!["#   #", "## ##", "# # #", "#   #", "#   #"]),
        ('N', vec!["#   #", "##  #", "# # #", "#  ##", "#   #"]),
        ('O', vec![" ### ", "#   #", "#   #", "#   #", " ### "]),
        ('P', vec!["#### ", "#   #", "#### ", "#    ", "#    "]),
        ('Q', vec![" ### ", "#   #", "#   #", "#  ##", " ####"]),
        ('R', vec!["#### ", "#   #", "#### ", "#  # ", "#   #"]),
        ('S', vec![" ####", "#    ", " ### ", "    #", "#### "]),
        ('T', vec!["#####", "  #  ", "  #  ", "  #  ", "  #  "]),
        ('U', vec!["#   #", "#   #", "#   #", "#   #", " ### "]),
        ('V', vec!["#   #", "#   #", "#   #", " # # ", "  #  "]),
        ('W', vec!["#   #", "#   #", "# # #", "## ##", "#   #"]),
        ('X', vec!["#   #", " # # ", "  #  ", " # # ", "#   #"]),
        ('Y', vec!["#   #", " # # ", "  #  ", "  #  ", "  #  "]),
        ('Z', vec!["#####", "   # ", "  #  ", " #   ", "#####"]),
    ]
    .iter()
    .cloned()
    .collect();

    patterns
        .get(&letter.to_ascii_uppercase())
        .map(|lines| lines.iter().map(|s| s.to_string()).collect())
        .unwrap_or_else(|| {
            vec![
                "     ".to_string(),
                "     ".to_string(),
                "     ".to_string(),
                "     ".to_string(),
                "     ".to_string(),
            ]
        })
}

/// Creates ASCII art of a word using another word as filler
///
/// This function takes two words: the main word to display in large ASCII art,
/// and a filler word whose characters will be used to fill the pattern.
///
/// # Arguments
///
/// * `main_word` - The word to display in ASCII art
/// * `filler_word` - The word to use as filler characters
///
/// # Returns
///
/// A `String` containing the ASCII art representation
///
/// # Examples
///
/// ```
/// use augusto::ascii_art::word_art;
///
/// let art = word_art("RUST", "code");
/// println!("{}", art);
/// ```
///
/// # Features
///
/// - Converts letters to uppercase automatically
/// - Uses the filler word cyclically if the pattern needs more characters
/// - Each letter is displayed using a 5x5 character grid
/// - Letters are separated by one space
pub fn word_art(main_word: &str, filler_word: &str) -> String {
    if main_word.is_empty() || filler_word.is_empty() {
        return String::new();
    }

    let main_chars: Vec<char> = main_word.chars().collect();
    let filler_chars: Vec<char> = filler_word.chars().collect();
    let mut filler_index = 0;

    // Get patterns for all letters
    let patterns: Vec<Vec<String>> = main_chars.iter().map(|&c| get_letter_pattern(c)).collect();

    // Build the output line by line
    let mut output = String::new();

    for line_num in 0..5 {
        let mut line = String::new();

        for (letter_idx, pattern) in patterns.iter().enumerate() {
            let pattern_line = &pattern[line_num];

            // Replace each '#' with characters from filler word
            for ch in pattern_line.chars() {
                if ch == '#' {
                    line.push(filler_chars[filler_index % filler_chars.len()]);
                    filler_index += 1;
                } else {
                    line.push(' ');
                }
            }

            // Add spacing between letters (except for the last one)
            if letter_idx < patterns.len() - 1 {
                line.push(' ');
            }
        }

        output.push_str(&line);
        output.push('\n');
    }

    output
}

/// Creates ASCII art with custom styling options
///
/// This is an enhanced version that allows for more control over the output.
///
/// # Arguments
///
/// * `main_word` - The word to display in ASCII art
/// * `filler_word` - The word to use as filler characters
/// * `spacing` - Number of spaces between letters
///
/// # Returns
///
/// A `String` containing the ASCII art representation
pub fn word_art_with_spacing(main_word: &str, filler_word: &str, spacing: usize) -> String {
    if main_word.is_empty() || filler_word.is_empty() {
        return String::new();
    }

    let main_chars: Vec<char> = main_word.chars().collect();
    let filler_chars: Vec<char> = filler_word.chars().collect();
    let mut filler_index = 0;

    let patterns: Vec<Vec<String>> = main_chars.iter().map(|&c| get_letter_pattern(c)).collect();

    let mut output = String::new();

    for line_num in 0..5 {
        let mut line = String::new();

        for (letter_idx, pattern) in patterns.iter().enumerate() {
            let pattern_line = &pattern[line_num];

            for ch in pattern_line.chars() {
                if ch == '#' {
                    line.push(filler_chars[filler_index % filler_chars.len()]);
                    filler_index += 1;
                } else {
                    line.push(' ');
                }
            }

            if letter_idx < patterns.len() - 1 {
                line.push_str(&" ".repeat(spacing));
            }
        }

        output.push_str(&line);
        output.push('\n');
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_art_basic() {
        let result = word_art("A", "x");
        assert!(!result.is_empty());
        assert!(result.contains('x'));
    }

    #[test]
    fn test_word_art_with_multiple_letters() {
        let result = word_art("HI", "rust");
        assert!(!result.is_empty());
        // Check that filler characters are used
        assert!(
            result.contains('r')
                || result.contains('u')
                || result.contains('s')
                || result.contains('t')
        );
    }

    #[test]
    fn test_word_art_empty_input() {
        let result = word_art("", "test");
        assert!(result.is_empty());

        let result = word_art("test", "");
        assert!(result.is_empty());
    }

    #[test]
    fn test_filler_cycles() {
        let result = word_art("AAA", "x");
        // With lots of letters, filler should repeat
        let x_count = result.chars().filter(|&c| c == 'x').count();
        assert!(x_count > 1);
    }

    #[test]
    fn test_word_art_with_spacing() {
        let result1 = word_art_with_spacing("HI", "rust", 1);
        let result2 = word_art_with_spacing("HI", "rust", 3);

        // Result with more spacing should be longer
        assert!(result2.len() > result1.len());
    }

    #[test]
    fn test_luxo_lixo() {
        // Tribute to Augusto de Campos' iconic concrete poem LUXO/LIXO
        // "LUXO" (luxury) written with "LIXO" (trash) - a powerful social commentary
        let result = word_art("LUXO", "LIXO");
        
        assert!(!result.is_empty());
        
        // Check that the output contains letters from LIXO
        assert!(result.contains('L') || result.contains('I') || 
                result.contains('X') || result.contains('O'));
        
        // Verify the result has 5 lines (one for each row in the 5x5 grid)
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 5);
        
        // Each line should have content (not be empty)
        for line in lines {
            assert!(!line.trim().is_empty());
        }
    }
}
