//! Utilities for launching and managing a local ChromeDriver instance.
//!
//! This module provides helpers for starting ChromeDriver and creating a
//! connected Fantoccini WebDriver client.

use std::process::Stdio;

use anyhow::{Result, anyhow};
use fantoccini::wd::Capabilities;
use fantoccini::{Client, ClientBuilder};
use tokio::process::{Child, Command};
use tokio::sync::OnceCell;

use crate::PortFinder;

/// Port assigned to the running `ChromeDriver` instance
pub static CHROME_PORT: OnceCell<u16> = OnceCell::const_new();

/// A WebDriver session backed by a local ChromeDriver process.
#[derive(Debug)]
pub struct Webdriver {
    /// Fantoccini client used to control the browser.
    pub client: Client,
}

/// Handle to a spawned `ChromeDriver` process.
pub struct ChromeDriverCommand {
    child: Child,
}

impl Webdriver {
    /// Create a `WebDriver` session.
    ///
    /// `spawn_chrome_driver()` must be called before this method so a
    /// `ChromeDriver` instance is running and its port has been initialized.
    ///
    /// # Erros
    ///
    /// Returns an error if a `WebDriver` session cannot be establised.
    pub async fn new() -> Result<Self> {
        let client = Self::build_chromedriver().await?;

        Ok(Self { client })
    }

    /// Connects to the running `ChromeDriver` instance.
    ///
    /// The `ChromeDriver` process must already be running, and
    /// [`CHROME_PORT`] must have been initialized.
    ///
    /// The browser session is configured to:
    /// - run in headless mode;
    /// - enable browser logs;
    /// - enable performance logs.
    ///
    /// # Errors
    ///
    /// Returns an error if the `ChromeDriver` port has not been initialized,
    /// the session cannot be created, or the browser capabilities are invalid.
    async fn build_chromedriver() -> Result<Client> {
        let port = CHROME_PORT.get().ok_or(anyhow!("Port not initialize"))?;

        // Configure capabilities for Chrome
        let cap: Capabilities = serde_json::from_str(
            r#"{
            "goog:loggingPrefs": {
                "browser": "ALL",
                "performance": "ALL"
            },
            "browserName": "chrome",
            "goog:chromeOptions": {
                "args": ["--headless"]
            }
        }"#,
        )?;

        // Connect `Fantoccini` client
        let client = ClientBuilder::native()
            .capabilities(cap)
            .connect(&format!("http://localhost:{port}"))
            .await?;

        Ok(client)
    }

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
    pub async fn spawn_chrome_driver() -> Result<ChromeDriverCommand> {
        let port = PortFinder::get_available_port()
            .await
            .map_err(|e| anyhow!("{e}"))?;

        // Init to global
        CHROME_PORT.get_or_init(|| async move { port }).await;

        // Spawn chromedriver process
        let child = Command::new("chromedriver")
            .arg(format!("--port={port}"))
            .stdout(Stdio::null()) // Silence output
            .stderr(Stdio::null())
            .spawn()?;

        // Wait for chromedriver to initialize
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        tracing::info!(port = port, "ChromeDriver is ready and listening");

        Ok(ChromeDriverCommand { child })
    }
}

impl ChromeDriverCommand {
    /// Terminates the `ChromeDriver` process and waits for it to exit.
    ///
    /// # Errors
    ///
    /// Returns an error if the process cannot be terminated or waited on.
    pub async fn shutdown(mut self) -> Result<()> {
        self.child.kill().await?;
        self.child.wait().await?;

        Ok(())
    }
}
