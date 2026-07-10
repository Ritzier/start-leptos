//! Console log validation for browser testing.
//!
//! Captures and validates JavaScript console output during tests.

use std::time::Duration;

use anyhow::Result;
use cucumber::gherkin::Table;
use serde::{Deserialize, Deserializer, Serialize};

use super::AppWorld;

/// Represents a browser console log entry.
///
/// Captures level (log/info/warn/error/debug) and message content.
///
/// # Example
/// ```ignore
/// let log = ConsoleLog::new("log", "User logged in");
/// let error = ConsoleLog::new("error", "Connection failed");
/// ```
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ConsoleLog {
    /// Log level (log, info, warn, error, debug).
    pub level: String,

    /// Log message content (can be multiple parts for multiple arguments).
    #[serde(deserialize_with = "deserialize_trimmed_strings")]
    pub message: Vec<String>,
}

/// Custom deserializer that trims whitespace from all message parts.
fn deserialize_trimmed_strings<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let strings: Vec<String> = Vec::deserialize(deserializer)?;
    Ok(strings.into_iter().map(|s| s.trim().to_string()).collect())
}

impl ConsoleLog {
    /// Creates a new ConsoleLog entry.
    ///
    /// # Arguments
    /// * `level` - Log level (converted to lowercase)
    /// * `message` - Log message (whitespace trimmed)
    ///
    /// # Example
    /// ```ignore
    /// let log = ConsoleLog::new("log", "Operation complete");
    /// let error = ConsoleLog::new("ERROR", "Failed"); // Converted to "error"
    /// ```
    pub fn new(level: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            level: level.into().to_lowercase(),
            message: vec![message.into().trim().to_string()],
        }
    }

    /// Converts a Gherkin table into a Vec of ConsoleLog.
    ///
    /// Expected table format:
    /// ```gherkin
    /// | message             | level |
    /// | "User logged in"    | log   |
    /// | "Connection failed" | error |
    /// ```
    ///
    /// # Arguments
    /// * `table` - Gherkin table from step definition
    ///
    /// # Returns
    /// Vector of ConsoleLog entries
    ///
    /// # Errors
    /// - Row doesn't have exactly 2 columns
    ///
    /// # Example
    /// ```ignore
    /// // In step definition:
    /// let logs = ConsoleLog::from_table(step.table.as_ref().unwrap())?;
    /// ```
    pub fn from_table(table: &Table) -> Result<Vec<Self>> {
        table
            .rows
            .iter()
            .map(|row| {
                // Validate row structure
                if row.len() < 2 {
                    return Err(anyhow::Error::msg(format!(
                        "Expected 2 columns (message, level), found {} columns in row: {row:?}",
                        row.len()
                    )));
                }

                let message = row[0].trim().to_string();
                let level = row[1].trim().to_lowercase().to_string();

                Ok(ConsoleLog::new(level, message))
            })
            .collect::<Result<Vec<_>>>()
    }
}

impl AppWorld {
    /// Retrieves all captured console logs from the browser.
    ///
    /// Reads logs from `sessionStorage.__consoleLogs__` which are
    /// populated by the JavaScript injected in `goto_path()`.
    ///
    /// # Returns
    /// Vector of console log entries
    ///
    /// # Errors
    /// - JavaScript execution fails
    /// - JSON parsing fails
    ///
    /// # Example
    /// ```ignore
    /// let logs = world.get_console_logs().await?;
    /// for log in logs {
    ///     println!("[{}] {}", log.level, log.message.join(" "));
    /// }
    /// ```
    pub async fn get_console_logs(&mut self) -> Result<Vec<ConsoleLog>> {
        let logs_json = self
            .execute(
                "return JSON.parse(sessionStorage.getItem('__consoleLogs__') || '[]');",
                vec![],
            )
            .await?;

        // Deserialize JSON to Vec<ConsoleLog>
        let logs: Vec<ConsoleLog> = serde_json::from_value(logs_json)
            .map_err(|e| anyhow::Error::msg(format!("Failed to parse console logs: {}", e)))?;

        Ok(logs)
    }

    /// Waits until the browser console logs match the expected logs.
    ///
    /// Polls the browser at a fixed interval (`timeout_dur / 5`) until either:
    /// - The console logs match the expected logs exactly
    /// - The maximum number of polling attempts is reached
    ///
    /// # Arguments
    /// * `expected` - Expected console logs to match
    /// * `timeout_dur` - Maximum duration to wait before returning an error
    ///
    /// # Returns
    /// `Ok(())` when the expected logs are observed.
    ///
    /// # Errors
    /// Returns an error when:
    /// - The expected console logs do not appear before the timeout
    /// - Retrieving browser console logs fails
    ///
    /// The timeout error includes both the expected logs and the latest
    /// console logs observed from the browser for easier debugging.
    ///
    /// # Example
    /// ```ignore
    /// let expected = vec![ConsoleLog::new("log", "Ready")];
    /// world
    ///     .wait_for_console_logs(&expected, Duration::from_secs(2))
    ///     .await?;
    /// ```
    pub async fn wait_for_console_logs(
        &mut self,
        expected: &[ConsoleLog],
        timeout_dur: Duration,
    ) -> Result<()> {
        let tick_duration = timeout_dur / 5;
        let mut latest_logs = Vec::new();

        for _ in 0..5 {
            let logs = self.get_console_logs().await?;

            if logs.as_slice() == expected {
                return Ok(());
            }

            latest_logs = logs;

            tokio::time::sleep(tick_duration).await;
        }

        Err(anyhow::anyhow!(
            "Timed out waiting for expected console logs\n\n\
         Expected:\n{:#?}\n\n\
         Latest actual logs:\n{:#?}",
            expected,
            latest_logs
        ))
    }

    /// Clears all captured console logs.
    ///
    /// Removes the `__consoleLogs__` key from sessionStorage.
    /// Called after each test step to ensure clean state.
    ///
    /// # Errors
    /// - JavaScript execution fails
    ///
    /// # Example
    /// ```ignore
    /// world.clear_console_logs().await?;
    /// ```
    pub async fn clear_console_logs(&mut self) -> Result<()> {
        self.execute("sessionStorage.removeItem('__consoleLogs__');", vec![])
            .await?;
        Ok(())
    }
}
