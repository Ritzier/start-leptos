use std::time::Duration;

use color_eyre::owo_colors::OwoColorize;
use color_eyre::{Result, eyre::eyre};
use e2e_tests::{AppWorld, ConsoleLog, LeptosServer};
use fantoccini::Locator;
use tokio::time::Instant;

use super::benchmark_result::BenchmarkResults;

pub struct Benchmarks {
    app_world: AppWorld,
    iteration: usize,
}

impl Benchmarks {
    pub async fn new(iteration: usize) -> Result<Self> {
        LeptosServer::serve_and_wait(5).await?;
        let app_world = AppWorld::new().await.map_err(|e| eyre!(e))?;

        Ok(Self {
            app_world,
            iteration,
        })
    }

    pub async fn start(mut self) -> Result<BenchmarkResults> {
        let mut results = BenchmarkResults::new(self.iteration);

        self.navigate_home().await?;

        for i in 1..=self.iteration {
            println!(
                "\n{}",
                format!("=== Iteration {}/{} ===", i, self.iteration).cyan()
            );
            {%- if websocket == true %}

            // Benchmark connect
            let connect_time = self.benchmark_connect().await?;
            results.add_timing("connect", connect_time);
            println!("{} {}ms", "Connect:".green(), connect_time.as_millis());

            // Benchmark disconnect
            let disconnect_time = self.benchmark_disconnect().await?;
            results.add_timing("disconnect", disconnect_time);
            println!(
                "{} {}ms",
                "Disconnect:".green(),
                disconnect_time.as_millis()
            );
            {%- else %}

            // Benchmark update num
            let time = self.benchmark_num_button(i).await?;
            results.add_timing("num", time);
            println!("{} {}ms", "Update num:".green(), time.as_millis());
            {%- endif %}
        }

        Ok(results)
    }

    /// Navigates to the homepage
    async fn navigate_home(&mut self) -> Result<()> {
        self.app_world
            .goto_path("/")
            .await
            .map_err(|e| eyre!("Failed to navigate to /: {e}"))
    }
    {%- if websocket == true %}

    /// Benchmarks the connect button click and WebSocket handshake
    pub async fn benchmark_connect(&mut self) -> Result<Duration> {
        let button = self.find_button_with_text("Connect").await?;

        let start = Instant::now();
        button.click().await?;

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

    /// Benchmarks the disconnect button click and WebSocket closure
    pub async fn benchmark_disconnect(&mut self) -> Result<Duration> {
        let button = self.find_button_with_text("Disconnect").await?;

        let start = Instant::now();
        button.click().await?;

        let expected = vec![
            ConsoleLog::new("error", "WebSocket Closed: code: 1005, reason:"),
            ConsoleLog::new(
                "log",
                "Websocket closed: error reaching server to call server function: WebSocket Closed: code: 1005, reason:",
            ),
        ];

        self.wait_for_logs(&expected, Duration::from_secs(5))
            .await
            .map_err(|e| eyre!("Disconnect closure failed: {e}"))?;

        let elapsed = start.elapsed();
        self.clear_console_logs().await?;

        Ok(elapsed)
    }
    {%- else %}

    pub async fn benchmark_num_button(&mut self, i: usize) -> Result<Duration> {
        let button = self
            .find_button_with_text(&format!("Click Me: {}", i - 1))
            .await?;

        let start = Instant::now();
        button.click().await?;

        let expected = [ConsoleLog::new("log", format!("Update num: {}", i))];
        self.wait_for_logs(&expected, Duration::from_secs(1))
            .await?;

        let elapsed = start.elapsed();
        self.clear_console_logs().await?;
        Ok(elapsed)
    }
    {%- endif %}

    /// Finds a button and verifies its text
    async fn find_button_with_text(
        &mut self,
        expected_text: &str,
    ) -> Result<fantoccini::elements::Element> {
        let button = self
            .app_world
            .find(Locator::Css("button"))
            .await
            .map_err(|e| eyre!("Failed to find button: {e}"))?;

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

    /// Waits for expected console logs with timeout
    async fn wait_for_logs(&mut self, expected: &[ConsoleLog], timeout: Duration) -> Result<()> {
        self.app_world
            .wait_for_console_logs(expected, timeout)
            .await
            .map_err(|e| eyre!(e))?;

        Ok(())
    }

    async fn clear_console_logs(&mut self) -> Result<()> {
        self.app_world
            .clear_console_logs()
            .await
            .map_err(|e| eyre!("Failed to clear logs: {e}"))?;

        Ok(())
    }
}
