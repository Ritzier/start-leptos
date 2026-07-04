use tokio_util::sync::CancellationToken;

pub struct ShutdownManager {
    token: CancellationToken,
}

impl ShutdownManager {
    pub fn new() -> Self {
        Self {
            token: CancellationToken::new(),
        }
    }

    /// Returns a child token that gets cancelled when the root is cancelled.
    /// Pass this to subsystems (GlobalServer, axum::serve, etc.)
    pub fn child(&self) -> CancellationToken {
        self.token.child_token()
    }

    /// Cancel the root token, propagating cancellation to all children.
    pub fn shutdown(&self) {
        self.token.cancel();
    }

    /// Returns a future that resolves when the manager is shut down.
    /// Useful for `tokio::select!` branches.
    pub async fn cancelled(&self) {
        self.token.cancelled().await;
    }

    /// Whether shutdown has already been triggered.
    pub fn is_cancelled(&self) -> bool {
        self.token.is_cancelled()
    }
}

impl Default for ShutdownManager {
    fn default() -> Self {
        Self::new()
    }
}
