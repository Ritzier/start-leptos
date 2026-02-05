use std::collections::BTreeMap;
use std::path::PathBuf;

use anyhow::{Context, Result};
use insta::{Settings, assert_json_snapshot};
use tempfile::TempDir;
use tokio::fs;
use tokio::process::Command;
use walkdir::{DirEntry, WalkDir};

use super::{CargoGenerate, NAME};

/// Represents the result of a `cargo-generate` template generation
///
/// Provides methods to validate generated projects through automated testing:
/// - Type checking with `cargo check`
/// - Linting with `cargo clippy`
/// - Snapshot testing with `insta`
/// - End-to-end testing with Cucumber (if enabled)
///
/// # Example
/// ```no_run
/// let result = GenerateResult::new(temp_dir, config)?;
/// result.tests("my_snapshot").await?;
/// ```
pub struct GenerateResult {
    pub temp_dir: TempDir,
    config: CargoGenerate,
}

#[derive(Debug)]
pub enum Content {
    String(String),
    Binary,
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
    /// 3. **Snapshot**: `insta` JSON snapshot for file structure verification
    /// 4. **E2E Tests**: `cucumber_test` (if cucumber feature enabled)    /// # Arguments
    ///
    /// * `snapshot` - Snapshot name for `insta` (e.g., `"my_template"`)
    pub async fn tests(&self, snapshot: &str) -> Result<()> {
        let proj_dir = self.get_path();

        // Step 1: Type checking
        self.cargo_check(&proj_dir).await?;

        // Step 2: Linting
        self.check_clippy(&proj_dir).await?;

        // Step 3: Snapshot testing
        self.insta(snapshot).await?;

        // Step 4: End-to-den testing (conditional)
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

    /// Creates and verifies an `insta` JSON snapshot
    ///
    /// Captures the entire project file structure and content, then compares
    /// against a stored snapshot to detect unintended changes.
    ///
    /// # Arguments
    /// * `snapshot` - Name for the snapshot file (stored in `../snapshots/`)
    ///
    /// # Errors
    /// Returns error if snapshot creation or comparison fails
    async fn insta(&self, snapshot: &str) -> Result<()> {
        let mut settings = Settings::new();
        settings.set_snapshot_path("../snapshots");

        let files = self.to_snapshot().await?;
        let files_json = serde_json::to_string_pretty(&files)?;

        settings.bind(|| {
            assert_json_snapshot!(snapshot, files_json);
        });

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

// ===== File Collection & Snapshot Helpers (private) =====
impl GenerateResult {
    /// Returns the absolute path to the generated project root
    fn get_path(&self) -> PathBuf {
        self.temp_dir.as_ref().to_path_buf().join(NAME)
    }

    /// Recursively collects all files from the generated project
    ///
    /// # Filtering Rules
    /// - Excludes `.git/` directories (skips recursion entirely)
    /// - Excludes hidden files/directories (starting with `.`)
    /// - Handles binary files gracefully as `Content::Binary`
    ///
    /// # Returns
    /// A sorted map of relative paths to file content
    ///
    /// # Errors
    /// Returns error if file reading fails (except for binary files)
    async fn collect_files(&self) -> Result<BTreeMap<PathBuf, Content>> {
        let root = self.get_path();
        let mut map = BTreeMap::new();

        // Walk the directory tree, filtering out hidden/system directories
        for entry in WalkDir::new(&root)
            .follow_links(false)
            .into_iter()
            .filter_entry(is_visible) // Skip .git and hidden files at walk-time
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            let rel = path.strip_prefix(&root).unwrap().to_path_buf();

            // Attempt to read as UTF-8 text, fallback to binary marker
            match fs::read_to_string(path).await {
                Ok(content) => {
                    map.insert(rel, Content::String(content));
                }
                Err(e) if e.kind() == std::io::ErrorKind::InvalidData => {
                    // Mark binary files instead of failing
                    map.insert(rel, Content::Binary);
                }
                Err(e) => return Err(e).with_context(|| format!("reading {}", path.display())),
            }
        }

        Ok(map)
    }

    /// Converts collected files to JSON-serializable snapshot format
    ///
    /// # Transformations
    /// - Converts `PathBuf` to strings for JSON serialization
    /// - Replaces `Content::Binary` with `"binary"` placeholder
    /// - Maintains sorted order via `BTreeMap` for consistent snapshots
    ///
    /// # Returns
    /// A sorted map of file paths (as strings) to JSON values
    async fn to_snapshot(&self) -> Result<BTreeMap<String, serde_json::Value>> {
        let files = self.collect_files().await?;
        let mut map = BTreeMap::new();

        for (path, content) in files {
            let key = path.to_string_lossy().to_string();
            let value = match content {
                Content::String(s) => serde_json::Value::String(s),
                Content::Binary => "binary".into(),
            };
            map.insert(key, value);
        }
        Ok(map)
    }
}

/// Skip hidden/system dirs/files at walkdir level
fn is_visible(entry: &DirEntry) -> bool {
    let name = entry.file_name().to_string_lossy();
    !(name == ".git" || name.starts_with('.'))
}
