use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Io: {0}")]
    Io(#[from] std::io::Error),

    // ====== Server =====
    #[error("Server: {0}")]
    Server(#[from] crate::server::ServerError),
}
