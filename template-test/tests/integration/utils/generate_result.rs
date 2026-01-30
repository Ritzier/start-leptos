use std::collections::BTreeMap;
use std::path::PathBuf;

use anyhow::{Context, Result};
use insta::{Settings, assert_json_snapshot};
use tempfile::TempDir;
use tokio::fs;
use tokio::process::Command;
use walkdir::{DirEntry, WalkDir};

use super::NAME;

/// Captures a generated template project for snapshot testing and linting
pub struct GenerateResult {
    pub temp_dir: TempDir,
}

#[derive(Debug)]
pub enum Content {
    String(String),
    Binary,
}

impl GenerateResult {
    /// Creates a new result from a temporary directory
    pub fn new(temp_dir: TempDir) -> Result<Self> {
        Ok(Self { temp_dir })
    }

    /// Runs complete integration test suite on generated template
    ///
    /// 1. **Snapshot test**: Verifies exact file structure/content via `insta`
    /// 2. **Clippy linting**: Ensures clean code quality
    pub async fn tests(&self, snapshot: &str) -> Result<()> {
        let proj_dir = self.get_path();

        // Clippy
        self.check_clippy(&proj_dir).await?;

        // Insta
        let mut settings = Settings::new();
        settings.set_snapshot_path("../snapshots");

        let files = self.to_snapshot().await?;
        let files_json = serde_json::to_string_pretty(&files)?;

        settings.bind(|| {
            assert_json_snapshot!(snapshot, files_json);
        });

        Ok(())
    }

    /// Returns the full path to the generated project directory
    fn get_path(&self) -> PathBuf {
        self.temp_dir.as_ref().to_path_buf().join(NAME)
    }

    /// Recursively collects all files from the generated project
    ///
    /// Filters out `.git/` directories entirely (no recursion) and hidden files.
    /// Handles binary files gracefully by marking them as `Content::Binary`.
    async fn collect_files(&self) -> Result<BTreeMap<PathBuf, Content>> {
        let root = self.get_path();
        let mut map = BTreeMap::new();

        // Filter .git at WALK TIME - skips recursion!
        for entry in WalkDir::new(&root)
            .follow_links(false)
            .into_iter()
            .filter_entry(is_visible) // ← KEY: skips .git entirely
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            let rel = path.strip_prefix(&root).unwrap().to_path_buf();

            match fs::read_to_string(path).await {
                Ok(content) => {
                    map.insert(rel, Content::String(content));
                }
                Err(e) if e.kind() == std::io::ErrorKind::InvalidData => {
                    map.insert(rel, Content::Binary);
                }
                Err(e) => return Err(e).with_context(|| format!("reading {}", path.display())),
            }
        }

        Ok(map)
    }

    /// Converts collected files to sorted JSON snapshot format
    ///
    /// Paths → strings, `Content::Binary` → `"binary"` placeholder.
    /// BTreeMap ensures consistent ordering for snapshots.
    async fn to_snapshot(&self) -> Result<BTreeMap<String, serde_json::Value>> {
        let files = self.collect_files().await?;
        let mut map = BTreeMap::new(); // Sorted keys!

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

    /// Run `cargo clippy` on the generated template project
    ///
    /// Verifies no errors/warnings/suggestions with `-D warnings`.
    /// Called by integration tests to ensure template quality.
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
                "`cargo clippy` failed with status {:?}\nStdout: {}\nStderr: {}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            )
        );

        Ok(())
    }
}

/// Skip hidden/system dirs/files at walkdir level
fn is_visible(entry: &DirEntry) -> bool {
    let name = entry.file_name().to_string_lossy();
    !(name == ".git" || name.starts_with('.'))
}
