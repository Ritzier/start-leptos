//! Cucumber test runner binary.
//!
//! This binary:
//! 1. Compiles the frontend WASM
//! 2. Starts the Leptos server
//! 3. Runs all `.feature` files in `e2e-tests/features/`
//!
//! # Usage
//! ```bash
//! cargo run --bin cucumber
//!
//! # With specific WebDriver
//! WEBDRIVER=geckodriver cargo run --bin cucumber
//! ```
//!
//! # Environment Variables
//! - `WEBDRIVER`: Choose driver (`chromedriver` or `geckodriver`)
//! - `RUST_LOG`: Set log level (e.g., `debug`, `info`)

use e2e_tests::{LeptosServer, Trace, cucumber_test};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Install color-eyre for beautiful error messages
    color_eyre::install()?;

    // Setup tracing subscriber for logging
    Trace::setup();

    // Compile frontend and start server (5 second timeout)
    LeptosServer::serve_and_wait(5).await?;

    // Run all feature files in e2e-tests/features/
    cucumber_test("e2e-tests/features").await?;

    Ok(())
}
