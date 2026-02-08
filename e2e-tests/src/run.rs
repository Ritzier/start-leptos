//! Cucumber test execution utilities.
//!
//! Scans a directory for `.feature` files and runs them through Cucumber.

use std::ffi::OsStr;
use std::path::Path;

use color_eyre::eyre::Result;
use cucumber::World;
use tokio::fs;

use crate::AppWorld;

/// Runs all Cucumber feature files in a directory.
///
/// This function:
/// 1. Scans the directory for `.feature` files
/// 2. Runs each file through Cucumber
/// 3. Fails if any scenario fails or is skipped
///
/// # Arguments
/// * `path` - Directory containing `.feature` files
///
/// # Errors
/// - Directory doesn't exist
/// - Feature file parsing fails
/// - Any scenario fails
///
/// # Example
/// ```rust
/// cucumber_test("e2e-tests/features").await?;
/// ```
pub async fn cucumber_test<P: AsRef<Path>>(path: P) -> Result<()> {
    let mut dir = fs::read_dir(path).await?;

    // Iterate through all files in directory
    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();

        // Only process .feature files
        if path.extension() == Some(OsStr::new("feature")) {
            AppWorld::cucumber()
                .fail_on_skipped() // Treat skipped tests as failures
                .run_and_exit(path) // Run and exit with appropriate code
                .await;
        }
    }

    Ok(())
}
