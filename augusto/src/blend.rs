//! Word blending (portmanteau) module
//!
//! Creates portmanteau words by combining the prefix of one word with the
//! suffix of another, in the tradition of Lewis Carroll's *blending*
//! (e.g., "smoke" + "fog" → "smog").
//!
//! ## Algorithm
//!
//! For two input words A and B, every blend of the form
//! `A[0..i] + B[j..]` and `B[0..i] + A[j..]` is generated for all valid
//! split indices i and j (at least one character from each word).
//!
//! Results are deduplicated (case-insensitively), filtered to exclude the
//! original inputs, and sorted by how close each blend's length is to the
//! average length of the two inputs — favouring "balanced" blends that
//! contribute roughly equally from both words.

use std::collections::HashSet;

/// Generates all portmanteau blends of two words.
///
/// Each blend is formed by taking a prefix of one word and appending a suffix
/// of the other. Both orderings (A-prefix + B-suffix and B-prefix + A-suffix)
/// are produced.
///
/// # Arguments
///
/// * `word_a` - First input word
/// * `word_b` - Second input word
///
/// # Returns
///
/// A deduplicated `Vec<String>` of blends, sorted so that blends whose length
/// is closest to the average of the two input lengths appear first.
///
/// # Examples
///
/// ```
/// use augusto::blend::blend_words;
///
/// let blends = blend_words("smoke", "fog");
/// // "smog" = prefix "sm" from "smoke" + suffix "og" from "fog"
/// assert!(blends.contains(&"smog".to_string()));
/// ```
pub fn blend_words(word_a: &str, word_b: &str) -> Vec<String> {
    let a: Vec<char> = word_a.chars().collect();
    let b: Vec<char> = word_b.chars().collect();
    let a_lower = word_a.to_lowercase();
    let b_lower = word_b.to_lowercase();

    let mut seen: HashSet<String> = HashSet::new();
    let mut blends: Vec<String> = Vec::new();

    // Helper: attempt to add a blend if it is new and not an original word.
    // `seen` stores lowercase versions to provide case-insensitive deduplication
    // while `blends` keeps the blend in its original generated casing.
    let mut try_add = |blend: String| {
        let blend_lower = blend.to_lowercase();
        if blend_lower != a_lower && blend_lower != b_lower && seen.insert(blend_lower) {
            blends.push(blend);
        }
    };

    // prefix of A (length i) + suffix of B (starting at j)
    for i in 1..a.len() {
        for j in 1..b.len() {
            let blend: String = a[..i].iter().chain(b[j..].iter()).collect();
            try_add(blend);
        }
    }

    // prefix of B (length i) + suffix of A (starting at j)
    for i in 1..b.len() {
        for j in 1..a.len() {
            let blend: String = b[..i].iter().chain(a[j..].iter()).collect();
            try_add(blend);
        }
    }

    // Sort by distance from the ideal blend length (average of both inputs).
    // Blends closest to the midpoint length are considered most "balanced".
    let ideal = (a.len() + b.len()) / 2;
    blends.sort_by_key(|s| {
        let len = s.chars().count();
        (len as isize - ideal as isize).unsigned_abs()
    });

    blends
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_blend_smoke_fog() {
        let blends = blend_words("smoke", "fog");
        // "smog" = "sm" (from smoke) + "og" (from fog)
        assert!(
            blends.contains(&"smog".to_string()),
            "Expected 'smog' in blends, got: {:?}",
            blends
        );
    }

    #[test]
    fn test_blend_is_deduplicated() {
        let blends = blend_words("ab", "cd");
        let unique: HashSet<&String> = blends.iter().collect();
        assert_eq!(blends.len(), unique.len(), "Blend list should have no duplicates");
    }

    #[test]
    fn test_blend_excludes_originals() {
        let blends = blend_words("hello", "world");
        assert!(!blends.contains(&"hello".to_string()));
        assert!(!blends.contains(&"world".to_string()));
    }

    #[test]
    fn test_blend_returns_results() {
        let blends = blend_words("breakfast", "lunch");
        assert!(!blends.is_empty(), "Should produce at least one blend");
    }

    #[test]
    fn test_blend_single_char_words() {
        // Single-char words have no valid interior split point, so no blends.
        let blends = blend_words("a", "b");
        assert!(blends.is_empty());
    }

    #[test]
    fn test_blend_case_insensitive_dedup() {
        let blends = blend_words("Smoke", "Fog");
        // Lowercase versions of all blends should be unique.
        let lower: Vec<String> = blends.iter().map(|s| s.to_lowercase()).collect();
        let unique: HashSet<&String> = lower.iter().collect();
        assert_eq!(lower.len(), unique.len(), "Case-insensitive duplicates found");
    }

    #[test]
    fn test_blend_sorted_by_balance() {
        let blends = blend_words("smoke", "fog");
        // The ideal length is (5 + 3) / 2 = 4.
        // All blends should be sorted by |len - 4|.
        let ideal = (5 + 3) / 2_usize;
        let mut prev_dist = 0usize;
        for blend in &blends {
            let len = blend.chars().count();
            let dist = (len as isize - ideal as isize).unsigned_abs();
            assert!(dist >= prev_dist, "Blends are not sorted by balance");
            prev_dist = dist;
        }
    }
}
