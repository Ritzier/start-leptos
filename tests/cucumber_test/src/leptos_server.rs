use std::path::Path;
use std::process::Stdio;

use color_eyre::{Result, eyre};
use server::Server;
use tokio::fs;
use tokio::process::Command;

use crate::PortFinder;

pub struct LeptosServer;

impl LeptosServer {
    pub async fn build() -> Result<()> {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .unwrap();

        println!("Compiling");
        let output = Command::new("cargo")
            .arg("leptos")
            .arg("build")
            .arg("--split")
            .current_dir(manifest_dir)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);

            return Err(eyre::eyre!(
                "`cargo leptos build` failed\n\
            Exit code: {:?}\n\
            Stderr: {}\n\
            Stdout: {}",
                output.status.code(),
                stderr.trim(),
                stdout.trim()
            ));
        }

        Ok(())
    }

    pub async fn serve() -> Result<()> {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .unwrap();
        let cargo_toml_path = manifest_dir.join("Cargo.toml");
        let port = PortFinder::get_available_port().await?;

        Server::cucumber_setup(port, Some(cargo_toml_path.to_str().unwrap())).await?;

        Ok(())
    }
}
