use std::collections::HashMap;
use std::time::Duration;

use color_eyre::owo_colors::OwoColorize;

/// Stores and analyzes benchmark results with dynamic string-based keys.
///
/// This structure allows flexible benchmark tracking where test names can be
/// determined at runtime. Each benchmark name maps to a vector of timing
/// measurements collected across multiple iterations.
///
/// # Design Choices
/// - **HashMap<String, Vec<u128>>**: Allows dynamic benchmark names without enum changes
/// - **u128 milliseconds**: High precision for sub-millisecond measurements
/// - **Vec storage**: Preserves all raw data for statistical analysis
///
/// # Example
/// ```
/// let mut results = BenchmarkResults::new(100);
///
/// for i in 0..100 {
///     let start = Instant::now();
///     perform_operation();
///     results.add_timing("operation", start.elapsed());
/// }
///
/// results.print_summary(); // Prints avg, min, max, median, stddev
/// ```
#[derive(Debug)]
pub struct BenchmarkResults {
    /// Total number of iterations planned for each benchmark.
    /// Used for display purposes and capacity pre-allocation.
    iteration: usize,

    /// Maps benchmark names to vectors of timing measurements in milliseconds.
    /// Using HashMap allows benchmarks to be added dynamically at runtime
    /// without modifying the struct definition.
    timings: HashMap<String, Vec<u128>>,
}

impl BenchmarkResults {
    /// Creates a new benchmark results collector.
    ///
    /// # Arguments
    /// * `iteration` - Expected number of iterations per benchmark (for display only)
    ///
    /// # Example
    /// ```
    /// let results = BenchmarkResults::new(50);
    /// ```
    pub fn new(iteration: usize) -> Self {
        Self {
            iteration,
            timings: HashMap::new(),
        }
    }

    /// Adds a timing measurement for a specific benchmark.
    ///
    /// If the benchmark name doesn't exist, it's automatically created.
    /// Timings are stored in milliseconds as u128 for high precision.
    ///
    /// # Arguments
    /// * `name` - Benchmark identifier (accepts String, &str, or any Into<String>)
    /// * `duration` - Elapsed time measurement from `Instant::elapsed()`
    ///
    /// # Example
    /// ```
    /// results.add_timing("connect", Duration::from_millis(45));
    /// results.add_timing("disconnect", Duration::from_millis(23));
    ///
    /// // Dynamic names work too
    /// results.add_timing(format!("test_{}", id), duration);
    /// ```
    pub fn add_timing(&mut self, name: impl Into<String>, duration: Duration) {
        self.timings
            .entry(name.into())
            .or_default()
            .push(duration.as_millis());
    }

    /// Prints a formatted, colorized summary of all benchmark results.
    ///
    /// Benchmarks are displayed in alphabetical order for consistency.
    /// Each benchmark shows: average, minimum, maximum, median, and standard deviation.
    ///
    /// # Output Format
    /// ```text
    /// === Summary (20 iterations) ===
    /// connect: avg=45ms, min=32ms, max=78ms, median=43ms, stddev=8.23ms
    /// disconnect: avg=23ms, min=15ms, max=45ms, median=21ms, stddev=5.67ms
    /// ```
    ///
    /// # Color Coding
    /// - Header: Bright yellow + bold
    /// - Benchmark names: Bright cyan + bold
    /// - Average: Yellow
    /// - Min: Green (best case)
    /// - Max: Red (worst case)
    /// - Median: Blue
    /// - Std Dev: Magenta
    pub fn print_summary(&self) {
        println!(
            "\n{}",
            format!("=== Summary ({} iterations) ===", self.iteration)
                .bright_yellow()
                .bold()
        );

        // Sort keys alphabetically for consistent output across runs
        let mut keys: Vec<_> = self.timings.keys().collect();
        keys.sort();

        for key in keys {
            if let Some(timings) = self.timings.get(key) {
                self.print_stats(key, timings);
            }
        }
    }

    /// Prints colorized statistics for a single benchmark.
    ///
    /// # Arguments
    /// * `name` - Benchmark name to display
    /// * `timings` - Slice of timing measurements in milliseconds
    ///
    /// # Color Coding
    /// - Benchmark name: Cyan + Bold
    /// - Average: Yellow (typical performance)
    /// - Minimum: Green (best case scenario)
    /// - Maximum: Red (worst case / outliers)
    /// - Median: Blue (robust central tendency)
    /// - Std Dev: Magenta (consistency indicator - lower is more predictable)
    fn print_stats(&self, name: &str, timings: &[u128]) {
        // Handle edge case where no data was collected
        if timings.is_empty() {
            println!("{}: {}", name.bright_cyan().bold(), "No data".red());
            return;
        }

        let stats = Statistics::from_timings(timings);

        println!(
            "{}: avg={}ms, min={}ms, max={}ms, median={}ms, stddev={:.2}ms",
            name.bright_cyan().bold(),
            stats.avg.to_string().yellow(),
            stats.min.to_string().green(),
            stats.max.to_string().red(),
            stats.median.to_string().blue(),
            format!("{:.2}", stats.stddev).magenta()
        );
    }

    /// Returns timing measurements for a specific benchmark.
    ///
    /// Useful for custom analysis or exporting data.
    ///
    /// # Arguments
    /// * `name` - Benchmark name to query
    ///
    /// # Returns
    /// * `Some(&[u128])` - Slice of timings if benchmark exists
    /// * `None` - If benchmark name not found
    ///
    /// # Example
    /// ```
    /// if let Some(timings) = results.get_timings("connect") {
    ///     println!("Collected {} samples", timings.len());
    ///     let sum: u128 = timings.iter().sum();
    ///     println!("Total time: {}ms", sum);
    /// }
    /// ```
    pub fn get_timings(&self, name: &str) -> Option<&[u128]> {
        self.timings.get(name).map(|v| v.as_slice())
    }

    /// Returns all benchmark names that have collected data.
    ///
    /// Names are returned in arbitrary order (HashMap iteration order).
    /// Use this to iterate over all available benchmarks dynamically.
    ///
    /// # Example
    /// ```
    /// for name in results.benchmark_names() {
    ///     println!("Benchmark '{}' has data", name);
    ///     if let Some(timings) = results.get_timings(name) {
    ///         println!("  Sample count: {}", timings.len());
    ///     }
    /// }
    /// ```
    pub fn benchmark_names(&self) -> Vec<&str> {
        self.timings.keys().map(|s| s.as_str()).collect()
    }
}

/// Statistical analysis of timing measurements.
///
/// Provides common statistical metrics for performance analysis:
/// - **Average (Mean)**: Sum of all values / count - affected by outliers
/// - **Min/Max**: Best and worst case performance
/// - **Median**: Middle value when sorted - robust against outliers
/// - **Std Dev**: Consistency measure - lower values mean more predictable performance
///
/// # Statistical Notes
/// - Median is preferred over average for skewed distributions
/// - Standard deviation indicates consistency (lower = more predictable)
/// - Min/Max help identify outliers and edge cases
#[derive(Debug, Clone)]
pub struct Statistics {
    /// Arithmetic mean of all measurements.
    /// Sensitive to outliers - a few slow measurements can skew this high.
    pub avg: u128,

    /// Minimum observed value (best case performance).
    /// Useful for identifying optimal conditions.
    pub min: u128,

    /// Maximum observed value (worst case performance).
    /// Helps identify performance bottlenecks or outliers.
    pub max: u128,

    /// Middle value when sorted (50th percentile).
    /// More robust than average for skewed distributions.
    pub median: u128,

    /// Standard deviation (measure of spread/consistency).
    /// Lower values indicate more predictable performance.
    /// Formula: sqrt(sum((x - mean)²) / N)
    pub stddev: f64,
}

impl Statistics {
    /// Calculates statistical metrics from a slice of timing measurements.
    ///
    /// # Arguments
    /// * `timings` - Slice of measurements in milliseconds
    ///
    /// # Returns
    /// Statistics struct with all metrics computed. Returns zeros for empty input.
    ///
    /// # Algorithm Notes
    /// - **Average**: Simple arithmetic mean (sum / count)
    /// - **Median**: For even-length arrays, averages two middle values
    /// - **Std Dev**: Population standard deviation (divides by N, not N-1)
    ///   - Uses N because we're analyzing the entire population, not a sample
    ///
    /// # Example
    /// ```
    /// let timings = vec!;[1][2][3][4][5]
    /// let stats = Statistics::from_timings(&timings);
    ///
    /// println!("Average: {}ms", stats.avg);     // 13ms
    /// println!("Median: {}ms", stats.median);   // 14ms
    /// println!("Range: {}-{}ms", stats.min, stats.max); // 10-18ms
    /// ```
    pub fn from_timings(timings: &[u128]) -> Self {
        // Handle edge case: no data provided
        if timings.is_empty() {
            return Self {
                avg: 0,
                min: 0,
                max: 0,
                median: 0,
                stddev: 0.0,
            };
        }

        // Calculate average (arithmetic mean)
        let sum: u128 = timings.iter().sum();
        let avg = sum / timings.len() as u128;

        // Find min and max values in O(n)
        let min = *timings.iter().min().unwrap();
        let max = *timings.iter().max().unwrap();

        // Calculate median (middle value when sorted)
        let mut sorted = timings.to_vec();
        sorted.sort_unstable(); // O(n log n) but doesn't preserve original order

        let median = if sorted.len().is_multiple_of(2) {
            // Even number of elements: average the two middle values
            // Example: [1, 2, 3, 4] -> (2 + 3) / 2 = 2.5
            let mid = sorted.len() / 2;
            (sorted[mid - 1] + sorted[mid]) / 2
        } else {
            // Odd number: take the exact middle element
            // Example: [1, 2, 3] -> 2
            sorted[sorted.len() / 2]
        };

        // Calculate standard deviation (measure of spread)
        // Formula: sqrt(sum((x - mean)²) / N)
        // Steps:
        // 1. For each value, compute difference from mean
        // 2. Square the difference
        // 3. Sum all squared differences
        // 4. Divide by count (population std dev, not sample)
        // 5. Take square root
        let variance: f64 = timings
            .iter()
            .map(|&x| {
                let diff = x as f64 - avg as f64;
                diff * diff // Square the difference
            })
            .sum::<f64>()
            / timings.len() as f64; // Divide by N (population std dev)

        let stddev = variance.sqrt();

        Self {
            avg,
            min,
            max,
            median,
            stddev,
        }
    }
}
