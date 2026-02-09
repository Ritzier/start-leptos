//! WebSocket trait abstraction for generic WebSocket handling.
//!
//! This module provides server-side traits and implementations for handling
//! WebSocket connections. It enables generic, reusable WebSocket backends
//! that work with any message type implementing the `WebSocketMessage` trait.
//!
//! # Architecture
//!
//! - `WebSocketMessage` - Trait defining message handling logic
//! - `GenericWebsocketBackend` - Generic server implementation
//!
//! # Example
//!
//! ```rust
//! pub struct MyMessageHandler;
//!
//! impl WebSocketMessage for MyMessageHandler {
//!     type Request = MyRequest;
//!     type Response = MyResponse;
//!
//!     fn handle_request(request: Self::Request, tx: &UnboundedSender<...>) -> bool {
//!         // Process request and send response
//!         tx.unbounded_send(Ok(MyResponse::Success)).ok();
//!         true // Continue processing
//!     }
//! }
//! ```

use futures::StreamExt;
use futures::channel::mpsc::UnboundedSender;
use leptos::prelude::*;
use leptos::server_fn::BoxedStream;

// ============================================================================
// WebSocketMessage Trait
// ============================================================================

/// Trait for WebSocket message handling on the server side.
///
/// Implement this trait to define how your WebSocket messages are processed
/// on the backend. Each implementation handles a specific message protocol
/// and business logic.
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
    /// - `tx.unbounded_send(Ok(response))` - Send a response
    /// - Don't send anything for one-way messages
    /// - Send multiple responses for streaming data
    ///
    /// # Example
    ///
    /// ```rust
    /// fn handle_request(request: Request, tx: &UnboundedSender<...>) -> bool {
    ///     match request {
    ///         Request::Handshake { uuid } => {
    ///             // Log connection
    ///             leptos::logging::log!("User connected: {uuid}");
    ///             
    ///             // Send acknowledgment
    ///             tx.unbounded_send(Ok(Response::Connected)).ok();
    ///             
    ///             true // Keep connection alive
    ///         }
    ///         Request::Disconnect { uuid } => {
    ///             leptos::logging::log!("User disconnecting: {uuid}");
    ///             false // Close connection
    ///         }
    ///         Request::Ping => {
    ///             tx.unbounded_send(Ok(Response::Pong)).ok();
    ///             true
    ///         }
    ///     }
    /// }
    /// ```
    fn handle_request(
        request: Self::Request,
        tx: &UnboundedSender<Result<Self::Response, ServerFnError>>,
    ) -> bool;
}

// ============================================================================
// GenericWebsocketBackend
// ============================================================================

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
/// let (tx, rx) = mpsc::unbounded();
/// let backend = GenericWebsocketBackend::<MyMessageHandler>::new(input, tx);
///
/// tokio::spawn(async move {
///     backend.serve().await;
/// });
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
    ///
    /// # Returns
    ///
    /// A new backend instance ready to call `serve()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// #[server(protocol = Websocket<RkyvEncoding, RkyvEncoding>)]
    /// pub async fn my_websocket(
    ///     input: BoxedStream<Request, ServerFnError>,
    /// ) -> Result<BoxedStream<Response, ServerFnError>, ServerFnError> {
    ///     let (tx, rx) = mpsc::unbounded();
    ///     let backend = GenericWebsocketBackend::<MyHandler>::new(input, tx);
    ///     
    ///     tokio::spawn(async move {
    ///         backend.serve().await;
    ///     });
    ///     
    ///     Ok(rx.into())
    /// }
    /// ```
    pub fn new(
        input: BoxedStream<T::Request, ServerFnError>,
        tx: UnboundedSender<Result<T::Response, ServerFnError>>,
    ) -> Self {
        Self { input, tx }
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
    ///     leptos::logging::log!("WebSocket connection closed");
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
                    if !self.handle_input_result(input_result) {
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
    fn handle_input_result(&self, input_result: Option<Result<T::Request, ServerFnError>>) -> bool {
        match input_result {
            // Successfully received and deserialized a request
            Some(Ok(request)) => {
                // Delegate to the trait implementation
                // Returns true to continue, false to close connection
                T::handle_request(request, &self.tx)
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
