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
//! # Environment Variables
//! - `RUST_LOG`: Set log level (e.g., `debug`, `info`)

use color_eyre::Result;
use color_eyre::eyre::eyre;
use e2e_tests::{ChromeDriver, Trace, cucumber_test};

#[tokio::main]
async fn main() -> Result<()> {
    // Install color-eyre for beautiful error messages
    color_eyre::install()?;

    // Setup tracing subscriber for logging
    Trace::setup();

    // Spawn `ChromeDriver` process
    let chrome_driver = ChromeDriver::new().await.map_err(|e| eyre!("{e:?}"))?;

    let result: Result<()> = async {
        // Run all feature files in `e2e-tests/features/` folder
        cucumber_test("e2e-tests/features").await?;
        Ok(())
    }
    .await;

    // Shut down `ChromeDriver`
    chrome_driver.shutdown().await.map_err(|e| eyre!("{e:?}"))?;

    // Propagate any error from the test run
    result?;

    Ok(())
}
