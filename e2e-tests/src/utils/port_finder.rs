//! Port allocation for parallel test execution.
//!
//! Finds available ports in the range 8000-8999 using atomic counter.

use std::sync::atomic::{AtomicU16, Ordering};
use tokio::net::TcpListener;

/// Atomic counter for port allocation starting at 8000.
static PORT_COUNTER: AtomicU16 = AtomicU16::new(8000);

/// Port finder utility for test infrastructure.
pub struct PortFinder;

impl PortFinder {
    /// Finds an available port in the range 8000-8999.
    ///
    /// Uses an atomic counter to reduce port conflicts in parallel tests.
    /// Tries to bind each port to verify availability.
    ///
    /// # Returns
    /// An available port number
    ///
    /// # Errors
    /// Returns error if no port is available after 1000 attempts
    ///
    /// # Algorithm
    /// 1. Atomically increment counter
    /// 2. Reset to 8000 if counter exceeds 9000
    /// 3. Try to bind port
    /// 4. Return port if successful, otherwise try next
    ///
    /// # Example
    /// ```rust
    /// let port = PortFinder::get_available_port().await?;
    /// let addr = SocketAddr::from((, port));[1]
    /// ```
    pub async fn get_available_port() -> Result<u16, String> {
        for _ in 0..1000 {
            // Atomically increment and get next port
            let port = PORT_COUNTER.fetch_add(1, Ordering::SeqCst);

            // Reset if we've exhausted the range
            if port >= 9000 {
                PORT_COUNTER.store(8000, Ordering::SeqCst);
                continue;
            }

            // Try to bind port to verify it's available
            if TcpListener::bind(("127.0.0.1", port)).await.is_ok() {
                return Ok(port);
            }
        }

        Err("Could not find available port in range 8000-8999 after 1000 attempts".to_string())
    }
}
