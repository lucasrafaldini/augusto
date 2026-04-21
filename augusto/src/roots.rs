//! Etymological roots analysis module
//!
//! Identifies Latin and Greek prefixes and suffixes in a given word and
//! reports their meaning and language of origin.
//!
//! The root database (~100 entries) is statically embedded — no external
//! dependencies or network access are required.

use std::fmt;

/// Language of origin for an etymological root.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    Latin,
    Greek,
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Origin::Latin => write!(f, "Latin"),
            Origin::Greek => write!(f, "Greek"),
        }
    }
}

/// Whether a root appears at the beginning or the end of a word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootPosition {
    Prefix,
    Suffix,
}

impl fmt::Display for RootPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RootPosition::Prefix => write!(f, "prefix"),
            RootPosition::Suffix => write!(f, "suffix"),
        }
    }
}

/// A single etymological root entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Root {
    /// Lowercase pattern to search for at the start or end of the word.
    pub pattern: &'static str,
    /// Whether this root is a prefix or a suffix.
    pub position: RootPosition,
    /// Human-readable meaning of the root.
    pub meaning: &'static str,
    /// Language of origin.
    pub origin: Origin,
}

/// A root match found in a word.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootMatch<'a> {
    /// The root entry that was matched.
    pub root: &'static Root,
    /// The exact substring of the original word that matched.
    pub matched: &'a str,
}

// ── Static root database ─────────────────────────────────────────────────────

static ROOTS: &[Root] = &[
    // ── Greek prefixes ───────────────────────────────────────────────────────
    Root { pattern: "a",      position: RootPosition::Prefix, meaning: "not, without",                  origin: Origin::Greek },
    Root { pattern: "an",     position: RootPosition::Prefix, meaning: "not, without",                  origin: Origin::Greek },
    Root { pattern: "amphi",  position: RootPosition::Prefix, meaning: "both, around",                  origin: Origin::Greek },
    Root { pattern: "ana",    position: RootPosition::Prefix, meaning: "up, again, back",               origin: Origin::Greek },
    Root { pattern: "anti",   position: RootPosition::Prefix, meaning: "against, opposite",             origin: Origin::Greek },
    Root { pattern: "apo",    position: RootPosition::Prefix, meaning: "away from, off",                origin: Origin::Greek },
    Root { pattern: "arch",   position: RootPosition::Prefix, meaning: "chief, first, primitive",       origin: Origin::Greek },
    Root { pattern: "auto",   position: RootPosition::Prefix, meaning: "self",                          origin: Origin::Greek },
    Root { pattern: "bio",    position: RootPosition::Prefix, meaning: "life",                          origin: Origin::Greek },
    Root { pattern: "chron",  position: RootPosition::Prefix, meaning: "time",                          origin: Origin::Greek },
    Root { pattern: "cosmo",  position: RootPosition::Prefix, meaning: "universe, world",               origin: Origin::Greek },
    Root { pattern: "crypto", position: RootPosition::Prefix, meaning: "hidden, secret",                origin: Origin::Greek },
    Root { pattern: "dys",    position: RootPosition::Prefix, meaning: "bad, abnormal",                 origin: Origin::Greek },
    Root { pattern: "eco",    position: RootPosition::Prefix, meaning: "house, environment",            origin: Origin::Greek },
    Root { pattern: "eu",     position: RootPosition::Prefix, meaning: "good, well",                    origin: Origin::Greek },
    Root { pattern: "geo",    position: RootPosition::Prefix, meaning: "earth",                         origin: Origin::Greek },
    Root { pattern: "hemo",   position: RootPosition::Prefix, meaning: "blood",                         origin: Origin::Greek },
    Root { pattern: "hetero", position: RootPosition::Prefix, meaning: "different, other",              origin: Origin::Greek },
    Root { pattern: "homo",   position: RootPosition::Prefix, meaning: "same",                          origin: Origin::Greek },
    Root { pattern: "hyper",  position: RootPosition::Prefix, meaning: "over, excessive",               origin: Origin::Greek },
    Root { pattern: "hypo",   position: RootPosition::Prefix, meaning: "under, below normal",           origin: Origin::Greek },
    Root { pattern: "iso",    position: RootPosition::Prefix, meaning: "equal",                         origin: Origin::Greek },
    Root { pattern: "macro",  position: RootPosition::Prefix, meaning: "large, long",                   origin: Origin::Greek },
    Root { pattern: "mega",   position: RootPosition::Prefix, meaning: "great, million",                origin: Origin::Greek },
    Root { pattern: "meta",   position: RootPosition::Prefix, meaning: "beyond, after, change",         origin: Origin::Greek },
    Root { pattern: "micro",  position: RootPosition::Prefix, meaning: "small",                         origin: Origin::Greek },
    Root { pattern: "mono",   position: RootPosition::Prefix, meaning: "one, single",                   origin: Origin::Greek },
    Root { pattern: "neo",    position: RootPosition::Prefix, meaning: "new",                           origin: Origin::Greek },
    Root { pattern: "ortho",  position: RootPosition::Prefix, meaning: "straight, correct",             origin: Origin::Greek },
    Root { pattern: "pan",    position: RootPosition::Prefix, meaning: "all",                           origin: Origin::Greek },
    Root { pattern: "para",   position: RootPosition::Prefix, meaning: "beside, beyond",                origin: Origin::Greek },
    Root { pattern: "peri",   position: RootPosition::Prefix, meaning: "around",                        origin: Origin::Greek },
    Root { pattern: "photo",  position: RootPosition::Prefix, meaning: "light",                         origin: Origin::Greek },
    Root { pattern: "poly",   position: RootPosition::Prefix, meaning: "many",                          origin: Origin::Greek },
    Root { pattern: "proto",  position: RootPosition::Prefix, meaning: "first, original",               origin: Origin::Greek },
    Root { pattern: "pseudo", position: RootPosition::Prefix, meaning: "false",                         origin: Origin::Greek },
    Root { pattern: "psych",  position: RootPosition::Prefix, meaning: "mind, soul",                    origin: Origin::Greek },
    Root { pattern: "sym",    position: RootPosition::Prefix, meaning: "together, with",                origin: Origin::Greek },
    Root { pattern: "syn",    position: RootPosition::Prefix, meaning: "together, with",                origin: Origin::Greek },
    Root { pattern: "tele",   position: RootPosition::Prefix, meaning: "far, distant",                  origin: Origin::Greek },
    Root { pattern: "thermo", position: RootPosition::Prefix, meaning: "heat",                          origin: Origin::Greek },
    Root { pattern: "xeno",   position: RootPosition::Prefix, meaning: "foreign, strange",              origin: Origin::Greek },
    Root { pattern: "zoo",    position: RootPosition::Prefix, meaning: "animal",                        origin: Origin::Greek },
    // ── Latin prefixes ───────────────────────────────────────────────────────
    Root { pattern: "ab",     position: RootPosition::Prefix, meaning: "away from",                     origin: Origin::Latin },
    Root { pattern: "ad",     position: RootPosition::Prefix, meaning: "to, toward",                    origin: Origin::Latin },
    Root { pattern: "ante",   position: RootPosition::Prefix, meaning: "before",                        origin: Origin::Latin },
    Root { pattern: "bene",   position: RootPosition::Prefix, meaning: "good, well",                    origin: Origin::Latin },
    Root { pattern: "bi",     position: RootPosition::Prefix, meaning: "two",                           origin: Origin::Latin },
    Root { pattern: "circum", position: RootPosition::Prefix, meaning: "around",                        origin: Origin::Latin },
    Root { pattern: "co",     position: RootPosition::Prefix, meaning: "with, together",                origin: Origin::Latin },
    Root { pattern: "com",    position: RootPosition::Prefix, meaning: "with, together",                origin: Origin::Latin },
    Root { pattern: "con",    position: RootPosition::Prefix, meaning: "with, together",                origin: Origin::Latin },
    Root { pattern: "contra", position: RootPosition::Prefix, meaning: "against",                       origin: Origin::Latin },
    Root { pattern: "de",     position: RootPosition::Prefix, meaning: "down from, removal",            origin: Origin::Latin },
    Root { pattern: "dis",    position: RootPosition::Prefix, meaning: "not, apart",                    origin: Origin::Latin },
    Root { pattern: "ex",     position: RootPosition::Prefix, meaning: "out of, former",                origin: Origin::Latin },
    Root { pattern: "extra",  position: RootPosition::Prefix, meaning: "beyond, outside",               origin: Origin::Latin },
    Root { pattern: "in",     position: RootPosition::Prefix, meaning: "in, into / not",                origin: Origin::Latin },
    Root { pattern: "inter",  position: RootPosition::Prefix, meaning: "between, among",                origin: Origin::Latin },
    Root { pattern: "intra",  position: RootPosition::Prefix, meaning: "within",                        origin: Origin::Latin },
    Root { pattern: "intro",  position: RootPosition::Prefix, meaning: "inward",                        origin: Origin::Latin },
    Root { pattern: "mal",    position: RootPosition::Prefix, meaning: "bad, ill",                      origin: Origin::Latin },
    Root { pattern: "mis",    position: RootPosition::Prefix, meaning: "wrongly, badly",                origin: Origin::Latin },
    Root { pattern: "multi",  position: RootPosition::Prefix, meaning: "many",                          origin: Origin::Latin },
    Root { pattern: "non",    position: RootPosition::Prefix, meaning: "not",                           origin: Origin::Latin },
    Root { pattern: "ob",     position: RootPosition::Prefix, meaning: "against, in the way",           origin: Origin::Latin },
    Root { pattern: "per",    position: RootPosition::Prefix, meaning: "through, thoroughly",           origin: Origin::Latin },
    Root { pattern: "post",   position: RootPosition::Prefix, meaning: "after, behind",                 origin: Origin::Latin },
    Root { pattern: "pre",    position: RootPosition::Prefix, meaning: "before",                        origin: Origin::Latin },
    Root { pattern: "pro",    position: RootPosition::Prefix, meaning: "before, in favor of",           origin: Origin::Latin },
    Root { pattern: "re",     position: RootPosition::Prefix, meaning: "again, back",                   origin: Origin::Latin },
    Root { pattern: "retro",  position: RootPosition::Prefix, meaning: "backward",                      origin: Origin::Latin },
    Root { pattern: "semi",   position: RootPosition::Prefix, meaning: "half",                          origin: Origin::Latin },
    Root { pattern: "sub",    position: RootPosition::Prefix, meaning: "under, below",                  origin: Origin::Latin },
    Root { pattern: "super",  position: RootPosition::Prefix, meaning: "above, over",                   origin: Origin::Latin },
    Root { pattern: "trans",  position: RootPosition::Prefix, meaning: "across, beyond",                origin: Origin::Latin },
    Root { pattern: "tri",    position: RootPosition::Prefix, meaning: "three",                         origin: Origin::Latin },
    Root { pattern: "ultra",  position: RootPosition::Prefix, meaning: "beyond, extreme",               origin: Origin::Latin },
    Root { pattern: "uni",    position: RootPosition::Prefix, meaning: "one",                           origin: Origin::Latin },
    Root { pattern: "vice",   position: RootPosition::Prefix, meaning: "in place of",                   origin: Origin::Latin },
    // ── Greek suffixes ───────────────────────────────────────────────────────
    Root { pattern: "algia",   position: RootPosition::Suffix, meaning: "pain",                         origin: Origin::Greek },
    Root { pattern: "cracy",   position: RootPosition::Suffix, meaning: "rule, government",             origin: Origin::Greek },
    Root { pattern: "crat",    position: RootPosition::Suffix, meaning: "ruler",                        origin: Origin::Greek },
    Root { pattern: "gamy",    position: RootPosition::Suffix, meaning: "marriage",                     origin: Origin::Greek },
    Root { pattern: "genesis", position: RootPosition::Suffix, meaning: "origin, creation",             origin: Origin::Greek },
    Root { pattern: "graph",   position: RootPosition::Suffix, meaning: "writing, recording",           origin: Origin::Greek },
    Root { pattern: "graphy",  position: RootPosition::Suffix, meaning: "process of writing/recording", origin: Origin::Greek },
    Root { pattern: "logy",    position: RootPosition::Suffix, meaning: "study of",                     origin: Origin::Greek },
    Root { pattern: "logia",   position: RootPosition::Suffix, meaning: "study of",                     origin: Origin::Greek },
    Root { pattern: "lysis",   position: RootPosition::Suffix, meaning: "breaking down, dissolution",   origin: Origin::Greek },
    Root { pattern: "mania",   position: RootPosition::Suffix, meaning: "obsession, madness",           origin: Origin::Greek },
    Root { pattern: "meter",   position: RootPosition::Suffix, meaning: "measure",                      origin: Origin::Greek },
    Root { pattern: "metry",   position: RootPosition::Suffix, meaning: "measurement",                  origin: Origin::Greek },
    Root { pattern: "morph",   position: RootPosition::Suffix, meaning: "form, shape",                  origin: Origin::Greek },
    Root { pattern: "nomy",    position: RootPosition::Suffix, meaning: "law, arrangement",             origin: Origin::Greek },
    Root { pattern: "pathy",   position: RootPosition::Suffix, meaning: "feeling, disease",             origin: Origin::Greek },
    Root { pattern: "phage",   position: RootPosition::Suffix, meaning: "eating, devouring",            origin: Origin::Greek },
    Root { pattern: "phile",   position: RootPosition::Suffix, meaning: "lover of",                     origin: Origin::Greek },
    Root { pattern: "philia",  position: RootPosition::Suffix, meaning: "love, attraction",             origin: Origin::Greek },
    Root { pattern: "phobia",  position: RootPosition::Suffix, meaning: "fear of",                      origin: Origin::Greek },
    Root { pattern: "phone",   position: RootPosition::Suffix, meaning: "sound, voice",                 origin: Origin::Greek },
    Root { pattern: "scope",   position: RootPosition::Suffix, meaning: "viewing instrument",           origin: Origin::Greek },
    Root { pattern: "sophy",   position: RootPosition::Suffix, meaning: "wisdom",                       origin: Origin::Greek },
    Root { pattern: "tomy",    position: RootPosition::Suffix, meaning: "cutting, incision",            origin: Origin::Greek },
    Root { pattern: "trophy",  position: RootPosition::Suffix, meaning: "nourishment, growth",          origin: Origin::Greek },
    Root { pattern: "type",    position: RootPosition::Suffix, meaning: "impression, model",            origin: Origin::Greek },
    // ── Latin suffixes ───────────────────────────────────────────────────────
    Root { pattern: "able",    position: RootPosition::Suffix, meaning: "capable of, worthy of",        origin: Origin::Latin },
    Root { pattern: "ible",    position: RootPosition::Suffix, meaning: "capable of, worthy of",        origin: Origin::Latin },
    Root { pattern: "ance",    position: RootPosition::Suffix, meaning: "state, quality",               origin: Origin::Latin },
    Root { pattern: "ence",    position: RootPosition::Suffix, meaning: "state, quality",               origin: Origin::Latin },
    Root { pattern: "ant",     position: RootPosition::Suffix, meaning: "performing, causing",          origin: Origin::Latin },
    Root { pattern: "arium",   position: RootPosition::Suffix, meaning: "place for",                    origin: Origin::Latin },
    Root { pattern: "ation",   position: RootPosition::Suffix, meaning: "action, process",              origin: Origin::Latin },
    Root { pattern: "ent",     position: RootPosition::Suffix, meaning: "performing, causing",          origin: Origin::Latin },
    Root { pattern: "fy",      position: RootPosition::Suffix, meaning: "to make, to cause",            origin: Origin::Latin },
    Root { pattern: "ify",     position: RootPosition::Suffix, meaning: "to make, to cause",            origin: Origin::Latin },
    Root { pattern: "ion",     position: RootPosition::Suffix, meaning: "action, result",               origin: Origin::Latin },
    Root { pattern: "ism",     position: RootPosition::Suffix, meaning: "belief, practice",             origin: Origin::Latin },
    Root { pattern: "ist",     position: RootPosition::Suffix, meaning: "one who practices",            origin: Origin::Latin },
    Root { pattern: "ity",     position: RootPosition::Suffix, meaning: "state, quality",               origin: Origin::Latin },
    Root { pattern: "ive",     position: RootPosition::Suffix, meaning: "having the quality of",        origin: Origin::Latin },
    Root { pattern: "ize",     position: RootPosition::Suffix, meaning: "to make, to cause",            origin: Origin::Latin },
    Root { pattern: "ment",    position: RootPosition::Suffix, meaning: "result, action",               origin: Origin::Latin },
    Root { pattern: "ness",    position: RootPosition::Suffix, meaning: "state, quality",               origin: Origin::Latin },
    Root { pattern: "or",      position: RootPosition::Suffix, meaning: "one who does",                 origin: Origin::Latin },
    Root { pattern: "ous",     position: RootPosition::Suffix, meaning: "having the quality of",        origin: Origin::Latin },
    Root { pattern: "sion",    position: RootPosition::Suffix, meaning: "action, result",               origin: Origin::Latin },
    Root { pattern: "tion",    position: RootPosition::Suffix, meaning: "action, result",               origin: Origin::Latin },
    Root { pattern: "ture",    position: RootPosition::Suffix, meaning: "act, result, state",           origin: Origin::Latin },
    Root { pattern: "ure",     position: RootPosition::Suffix, meaning: "act, condition",               origin: Origin::Latin },
];

// ── Public API ────────────────────────────────────────────────────────────────

/// Finds all known Latin/Greek roots (prefixes and suffixes) present in a word.
///
/// The search is case-insensitive. Only roots whose pattern is strictly shorter
/// than the word are considered, so the word itself is never matched as its own
/// root. Results are sorted by pattern length (longest first) so that more
/// specific roots take precedence over shorter, more general ones.
///
/// All patterns in the database are ASCII; the matched slice from `word` always
/// aligns with a valid UTF-8 boundary because a prefix/suffix match against an
/// ASCII pattern guarantees the boundary characters are themselves ASCII.
///
/// # Arguments
///
/// * `word` - The word to analyse
///
/// # Returns
///
/// A `Vec<RootMatch>` of all matching roots. Empty if none are found.
///
/// # Examples
///
/// ```
/// use augusto::roots::find_roots;
///
/// let matches = find_roots("biology");
/// let patterns: Vec<&str> = matches.iter().map(|m| m.root.pattern).collect();
/// assert!(patterns.contains(&"bio"));
/// assert!(patterns.contains(&"logy"));
/// ```
pub fn find_roots(word: &str) -> Vec<RootMatch<'_>> {
    if word.is_empty() {
        return vec![];
    }

    let word_lower = word.to_lowercase();
    let min_remaining = 1; // at least one character must remain beyond the root

    let mut matches: Vec<RootMatch<'_>> = ROOTS
        .iter()
        .filter(|root| {
            let pat = root.pattern;
            if pat.len() >= word_lower.len() {
                return false;
            }
            match root.position {
                RootPosition::Prefix => {
                    word_lower.starts_with(pat) && word_lower.len() - pat.len() >= min_remaining
                }
                RootPosition::Suffix => {
                    word_lower.ends_with(pat) && word_lower.len() - pat.len() >= min_remaining
                }
            }
        })
        .map(|root| {
            // Safety: all patterns are ASCII so the byte length equals the char
            // length. Because word_lower.starts_with / ends_with succeeded on
            // those exact bytes, the slice boundary in `word` is valid UTF-8.
            let matched = match root.position {
                RootPosition::Prefix => &word[..root.pattern.len()],
                RootPosition::Suffix => &word[word.len() - root.pattern.len()..],
            };
            RootMatch { root, matched }
        })
        .collect();

    // Sort longest pattern first so specific roots appear before general ones.
    matches.sort_by(|a, b| b.root.pattern.len().cmp(&a.root.pattern.len()));

    matches
}

/// Formats a complete etymological roots report for a word.
///
/// # Arguments
///
/// * `word` - The word to analyse
///
/// # Returns
///
/// A formatted multi-line string listing every found root with its position,
/// origin, and meaning. If no roots are found, a brief message is returned.
pub fn analyze_roots(word: &str) -> String {
    let matches = find_roots(word);

    if matches.is_empty() {
        return format!(
            "Word: {}\nNo known Latin or Greek roots found.",
            word
        );
    }

    let mut output = format!("Word: {}\n\nRoots found:\n", word);
    for m in &matches {
        output.push_str(&format!(
            "  {:10} ({}, {}) — {}\n",
            m.matched, m.root.position, m.root.origin, m.root.meaning,
        ));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_roots_biology() {
        let matches = find_roots("biology");
        let patterns: Vec<&str> = matches.iter().map(|m| m.root.pattern).collect();
        assert!(patterns.contains(&"bio"),  "Should find 'bio' prefix");
        assert!(patterns.contains(&"logy"), "Should find 'logy' suffix");
    }

    #[test]
    fn test_find_roots_telescope() {
        let matches = find_roots("telescope");
        let patterns: Vec<&str> = matches.iter().map(|m| m.root.pattern).collect();
        assert!(patterns.contains(&"tele"),  "Should find 'tele' prefix");
        assert!(patterns.contains(&"scope"), "Should find 'scope' suffix");
    }

    #[test]
    fn test_find_roots_antibiotic() {
        let matches = find_roots("antibiotic");
        let patterns: Vec<&str> = matches.iter().map(|m| m.root.pattern).collect();
        // "anti" is a prefix at the start of "antibiotic"
        assert!(patterns.contains(&"anti"), "Should find 'anti' prefix");
        // "bio" is not at the start of "antibiotic" so it is NOT matched as a prefix
        assert!(!patterns.contains(&"bio"), "'bio' should not match mid-word");
    }

    #[test]
    fn test_find_roots_empty() {
        assert!(find_roots("").is_empty());
    }

    #[test]
    fn test_find_roots_no_full_word_match() {
        // "bio" itself should not match its own "bio" prefix (pattern length
        // would equal word length, which is filtered out).
        let matches = find_roots("bio");
        let patterns: Vec<&str> = matches.iter().map(|m| m.root.pattern).collect();
        assert!(!patterns.contains(&"bio"), "'bio' should not match itself as a root");
    }

    #[test]
    fn test_roots_sorted_by_length_descending() {
        // "pseudoscope" has "pseudo" (6 chars) and "scope" (5 chars).
        let matches = find_roots("pseudoscope");
        for window in matches.windows(2) {
            assert!(
                window[0].root.pattern.len() >= window[1].root.pattern.len(),
                "Roots should be sorted by pattern length descending"
            );
        }
    }

    #[test]
    fn test_find_roots_case_insensitive() {
        let lower = find_roots("biology");
        let upper = find_roots("BIOLOGY");
        assert_eq!(lower.len(), upper.len(), "Case should not affect the number of matched roots");
    }

    #[test]
    fn test_analyze_roots_no_roots() {
        let result = analyze_roots("cat");
        // May or may not match short patterns; just ensure it doesn't panic.
        assert!(!result.is_empty());
    }

    #[test]
    fn test_analyze_roots_with_roots() {
        let result = analyze_roots("biology");
        assert!(result.contains("bio"));
        assert!(result.contains("logy"));
        assert!(result.contains("Greek"));
    }
}
