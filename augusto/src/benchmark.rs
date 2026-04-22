//! Benchmarking and performance testing module
//!
//! This module provides functionality for measuring and reporting performance
//! statistics for various word operations in the augusto tool.
//!
//! # Examples
//!
//! ```
//! use augusto::benchmark::benchmark_operation;
//!
//! let stats = benchmark_operation("anagram", "test", || {
//!     // Your operation here
//! });
//! println!("{}", stats);
//! ```

use std::time::{Duration, Instant};

/// Performance statistics for an operation
#[derive(Debug, Clone)]
pub struct BenchmarkStats {
    /// Name of the operation being benchmarked
    pub operation: String,
    /// Input used for the benchmark
    pub input: String,
    /// Total execution time
    pub duration: Duration,
    /// Number of iterations performed
    pub iterations: usize,
    /// Average time per iteration
    pub avg_duration: Duration,
    /// Output size (if applicable)
    pub output_size: Option<usize>,
}

impl BenchmarkStats {
    /// Create a new BenchmarkStats instance
    pub fn new(operation: String, input: String, duration: Duration, iterations: usize) -> Self {
        let avg_duration = if iterations > 0 {
            duration / iterations as u32
        } else {
            duration
        };

        Self {
            operation,
            input,
            duration,
            iterations,
            avg_duration,
            output_size: None,
        }
    }

    /// Set the output size
    pub fn with_output_size(mut self, size: usize) -> Self {
        self.output_size = Some(size);
        self
    }

    /// Format duration in human-readable form
    fn format_duration(duration: Duration) -> String {
        let micros = duration.as_micros();
        if micros < 1000 {
            format!("{}μs", micros)
        } else if micros < 1_000_000 {
            format!("{:.2}ms", micros as f64 / 1000.0)
        } else {
            format!("{:.2}s", duration.as_secs_f64())
        }
    }

    /// Get a formatted string representation of the stats
    pub fn format(&self) -> String {
        let mut output = String::new();

        output.push_str("\n╔════════════════════════════════════════════════════════════╗\n");
        output.push_str("║              PERFORMANCE BENCHMARK RESULTS                 ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════╝\n\n");

        output.push_str(&format!("Operation:        {}\n", self.operation));
        output.push_str(&format!("Input:            \"{}\"\n", self.input));
        output.push_str(&format!(
            "Input length:     {} character(s)\n",
            self.input.len()
        ));

        if let Some(size) = self.output_size {
            output.push_str(&format!("Output size:      {} item(s)\n", size));
        }

        output.push('\n');
        output.push_str(&format!(
            "Total time:       {}\n",
            Self::format_duration(self.duration)
        ));
        output.push_str(&format!("Iterations:       {}\n", self.iterations));
        output.push_str(&format!(
            "Avg per run:      {}\n",
            Self::format_duration(self.avg_duration)
        ));

        // Calculate throughput
        if self.duration.as_micros() > 0 {
            let ops_per_sec = (self.iterations as f64 / self.duration.as_secs_f64()) as u64;
            output.push_str(&format!("Throughput:       {} ops/sec\n", ops_per_sec));
        }

        output.push('\n');
        output.push_str("╚════════════════════════════════════════════════════════════╝\n");

        output
    }
}

impl std::fmt::Display for BenchmarkStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}

/// Benchmark a simple operation
///
/// Runs the operation multiple times and returns statistics
///
/// # Arguments
///
/// * `operation` - Name of the operation
/// * `input` - Input string
/// * `f` - Function to benchmark
///
/// # Returns
///
/// BenchmarkStats with performance information
pub fn benchmark_operation<F, T>(operation: &str, input: &str, mut f: F) -> BenchmarkStats
where
    F: FnMut() -> T,
{
    // Warm-up run
    let _ = f();

    // Determine number of iterations based on expected complexity
    let iterations = calculate_iterations(input);

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = f();
    }
    let duration = start.elapsed();

    BenchmarkStats::new(
        operation.to_string(),
        input.to_string(),
        duration,
        iterations,
    )
}

/// Benchmark an operation that returns a result
///
/// Similar to benchmark_operation but captures the output size
pub fn benchmark_with_result<F, T>(operation: &str, input: &str, mut f: F) -> BenchmarkStats
where
    F: FnMut() -> T,
    T: IntoIterator,
    T::Item: Sized,
{
    // Warm-up and get output size
    let result = f();
    let output_size = result.into_iter().count();

    // Determine number of iterations
    let iterations = calculate_iterations(input);

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = f();
    }
    let duration = start.elapsed();

    BenchmarkStats::new(
        operation.to_string(),
        input.to_string(),
        duration,
        iterations,
    )
    .with_output_size(output_size)
}

/// Calculate appropriate number of iterations based on input
fn calculate_iterations(input: &str) -> usize {
    let len = input.len();

    // For very short inputs, do more iterations
    if len <= 3 {
        10000
    } else if len <= 5 {
        1000
    } else if len <= 7 {
        100
    } else if len <= 9 {
        10
    } else {
        // For very long inputs, just do a few runs
        1
    }
}

/// Benchmark multiple operations and compare them
pub struct BenchmarkSuite {
    results: Vec<BenchmarkStats>,
}

impl BenchmarkSuite {
    /// Create a new benchmark suite
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    /// Add a benchmark result
    pub fn add(&mut self, stats: BenchmarkStats) {
        self.results.push(stats);
    }

    /// Format comparison results
    pub fn format_comparison(&self) -> String {
        if self.results.is_empty() {
            return String::from("No benchmark results to compare.\n");
        }

        let mut output = String::new();

        output.push_str("\n╔════════════════════════════════════════════════════════════╗\n");
        output.push_str("║              BENCHMARK COMPARISON                          ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════╝\n\n");

        for (i, stat) in self.results.iter().enumerate() {
            output.push_str(&format!(
                "{}. {} with input \"{}\"\n",
                i + 1,
                stat.operation,
                stat.input
            ));
            output.push_str(&format!(
                "   Time: {} (avg: {})\n",
                BenchmarkStats::format_duration(stat.duration),
                BenchmarkStats::format_duration(stat.avg_duration)
            ));
            if let Some(size) = stat.output_size {
                output.push_str(&format!("   Output: {} items\n", size));
            }
            output.push('\n');
        }

        output
    }
}

impl Default for BenchmarkSuite {
    fn default() -> Self {
        Self::new()
    }
}

// ── Benchmark table ───────────────────────────────────────────────────────────

/// A single row in a benchmark results table, capturing one operation and its
/// example output so results can be compared side by side.
#[derive(Debug, Clone)]
pub struct BenchmarkTableRow {
    /// Human-readable name of the operation (e.g., "Phonetic Pattern").
    pub operation: String,
    /// Input word(s) used for the measurement.
    pub input: String,
    /// Short sample of the transformed output (e.g., `"CVCC"`, `"com-pu-ter"`).
    pub output_example: String,
    /// Average measured duration per iteration.
    pub avg_duration: Duration,
}

/// A formatted ASCII table comparing benchmark results across multiple
/// operations and input words.
///
/// Each row shows the operation name, the input, an example of the
/// transformed output, and the average execution time.
///
/// # Examples
///
/// ```
/// use augusto::benchmark::{BenchmarkTable, BenchmarkTableRow};
/// use std::time::Duration;
///
/// let mut table = BenchmarkTable::new();
/// table.add_row(BenchmarkTableRow {
///     operation:      "Phonetic Pattern".to_string(),
///     input:          "rust".to_string(),
///     output_example: "CVCC".to_string(),
///     avg_duration:   Duration::from_micros(1),
/// });
/// println!("{}", table.format());
/// ```
#[derive(Default)]
pub struct BenchmarkTable {
    rows: Vec<BenchmarkTableRow>,
}

impl BenchmarkTable {
    /// Create an empty benchmark table.
    pub fn new() -> Self {
        Self { rows: Vec::new() }
    }    /// Append a row to the table.
    pub fn add_row(&mut self, row: BenchmarkTableRow) {
        self.rows.push(row);
    }

    /// Render the table as a formatted ASCII string with aligned columns.
    ///
    /// Column widths are determined dynamically so that every cell fits without
    /// truncation.
    pub fn format(&self) -> String {
        if self.rows.is_empty() {
            return String::from("No benchmark results.\n");
        }

        // Determine column widths dynamically.
        let op_w = self
            .rows
            .iter()
            .map(|r| r.operation.len())
            .max()
            .unwrap_or(0)
            .max("Operation".len());
        let in_w = self
            .rows
            .iter()
            .map(|r| r.input.len())
            .max()
            .unwrap_or(0)
            .max("Input".len());
        let ex_w = self
            .rows
            .iter()
            .map(|r| r.output_example.len())
            .max()
            .unwrap_or(0)
            .max("Output example".len());
        let tm_w = self
            .rows
            .iter()
            .map(|r| BenchmarkStats::format_duration(r.avg_duration).len())
            .max()
            .unwrap_or(0)
            .max("Avg time".len());

        let sep = format!(
            "+-{}-+-{}-+-{}-+-{}-+",
            "-".repeat(op_w),
            "-".repeat(in_w),
            "-".repeat(ex_w),
            "-".repeat(tm_w),
        );

        let mut out = String::new();
        out.push_str(&sep);
        out.push('\n');
        out.push_str(&format!(
            "| {:<op_w$} | {:<in_w$} | {:<ex_w$} | {:<tm_w$} |",
            "Operation",
            "Input",
            "Output example",
            "Avg time",
            op_w = op_w,
            in_w = in_w,
            ex_w = ex_w,
            tm_w = tm_w
        ));
        out.push('\n');
        out.push_str(&sep);
        out.push('\n');

        for row in &self.rows {
            out.push_str(&format!(
                "| {:<op_w$} | {:<in_w$} | {:<ex_w$} | {:<tm_w$} |",
                row.operation,
                row.input,
                row.output_example,
                BenchmarkStats::format_duration(row.avg_duration),
                op_w = op_w,
                in_w = in_w,
                ex_w = ex_w,
                tm_w = tm_w
            ));
            out.push('\n');
        }
        out.push_str(&sep);
        out.push('\n');

        out
    }
}

/// Benchmark a closure and return a [`BenchmarkTableRow`] ready to be added to
/// a [`BenchmarkTable`].
///
/// # Arguments
///
/// * `operation`      - Human-readable operation name shown in the table.
/// * `input`          - The input string associated with this measurement.
/// * `output_example` - A short sample of the transformed output to display.
/// * `f`              - The closure to benchmark (called many times).
///
/// # Examples
///
/// ```
/// use augusto::benchmark::benchmark_to_table_row;
///
/// let row = benchmark_to_table_row("Pattern", "rust", "CVCC", || {
///     let _ = "rust".len();
/// });
/// assert_eq!(row.operation, "Pattern");
/// assert_eq!(row.output_example, "CVCC");
/// ```
pub fn benchmark_to_table_row<F, S>(
    operation: &str,
    input: &str,
    output_example: S,
    mut f: F,
) -> BenchmarkTableRow
where
    F: FnMut(),
    S: Into<String>,
{
    // Warm-up run to avoid cold-start bias.
    f();

    let iterations = calculate_iterations(input);
    let start = Instant::now();
    for _ in 0..iterations {
        f();
    }
    let elapsed = start.elapsed();
    let avg_duration = if iterations > 0 {
        elapsed / iterations as u32
    } else {
        elapsed
    };

    BenchmarkTableRow {
        operation: operation.to_string(),
        input: input.to_string(),
        output_example: output_example.into(),
        avg_duration,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_stats_creation() {
        let stats = BenchmarkStats::new(
            "test_op".to_string(),
            "input".to_string(),
            Duration::from_millis(100),
            10,
        );

        assert_eq!(stats.operation, "test_op");
        assert_eq!(stats.input, "input");
        assert_eq!(stats.iterations, 10);
        assert!(stats.duration.as_millis() == 100);
    }

    #[test]
    fn test_benchmark_with_output_size() {
        let stats = BenchmarkStats::new(
            "test_op".to_string(),
            "input".to_string(),
            Duration::from_millis(50),
            5,
        )
        .with_output_size(42);

        assert_eq!(stats.output_size, Some(42));
    }

    #[test]
    fn test_calculate_iterations() {
        assert_eq!(calculate_iterations("ab"), 10000);
        assert_eq!(calculate_iterations("abcd"), 1000);
        assert_eq!(calculate_iterations("abcdef"), 100);
        assert_eq!(calculate_iterations("abcdefgh"), 10);
        assert_eq!(calculate_iterations("abcdefghijk"), 1);
    }

    #[test]
    fn test_benchmark_operation() {
        let stats = benchmark_operation("test", "abc", || {
            // Simple operation
            let _ = "abc".len();
        });

        assert_eq!(stats.operation, "test");
        assert_eq!(stats.input, "abc");
        assert!(stats.iterations > 0);
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(
            BenchmarkStats::format_duration(Duration::from_micros(500)),
            "500μs"
        );
        assert_eq!(
            BenchmarkStats::format_duration(Duration::from_millis(5)),
            "5.00ms"
        );
        assert_eq!(
            BenchmarkStats::format_duration(Duration::from_secs(2)),
            "2.00s"
        );
    }

    #[test]
    fn test_benchmark_suite() {
        let mut suite = BenchmarkSuite::new();

        let stats1 = BenchmarkStats::new(
            "op1".to_string(),
            "test".to_string(),
            Duration::from_millis(10),
            100,
        );

        suite.add(stats1);

        let comparison = suite.format_comparison();
        assert!(comparison.contains("op1"));
        assert!(comparison.contains("test"));
    }

    // ── BenchmarkTable tests ──────────────────────────────────────────────────

    #[test]
    fn test_benchmark_table_empty() {
        let table = BenchmarkTable::new();
        assert!(table.format().contains("No benchmark results"));
    }

    #[test]
    fn test_benchmark_table_default_is_empty() {
        let table = BenchmarkTable::default();
        assert!(table.format().contains("No benchmark results"));
    }

    #[test]
    fn test_benchmark_table_single_row_contains_data() {
        let mut table = BenchmarkTable::new();
        table.add_row(BenchmarkTableRow {
            operation: "Pattern".to_string(),
            input: "rust".to_string(),
            output_example: "CVCC".to_string(),
            avg_duration: Duration::from_micros(5),
        });
        let rendered = table.format();
        assert!(rendered.contains("Pattern"));
        assert!(rendered.contains("rust"));
        assert!(rendered.contains("CVCC"));
    }

    #[test]
    fn test_benchmark_table_has_headers() {
        let mut table = BenchmarkTable::new();
        table.add_row(BenchmarkTableRow {
            operation: "X".to_string(),
            input: "y".to_string(),
            output_example: "z".to_string(),
            avg_duration: Duration::from_nanos(100),
        });
        let rendered = table.format();
        assert!(rendered.contains("Operation"));
        assert!(rendered.contains("Input"));
        assert!(rendered.contains("Output example"));
        assert!(rendered.contains("Avg time"));
    }

    #[test]
    fn test_benchmark_table_multiple_rows() {
        let mut table = BenchmarkTable::new();
        for (op, inp, ex) in &[
            ("Pattern", "rust", "CVCC"),
            ("Palindrome", "racecar", "palindrome ✓"),
            ("Syllable", "computer", "com-pu-ter"),
        ] {
            table.add_row(BenchmarkTableRow {
                operation: op.to_string(),
                input: inp.to_string(),
                output_example: ex.to_string(),
                avg_duration: Duration::from_micros(10),
            });
        }
        let rendered = table.format();
        assert!(rendered.contains("Pattern"));
        assert!(rendered.contains("Palindrome"));
        assert!(rendered.contains("Syllable"));
        assert!(rendered.contains("com-pu-ter"));
    }

    #[test]
    fn test_benchmark_to_table_row_fields() {
        let row = benchmark_to_table_row("Pattern", "rust", "CVCC", || {
            let _ = "rust".len();
        });
        assert_eq!(row.operation, "Pattern");
        assert_eq!(row.input, "rust");
        assert_eq!(row.output_example, "CVCC");
        // avg_duration must be non-negative (always true for Duration, just
        // assert it is accessible and the struct was constructed properly).
        let _ = row.avg_duration.as_nanos();
    }
}
