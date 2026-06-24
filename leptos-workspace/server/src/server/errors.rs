use std::net::SocketAddr;

use leptos::config::errors::LeptosConfigError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    // ====== Core (Server) =====
    #[error("Join error: {0}")]
    Join(#[from] tokio::task::JoinError),

    // ====== Leptos Axum =====
    #[error("LeptosConfig: {0}")]
    LeptosConfig(#[from] LeptosConfigError),

    #[error("{addr} Adress is used: {source}")]
    AdressUsed {
        addr: SocketAddr,
        #[source]
        source: std::io::Error,
    },
    {%- if cucumber == true %}

    // ====== feature `cucumber` =====
    #[cfg(feature = "cucumber")]
    #[error("Io: {0}")]
    Io(#[from] std::io::Error),
    {%- endif %}
}
