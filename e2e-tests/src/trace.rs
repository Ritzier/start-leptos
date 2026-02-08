//! Tracing setup for structured logging.
//!
//! Configures `tracing-subscriber` with environment-based filtering.

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer, fmt};

/// Tracing configuration utility.
pub struct Trace;

impl Trace {
    /// Initializes the tracing subscriber.
    ///
    /// # Log Levels
    /// - **Debug build**: `e2e_tests=debug, cargo_leptos=debug`
    /// - **Release build**: `e2e_tests=info, cargo_leptos=info`
    ///
    /// Override with `RUST_LOG` environment variable:
    /// ```bash
    /// RUST_LOG=debug cargo run --bin cucumber
    /// RUST_LOG=e2e_tests=trace cargo run --bin cucumber
    /// ```
    pub fn setup() {
        let cargo_crate_name = env!("CARGO_CRATE_NAME");

        // Different log levels for debug vs release
        let base_filter = match cfg!(debug_assertions) {
            true => format!("{cargo_crate_name}=debug,cargo_leptos=debug"),
            false => format!("{cargo_crate_name}=info,cargo_leptos=info"),
        };

        tracing_subscriber::registry()
            .with(fmt::layer().with_writer(std::io::stdout).with_filter(
                // Use RUST_LOG env var, or fallback to base_filter
                EnvFilter::try_from_default_env().unwrap_or_else(|_| base_filter.into()),
            ))
            .init();
    }
}
