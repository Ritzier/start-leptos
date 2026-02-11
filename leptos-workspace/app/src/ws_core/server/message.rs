//! WebSocket message handling trait.
//!
//! This module defines the `WebSocketMessage` trait that must be implemented
//! by any WebSocket message handler. It provides the interface for processing
//! incoming requests and sending responses.

use std::future::Future;

use futures::channel::mpsc::UnboundedSender;
use leptos::prelude::ServerFnError;

/// Trait for WebSocket message handling on the server side.
///
/// Implement this trait to define how your WebSocket messages are processed
/// on the backend. Each implementation handles a specific message protocol
/// and business logic.
///
/// Use the `ResponseSender` extension trait for clean response sending:
/// `tx.send_response(response)` instead of `tx.unbounded_send(Ok(response))`.
///
/// # Type Parameters
///
/// * `Request` - Messages received from client
/// * `Response` - Messages sent to client
///
/// # Lifecycle
///
/// 1. Client connects and sends requests
/// 2. `handle_request()` is called for each incoming request
/// 3. Implementation processes request and optionally sends responses
/// 4. Returns `true` to continue or `false` to close connection
///
/// # Thread Safety
///
/// All types must implement Send to work across async boundaries.
///
/// # Example
///
/// ```rust
/// use crate::ws_core::server::{WebSocketMessage, ResponseSender};
///
/// pub struct ChatHandler {
///     user_id: Uuid,
/// }
///
/// impl WebSocketMessage for ChatHandler {
///     type Request = ChatRequest;
///     type Response = ChatResponse;
///
///     async fn handle_request(&mut self, request: Self::Request, tx: &UnboundedSender<...>) -> bool {
///         match request {
///             ChatRequest::SendMessage { content } => {
///                 // Process message...
///                 tx.send_response(ChatResponse::MessageSent);
///                 true
///             }
///             ChatRequest::Disconnect => {
///                 tx.send_response(ChatResponse::Goodbye);
///                 false // Close connection
///             }
///         }
///     }
/// }
/// ```
pub trait WebSocketMessage: Send + 'static {
    /// Request type from client to server.
    ///
    /// This is the message structure the client sends. Must be serializable
    /// with the encoding specified in the server function (e.g., Rkyv, JSON).
    type Request: Send + 'static;

    /// Response type from server to client.
    ///
    /// This is the message structure sent back to the client. Must be
    /// serializable with the same encoding as Request.
    type Response: Send + 'static;

    /// Handle an incoming request and optionally send a response.
    ///
    /// This method is called for each message received from the client.
    /// Implement your business logic here: validate requests, update state,
    /// send responses, broadcast to other clients, etc.
    ///
    /// Use `tx.send_response(response)` for clean, automatic error handling.
    ///
    /// # Arguments
    ///
    /// * `request` - The incoming request to handle
    /// * `tx` - Channel to send responses back to the client
    ///
    /// # Returns
    ///
    /// * `true` - Continue processing messages (keep connection alive)
    /// * `false` - Close the WebSocket connection (triggers cleanup)
    ///
    /// # Response Handling
    ///
    /// You can send zero, one, or multiple responses per request:
    /// - `tx.send_response(response)` - Send a response (recommended)
    /// - Don't send anything for one-way messages
    /// - Send multiple responses for streaming data
    ///
    /// # Example
    ///
    /// ```rust
    /// async fn handle_request(&mut self, request: Request, tx: &UnboundedSender<...>) -> bool {
    ///     match request {
    ///         Request::Handshake { uuid } => {
    ///             tracing::info!("User connected: {uuid}");
    ///             tx.send_response(Response::Connected);
    ///             true // Keep connection alive
    ///         }
    ///         Request::Disconnect { uuid } => {
    ///             tracing::info!("User disconnecting: {uuid}");
    ///             false // Close connection
    ///         }
    ///         Request::Ping => {
    ///             tx.send_response(Response::Pong);
    ///             true
    ///         }
    ///     }
    /// }
    /// ```
    fn handle_request(
        &mut self,
        request: Self::Request,
        tx: &UnboundedSender<Result<Self::Response, ServerFnError>>,
    ) -> impl Future<Output = bool> + Send;
}
