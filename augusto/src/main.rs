//! Augusto - A command-line tool for creative word operations
//!
//! Inspired by Brazilian concrete poet Augusto de Campos, this tool provides
//! various word manipulation operations, including anagram generation, ASCII
//! art, phonetic patterns, palindromes, syllable splitting, word blending,
//! and etymological root analysis.
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
//!
//! # Phonetic pattern
//! augusto pattern "rust"
//!
//! # Palindrome analysis
//! augusto palindrome "racecar"
//!
//! # Syllable splitting
//! augusto syllable "beautiful"
//!
//! # Word blending (portmanteau)
//! augusto blend "smoke" "fog"
//!
//! # Etymological roots
//! augusto roots "biology"
//! ```

use std::{collections::HashSet, env};
mod anagram;
mod ascii_art;
mod benchmark;
mod blend;
mod palindrome;
mod pattern;
mod roots;
mod syllable;

/// Main entry point for the augusto CLI tool
///
/// # Commands
///
/// - `anagram <word>` - Generate all anagrams of a word
/// - `art <main_word> <filler_word>` - Create ASCII art using one word to fill another
/// - `bench <operation> <args...>` - Benchmark an operation and show performance stats
/// - `pattern <word> [word2]` - Show the vowel/consonant pattern of a word
/// - `palindrome <word>` - Analyse palindrome properties of a word
/// - `syllable <word>` - Split a word into syllables
/// - `blend <word1> <word2>` - Generate portmanteau blends of two words
/// - `roots <word>` - Identify Latin/Greek etymological roots in a word
///
/// # Examples
///
/// ```bash
/// augusto anagram "cat"
/// augusto art "RUST" "code"
/// augusto bench anagram "test"
/// augusto pattern "rust"
/// augusto palindrome "racecar"
/// augusto syllable "beautiful"
/// augusto blend "smoke" "fog"
/// augusto roots "biology"
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
        "pattern" | "pat" => {
            if args.len() < 3 {
                eprintln!("Error: Missing word for pattern analysis");
                eprintln!("\nUsage: augusto pattern <word> [word2]");
                eprintln!("Example: augusto pattern \"rust\"");
                eprintln!("         augusto pattern \"rust\" \"poesia\"");
                std::process::exit(1);
            }
            if let Some(word2) = args.get(3) {
                // Comparison mode: show two words side by side
                println!("{}", pattern::analyze_pattern(&args[2]));
                println!();
                println!("{}", pattern::analyze_pattern(word2));
            } else {
                println!("{}", pattern::analyze_pattern(&args[2]));
            }
        }
        "palindrome" | "pal" => {
            if args.len() < 3 {
                eprintln!("Error: Missing word for palindrome analysis");
                eprintln!("\nUsage: augusto palindrome <word>");
                eprintln!("Example: augusto palindrome \"racecar\"");
                std::process::exit(1);
            }
            println!("{}", palindrome::analyze_palindrome(&args[2]));
        }
        "syllable" | "syl" | "syllables" => {
            if args.len() < 3 {
                eprintln!("Error: Missing word for syllable splitting");
                eprintln!("\nUsage: augusto syllable <word>");
                eprintln!("Example: augusto syllable \"beautiful\"");
                std::process::exit(1);
            }
            println!("{}", syllable::analyze_syllables(&args[2]));
        }
        "blend" | "bld" => {
            if args.len() < 4 {
                eprintln!("Error: Two words are required for blending");
                eprintln!("\nUsage: augusto blend <word1> <word2>");
                eprintln!("Example: augusto blend \"smoke\" \"fog\"");
                std::process::exit(1);
            }
            let blends = blend::blend_words(&args[2], &args[3]);
            if blends.is_empty() {
                println!("No blends could be generated for these words.");
            } else {
                println!("Blends of \"{}\" + \"{}\":", args[2], args[3]);
                for b in &blends {
                    println!("  {}", b);
                }
            }
        }
        "roots" | "etymology" | "etym" => {
            if args.len() < 3 {
                eprintln!("Error: Missing word for roots analysis");
                eprintln!("\nUsage: augusto roots <word>");
                eprintln!("Example: augusto roots \"biology\"");
                std::process::exit(1);
            }
            println!("{}", roots::analyze_roots(&args[2]));
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
    println!("    pattern <word> [word2]              Show vowel/consonant pattern (V/C)");
    println!("    palindrome <word>                   Analyse palindrome properties");
    println!("    syllable <word>                     Split a word into syllables");
    println!("    blend <word1> <word2>               Generate portmanteau blends");
    println!("    roots <word>                        Find Latin/Greek etymological roots");
    println!("    help                                Show this help message");
    println!();
    println!("BENCH OPERATIONS:");
    println!("    bench anagram <word>                Benchmark anagram generation");
    println!("    bench art <main> <filler>           Benchmark ASCII art generation");
    println!("    bench pattern <word>                Benchmark phonetic pattern");
    println!("    bench palindrome <word>             Benchmark palindrome analysis");
    println!("    bench syllable <word>               Benchmark syllable splitting");
    println!("    bench blend <word1> <word2>         Benchmark word blending");
    println!("    bench roots <word>                  Benchmark roots analysis");
    println!("    bench table                         Show full benchmark table (all ops)");
    println!();
    println!("EXAMPLES:");
    println!("    augusto anagram \"cat\"");
    println!("    augusto art \"RUST\" \"code\"");
    println!("    augusto art \"RUST\" \"code\" 2");
    println!("    augusto bench anagram \"test\"");
    println!("    augusto bench art \"HI\" \"rust\"");
    println!("    augusto compare \"cat\" \"test\" \"program\"");
    println!("    augusto pattern \"rust\"");
    println!("    augusto pattern \"rust\" \"poesia\"");
    println!("    augusto palindrome \"racecar\"");
    println!("    augusto syllable \"beautiful\"");
    println!("    augusto blend \"smoke\" \"fog\"");
    println!("    augusto roots \"biology\"");
    println!("    augusto bench table");
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

    // Generate ASCII art; use word_art for default spacing (0 or 1)
    let art = if spacing == 0 || spacing == 1 {
        ascii_art::word_art(main_word, filler_word)
    } else {
        ascii_art::word_art_with_spacing(main_word, filler_word, spacing)
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
                || ascii_art::word_art(main_word, filler_word),
            );

            println!("{}", stats);
        }
        "pattern" | "pat" => {
            if args.len() < 2 {
                eprintln!("Error: Missing word for pattern benchmark");
                eprintln!("\nUsage: augusto bench pattern <word>");
                std::process::exit(1);
            }
            let input = &args[1];
            let example = pattern::phonetic_pattern(input);
            let stats = benchmark::benchmark_operation("Phonetic Pattern", input, || {
                pattern::phonetic_pattern(input);
            });
            println!("Output example: \"{}\" → {}", input, example);
            println!("{}", stats);
        }
        "palindrome" | "pal" => {
            if args.len() < 2 {
                eprintln!("Error: Missing word for palindrome benchmark");
                eprintln!("\nUsage: augusto bench palindrome <word>");
                std::process::exit(1);
            }
            let input = &args[1];
            let is_pal = palindrome::is_palindrome(input);
            let stats = benchmark::benchmark_operation("Palindrome Analysis", input, || {
                palindrome::is_palindrome(input);
                palindrome::mirror(input);
                palindrome::longest_palindromic_substring(input);
            });
            println!(
                "Output example: \"{}\" is palindrome: {}",
                input,
                if is_pal { "Yes ✓" } else { "No" }
            );
            println!("{}", stats);
        }
        "syllable" | "syl" => {
            if args.len() < 2 {
                eprintln!("Error: Missing word for syllable benchmark");
                eprintln!("\nUsage: augusto bench syllable <word>");
                std::process::exit(1);
            }
            let input = &args[1];
            let syllables = syllable::split_syllables(input);
            let stats = benchmark::benchmark_operation("Syllable Split", input, || {
                syllable::split_syllables(input);
            });
            println!("Output example: \"{}\" → {}", input, syllables.join("-"));
            println!("{}", stats);
        }
        "blend" | "bld" => {
            if args.len() < 3 {
                eprintln!("Error: Two words required for blend benchmark");
                eprintln!("\nUsage: augusto bench blend <word1> <word2>");
                std::process::exit(1);
            }
            let word_a = &args[1];
            let word_b = &args[2];
            let blends = blend::blend_words(word_a, word_b);
            let example = blends.first().cloned().unwrap_or_default();
            let input_label = format!("{}+{}", word_a, word_b);
            let stats = benchmark::benchmark_operation("Word Blend", &input_label, || {
                blend::blend_words(word_a, word_b);
            });
            println!(
                "Output example: \"{}\" + \"{}\" → \"{}\"",
                word_a, word_b, example
            );
            println!("{}", stats);
        }
        "roots" | "etym" => {
            if args.len() < 2 {
                eprintln!("Error: Missing word for roots benchmark");
                eprintln!("\nUsage: augusto bench roots <word>");
                std::process::exit(1);
            }
            let input = &args[1];
            let matches = roots::find_roots(input);
            let example = matches
                .iter()
                .map(|m| m.root.pattern)
                .collect::<Vec<_>>()
                .join(", ");
            let stats = benchmark::benchmark_operation("Roots Analysis", input, || {
                roots::find_roots(input);
            });
            println!(
                "Output example: roots of \"{}\" → [{}]",
                input, example
            );
            println!("{}", stats);
        }
        "table" => {
            run_bench_table();
        }
        _ => {
            eprintln!("Error: Unknown operation '{}' for benchmark", operation);
            eprintln!("\nSupported operations:");
            eprintln!("  - anagram <word>");
            eprintln!("  - art <main_word> <filler_word>");
            eprintln!("  - pattern <word>");
            eprintln!("  - palindrome <word>");
            eprintln!("  - syllable <word>");
            eprintln!("  - blend <word1> <word2>");
            eprintln!("  - roots <word>");
            eprintln!("  - table");
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

/// Run a multi-operation benchmark table showing example word transformations
/// for all supported operations side by side.
///
/// The table is printed to stdout in ASCII box-drawing format.
fn run_bench_table() {
    let mut table = benchmark::BenchmarkTable::new();

    // ── Phonetic pattern ─────────────────────────────────────────────────────
    for word in &["rust", "hello", "poesia", "beautiful"] {
        let example = pattern::phonetic_pattern(word);
        let row = benchmark::benchmark_to_table_row(
            "Phonetic Pattern",
            word,
            example,
            || {
                pattern::phonetic_pattern(word);
            },
        );
        table.add_row(row);
    }

    // ── Palindrome ───────────────────────────────────────────────────────────
    for word in &["racecar", "hello", "level", "arara"] {
        let example = if palindrome::is_palindrome(word) {
            "palindrome ✓".to_string()
        } else {
            format!("mirror: {}", palindrome::mirror(word))
        };
        let row = benchmark::benchmark_to_table_row(
            "Palindrome",
            word,
            example,
            || {
                palindrome::is_palindrome(word);
                palindrome::mirror(word);
                palindrome::longest_palindromic_substring(word);
            },
        );
        table.add_row(row);
    }

    // ── Syllable split ───────────────────────────────────────────────────────
    for word in &["rust", "computer", "beautiful", "program"] {
        let syllables = syllable::split_syllables(word);
        let example = syllables.join("-");
        let row = benchmark::benchmark_to_table_row(
            "Syllable Split",
            word,
            example,
            || {
                syllable::split_syllables(word);
            },
        );
        table.add_row(row);
    }

    // ── Word blend ───────────────────────────────────────────────────────────
    let blend_pairs = [
        ("smoke", "fog"),
        ("breakfast", "lunch"),
        ("motor", "hotel"),
        ("web", "log"),
    ];
    for &(a, b) in &blend_pairs {
        let blends = blend::blend_words(a, b);
        let example = blends
            .first()
            .cloned()
            .unwrap_or_else(|| "(none)".to_string());
        let input_label = format!("{}+{}", a, b);
        let row = benchmark::benchmark_to_table_row(
            "Word Blend",
            &input_label,
            example,
            || {
                blend::blend_words(a, b);
            },
        );
        table.add_row(row);
    }

    // ── Roots analysis ───────────────────────────────────────────────────────
    for word in &["biology", "telescope", "thermometer", "autobiography"] {
        let matches = roots::find_roots(word);
        let example = matches
            .iter()
            .map(|m| m.root.pattern)
            .collect::<Vec<_>>()
            .join("+");
        let row = benchmark::benchmark_to_table_row(
            "Roots Analysis",
            word,
            example,
            || {
                roots::find_roots(word);
            },
        );
        table.add_row(row);
    }

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║         BENCHMARK TABLE — ALL OPERATIONS                     ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
    print!("{}", table.format());
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
