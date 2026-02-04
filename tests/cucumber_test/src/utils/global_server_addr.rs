use std::net::SocketAddr;
use std::sync::OnceLock;

// Global storage for server address
static SERVER_ADDR: OnceLock<SocketAddr> = OnceLock::new();

pub fn set_server_addr(addr: SocketAddr) {
    SERVER_ADDR.set(addr).expect("Server address already set");
}

pub fn get_server_addr() -> SocketAddr {
    *SERVER_ADDR.get().expect("Server address not initialized")
}
