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

use e2e_tests::{ChromeDriver, LeptosServer, Trace, cucumber_test};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Install color-eyre for beautiful error messages
    color_eyre::install()?;

    // Setup tracing subscriber for logging
    Trace::setup();

    // Spawn `ChromeDriver` process
    let chrome_driver = ChromeDriver::new().await?;

    let result: color_eyre::Result<()> = async {
        // Compile frontend and start server (5 second timeout)
        LeptosServer::serve_and_wait(5).await?;

        // Run all feature files in e2e-tests/features/
        cucumber_test("e2e-tests/features").await?;
        Ok(())
    }
    .await;

    // Shut down `ChromeDriver`
    chrome_driver.shutdown().await?;

    // Propagate any error from the test run
    result?;

    Ok(())
}
