use std::time::Duration;

use anyhow::Result;
use cucumber::gherkin::Table;
use serde::{Deserialize, Deserializer, Serialize};

use super::AppWorld;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ConsoleLog {
    pub level: String,
    #[serde(deserialize_with = "deserialize_trimmed_strings")]
    pub message: Vec<String>,
}

fn deserialize_trimmed_strings<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let strings: Vec<String> = Vec::deserialize(deserializer)?;
    Ok(strings.into_iter().map(|s| s.trim().to_string()).collect())
}

impl ConsoleLog {
    /// Creates a new ConsoleLog with validation
    pub fn new(level: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            level: level.into().to_lowercase(),
            message: vec![message.into().trim().to_string()],
        }
    }

    /// Converts a Gherkin table into a Vec of ConsoleLog
    ///
    /// Expected table format:
    /// | message | level |
    /// | "some message" | "info" |
    pub fn from_table(table: &Table) -> Result<Vec<Self>> {
        table
            .rows
            .iter()
            .map(|row| {
                // Validate row has at least 2 columns
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
    pub async fn get_console_logs(&mut self) -> Result<Vec<ConsoleLog>> {
        let logs_json = self
            .execute(
                "return JSON.parse(sessionStorage.getItem('__consoleLogs__') || '[]');",
                vec![],
            )
            .await?;

        // Deserialize to Vec<ConsoleLog>
        let logs: Vec<ConsoleLog> = serde_json::from_value(logs_json)
            .map_err(|e| anyhow::Error::msg(format!("Failed to parse console logs: {}", e)))?;

        Ok(logs)
    }

    pub async fn wait_for_console_logs(
        &mut self,
        expected: &[ConsoleLog],
        timeout_dur: Duration,
    ) -> Result<Vec<ConsoleLog>> {
        tokio::time::timeout(timeout_dur, async {
            loop {
                let logs = self.get_console_logs().await?;

                if logs.as_slice() == expected {
                    return Ok(logs);
                }

                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .map_err(|_| anyhow::Error::msg("Timed out waiting for expected console logs"))?
    }

    pub async fn clear_console_logs(&mut self) -> Result<()> {
        self.execute("sessionStorage.removeItem('__consoleLogs__');", vec![])
            .await?;
        Ok(())
    }
}
