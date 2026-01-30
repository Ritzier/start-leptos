use std::collections::BTreeMap;
use std::path::PathBuf;

use anyhow::{Context, Result};
use tempfile::TempDir;
use tokio::fs;
use tokio::process::Command;
use walkdir::{DirEntry, WalkDir};

use super::NAME;

pub struct GenerateResult {
    pub temp_dir: TempDir,
}

#[derive(Debug)]
pub enum Content {
    String(String),
    Binary,
}

impl GenerateResult {
    pub fn new(temp_dir: TempDir) -> Result<Self> {
        Ok(Self { temp_dir })
    }

    pub fn get_path(&self) -> PathBuf {
        self.temp_dir.as_ref().to_path_buf().join(NAME)
    }

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

    pub async fn to_snapshot(&self) -> Result<BTreeMap<String, serde_json::Value>> {
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

    /// Run `clippy` on the generated template, check errors, warnings, suggestions
    pub async fn check_clippy(&self) -> Result<()> {
        let proj_dir = self.get_path();

        let output = Command::new("cargo")
            .current_dir(&proj_dir)
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
