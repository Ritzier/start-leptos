use std::process::Stdio;

use anyhow::{Result, anyhow};
use tokio::process::{Child, Command};
use tokio::sync::OnceCell;

use super::PortFinder;

/// Port assigned to the running `ChromeDriver` instance
pub static CHROME_PORT: OnceCell<u16> = OnceCell::const_new();

/// Handle to a spawned `ChromeDriver` process.
pub struct ChromeDriver {
    process: Child,
}

impl ChromeDriver {
    /// Starts a `ChromeDriver` process on an available local port.
    ///
    /// The selected port is stored in [`CHROME_PORT`] for use by
    /// [`Webdriver::new`].
    ///
    /// Returns a [`ChromeDriverCommand`] that can be used to terminated the
    /// `ChromeDriver` process when it is no longer need.
    ///
    /// # Errors
    ///
    /// Returns an error if an available port cannot be found or if
    /// `ChromeDriver` fails to start.
    pub async fn new() -> Result<Self> {
        let port = PortFinder::get_available_port()
            .await
            .map_err(|e| anyhow!("{e}"))?;

        // Init to global
        CHROME_PORT.get_or_init(|| async move { port }).await;

        // Spawn `ChromeDriver` process
        let process = Command::new("chromedriver")
            .arg(format!("--port={port}"))
            .stdout(Stdio::null()) // Silence output
            .stderr(Stdio::null())
            .spawn()?;

        // Wait for `ChromeDriver` to initialize
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        tracing::info!(port = port, "ChromeDriver is ready and listening");

        Ok(Self { process })
    }

    /// Terminates the `ChromeDriver` process and waits for it to exit.
    ///
    /// # Errors
    ///
    /// Returns an error if the process cannot be terminated or waited on.
    pub async fn shutdown(mut self) -> Result<()> {
        self.process.kill().await?;
        self.process.wait().await?;

        Ok(())
    }
}
