use std::time::Duration;

use color_eyre::owo_colors::OwoColorize;
use color_eyre::{Result, eyre::eyre};
use e2e_tests::{AppWorld, ConsoleLog, LeptosServer};
use fantoccini::Locator;
use tokio::time::Instant;

use super::benchmark_result::BenchmarkResults;

/// Core benchmark runner that measures application performance.
///
/// This struct manages the benchmark lifecycle:
/// 1. Starts the Leptos server
/// 2. Connects WebDriver browser automation
/// 3. Runs specified number of iterations
/// 4. Collects timing data
/// 5. Returns statistical results
///
/// # Benchmark Types
/// - **WebSocket mode**: Measures connect/disconnect handshake timings
/// - **Default mode**: Measures button click and state update latency
pub struct Benchmarks {
    /// WebDriver instance for browser automation.
    /// Provides methods to find elements, click buttons, and verify console logs.
    app_world: AppWorld,
    
    /// Number of iterations to run for each benchmark.
    /// Higher values provide more accurate statistical analysis.
    iteration: usize,
}

impl Benchmarks {
    /// Initializes the benchmark environment.
    ///
    /// This method:
    /// 1. Compiles the frontend WASM if needed
    /// 2. Starts the Leptos server on an available port
    /// 3. Connects WebDriver (chromedriver or geckodriver)
    /// 4. Waits for server to be ready (5 second timeout)
    ///
    /// # Arguments
    /// * `iteration` - Number of times to run each benchmark
    ///
    /// # Errors
    /// - Frontend compilation fails
    /// - Server fails to start within timeout
    /// - WebDriver connection fails (chromedriver/geckodriver not running)
    ///
    /// # Example
    /// ```
    /// let benchmark = Benchmarks::new(20).await?;
    /// ```
    pub async fn new(iteration: usize) -> Result<Self> {
        // Start server and wait for it to be ready
        LeptosServer::serve_and_wait(5).await?;
        
        // Connect WebDriver browser automation
        let app_world = AppWorld::new().await.map_err(|e| eyre!(e))?;

        Ok(Self {
            app_world,
            iteration,
        })
    }

    /// Runs all benchmarks for the specified number of iterations.
    ///
    /// This method:
    /// 1. Navigates to the homepage
    /// 2. Runs each iteration:
    ///    - WebSocket mode: connect + disconnect
    ///    - Default mode: button click
    /// 3. Prints per-iteration timings
    /// 4. Returns aggregate results
    ///
    /// # Returns
    /// `BenchmarkResults` containing all timing data and statistics
    ///
    /// # Errors
    /// - Navigation fails
    /// - Element not found
    /// - Expected console logs don't appear within timeout
    ///
    /// # Example
    /// ```
    /// let results = benchmark.start().await?;
    /// results.print_summary();
    /// ```
    pub async fn start(mut self) -> Result<BenchmarkResults> {
        let mut results = BenchmarkResults::new(self.iteration);

        // Navigate to homepage once before starting benchmarks
        self.navigate_home().await?;

        // Run each iteration
        for i in 1..=self.iteration {
            println!(
                "\n{}",
                format!("=== Iteration {}/{} ===", i, self.iteration).cyan()
            );
            
            {% if websocket == true -%}
            // WebSocket mode: measure connect and disconnect operations
            
            // Benchmark connect: Click button -> Wait for handshake -> Record time
            let connect_time = self.benchmark_connect().await?;
            results.add_timing("connect", connect_time);
            println!("{} {}ms", "Connect:".green(), connect_time.as_millis());

            // Benchmark disconnect: Click button -> Wait for closure -> Record time
            let disconnect_time = self.benchmark_disconnect().await?;
            results.add_timing("disconnect", disconnect_time);
            println!(
                "{} {}ms",
                "Disconnect:".green(),
                disconnect_time.as_millis()
            );
            {%- else -%}
            // Default mode: measure button click and state update
            
            // Benchmark: Click button -> Wait for console log -> Record time
            let time = self.benchmark_num_button(i).await?;
            results.add_timing("num", time);
            println!("{} {}ms", "Update num:".green(), time.as_millis());
            {%- endif %}
        }

        Ok(results)
    }

    /// Navigates to the homepage and sets up console log capture.
    ///
    /// This is called once before running benchmarks to ensure
    /// we start from a clean state.
    ///
    /// # Errors
    /// Returns error if navigation fails or page doesn't load
    async fn navigate_home(&mut self) -> Result<()> {
        self.app_world
            .goto_path("/")
            .await
            .map_err(|e| eyre!("Failed to navigate to /: {e}"))
    }
    
    {% if websocket == true -%}
    /// Benchmarks the WebSocket connect operation.
    ///
    /// Measures the time from clicking "Connect" button until
    /// the WebSocket handshake completes (confirmed by console log).
    ///
    /// # Process
    /// 1. Find "Connect" button
    /// 2. Start timer
    /// 3. Click button
    /// 4. Wait for handshake log: "Received: FrontendResponse::HandshakeResponse"
    /// 5. Record elapsed time
    /// 6. Clear console logs for next iteration
    ///
    /// # Returns
    /// Duration from button click to handshake completion
    ///
    /// # Errors
    /// - Button not found or has wrong text
    /// - Handshake doesn't complete within 5 seconds
    pub async fn benchmark_connect(&mut self) -> Result<Duration> {
        let button = self.find_button_with_text("Connect").await?;

        let start = Instant::now();
        button.click().await?;

        // Wait for WebSocket handshake confirmation in console
        let expected = vec![ConsoleLog::new(
            "log",
            "Received: FrontendResponse::HandshakeResponse",
        )];

        self.wait_for_logs(&expected, Duration::from_secs(5))
            .await
            .map_err(|e| eyre!("Connect handshake failed: {e}"))?;

        let elapsed = start.elapsed();
        self.clear_console_logs().await?;

        Ok(elapsed)
    }

    /// Benchmarks the WebSocket disconnect operation.
    ///
    /// Measures the time from clicking "Disconnect" button until
    /// the WebSocket closure completes (confirmed by console logs).
    ///
    /// # Process
    /// 1. Find "Disconnect" button
    /// 2. Start timer
    /// 3. Click button
    /// 4. Wait for closure logs (error + info messages)
    /// 5. Record elapsed time
    /// 6. Clear console logs for next iteration
    ///
    /// # Returns
    /// Duration from button click to connection closure
    ///
    /// # Errors
    /// - Button not found or has wrong text
    /// - Connection doesn't close within 5 seconds
    pub async fn benchmark_disconnect(&mut self) -> Result<Duration> {
        let button = self.find_button_with_text("Disconnect").await?;

        let start = Instant::now();
        button.click().await?;

        // Wait for WebSocket closure confirmation (two log messages)
        let expected = vec![
            ConsoleLog::new("error", "WebSocket Closed: code: 1005, reason:"),
            ConsoleLog::new(
                "error",
                "error: error reaching server to call server function: WebSocket Closed: code: 1005, reason:",
            ),
        ];

        self.wait_for_logs(&expected, Duration::from_secs(5))
            .await
            .map_err(|e| eyre!("Disconnect closure failed: {e}"))?;

        let elapsed = start.elapsed();
        self.clear_console_logs().await?;

        Ok(elapsed)
    }
    {%- else -%}
    /// Benchmarks button click and state update in default mode.
    ///
    /// Measures the time from clicking the counter button until
    /// the state update is reflected in the console log.
    ///
    /// # Process
    /// 1. Find button with current count (e.g., "Click Me: 5")
    /// 2. Start timer
    /// 3. Click button
    /// 4. Wait for console log: "Update num: 6"
    /// 5. Record elapsed time
    /// 6. Clear console logs for next iteration
    ///
    /// # Arguments
    /// * `i` - Current iteration number (used to verify expected count)
    ///
    /// # Returns
    /// Duration from button click to console log appearance
    ///
    /// # Errors
    /// - Button not found or has wrong count
    /// - Console log doesn't appear within 1 second
    pub async fn benchmark_num_button(&mut self, i: usize) -> Result<Duration> {
        // Find button with previous iteration's count
        let button = self
            .find_button_with_text(&format!("Click Me: {}", i - 1))
            .await?;

        let start = Instant::now();
        button.click().await?;

        // Wait for console log confirming state update
        let expected = [ConsoleLog::new("log", format!("Update num: {}", i))];
        self.wait_for_logs(&expected, Duration::from_secs(1))
            .await?;

        let elapsed = start.elapsed();
        self.clear_console_logs().await?;
        Ok(elapsed)
    }
    {%- endif %}

    /// Finds a button element and verifies its text content.
    ///
    /// # Arguments
    /// * `expected_text` - The exact text the button should contain
    ///
    /// # Returns
    /// The button element if found with matching text
    ///
    /// # Errors
    /// - No button element found on page
    /// - Button text doesn't match expected value
    ///
    /// # Example
    /// ```
    /// let button = self.find_button_with_text("Connect").await?;
    /// button.click().await?;
    /// ```
    async fn find_button_with_text(
        &mut self,
        expected_text: &str,
    ) -> Result<fantoccini::elements::Element> {
        let button = self
            .app_world
            .find(Locator::Css("button"))
            .await
            .map_err(|e| eyre!("Failed to find button: {e}"))?;

        // Verify button text matches expectation
        let actual_text = button.text().await?;
        if actual_text != expected_text {
            return Err(eyre!(
                "Expected button text '{}', found '{}'",
                expected_text,
                actual_text
            ));
        }

        Ok(button)
    }

    /// Waits for expected console logs to appear within a timeout.
    ///
    /// Polls the browser console logs every 10ms until either:
    /// - All expected logs appear in order
    /// - Timeout is reached
    ///
    /// # Arguments
    /// * `expected` - Slice of console logs to wait for
    /// * `timeout` - Maximum duration to wait
    ///
    /// # Errors
    /// Returns error if timeout is reached before logs appear
    ///
    /// # Example
    /// ```
    /// let expected = vec![ConsoleLog::new("log", "Operation complete")];
    /// self.wait_for_logs(&expected, Duration::from_secs(2)).await?;
    /// ```
    async fn wait_for_logs(&mut self, expected: &[ConsoleLog], timeout: Duration) -> Result<()> {
        self.app_world
            .wait_for_console_logs(expected, timeout)
            .await
            .map_err(|e| eyre!(e))?;

        Ok(())
    }

    /// Clears the browser console logs.
    ///
    /// Called after each iteration to ensure clean state for next measurement.
    /// Removes logs from sessionStorage to prevent accumulation.
    ///
    /// # Errors
    /// Returns error if JavaScript execution fails
    async fn clear_console_logs(&mut self) -> Result<()> {
        self.app_world
            .clear_console_logs()
            .await
            .map_err(|e| eyre!("Failed to clear logs: {e}"))?;

        Ok(())
    }
}
