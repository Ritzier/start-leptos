//! Benchmark binary entry point.
//!
//! Runs performance benchmarks against a Leptos application,
//! measuring interaction timings and generating statistical reports.
//!
//! # Usage
//! ```bash
//! cargo run --bin benchmark -- <iterations>
//! ```
//!
//! # Examples
//! ```bash
//! # Run 20 iterations
//! cargo run --bin benchmark -- 20
//!
//! # Run 100 iterations for better accuracy
//! cargo run --bin benchmark -- 100
//! ```
//!
//! # Output
//! The benchmark prints per-iteration timings and a final summary with:
//! - Average (mean value)
//! - Minimum (best case)
//! - Maximum (worst case)
//! - Median (50th percentile)
//! - Standard deviation (consistency measure)

use benchmark::{Benchmarks, Cli, Error};
use clap::Parser;
use color_eyre::Result;
use e2e_tests::Webdriver;

#[tokio::main]
async fn main() -> Result<()> {
    // Install color-eyre for better error messages
    color_eyre::install()?;

    // Parse command-line arguments
    let Cli { iteration } = Cli::parse();

    // Spawn `ChromeDriver` command
    let chrome_driver = Webdriver::spawn_chrome_driver()
        .await
        .map_err(Error::from)?;

    // Initialize benchmark runner (starts server, connects WebDriver)
    let benchmark = Benchmarks::new(iteration).await?;

    // Run all iterations and collect results
    let results = benchmark.start().await?;

    // Print colorized statistical summary
    results.print_summary();

    // Shutdown `ChromeDriver`
    chrome_driver.shutdown().await.map_err(Error::from)?;

    Ok(())
}
