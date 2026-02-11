//! WebSocket server-side abstractions.
//!
//! This module provides a trait-based abstraction for handling WebSocket
//! connections on the server side. It enables generic, reusable WebSocket
//! backends that work with any message type.
//!
//! # Architecture
//!
//! - [`ResponseSender`] - Extension trait for convenient response sending
//! - [`WebSocketMessage`] - Trait defining message handling logic
//! - [`GenericWebsocketBackend`] - Generic server implementation
//!
//! # Example
//!
//! ```rust
//! use crate::ws_core::server::{WebSocketMessage, ResponseSender, GenericWebsocketBackend};
//! use futures::channel::mpsc;
//!
//! // 1. Define your message handler
//! pub struct MyMessageHandler;
//!
//! impl WebSocketMessage for MyMessageHandler {
//!     type Request = MyRequest;
//!     type Response = MyResponse;
//!
//!     async fn handle_request(&mut self, request: Self::Request, tx: &UnboundedSender<...>) -> bool {
//!         // Clean response sending with automatic error handling
//!         tx.send_response(MyResponse::Success);
//!         true
//!     }
//! }
//!
//! // 2. Create the backend in your server function
//! #[server(protocol = Websocket<RkyvEncoding, RkyvEncoding>)]
//! pub async fn my_websocket(
//!     input: BoxedStream<Request, ServerFnError>,
//! ) -> Result<BoxedStream<Response, ServerFnError>, ServerFnError> {
//!     let (tx, rx) = mpsc::unbounded();
//!     let backend = GenericWebsocketBackend::new(input, tx, MyMessageHandler);
//!
//!     tokio::spawn(async move {
//!         backend.serve().await;
//!     });
//!
//!     Ok(rx.into())
//! }
//! ```

mod backend;
mod message;
mod response_sender;

pub use backend::GenericWebsocketBackend;
pub use message::WebSocketMessage;
pub use response_sender::ResponseSender;
