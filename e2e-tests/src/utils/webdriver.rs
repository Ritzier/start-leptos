//! WebDriver setup and lifecycle management.
//!
//! Supports both ChromeDriver and GeckoDriver with automatic selection.

use std::env;
use std::process::Stdio;

use anyhow::{Result, anyhow};
use fantoccini::wd::Capabilities;
use fantoccini::{Client, ClientBuilder};
use serde_json::json;
use tokio::process::{Child, Command};

use crate::PortFinder;

/// WebDriver client with lifecycle management.
///
/// Automatically spawns and manages chromedriver or geckodriver process.
#[derive(Debug)]
pub struct Webdriver {
    /// Fantoccini client for browser automation.
    pub client: Client,

    /// Child process handle (chromedriver or geckodriver).
    /// Kept alive until Webdriver is dropped.
    #[expect(dead_code)]
    child: Child,
}

impl Webdriver {
    /// Creates a new WebDriver instance.
    ///
    /// Selects driver based on `WEBDRIVER` environment variable:
    /// - `chromedriver` or `chrome` → ChromeDriver (default)
    /// - `geckodriver` or `gecko` → GeckoDriver
    ///
    /// # Errors
    /// - Driver binary not found in PATH
    /// - Driver fails to start
    /// - Connection to driver fails
    ///
    /// # Example
    /// ```bash
    /// # Use ChromeDriver (default)
    /// cargo test
    ///
    /// # Use GeckoDriver
    /// WEBDRIVER=geckodriver cargo test
    /// ```
    pub async fn new() -> Result<Self> {
        let (client, child) = match env::var("WEBDRIVER") {
            Err(_) => build_chromedriver().await?, // Default to Chrome
            Ok(webdriver_env) => match webdriver_env.to_lowercase().as_str() {
                "chromedriver" | "chrome" => build_chromedriver().await?,
                "geckodriver" | "gecko" => build_geckodriver().await?,
                invalid => return Err(anyhow!("Invalid WEBDRIVER value: `{invalid}`")),
            },
        };

        Ok(Self { client, child })
    }
}

/// Builds a ChromeDriver client.
///
/// # Process
/// 1. Finds available port
/// 2. Spawns chromedriver process
/// 3. Waits 500ms for startup
/// 4. Connects Fantoccini client
///
/// # Capabilities
/// - Headless mode enabled
/// - Browser and performance logging enabled
///
/// # Errors
/// - chromedriver not in PATH
/// - Port binding fails
/// - Connection fails
async fn build_chromedriver() -> Result<(Client, Child)> {
    let port = PortFinder::get_available_port()
        .await
        .map_err(|e| anyhow!("{e}"))?;

    // Spawn chromedriver process
    let child = Command::new("chromedriver")
        .arg(format!("--port={port}"))
        .stdout(Stdio::null()) // Silence output
        .stderr(Stdio::null())
        .spawn()?;

    // Wait for chromedriver to initialize
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

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

    // Connect Fantoccini client
    let client = ClientBuilder::native()
        .capabilities(cap)
        .connect(&format!("http://localhost:{port}"))
        .await?;

    Ok((client, child))
}

/// Builds a GeckoDriver client.
///
/// # Process
/// 1. Finds available port
/// 2. Spawns geckodriver process
/// 3. Connects Fantoccini client
///
/// # Capabilities
/// - Headless mode enabled with `-headless` flag
///
/// # Errors
/// - geckodriver not in PATH
/// - Port binding fails
/// - Connection fails
async fn build_geckodriver() -> Result<(Client, Child)> {
    let port = PortFinder::get_available_port()
        .await
        .map_err(|e| anyhow!("{e}"))?;

    // Spawn geckodriver process
    let child = Command::new("geckodriver")
        .arg(format!("--port={port}"))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    // Configure capabilities for Firefox
    let mut caps = serde_json::Map::new();
    caps.insert(
        "moz:firefoxOptions".to_string(),
        json!({ "args": ["--headless", "-headless"] }),
    );

    // Connect Fantoccini client
    let webdriver_url = format!("http://localhost:{port}");
    let client = ClientBuilder::native()
        .capabilities(caps)
        .connect(&webdriver_url)
        .await?;

    Ok((client, child))
}
