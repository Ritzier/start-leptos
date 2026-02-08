use clap::Parser;

/// Command-line interface for benchmark configuration.
///
/// # Usage
/// ```bash
/// cargo run --bin benchmark -- 20
/// cargo run --bin benchmark -- 100
/// ```
///
/// # Arguments
/// - `iteration`: Number of benchmark iterations to run
#[derive(Parser)]
pub struct Cli {
    /// Number of iterations to run for each benchmark.
    /// Higher values provide more accurate statistical analysis.
    ///
    /// # Examples
    /// - `20` - Quick test run
    /// - `100` - Production-level accuracy
    /// - `1000` - High-precision profiling
    pub iteration: usize,
}
