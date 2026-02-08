//! Leptos server lifecycle management for testing.
//!
//! Handles frontend compilation and server startup with readiness signaling.

use std::path::Path;
use std::process::Stdio;
use std::time::Duration;

use color_eyre::{Result, eyre};
use server::Server;
use tokio::process::Command;
use tokio::sync::oneshot;

use crate::{PortFinder, set_server_addr};

/// Leptos server manager for e2e tests.
pub struct LeptosServer;

impl LeptosServer {
    /// Compiles the frontend WASM using `cargo-leptos`.
    ///
    /// Runs `cargo leptos build --split --frontend-only --release`
    /// in the project root directory.
    ///
    /// # Errors
    /// - `cargo-leptos` not installed
    /// - Compilation fails
    /// - Project root not found
    ///
    /// # Notes
    /// - Stdout and stderr are suppressed for cleaner test output
    /// - Uses `--split` for code splitting
    /// - Uses `--release` for optimized builds
    async fn compile_frontend() -> Result<()> {
        // Navigate to project root (parent of e2e-tests/)
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(1)
            .ok_or_else(|| eyre::eyre!("Failed to find project root directory"))?;

        let output = Command::new("cargo")
            .arg("leptos")
            .arg("build")
            .arg("--split") // Enable code splitting
            .arg("--frontend-only") // Only build WASM, not server
            .arg("--release") // Optimized build
            .current_dir(manifest_dir)
            .stdout(Stdio::null()) // Suppress stdout
            .stderr(Stdio::null()) // Suppress stderr
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

    /// Starts the Leptos server on an available port.
    ///
    /// This method:
    /// 1. Finds an available port (8000-8999)
    /// 2. Stores the address globally for tests to access
    /// 3. Starts the server
    /// 4. Signals readiness via oneshot channel
    ///
    /// # Arguments
    /// * `sender` - Oneshot channel to signal server readiness
    ///
    /// # Errors
    /// - No available ports
    /// - Server fails to bind
    /// - Cargo.toml path invalid
    async fn serve(sender: oneshot::Sender<()>) -> Result<()> {
        // Navigate to project root
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(1)
            .ok_or_else(|| eyre::eyre!("Failed to find project root directory"))?;
        let cargo_toml_path = manifest_dir.join("Cargo.toml");

        // Find an available port to avoid conflicts with other tests
        let port = PortFinder::get_available_port()
            .await
            .map_err(|e| eyre::eyre!("{e}"))?;
        let addr = std::net::SocketAddr::new(
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
            port,
        );

        // Store address in global static for AppWorld to access
        set_server_addr(addr);

        // Start server with cucumber-specific setup
        let cargo_toml_str = cargo_toml_path
            .to_str()
            .ok_or_else(|| eyre::eyre!("Invalid UTF-8 in Cargo.toml path"))?;

        Server::cucumber_setup(addr, Some(cargo_toml_str), sender).await?;

        Ok(())
    }

    /// Builds the frontend and starts the server, waiting for readiness.
    ///
    /// This is the main entry point for test setup. It:
    /// 1. Compiles the frontend WASM
    /// 2. Spawns the server in a background task
    /// 3. Waits for the server to signal readiness (or times out)
    ///
    /// # Arguments
    /// * `timeout` - Maximum seconds to wait for server startup
    ///
    /// # Errors
    /// - Frontend compilation fails
    /// - Server crashes during startup
    /// - Timeout is reached before server is ready
    ///
    /// # Example
    /// ```rust
    /// // Wait up to 5 seconds for server to start
    /// LeptosServer::serve_and_wait(5).await?;
    /// ```
    pub async fn serve_and_wait(timeout: u64) -> Result<()> {
        // Step 1: Compile frontend WASM
        Self::compile_frontend().await?;

        // Step 2: Create oneshot channel for readiness signal
        let (tx, rx) = oneshot::channel();

        // Step 3: Spawn server in background task
        let server_handle = tokio::spawn(async move {
            if let Err(e) = Self::serve(tx).await {
                eprintln!("Server error: {e}");
            }
        });

        // Step 4: Wait for server to be ready with timeout
        match tokio::time::timeout(Duration::from_secs(timeout), rx).await {
            Ok(Ok(())) => {
                tracing::info!("Server is ready!");
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
