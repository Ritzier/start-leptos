use tokio::net::TcpListener;

use std::sync::atomic::{AtomicU16, Ordering};

static PORT_COUNTER: AtomicU16 = AtomicU16::new(8000);

pub struct PortFinder;

impl PortFinder {
    pub async fn get_available_port() -> Result<u16, String> {
        for _ in 0..1000 {
            let port = PORT_COUNTER.fetch_add(1, Ordering::SeqCst);

            if port >= 9000 {
                // Reset if we've exhausted the range
                PORT_COUNTER.store(8000, Ordering::SeqCst);
                continue;
            }

            if TcpListener::bind(("127.0.0.1", port)).await.is_ok() {
                return Ok(port);
            }
        }

        Err("Could not find available port in range 8000-8999 after 1000 attempts".to_string())
    }
}
