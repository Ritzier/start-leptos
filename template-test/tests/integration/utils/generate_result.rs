use std::path::PathBuf;

use anyhow::{Context, Result};
use tempfile::TempDir;
use tokio::process::Command;

use super::{CargoGenerate, NAME};

/// Represents the result of a `cargo-generate` template generation
///
/// Provides methods to validate generated projects through automated testing:
/// - Type checking with `cargo check`
/// - Linting with `cargo clippy`
/// - End-to-end testing with Cucumber (if enabled)
///
/// # Example
/// ```no_run
/// let result = GenerateResult::new(temp_dir, config)?;
/// result.tests().await?;
/// ```
pub struct GenerateResult {
    pub temp_dir: TempDir,
    config: CargoGenerate,
}

// ===== Public API =====
impl GenerateResult {
    /// Creates a new result from a temporary directory
    pub fn new(temp_dir: TempDir, config: CargoGenerate) -> Result<Self> {
        Ok(Self { temp_dir, config })
    }

    /// Runs complete integration test suite on the generated template
    ///
    /// # Test Pipeline (executed in order)
    /// 1. **Type Check**: `cargo check --workspace --features ssr hydrate`
    /// 2. **Linting**: `cargo clippy -D warnings` (treats all warnings as errors)
    /// 3. **E2E Tests**: `cucumber_test` (if cucumber feature enabled)    /// # Arguments
    pub async fn tests(&self) -> Result<()> {
        let proj_dir = self.get_path();

        // Step 1: Type checking
        self.cargo_check(&proj_dir).await?;

        // Step 2: Linting
        self.check_clippy(&proj_dir).await?;

        // Step 3: End-to-end testing (conditional)
        if self.config.cucumber {
            self.cucumber_test(&proj_dir).await?;
        }

        Ok(())
    }
}

// ===== Validation Methods =====
impl GenerateResult {
    /// Runs `cargo check --workspace --features ssr --features hydrate`
    ///
    /// Verifies that the generated project compiles without errors.
    /// Checks both SSR (server-side rendering) and hydrate features.
    ///
    /// # Errors
    /// Returns error if compilation fails
    async fn cargo_check(&self, proj_dir: &PathBuf) -> Result<()> {
        let output = Command::new("cargo")
            .current_dir(proj_dir)
            .arg("check")
            .arg("--workspace")
            .arg("--features")
            .arg("ssr")
            .arg("--features")
            .arg("hydrate")
            .output()
            .await
            .context("`cargo check --features ssr --features hydrate` failed")?;

        anyhow::ensure!(
            output.status.success(),
            anyhow::anyhow!(
                "`cargo check` failed with status {:?}\nStdout:\n{}\n\nStderr:\n{}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            )
        );

        Ok(())
    }

    /// Runs `cargo clippy -- -D warnings`
    ///
    /// Ensures the generated code passes all Clippy lints without warnings.
    /// The `-D warnings` flag treats warnings as compilation errors.
    ///
    /// # Errors
    /// Returns error if any clippy warnings/errors are found
    async fn check_clippy(&self, proj_dir: &PathBuf) -> Result<()> {
        let output = Command::new("cargo")
            .current_dir(proj_dir)
            .arg("clippy")
            .arg("--")
            .arg("-D")
            .arg("warnings")
            .output()
            .await
            .context("`cargo clippy` failed")?;

        anyhow::ensure!(
            output.status.success(),
            anyhow::anyhow!(
                "`cargo clippy` failed with status {:?}\nStdout:\n{}\n\nStderr:\n{}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            )
        );

        Ok(())
    }

    /// Runs `cargo run --package cucumber_test`
    ///
    /// Executes end-to-end Cucumber BDD tests if the template was generated
    /// with Cucumber support enabled.
    ///
    /// # Errors
    /// Returns error if Cucumber tests fail
    async fn cucumber_test(&self, proj_dir: &PathBuf) -> Result<()> {
        let output = Command::new("cargo")
            .current_dir(proj_dir)
            .arg("run")
            .arg("--package")
            .arg("cucumber_test")
            .output()
            .await
            .context("`cargo run --package cucumber_test` failed")?;

        anyhow::ensure!(
            output.status.success(),
            anyhow::anyhow!(
                "`cargo run --package cucumber_test` failed with status {:?}\nStdout:\n{}\n\nStderr:\n{}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            )
        );

        Ok(())
    }
}

impl GenerateResult {
    /// Returns the absolute path to the generated project root
    fn get_path(&self) -> PathBuf {
        self.temp_dir.as_ref().to_path_buf().join(NAME)
    }
}
