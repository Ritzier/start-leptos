//! Utilities for launching and managing a local ChromeDriver instance.
//!
//! This module provides helpers for starting ChromeDriver and creating a
//! connected Fantoccini WebDriver client.

use anyhow::{Result, anyhow};
use fantoccini::wd::Capabilities;
use fantoccini::{Client, ClientBuilder};

use super::chrome_driver::CHROME_PORT;

/// A WebDriver session backed by a local ChromeDriver process.
#[derive(Debug)]
pub struct WebDriver {
    /// Fantoccini client used to control the browser.
    pub client: Client,
}

impl WebDriver {
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
}
