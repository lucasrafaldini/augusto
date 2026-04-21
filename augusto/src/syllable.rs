//! Syllable splitting module
//!
//! Provides heuristic syllable splitting based on vowel/consonant group
//! patterns. The algorithm works reasonably well for English and Portuguese.
//!
//! ## Algorithm
//!
//! Characters are first grouped into consecutive runs of vowels or consonants.
//! Consonant clusters between two vowel groups are distributed using the
//! onset-maximization rule:
//!
//! - **Single consonant** between two vowel groups: it goes with the *following*
//!   vowel group (V | CV).
//! - **Two or more consonants** between two vowel groups: the first stays with
//!   the preceding vowel group, the rest go with the following one (VC | CV,
//!   VC | CCV, …).
//!
//! **Note**: This is a heuristic and may not perfectly match all linguistic
//! syllabification rules. It handles common English and Portuguese patterns
//! well but does not model language-specific diphthong or hiatus rules.

const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];

fn is_vowel(c: char) -> bool {
    VOWELS.contains(&c.to_ascii_lowercase())
}

/// Whether a group of consecutive characters is all vowels or all consonants.
#[derive(Debug, Clone, PartialEq, Eq)]
enum GroupKind {
    Vowel,
    Consonant,
}

/// A consecutive run of characters that are all vowels or all consonants.
#[derive(Debug, Clone)]
struct CharGroup {
    kind: GroupKind,
    /// Inclusive start index (char-indexed, not byte-indexed).
    start: usize,
    /// Exclusive end index.
    end: usize,
}

/// Partitions `chars` into consecutive runs of vowels and consonants.
fn group_chars(chars: &[char]) -> Vec<CharGroup> {
    let mut groups: Vec<CharGroup> = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let vowel = is_vowel(chars[i]);
        let kind = if vowel {
            GroupKind::Vowel
        } else {
            GroupKind::Consonant
        };
        let start = i;
        while i < chars.len() && is_vowel(chars[i]) == vowel {
            i += 1;
        }
        groups.push(CharGroup { kind, start, end: i });
    }
    groups
}

/// Splits a word into syllables using a heuristic vowel/consonant group
/// approach.
///
/// # Arguments
///
/// * `word` - The input word
///
/// # Returns
///
/// A `Vec<String>` of syllables. Returns an empty vector for empty input.
///
/// # Examples
///
/// ```
/// use augusto::syllable::split_syllables;
///
/// assert_eq!(split_syllables("rust"),      vec!["rust"]);
/// assert_eq!(split_syllables("computer"),  vec!["com", "pu", "ter"]);
/// assert_eq!(split_syllables("beautiful"), vec!["beau", "ti", "ful"]);
/// ```
pub fn split_syllables(word: &str) -> Vec<String> {
    if word.is_empty() {
        return vec![];
    }

    let chars: Vec<char> = word.chars().collect();
    let n = chars.len();
    let groups = group_chars(&chars);

    // Collect syllable break positions (char indices).
    // A break at position `p` means a new syllable starts at char index `p`.
    let mut breaks: Vec<usize> = vec![0];

    for (gi, group) in groups.iter().enumerate() {
        if group.kind == GroupKind::Consonant {
            let prev_is_vowel = gi > 0 && groups[gi - 1].kind == GroupKind::Vowel;
            let next_is_vowel = gi + 1 < groups.len() && groups[gi + 1].kind == GroupKind::Vowel;

            if prev_is_vowel && next_is_vowel {
                let cluster_len = group.end - group.start;
                // Single consonant → goes with the next vowel; break before it.
                // Two or more → first consonant stays with previous vowel; break
                // after the first consonant.
                let break_pos = if cluster_len == 1 {
                    group.start
                } else {
                    group.start + 1
                };
                if break_pos > *breaks.last().unwrap_or(&0) {
                    breaks.push(break_pos);
                }
            }
        }
    }

    breaks.push(n);

    breaks
        .windows(2)
        .filter_map(|w| {
            let (start, end) = (w[0], w[1]);
            if start < end {
                Some(chars[start..end].iter().collect())
            } else {
                None
            }
        })
        .collect()
}

/// Formats a syllable analysis report for a word.
///
/// # Arguments
///
/// * `word` - The input word
///
/// # Returns
///
/// A formatted string showing the syllable breakdown and count.
pub fn analyze_syllables(word: &str) -> String {
    let syllables = split_syllables(word);
    let count = syllables.len();
    let syllable_str = syllables.join("-");
    format!(
        "Word:      {}\nSyllables: {} ({} {})",
        word,
        syllable_str,
        count,
        if count == 1 { "syllable" } else { "syllables" }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_syllable() {
        assert_eq!(split_syllables("rust"), vec!["rust"]);
        assert_eq!(split_syllables("cat"), vec!["cat"]);
        assert_eq!(split_syllables("a"), vec!["a"]);
    }

    #[test]
    fn test_two_syllables() {
        assert_eq!(split_syllables("open"), vec!["o", "pen"]);
        assert_eq!(split_syllables("table"), vec!["tab", "le"]);
    }

    #[test]
    fn test_three_syllables() {
        assert_eq!(split_syllables("computer"), vec!["com", "pu", "ter"]);
        assert_eq!(split_syllables("beautiful"), vec!["beau", "ti", "ful"]);
    }

    #[test]
    fn test_empty() {
        assert!(split_syllables("").is_empty());
    }

    #[test]
    fn test_analyze_syllables_format() {
        let result = analyze_syllables("computer");
        assert!(result.contains("com-pu-ter"));
        assert!(result.contains("3 syllables"));
    }

    #[test]
    fn test_single_vowel_word() {
        assert_eq!(split_syllables("a"), vec!["a"]);
    }

    #[test]
    fn test_program() {
        let syllables = split_syllables("program");
        assert_eq!(syllables.len(), 2);
        assert_eq!(syllables, vec!["prog", "ram"]);
    }

    #[test]
    fn test_analyze_syllables_singular() {
        let result = analyze_syllables("cat");
        assert!(result.contains("1 syllable"));
        assert!(!result.contains("syllables"));
    }
}
