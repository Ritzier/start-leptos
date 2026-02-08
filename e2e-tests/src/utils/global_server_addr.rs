//! Global server address storage for tests.
//!
//! Uses `OnceLock` to store the server address once and share it
//! across all test contexts.

use std::net::SocketAddr;
use std::sync::OnceLock;

/// Global storage for server address (initialized once by `LeptosServer`).
static SERVER_ADDR: OnceLock<SocketAddr> = OnceLock::new();

/// Sets the global server address.
///
/// Called by `LeptosServer::serve()` after finding an available port.
/// Can only be called once - subsequent calls will panic.
///
/// # Panics
/// Panics if called more than once
///
/// # Example
/// ```rust
/// let addr = SocketAddr::from((, 8080));[1]
/// set_server_addr(addr);
/// ```
pub fn set_server_addr(addr: SocketAddr) {
    SERVER_ADDR.set(addr).expect("Server address already set");
}

/// Gets the global server address.
///
/// Called by `AppWorld::new()` to determine where to connect.
///
/// # Panics
/// Panics if server address hasn't been initialized yet
///
/// # Example
/// ```rust
/// let addr = get_server_addr();
/// let url = format!("http://{}/", addr);
/// ```
pub fn get_server_addr() -> SocketAddr {
    *SERVER_ADDR.get().expect("Server address not initialized")
}
