use std::path::Path;
use std::process::Stdio;
use std::time::Duration;

use color_eyre::{Result, eyre};
use server::Server;
use tokio::process::Command;
use tokio::sync::oneshot;

use crate::{PortFinder, set_server_addr};

pub struct LeptosServer;

impl LeptosServer {
    /// Compiles the `Frontend` with `cargo-leptos`
    async fn compile_frontend() -> Result<()> {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .ok_or_else(|| eyre::eyre!("Failed to find project root directory"))?;

        let output = Command::new("cargo")
            .arg("leptos")
            .arg("build")
            .arg("--split")
            .arg("--frontend-only")
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

    /// Starts the server and signals readiness via oneshot channel
    ///
    /// Finds an available port, binds the server, and sends a signal
    /// through the oneshot channel once the server is ready to accept requests
    async fn serve(sender: oneshot::Sender<()>) -> Result<()> {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .ok_or_else(|| eyre::eyre!("Failed to find project root directory"))?;
        let cargo_toml_path = manifest_dir.join("Cargo.toml");

        // Find an available port to avoid conflicts
        let port = PortFinder::get_available_port()
            .await
            .map_err(|e| eyre::eyre!("{e}"))?;
        let addr = std::net::SocketAddr::new(
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
            port,
        );

        // Store address for tests to access
        set_server_addr(addr);

        // Start server
        let cargo_toml_str = cargo_toml_path
            .to_str()
            .ok_or_else(|| eyre::eyre!("Invalid UTF-8 in Cargo.toml path"))?;

        Server::cucumber_setup(addr, Some(cargo_toml_str), sender).await?;

        Ok(())
    }

    /// Builds the frontend and starts the server, waiting for it to be ready
    ///
    /// This method:
    /// 1. Compiles the frontend WASM
    /// 2. Spawns the server in a background task
    /// 3. Waits for the server to signal readiness (or times out)
    ///
    /// # Arguments
    /// * `timeout_secs` - Maximum seconds to wait for server startup
    ///
    /// # Errors
    /// Returns error if:
    /// - Frontend compilation fails
    /// - Server crashes during startup
    /// - Timeout is reached before server is ready
    pub async fn serve_and_wait(timeout: u64) -> Result<()> {
        // Compile frontend
        Self::compile_frontend().await?;

        // Create channel for server
        let (tx, rx) = oneshot::channel();

        // Spawn server in task
        let server_handle = tokio::spawn(async move {
            if let Err(e) = Self::serve(tx).await {
                eprintln!("Server error: {e}");
            }
        });

        // Wait for server to be ready with timeout
        match tokio::time::timeout(Duration::from_secs(timeout), rx).await {
            Ok(Ok(())) => {
                println!("Server is ready!");
                Ok(())
            }
            Ok(Err(_)) => {
                server_handle.abort();
                Err(eyre::eyre!("Server crashed before becoming ready"))
            }
            Err(_) => {
                server_handle.abort();
                Err(eyre::eyre!(
                    "Server failed to start within {} seconds",
                    timeout
                ))
            }
        }
    }
}
