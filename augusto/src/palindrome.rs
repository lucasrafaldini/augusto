//! Palindrome analysis module
//!
//! Provides palindrome detection, word mirroring, and longest palindromic
//! substring discovery using the expand-around-center algorithm.

/// Returns `true` if the word is a palindrome.
///
/// The comparison is case-insensitive and considers only alphabetic characters,
/// so punctuation and spaces are ignored.
///
/// # Arguments
///
/// * `word` - The input word
///
/// # Examples
///
/// ```
/// use augusto::palindrome::is_palindrome;
///
/// assert!(is_palindrome("racecar"));
/// assert!(is_palindrome("Ana"));
/// assert!(!is_palindrome("hello"));
/// ```
pub fn is_palindrome(word: &str) -> bool {
    let normalized: Vec<char> = word
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    normalized.iter().eq(normalized.iter().rev())
}

/// Returns the mirror (character-by-character reverse) of a word.
///
/// # Arguments
///
/// * `word` - The input word
///
/// # Examples
///
/// ```
/// use augusto::palindrome::mirror;
///
/// assert_eq!(mirror("rust"), "tsur");
/// assert_eq!(mirror("hello"), "olleh");
/// ```
pub fn mirror(word: &str) -> String {
    word.chars().rev().collect()
}

/// Expands around a center point in `chars` and returns `(start, length)`
/// of the longest palindromic substring centered there.
///
/// Both odd-length (single center) and even-length (two-element center)
/// expansions are supported by passing different `left_start`/`right_start`.
///
/// Returns `(0, 0)` when the initial characters do not match.
fn expand_palindrome(chars: &[char], left_start: usize, right_start: usize) -> (usize, usize) {
    let n = chars.len();
    let mut left = left_start as isize;
    let mut right = right_start;
    let mut best_start = left_start;
    let mut best_len = 0usize;

    while left >= 0
        && right < n
        && chars[left as usize].eq_ignore_ascii_case(&chars[right])
    {
        let len = right - left as usize + 1;
        if len > best_len {
            best_start = left as usize;
            best_len = len;
        }
        left -= 1;
        right += 1;
    }
    (best_start, best_len)
}

/// Finds the longest palindromic substring using the expand-around-center
/// algorithm.
///
/// The comparison is case-insensitive. When multiple substrings share the
/// maximum length the leftmost one is returned.
///
/// Time complexity: O(n²)
/// Space complexity: O(n) for the character buffer
///
/// # Arguments
///
/// * `word` - The input string to search
///
/// # Returns
///
/// A `&str` slice into `word` at the longest palindromic substring.
///
/// # Examples
///
/// ```
/// use augusto::palindrome::longest_palindromic_substring;
///
/// assert_eq!(longest_palindromic_substring("babad"), "bab");
/// assert_eq!(longest_palindromic_substring("racecar"), "racecar");
/// assert_eq!(longest_palindromic_substring("cbbd"), "bb");
/// ```
pub fn longest_palindromic_substring(word: &str) -> &str {
    if word.is_empty() {
        return word;
    }

    let chars: Vec<char> = word.chars().collect();
    let n = chars.len();
    let mut best_start = 0usize;
    let mut best_len = 1usize;

    for center in 0..n {
        // Odd-length palindromes centered at `center`
        let (s, l) = expand_palindrome(&chars, center, center);
        if l > best_len {
            best_start = s;
            best_len = l;
        }

        // Even-length palindromes centered between `center` and `center + 1`
        if center + 1 < n {
            let (s, l) = expand_palindrome(&chars, center, center + 1);
            if l > best_len {
                best_start = s;
                best_len = l;
            }
        }
    }

    // Map char-indexed bounds back to byte offsets for the final slice.
    // All patterns are ASCII so the char boundary aligns with byte boundary,
    // but we use char_indices for correctness with any Unicode input.
    let byte_start = word
        .char_indices()
        .nth(best_start)
        .map(|(i, _)| i)
        .unwrap_or(0);
    let byte_end = word
        .char_indices()
        .nth(best_start + best_len)
        .map(|(i, _)| i)
        .unwrap_or(word.len());

    &word[byte_start..byte_end]
}

/// Formats a complete palindrome analysis report for a word.
///
/// # Arguments
///
/// * `word` - The input word
///
/// # Returns
///
/// A formatted multi-line string with palindrome status, mirror, and the
/// longest palindromic substring.
pub fn analyze_palindrome(word: &str) -> String {
    let is_pal = is_palindrome(word);
    let reversed = mirror(word);
    let longest = longest_palindromic_substring(word);

    let mut output = format!("Word:             {}\n", word);
    output.push_str(&format!(
        "Is palindrome:    {}\n",
        if is_pal { "Yes ✓" } else { "No" }
    ));
    output.push_str(&format!("Mirror (reverse): {}\n", reversed));
    if word.len() > 1 {
        output.push_str(&format!(
            "Longest palindromic substring: \"{}\"",
            longest
        ));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_true() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("level"));
        assert!(is_palindrome("Ana"));
        assert!(is_palindrome("madam"));
        assert!(is_palindrome("arara")); // Portuguese: macaw
    }

    #[test]
    fn test_is_palindrome_false() {
        assert!(!is_palindrome("hello"));
        assert!(!is_palindrome("rust"));
        assert!(!is_palindrome("word"));
    }

    #[test]
    fn test_is_palindrome_single_char() {
        assert!(is_palindrome("a"));
        assert!(is_palindrome("Z"));
    }

    #[test]
    fn test_is_palindrome_empty() {
        assert!(is_palindrome(""));
    }

    #[test]
    fn test_mirror() {
        assert_eq!(mirror("rust"), "tsur");
        assert_eq!(mirror("hello"), "olleh");
        assert_eq!(mirror("a"), "a");
        assert_eq!(mirror(""), "");
    }

    #[test]
    fn test_longest_palindromic_substring_odd() {
        assert_eq!(longest_palindromic_substring("babad"), "bab");
        assert_eq!(longest_palindromic_substring("racecar"), "racecar");
    }

    #[test]
    fn test_longest_palindromic_substring_even() {
        assert_eq!(longest_palindromic_substring("cbbd"), "bb");
    }

    #[test]
    fn test_longest_palindromic_substring_single() {
        assert_eq!(longest_palindromic_substring("a"), "a");
    }

    #[test]
    fn test_longest_palindromic_substring_empty() {
        assert_eq!(longest_palindromic_substring(""), "");
    }

    #[test]
    fn test_analyze_palindrome_is_palindrome() {
        let result = analyze_palindrome("racecar");
        assert!(result.contains("Yes ✓"));
        assert!(result.contains("racecar"));
    }

    #[test]
    fn test_analyze_palindrome_not_palindrome() {
        let result = analyze_palindrome("hello");
        assert!(result.contains("No"));
        assert!(result.contains("olleh"));
    }

    #[test]
    fn test_expand_palindrome_no_match() {
        let chars: Vec<char> = "ab".chars().collect();
        let (_, l) = expand_palindrome(&chars, 0, 1);
        assert_eq!(l, 0);
    }
}
