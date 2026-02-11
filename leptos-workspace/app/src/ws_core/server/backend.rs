//! Generic WebSocket backend implementation.
//!
//! This module provides the `GenericWebsocketBackend` struct that handles
//! the server-side WebSocket connection lifecycle and event loop.

use futures::StreamExt;
use futures::channel::mpsc::UnboundedSender;
use leptos::prelude::ServerFnError;
use leptos::server_fn::BoxedStream;

use super::message::WebSocketMessage;

/// Generic WebSocket backend that works with any message type.
///
/// This backend handles the server-side WebSocket connection lifecycle and
/// delegates message handling to the trait implementation. It manages the
/// event loop, error handling, and connection cleanup.
///
/// # Type Parameters
///
/// * `T` - The WebSocketMessage implementation defining message handlers
///
/// # Lifecycle
///
/// 1. Created via `new()` with input stream and response channel
/// 2. `serve()` starts the event loop
/// 3. Processes messages until connection closes or error occurs
/// 4. Automatically cleans up resources on exit
///
/// # Example
///
/// ```rust
/// use futures::channel::mpsc;
/// use crate::ws_core::server::{GenericWebsocketBackend, MyHandler};
///
/// #[server(protocol = Websocket<RkyvEncoding, RkyvEncoding>)]
/// pub async fn my_websocket(
///     input: BoxedStream<Request, ServerFnError>,
/// ) -> Result<BoxedStream<Response, ServerFnError>, ServerFnError> {
///     let (tx, rx) = mpsc::unbounded();
///     let backend = GenericWebsocketBackend::new(input, tx, MyHandler::new());
///
///     tokio::spawn(async move {
///         backend.serve().await;
///     });
///
///     Ok(rx.into())
/// }
/// ```
pub struct GenericWebsocketBackend<T: WebSocketMessage> {
    /// Stream of incoming requests from the client.
    ///
    /// This stream is provided by the Leptos server function and
    /// deserializes messages from the WebSocket connection.
    input: BoxedStream<T::Request, ServerFnError>,

    /// Channel to send responses back to the client.
    ///
    /// Responses sent through this channel are serialized and
    /// transmitted over the WebSocket connection.
    tx: UnboundedSender<Result<T::Response, ServerFnError>>,

    /// The message handler implementation.
    ///
    /// This handler processes all incoming requests and generates responses.
    handler: T,
}

impl<T: WebSocketMessage> GenericWebsocketBackend<T> {
    /// Creates a new WebSocket backend.
    ///
    /// This constructor is typically called from within a Leptos server function
    /// annotated with `#[server(protocol = Websocket<...>)]`.
    ///
    /// # Arguments
    ///
    /// * `input` - Stream of incoming requests from the client
    /// * `tx` - Channel to send responses back to the client
    /// * `handler` - The message handler implementation
    ///
    /// # Returns
    ///
    /// A new backend instance ready to call `serve()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// let (tx, rx) = mpsc::unbounded();
    /// let backend = GenericWebsocketBackend::new(input, tx, MyHandler);
    ///
    /// tokio::spawn(async move {
    ///     backend.serve().await;
    /// });
    /// ```
    pub fn new(
        input: BoxedStream<T::Request, ServerFnError>,
        tx: UnboundedSender<Result<T::Response, ServerFnError>>,
        handler: T,
    ) -> Self {
        Self { input, tx, handler }
    }

    /// Starts the WebSocket message processing loop.
    ///
    /// This method runs until the connection is closed or an error occurs.
    /// It should be spawned in a separate task to avoid blocking.
    ///
    /// # Behavior
    ///
    /// - Continuously polls the input stream for new messages
    /// - Delegates each message to `handle_input_result()`
    /// - Exits loop when handler returns false or stream ends
    /// - Automatically cleans up resources on exit
    ///
    /// # Async Context
    ///
    /// Uses `tokio::select!` to handle async events. This allows future
    /// expansion for additional event sources (timeouts, broadcasts, etc.).
    ///
    /// # Example
    ///
    /// ```rust
    /// tokio::spawn(async move {
    ///     backend.serve().await; // Runs until connection closes
    ///     tracing::info!("WebSocket connection closed");
    /// });
    /// ```
    pub async fn serve(mut self) {
        // Main event loop
        loop {
            // Use tokio::select! for handling multiple async event sources
            // Currently only polls input stream, but easily extensible for:
            // - Timeouts
            // - Periodic pings
            // - Broadcast channels
            // - Server shutdown signals
            tokio::select! {
                input_result = self.input.next() => {
                    // Process the incoming message
                    if !self.handle_input_result(input_result).await {
                        // Handler returned false or stream ended - close connection
                        break;
                    }
                }
            }
        }
        // Implicit cleanup: tx and input are dropped here
        // This closes the response channel and releases resources
    }

    /// Handles a single input result from the stream.
    ///
    /// Processes one message from the client, handling success and error cases.
    /// Delegates actual message processing to the `WebSocketMessage` implementation.
    ///
    /// # Arguments
    ///
    /// * `input_result` - Result from the input stream
    ///   - `Some(Ok(request))` - Valid message received
    ///   - `Some(Err(e))` - Deserialization or network error
    ///   - `None` - Stream ended (client disconnected)
    ///
    /// # Returns
    ///
    /// * `true` - Continue processing (keep connection alive)
    /// * `false` - Stop processing (close connection)
    ///
    /// # Error Handling
    ///
    /// - Deserialization errors: Logged and connection closed
    /// - Handler returns false: Connection closed gracefully
    /// - Stream ends: Connection closed (client disconnected)
    async fn handle_input_result(
        &mut self,
        input_result: Option<Result<T::Request, ServerFnError>>,
    ) -> bool {
        match input_result {
            // Successfully received and deserialized a request
            Some(Ok(request)) => {
                // Delegate to the trait implementation
                // Returns true to continue, false to close connection
                self.handler.handle_request(request, &self.tx).await
            }

            // Error deserializing or receiving the message
            Some(Err(e)) => {
                {%- if tracing == true %}
                tracing::info!("Error receiving message: {e}");
                {%- else %}
                leptos::logging::log!("Error receiving message: {e}");
                {%- endif %}
                // Close connection on errors (could be network issue or bad data)
                false
            }

            // Stream ended (client disconnected or connection lost)
            None => {
                {%- if tracing == true %}
                tracing::info!("Input stream closed");
                {%- else %}
                leptos::logging::log!("Input stream closed");
                {%- endif %}
                // Clean up and close connection
                false
            }
        }
    }
}
