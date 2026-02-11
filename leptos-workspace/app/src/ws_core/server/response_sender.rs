//! Extension trait for convenient WebSocket response sending.
//!
//! This module provides the `ResponseSender` trait that adds a `send_response()`
//! method to `UnboundedSender`, reducing boilerplate code and providing
//! consistent error handling across WebSocket handlers.

use futures::channel::mpsc::UnboundedSender;
use leptos::prelude::ServerFnError;

/// Extension trait for convenient response sending with automatic error logging.
///
/// This trait adds a `send_response()` method to `UnboundedSender` that wraps
/// the response in `Ok()` and logs any send failures automatically. This reduces
/// boilerplate code in WebSocket message handlers.
///
/// # Benefits
///
/// - Cleaner, more readable code (one line instead of three)
/// - Consistent error handling across all handlers
/// - Automatic logging with appropriate level (warn)
/// - Type-safe (only works with correct sender type)
///
/// # Example
///
/// ```rust
/// use crate::ws_core::server::{WebSocketMessage, ResponseSender};
///
/// // Before: verbose error handling
/// if let Err(e) = tx.unbounded_send(Ok(Response::Success)) {
///     tracing::warn!("Failed to send response: {e}");
/// }
///
/// // After: clean and concise
/// tx.send_response(Response::Success);
/// ```
pub trait ResponseSender<T> {
    /// Send a response through the WebSocket channel with automatic error handling.
    ///
    /// This method wraps the response in `Ok()` and sends it through the channel.
    /// If sending fails (e.g., channel closed), the error is logged automatically.
    ///
    /// # Arguments
    ///
    /// * `response` - The response message to send to the client
    ///
    /// # Returns
    ///
    /// * `true` - Message sent successfully
    /// * `false` - Failed to send (typically because the channel is closed)
    ///
    /// # Example
    ///
    /// ```rust
    /// impl WebSocketMessage for MyHandler {
    ///     async fn handle_request(&mut self, request: Request, tx: &UnboundedSender<...>) -> bool {
    ///         match request {
    ///             Request::Ping => {
    ///                 tx.send_response(Response::Pong);
    ///                 true
    ///             }
    ///             Request::GetData => {
    ///                 if tx.send_response(Response::Data(data)) {
    ///                     tracing::debug!("Data sent successfully");
    ///                 }
    ///                 true
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    fn send_response(&self, response: T) -> bool;
}

impl<T> ResponseSender<T> for UnboundedSender<Result<T, ServerFnError>> {
    fn send_response(&self, response: T) -> bool {
        if let Err(e) = self.unbounded_send(Ok(response)) {
            {%- if tracing == true %}
            tracing::warn!("Failed to send response to client: {e}");
            {%- else %}
            leptos::logging::warn!("Failed to send response to client: {e}");
            {%- endif %}
            false
        } else {
            true
        }
    }
}
