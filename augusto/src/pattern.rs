//! Phonetic pattern analysis module
//!
//! Provides vowel/consonant pattern analysis of words, useful for rhythmic
//! and poetic analysis. Each alphabetic character is classified as a vowel
//! (`V`) or a consonant (`C`).

/// Returns the vowel/consonant pattern of a word.
///
/// Each alphabetic character is mapped to `V` (vowel) or `C` (consonant).
/// Non-alphabetic characters (spaces, hyphens, etc.) are preserved as-is.
///
/// # Arguments
///
/// * `word` - The input word
///
/// # Returns
///
/// A `String` of `V` and `C` characters representing the phonetic pattern.
///
/// # Examples
///
/// ```
/// use augusto::pattern::phonetic_pattern;
///
/// assert_eq!(phonetic_pattern("rust"), "CVCC");
/// assert_eq!(phonetic_pattern("poesia"), "CVVCVV");
/// ```
pub fn phonetic_pattern(word: &str) -> String {
    word.chars()
        .map(|c| match c.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => 'V',
            ch if ch.is_alphabetic() => 'C',
            other => other,
        })
        .collect()
}

/// Analyzes and formats the complete phonetic pattern report for a word.
///
/// # Arguments
///
/// * `word` - The input word
///
/// # Returns
///
/// A formatted multi-line string showing the pattern, vowel count, and
/// consonant count.
pub fn analyze_pattern(word: &str) -> String {
    let pattern = phonetic_pattern(word);
    let vowels = pattern.chars().filter(|&c| c == 'V').count();
    let consonants = pattern.chars().filter(|&c| c == 'C').count();
    format!(
        "Word:       {}\nPattern:    {}\nVowels:     {}\nConsonants: {}",
        word, pattern, vowels, consonants
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phonetic_pattern_basic() {
        assert_eq!(phonetic_pattern("rust"), "CVCC");
        assert_eq!(phonetic_pattern("poesia"), "CVVCVV");
        assert_eq!(phonetic_pattern("a"), "V");
        assert_eq!(phonetic_pattern("b"), "C");
    }

    #[test]
    fn test_phonetic_pattern_mixed_case() {
        assert_eq!(phonetic_pattern("Rust"), "CVCC");
        assert_eq!(phonetic_pattern("HELLO"), "CVCCV");
    }

    #[test]
    fn test_phonetic_pattern_empty() {
        assert_eq!(phonetic_pattern(""), "");
    }

    #[test]
    fn test_phonetic_pattern_preserves_non_alpha() {
        assert_eq!(phonetic_pattern("a-b"), "V-C");
        assert_eq!(phonetic_pattern("co de"), "CV CV");
    }

    #[test]
    fn test_analyze_pattern_counts() {
        let result = analyze_pattern("hello");
        assert!(result.contains("CVCCV"));
        assert!(result.contains("Vowels:     2"));
        assert!(result.contains("Consonants: 3"));
    }

    #[test]
    fn test_analyze_pattern_contains_word() {
        let result = analyze_pattern("rust");
        assert!(result.contains("rust"));
    }
}
