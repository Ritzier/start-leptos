const NAME: &str = "project-template";

use std::fmt::Display;
use std::path::Path;

use anyhow::{Context, Result};
use tempfile::TempDir;
use tokio::fs;
use tokio::process::Command;

mod generate_result;
pub use generate_result::GenerateResult;

#[derive(Debug, Default)]
pub struct CargoGenerate {
    pub websocket: bool,
    pub tracing: bool,
    pub style: Style,
    pub docker: bool,
    pub cucumber: bool,
}

#[derive(Debug, Default)]
pub enum Style {
    #[default]
    Default,
    Unocss,
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Default => "default",
            Self::Unocss => "unocss",
        };

        write!(f, "{str}")
    }
}

impl CargoGenerate {
    pub async fn build(self) -> Result<GenerateResult> {
        let Self {
            websocket,
            tracing,
            style,
            docker,
            cucumber,
        } = &self;

        let tempfile = TempDir::new()?;
        let output_dir = tempfile.as_ref().to_path_buf();

        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let template_dir = manifest_dir.ancestors().nth(1).context("No parent dir")?;

        let mut binding = Command::new("cargo");
        let cmd = binding
            .arg("generate")
            .arg("--path")
            .arg(template_dir.display().to_string())
            .arg("--name")
            .arg(NAME)
            .arg("--destination")
            .arg(output_dir.display().to_string())
            .arg("-d")
            .arg(format!(
                "websocket={}",
                websocket.to_string().to_lowercase()
            ))
            .arg("-d")
            .arg(format!("tracing={}", tracing.to_string().to_lowercase()))
            .arg("-d")
            .arg(format!("style={}", style))
            .arg("-d")
            .arg(format!("docker={}", docker.to_string().to_lowercase()))
            .arg("-d")
            .arg(format!("cucumber={}", cucumber.to_string().to_lowercase()));

        unsafe {
            cmd.pre_exec(move || {
                libc::ioctl(libc::STDOUT_FILENO, libc::TIOCSCTTY, 0);
                Ok(())
            });
        }

        let output = cmd.output().await.context("`cargo generate` failed")?;

        anyhow::ensure!(
            output.status.success(),
            anyhow::anyhow!(
                "`cargo generate` failed with status {:?}\nStdout: {}\nStderr: {}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            )
        );

        // Verify generated project dir exists
        let proj_dir = output_dir.join(NAME);
        let proj_meta = fs::metadata(&proj_dir)
            .await
            .context("test-proj dir missing")?;
        anyhow::ensure!(proj_meta.is_dir(), "test-proj is not a directory");

        // Generate Result
        GenerateResult::new(tempfile, self)
    }
}
