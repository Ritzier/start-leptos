//! Client-side WebSocket manager trait.
//!
//! This module provides a trait-based abstraction for WebSocket client management
//! with generic message handling. It enables type-safe WebSocket communication
//! between client and server with custom message types.
//!
//! # Example
//!
//! ```rust
//! // Define your message types
//! enum Request { Ping, Disconnect }
//! enum Response { Pong }
//!
//! // Implement the WebSocketClient trait
//! impl WebSocketClient for MyClient {
//!     type Request = Request;
//!     type Response = Response;
//!     // ... implement required methods
//! }
//!
//! // Create and use the manager
//! let manager = MyClient::new().create_manager();
//! manager.connect();
//! ```

use futures::StreamExt;
use futures::channel::mpsc::{self, UnboundedSender};
use leptos::prelude::*;
use leptos::server_fn::BoxedStream;

// ============================================================================
// Type Aliases
// ============================================================================

/// Channel sender for WebSocket requests wrapped in Result.
///
/// This sender transmits requests from client to server, with each request
/// wrapped in a Result to handle potential serialization errors.
type RequestSender<T> = UnboundedSender<Result<T, ServerFnError>>;

/// Optional sender stored in reactive context.
///
/// The Option allows the sender to be None when disconnected and Some when connected.
/// StoredValue provides reactive storage that persists across component re-renders.
type OptionalSender<T> = Option<RequestSender<T>>;

// ============================================================================
// WebSocketClient Trait
// ============================================================================

/// Trait for client-side WebSocket message handling.
///
/// Implement this trait to define client-side WebSocket behavior for different
/// message types. This trait abstracts the WebSocket lifecycle (connect, send,
/// receive, disconnect) while allowing custom message handling logic.
///
/// # Type Parameters
///
/// * `Request` - Messages sent from client to server
/// * `Response` - Messages received from server to client
///
/// # Required Methods
///
/// * `create_handshake_request` - Initial connection message
/// * `create_disconnect_request` - Graceful disconnection message
/// * `handle_response` - Process incoming server responses
/// * `get_stream` - Establish WebSocket connection
pub trait WebSocketClient: Clone + 'static {
    /// Request type sent to server.
    ///
    /// Must implement Send for cross-thread safety in async contexts.
    type Request: Send + 'static;

    /// Response type received from server.
    ///
    /// Must implement Send for cross-thread safety in async contexts.
    type Response: Send + 'static;

    /// Create a new WebSocket manager instance from this client.
    ///
    /// This is a convenience method that wraps the client in a
    /// GenericWebSocketManager for easier instantiation.
    ///
    /// # Returns
    ///
    /// A new WebSocket manager configured with this client implementation.
    fn create_manager(self) -> GenericWebSocketManager<Self> {
        GenericWebSocketManager::new_with_client(self)
    }

    /// Create a handshake request to establish the WebSocket connection.
    ///
    /// This is called automatically when `connect()` is invoked. Typically
    /// includes authentication data, session IDs, or other initialization info.
    ///
    /// # Returns
    ///
    /// The initial request message sent to the server.
    fn create_handshake_request(&self) -> Self::Request;

    /// Create a disconnect request for graceful connection closure.
    ///
    /// This is called when `disconnect()` is invoked. Allows the server to
    /// clean up resources (e.g., remove from active connections, log session end).
    ///
    /// # Returns
    ///
    /// The disconnection request message sent to the server.
    fn create_disconnect_request(&self) -> Self::Request;

    /// Handle an incoming response from the server.
    ///
    /// This method is called for each message received from the server.
    /// Implement custom logic to process responses and update UI state.
    ///
    /// # Arguments
    ///
    /// * `response` - The response message to handle
    /// * `is_connected` - Reactive signal to update connection state
    ///
    /// # Example
    ///
    /// ```rust
    /// fn handle_response(response: Response, is_connected: RwSignal<bool>) {
    ///     match response {
    ///         Response::Pong => {
    ///             is_connected.set(true);
    ///             log!("Connection confirmed");
    ///         }
    ///         Response::Error(msg) => {
    ///             log!("Error: {}", msg);
    ///         }
    ///     }
    /// }
    /// ```
    fn handle_response(response: Self::Response, is_connected: RwSignal<bool>);

    /// Get the WebSocket stream from the server.
    ///
    /// This method calls the actual server function that establishes the
    /// WebSocket connection. The server function should be annotated with
    /// `#[server(protocol = Websocket<...>)]`.
    ///
    /// # Arguments
    ///
    /// * `rx` - Receiver for outgoing requests (client → server)
    ///
    /// # Returns
    ///
    /// * `Ok(BoxedStream)` - Stream of incoming responses (server → client)
    /// * `Err(ServerFnError)` - Connection establishment failed
    fn get_stream(
        rx: futures::channel::mpsc::UnboundedReceiver<Result<Self::Request, ServerFnError>>,
    ) -> impl std::future::Future<
        Output = Result<BoxedStream<Self::Response, ServerFnError>, ServerFnError>,
    > + Send;
}

// ============================================================================
// GenericWebSocketManager
// ============================================================================

/// Generic WebSocket manager for client-side connections.
///
/// Manages the WebSocket connection lifecycle (connect, send, disconnect) and
/// message handling on the client side. This struct is generic over any type
/// implementing `WebSocketClient`, enabling reusable WebSocket logic.
///
/// # Type Parameters
///
/// * `T` - The WebSocket client implementation defining message types and handlers
///
/// # Fields
///
/// * `tx` - Channel sender for outgoing requests (stored reactively)
/// * `is_connected` - Reactive signal tracking connection state
/// * `client` - The client implementation containing business logic
///
/// # Example
///
/// ```rust
/// let manager = WebSocketManager::new(uuid);
/// manager.connect();
/// manager.send(Request::Ping)?;
/// manager.disconnect();
/// ```
#[derive(Clone)]
pub struct GenericWebSocketManager<T: WebSocketClient> {
    /// Channel sender for outgoing requests to the server.
    ///
    /// Stored in a reactive StoredValue to persist across re-renders.
    /// None when disconnected, Some when connected.
    tx: StoredValue<OptionalSender<T::Request>>,

    /// Reactive signal indicating connection status.
    ///
    /// - `false` - Disconnected or connection failed
    /// - `true` - Connected and ready to send/receive
    ///
    /// This signal can be used in UI components to show connection state.
    pub is_connected: RwSignal<bool>,

    /// The client implementation defining message types and handlers.
    ///
    /// Contains the business logic for creating requests and handling responses.
    client: T,
}

impl<T: WebSocketClient> GenericWebSocketManager<T> {
    /// Creates a new WebSocket manager with the given client implementation.
    ///
    /// This is used internally by the `create_manager()` trait method.
    /// Prefer using type-specific constructors (e.g., `WebSocketManager::new(uuid)`)
    /// for better ergonomics.
    ///
    /// # Arguments
    ///
    /// * `client` - The WebSocket client implementation
    ///
    /// # Returns
    ///
    /// A new manager instance in disconnected state.
    fn new_with_client(client: T) -> Self {
        Self {
            tx: StoredValue::new(None),
            is_connected: RwSignal::new(false),
            client,
        }
    }

    /// Establishes WebSocket connection and starts listening for responses.
    ///
    /// This method:
    /// 1. Creates a new unbounded channel for bidirectional communication
    /// 2. Sends a handshake request to the server
    /// 3. Spawns an async task to listen for incoming responses
    /// 4. Updates the `is_connected` signal based on connection state
    ///
    /// # Behavior
    ///
    /// - Non-blocking: Spawns a background task to handle responses
    /// - Idempotent: Safe to call multiple times (creates new connection each time)
    /// - Error handling: Logs errors and sets `is_connected` to false on failure
    ///
    /// # Example
    ///
    /// ```rust
    /// let manager = WebSocketManager::new(uuid);
    /// manager.connect(); // Starts connection in background
    /// ```
    pub fn connect(&self) {
        // Create unbounded channel for bidirectional communication
        // tx: send requests to server
        // rx: will be converted to stream by server function
        let (tx, rx) = mpsc::unbounded();

        // Send initial handshake request to establish connection
        let handshake = self.client.create_handshake_request();
        if let Err(e) = tx.unbounded_send(Ok(handshake)) {
            leptos::logging::error!("Failed to send handshake: {e}");
            return;
        }

        // Store the sender for future use in send() method
        self.tx.set_value(Some(tx));
        let is_connected = self.is_connected;

        // Spawn async task to handle incoming responses
        leptos::task::spawn_local(async move {
            // Establish WebSocket stream via server function
            let mut stream = match T::get_stream(rx).await {
                Ok(stream) => stream,
                Err(e) => {
                    leptos::logging::error!("Failed to connect websocket: {e}");
                    is_connected.set(false);
                    return;
                }
            };

            // Listen for incoming responses until connection closes
            while let Some(response) = stream.next().await {
                let response = match response {
                    Ok(response) => response,
                    Err(e) => {
                        // Handle WebSocket closure (code 1005 = normal closure)
                        match e.to_string().as_ref() {
                            "error reaching server to call server function: WebSocket Closed: code: 1005, reason:" =>
                            {
                                leptos::logging::log!("Websocket closed: {e}");
                                is_connected.set(false);
                                return;
                            }
                            // Log other errors but continue listening
                            _ => {
                                leptos::logging::error!("error: {e}");
                                continue;
                            }
                        }
                    }
                };

                // Delegate response handling to client implementation
                T::handle_response(response, is_connected);
            }
        });
    }

    /// Sends a request through the WebSocket connection.
    ///
    /// Requires an active connection established via `connect()`.
    /// Messages are queued and sent asynchronously.
    ///
    /// # Arguments
    ///
    /// * `request` - The request message to send to the server
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Message queued successfully
    /// * `Err(String)` - Connection unavailable or send failed
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Not connected (tx is None)
    /// - Channel is closed (server disconnected)
    ///
    /// # Example
    ///
    /// ```rust
    /// manager.send(Request::Ping)?;
    /// manager.send(Request::Message("Hello".to_string()))?;
    /// ```
    pub fn send(&self, request: T::Request) -> Result<(), String> {
        match self.tx.get_value() {
            Some(tx) => {
                // Send request through the channel
                tx.unbounded_send(Ok(request))
                    .map_err(|e| format!("Failed to send request: {e}"))
            }
            None => {
                // Connection not established or already closed
                leptos::logging::error!("tx value is None");
                self.is_connected.set(false);
                Err("Connection not available".to_string())
            }
        }
    }

    /// Gracefully disconnects the WebSocket.
    ///
    /// Sends a disconnect request to notify the server, then updates the
    /// connection state. The server should clean up resources and close
    /// the connection upon receiving the disconnect request.
    ///
    /// # Behavior
    ///
    /// 1. Sends disconnect request to server
    /// 2. Sets `is_connected` to false
    /// 3. Logs any errors during disconnection
    ///
    /// # Example
    ///
    /// ```rust
    /// manager.disconnect(); // Graceful shutdown
    /// ```
    pub fn disconnect(&self) {
        // Create and send disconnect request
        let disconnect = self.client.create_disconnect_request();
        if let Err(e) = self.send(disconnect) {
            leptos::logging::error!("{e}");
        }

        // Update connection state immediately
        // The listening task will terminate when the stream closes
        self.is_connected.set(false);
    }
}
