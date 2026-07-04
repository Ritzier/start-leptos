//! Port allocation for parallel test execution.
//!
//! Finds available ports in the range 8000-8999 using atomic counter.

use std::sync::atomic::{AtomicU16, Ordering};
use tokio::net::TcpListener;

/// Atomic counter for port allocation starting at 8999.
static PORT_COUNTER: AtomicU16 = AtomicU16::new(8999);

/// Port finder utility for test infrastructure.
pub struct PortFinder;

impl PortFinder {
    /// Finds an available port in the range 8000-8999 (descending).
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
    /// 1. Atomically decrement counter
    /// 2. Wrap to 8999 if below 8000
    /// 3. Try to bind port
    /// 4. Return port if successful, otherwise try next
    ///
    /// # Example
    /// ```ignore
    /// let port = PortFinder::get_available_port().await?;
    /// let addr = SocketAddr::from((, port));[1]
    /// ```
    pub async fn get_available_port() -> Result<u16, String> {
        for _ in 0..1000 {
            // Atomically increment and get next port
            let port = PORT_COUNTER.fetch_sub(1, Ordering::SeqCst);

            // Wrap if out of range (since u16 will underflow naturally, then correct it)
            if !(8000..=8999).contains(&port) {
                PORT_COUNTER.store(8999, Ordering::SeqCst);
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
