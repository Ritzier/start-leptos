//! Core AppWorld implementation for Cucumber testing.
//!
//! Provides the test context with WebDriver automation capabilities.

use anyhow::{Context, Result};
use cucumber::World;
use fantoccini::Locator;
use fantoccini::elements::Element;
use serde_json::Value;

use crate::{LeptosServer, WebDriver};

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
    pub webdriver: WebDriver,

    /// Leptos test server backing this scenario.
    ///
    /// Started fresh in [`AppWorld::new`] on a randomly assigned free port,
    /// and used by [`AppWorld::goto_path`] to build navigation URLs via gets
    /// its own isolated server instance.
    leptos_server: LeptosServer,
}

impl AppWorld {
    /// Creates a new `AppWorld` instance for a single `Cucumber` scenario.
    ///
    /// Initializes `WebDriver` session and starts a dedicated `Leptos` server
    /// (compiling the frontend and binding to an available port) for this
    /// scenario to use.
    ///
    /// # Errors
    /// - `WebDriver` fails to connect (`ChromeDriver` not running)
    /// - Leptos server fails to compile or start
    ///
    /// # Example
    /// ```ignore
    /// let world = AppWorld::new().await?;
    /// ```
    pub async fn new() -> Result<Self> {
        tracing::info!("AppWorld::new()");

        // Spawn `ChromeDriver` command
        let webdriver = WebDriver::new().await?;

        // Each scenario gets its own `LeptosServer` instance so tests
        // running in parallel don't share state or ports.
        let mut leptos_server = LeptosServer::default();

        leptos_server.start().await?;

        Ok(Self {
            webdriver,
            leptos_server,
        })
    }

    /// Navigates to a specific path and sets up console log capture.
    ///
    /// This method:
    /// 1. Constructs the full URL from the scenario's `Leptos` server address and path
    /// 2. Navigates the browser to that URL
    /// 3. Injects `JavaScript` (once per page) to capture console logs into `SessionStorage`
    ///
    /// # Arguments
    /// * `path` - Relative path (e.g., "/", "/about"); a leading slash is optional
    ///
    /// # Console Log Capture
    /// Intercepts `console.log`, `console.info`, `console.warn`, `console.error`,
    /// and `console.debug` calls, storing them in `sessionStorage.__consoleLogs__`.
    ///
    /// # Errors
    /// - Server port is unavailable (server not started or already stopped)
    /// - Navigation fails
    /// - `JavaScript` injection fails
    ///
    /// # Example
    /// ```ignore
    /// world.goto_path("/").await?;
    /// world.goto_path("/dashboard").await?;
    /// ```
    pub async fn goto_path(&mut self, path: &str) -> anyhow::Result<()> {
        // Strip leading slash if present, since we always join with "/" below.
        let path = path.strip_prefix('/').unwrap_or(path);
        let leptos_server_port = self.leptos_server_port()?;
        let target_url = format!("http://127.0.0.1:{leptos_server_port}/{path}");

        self.webdriver
            .client
            .goto(&target_url)
            .await
            .context(format!("Failed to navigate to {}", target_url))?;

        // Inject console log capture script.
        // Guarded by `__consoleLoggerInstalled__` so it only patches
        // `console.*` once per page, even if this is called again.
        // Uses `sessionStorage` to persist logs across page navigations.
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

    /// Executes arbitrary `JavaScript` in this current browser tab's context.
    ///
    /// Thin wrapper around [`fantoccini::Client::execute`] that converts its
    /// error type into `anyhow::Error` for consistency with the rest of `AppWorld`.
    ///
    /// # Arguments
    /// * `script` - `JavaScript` source to execute; use `return` to produce a value
    /// * `args` - Arguments passed to the script, accessible via `arguments[n]`
    ///
    /// # Returns
    /// The `JSON` value returned by the script (`Value::Null` if nothing is returned).
    ///
    /// # Example
    /// ```ignore
    /// let result = world.execute("return document.title;", vec![]).await?;
    /// ```
    pub async fn execute(&mut self, script: &str, args: Vec<Value>) -> Result<Value> {
        self.webdriver
            .client
            .execute(script, args)
            .await
            .map_err(Into::into)
    }

    /// Finds a single element by `CSS` selector or other locator.
    ///
    /// Thin wrapper around [`fantoccini::Client::find`] that converts its
    /// error type ito `anyhow::Error` for consistency with the rest of `AppWorld`
    ///
    /// # Arguments
    /// * `locator` - Element locator (CSS, XPath, etc.)
    ///
    /// # Returns
    /// The first matching element.
    ///
    /// # Errors
    /// - No element found matching the locator
    ///
    /// # Example
    /// ```ignore
    /// let button = world.find(Locator::Css("button.submit")).await?;
    /// button.click.await()?;
    ///```
    pub async fn find<'a>(&mut self, locator: Locator<'a>) -> Result<Element> {
        self.webdriver
            .client
            .find(locator)
            .await
            .map_err(Into::into)
    }

    // ====== `LeptosServer` helpers =====

    /// Returns the port this scenario's Leptos server is listening on.
    ///
    /// # Errors
    /// Returns an error if the server hasn't finished starting (or was
    /// already stopped), matching [`LeptosServer::get_port`].
    fn leptos_server_port(&self) -> Result<u16> {
        self.leptos_server.get_port()
    }
}
