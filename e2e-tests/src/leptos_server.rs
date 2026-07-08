//! Leptos server lifecycle management for testing.
//!
//! Handles frontend compilation and server startup with readiness signaling.

use std::net::SocketAddr;
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;

use anyhow::{Result, anyhow};
use server::Server;
use tokio::process::Command;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;

use crate::PortFinder;

/// Leptos server manager for e2e tests.
#[derive(Default, Debug)]
pub struct LeptosServer {
    handle: Option<JoinHandle<()>>,
    shutdown: Option<CancellationToken>,
    port: Option<u16>,
}

impl LeptosServer {
    /// Starts the Leptos server for testing, picking a free port automatically.
    ///
    /// This method:
    /// 1. Finds an available port via [`PortFinder`]
    /// 2. Creates a [`CancellationToken`] used later to trigger shutdown
    /// 3. Compiles the frontend and spawns the server, waiting up 5s for it to
    ///    become ready
    ///
    /// On success, the server's handle, shutdown token, and port are stored on
    /// `self` for later use by [`LeptosServer::stop`] and [`LeptosServer::get_port`]
    ///
    /// # Errors
    /// - No available port could be found
    /// - Frontend compilation fails
    /// - Server fails to start within the 5 seconds timeout
    pub async fn start(&mut self) -> Result<()> {
        // Find an available port to avoid conflicts with other tests
        let new_port = PortFinder::get_available_port()
            .await
            .map_err(|e| anyhow!("{e}"))?;

        let new_shutdown = CancellationToken::new();

        let new_handle = Self::serve_and_wait(5, new_shutdown.clone(), new_port).await?;

        let Self {
            handle,
            shutdown,
            port,
        } = self;

        *shutdown = Some(new_shutdown);
        *handle = Some(new_handle);
        *port = Some(new_port);

        Ok(())
    }

    /// Stop the running server gracefully, if one is running.
    ///
    /// This method:
    /// 1. Cancels the [`CancellationToken`] to signal the server task to shut
    ///    down
    /// 2. Waits for the server task to finish, up to a bounded timeout
    /// 3. Aborts the task if it doesn't shut down in time
    /// 4. Clears the stored port regardless of outcome
    ///
    /// Calling this on a server that was never stared (or already stopped)
    /// is a no-op and returns `Ok(())`
    ///
    /// # Errors
    /// - The server task did not shut down within the timeout and had to be aborted
    /// - The server task panicked while shutting down
    pub async fn stop(&mut self) -> Result<()> {
        const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(5);

        let Self {
            handle,
            shutdown,
            port,
        } = self;

        // Always clear the port, even if shutdown fails, so `get_port`
        // reflects reality after `stop` is called
        *port = None;

        if let Some(token) = shutdown.take() {
            token.cancel();
        }

        let Some(handle) = handle.take() else {
            // Nothing was running
            return Ok(());
        };

        match timeout(SHUTDOWN_TIMEOUT, handle).await {
            // Task finished cleanly
            Ok(Ok(())) => Ok(()),

            // Task panicked while shutting down
            Ok(Err(join_err)) => Err(anyhow!("server task panicked during shutdown: {join_err}")),

            // Task didn't finish in time; abort it and report the timeout
            Err(_) => Err(anyhow!(
                "server task did not shutdown within {SHUTDOWN_TIMEOUT:?}"
            )),
        }
    }

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
            .ok_or_else(|| anyhow!("Failed to find project root directory"))?;

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

            return Err(anyhow!(
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

    /// Builds the `Server` and drives it until completion or shutdown.
    ///
    /// This is the body of the spawned background task started in
    /// [`LeptosServer::serve_and_wait`]. It locates the project's
    /// `Cargo.toml`, constructs a cucumber-flavored [`Server`] bound the
    /// `127.0.0.1:${port}`, and runs it. The `sender` is passed through to the
    /// caller cancel the server later via [`LeptosServer::stop`].
    ///
    /// # Arguments
    /// * `sender` - Oneshot channel used by the server to signal readiness
    /// * `port` - Port to bind the server to on localhost
    /// * `shutdown` - Token that, when cancelled, tells the server to stop
    ///
    /// # Errors
    /// - No available port
    /// - Server fails to bind
    /// - `Cargo.toml` path invalid
    async fn serve(
        sender: oneshot::Sender<()>,
        port: u16,
        shutdown: CancellationToken,
    ) -> Result<SocketAddr> {
        // Navigate to project root
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(1)
            .ok_or_else(|| anyhow!("Failed to find project root directory"))?;
        let cargo_toml_path = manifest_dir.join("Cargo.toml");

        let addr = std::net::SocketAddr::new(
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
            port,
        );

        // Start server with cucumber-specific setup
        let cargo_toml_str = cargo_toml_path
            .to_str()
            .ok_or_else(|| anyhow!("Invalid UTF-8 in Cargo.toml path"))?;

        // Create `Server`
        let cucumber_server =
            Server::cucumber_new(addr, Some(cargo_toml_str), sender, shutdown).await?;

        // `Server` start serving
        cucumber_server.serve().await?;

        Ok(addr)
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
    /// ```ignore
    /// // Wait up to 5 seconds for server to start
    /// LeptosServer::serve_and_wait(5).await?;
    /// ```
    pub async fn serve_and_wait(
        timeout: u64,
        shutdown: CancellationToken,
        port: u16,
    ) -> Result<JoinHandle<()>> {
        tracing::info!("Starting server...");

        // Step 1: Compile frontend WASM
        tracing::info!("Compiling Leptos frontend");
        Self::compile_frontend().await?;
        tracing::info!("Leptos frontend compilation completed");

        // Step 2: Create oneshot channel for readiness signal
        let (tx, rx) = oneshot::channel();

        // Step 3: Spawn server in background task
        let server_handle = tokio::spawn(async move {
            if let Err(e) = Self::serve(tx, port, shutdown).await {
                eprintln!("Server error: {e}");
            }
        });

        // Step 4: Wait for server to be ready with timeout
        match tokio::time::timeout(Duration::from_secs(timeout), rx).await {
            Ok(Ok(())) => {
                tracing::info!("Server is listening on port: {port}!");
                Ok(server_handle)
            }
            Ok(Err(_)) => {
                server_handle.abort();
                Err(anyhow!("Server crashed before becoming ready"))
            }
            Err(_) => {
                server_handle.abort();
                Err(anyhow!("Server failed to start within {} seconds", timeout))
            }
        }
    }

    /// Returns the port the server is currently listening on.
    ///
    /// # Errors
    /// Returns an error if the server hasn't been started (or has since
    /// been stopped), i.e. [`LeptosServer::port`] is `None`.
    pub fn get_port(&self) -> Result<u16> {
        self.port.ok_or_else(|| {
            anyhow!(
                "server is not running: `start()` was never called or `stop()` was already invoked"
            )
        })
    }
}
