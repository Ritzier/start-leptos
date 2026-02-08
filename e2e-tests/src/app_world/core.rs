//! Core AppWorld implementation for Cucumber testing.
//!
//! Provides the test context with WebDriver automation capabilities.

use std::net::SocketAddr;

use anyhow::{Context, Result};
use cucumber::World;
use fantoccini::Locator;
use fantoccini::elements::Element;
use serde_json::Value;

use crate::{Webdriver, get_server_addr};

/// Cucumber World for browser-based testing.
///
/// This struct maintains test state including:
/// - WebDriver client for browser automation
/// - Server address for navigation
/// - Console log capture via JavaScript injection
#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct AppWorld {
    /// WebDriver client for browser automation.
    pub webdriver: Webdriver,

    /// Server address (set by `LeptosServer::serve_and_wait`).
    addr: SocketAddr,
}

impl AppWorld {
    /// Creates a new AppWorld instance.
    ///
    /// Initializes WebDriver and retrieves the server address
    /// from global storage.
    ///
    /// # Errors
    /// - WebDriver fails to connect (chromedriver/geckodriver not running)
    /// - Server address not initialized
    ///
    /// # Example
    /// ```rust
    /// let world = AppWorld::new().await?;
    /// ```
    pub async fn new() -> Result<Self> {
        let webdriver = Webdriver::new().await?;
        let addr = get_server_addr();

        Ok(Self { webdriver, addr })
    }

    /// Navigates to a specific path and sets up console log capture.
    ///
    /// This method:
    /// 1. Constructs the full URL from path
    /// 2. Navigates the browser
    /// 3. Injects JavaScript to capture console logs in sessionStorage
    ///
    /// # Arguments
    /// * `path` - Relative path (e.g., "/", "/about")
    ///
    /// # Console Log Capture
    /// Intercepts `console.log`, `console.info`, `console.warn`, `console.error`,
    /// and `console.debug` calls, storing them in `sessionStorage.__consoleLogs__`.
    ///
    /// # Errors
    /// - Navigation fails
    /// - JavaScript injection fails
    ///
    /// # Example
    /// ```rust
    /// world.goto_path("/").await?;
    /// world.goto_path("/dashboard").await?;
    /// ```
    pub async fn goto_path(&mut self, path: &str) -> anyhow::Result<()> {
        // Strip leading slash if present
        let path = path.strip_prefix('/').unwrap_or(path);
        let target_url = format!("http://{}/{}", self.addr, path);

        self.webdriver
            .client
            .goto(&target_url)
            .await
            .context(format!("Failed to navigate to {}", target_url))?;

        // Inject console log capture script
        // Uses sessionStorage to persist across page updates
        self.execute(
            r#"
            if (!window.__consoleLoggerInstalled__) {
                window.__consoleLoggerInstalled__ = true;
                
                // Initialize storage if not exists
                if (!sessionStorage.getItem('__consoleLogs__')) {
                    sessionStorage.setItem('__consoleLogs__', JSON.stringify([]));
                }
                
                // Intercept all console methods
                ['log', 'info', 'warn', 'error', 'debug'].forEach(method => {
                    const original = console[method];
                    console[method] = function(...args) {
                        const logs = JSON.parse(sessionStorage.getItem('__consoleLogs__') || '[]');
                        logs.push({
                            level: method,
                            message: args.map(arg => {
                                try {
                                    // Serialize objects to JSON
                                    return typeof arg === 'object' ? JSON.stringify(arg) : String(arg);
                                } catch(e) {
                                    return String(arg);
                                }
                            }),
                            timestamp: Date.now()
                        });
                        sessionStorage.setItem('__consoleLogs__', JSON.stringify(logs));
                        original.apply(console, args); // Call original method
                    };
                });
            }
            "#,
            vec![],
        )
        .await?;

        Ok(())
    }

    /// Executes arbitrary JavaScript in the browser context.
    ///
    /// # Arguments
    /// * `script` - JavaScript code to execute
    /// * `args` - Arguments to pass to the script
    ///
    /// # Returns
    /// JSON value returned by the script
    ///
    /// # Example
    /// ```rust
    /// let result = world.execute("return document.title;", vec![]).await?;
    /// ```
    pub async fn execute(&mut self, script: &str, args: Vec<Value>) -> Result<Value> {
        self.webdriver
            .client
            .execute(script, args)
            .await
            .map_err(Into::into)
    }

    /// Finds a single element by CSS selector or other locator.
    ///
    /// # Arguments
    /// * `locator` - Element locator (CSS, XPath, etc.)
    ///
    /// # Returns
    /// The first matching element
    ///
    /// # Errors
    /// - No element found matching the locator
    ///
    /// # Example
    /// ```rust
    /// let button = world.find(Locator::Css("button.submit")).await?;
    /// button.click().await?;
    /// ```
    pub async fn find<'a>(&mut self, locator: Locator<'a>) -> Result<Element> {
        self.webdriver
            .client
            .find(locator)
            .await
            .map_err(Into::into)
    }
}
