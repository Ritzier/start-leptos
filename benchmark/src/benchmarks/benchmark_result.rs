use std::collections::HashMap;
use std::time::Duration;

use color_eyre::owo_colors::OwoColorize;

/// Stores and analyzes benchmark results with dynamic string-based keys
#[derive(Debug)]
pub struct BenchmarkResults {
    iteration: usize,
    timings: HashMap<String, Vec<u128>>,
}

impl BenchmarkResults {
    pub fn new(iteration: usize) -> Self {
        Self {
            iteration,
            timings: HashMap::new(),
        }
    }

    /// Adds a timing result for any benchmark name
    pub fn add_timing(&mut self, name: impl Into<String>, duration: Duration) {
        self.timings
            .entry(name.into())
            .or_default()
            .push(duration.as_millis());
    }

    /// Prints a formatted summary of all benchmark results
    pub fn print_summary(&self) {
        println!(
            "\n{}",
            format!("=== Summary ({} iterations) ===", self.iteration)
                .bright_yellow()
                .bold()
        );

        // Sort keys alphabetically for consistent output
        let mut keys: Vec<_> = self.timings.keys().collect();
        keys.sort();

        for key in keys {
            if let Some(timings) = self.timings.get(key) {
                self.print_stats(key, timings);
            }
        }
    }

    fn print_stats(&self, name: &str, timings: &[u128]) {
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

    /// Returns timings for a specific benchmark name
    pub fn get_timings(&self, name: &str) -> Option<&[u128]> {
        self.timings.get(name).map(|v| v.as_slice())
    }

    /// Returns all benchmark names
    pub fn benchmark_names(&self) -> Vec<&str> {
        self.timings.keys().map(|s| s.as_str()).collect()
    }
}

// Statistics struct stays the same...
#[derive(Debug, Clone)]
pub struct Statistics {
    pub avg: u128,
    pub min: u128,
    pub max: u128,
    pub median: u128,
    pub stddev: f64,
}

impl Statistics {
    pub fn from_timings(timings: &[u128]) -> Self {
        if timings.is_empty() {
            return Self {
                avg: 0,
                min: 0,
                max: 0,
                median: 0,
                stddev: 0.0,
            };
        }

        let sum: u128 = timings.iter().sum();
        let avg = sum / timings.len() as u128;
        let min = *timings.iter().min().unwrap();
        let max = *timings.iter().max().unwrap();

        let mut sorted = timings.to_vec();
        sorted.sort_unstable();
        let median = if sorted.len().is_multiple_of(2) {
            let mid = sorted.len() / 2;
            (sorted[mid - 1] + sorted[mid]) / 2
        } else {
            sorted[sorted.len() / 2]
        };

        let variance: f64 = timings
            .iter()
            .map(|&x| {
                let diff = x as f64 - avg as f64;
                diff * diff
            })
            .sum::<f64>()
            / timings.len() as f64;
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
