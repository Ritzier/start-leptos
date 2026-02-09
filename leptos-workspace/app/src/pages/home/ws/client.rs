use leptos::prelude::*;
use leptos::server_fn::BoxedStream;
use uuid::Uuid;

use crate::ws_core::client::{GenericWebSocketManager, WebSocketClient};

use super::connection::rkyv_websocket;
use super::message::{Request, Response};

/// Rkyv WebSocket client implementation.
#[derive(Clone)]
pub struct RkyvWebSocketClient {
    uuid: StoredValue<Uuid>,
}

impl RkyvWebSocketClient {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid: StoredValue::new(uuid),
        }
    }
}

impl WebSocketClient for RkyvWebSocketClient {
    type Request = Request;
    type Response = Response;

    fn create_handshake_request(&self) -> Self::Request {
        Request::Handshake {
            uuid: self.uuid.get_value(),
        }
    }

    fn create_disconnect_request(&self) -> Self::Request {
        Request::Disconnect {
            uuid: self.uuid.get_value(),
        }
    }

    fn handle_response(response: Self::Response, is_connected: RwSignal<bool>) {
        match response {
            Response::HandshakeResponse => {
                is_connected.set(true);
                leptos::logging::log!("Received: FrontendResponse::HandshakeResponse");
            }
        }
    }

    async fn get_stream(
        rx: futures::channel::mpsc::UnboundedReceiver<Result<Self::Request, ServerFnError>>,
    ) -> Result<BoxedStream<Self::Response, ServerFnError>, ServerFnError> {
        rkyv_websocket(rx.into()).await
    }
}

/// WebSocket manager with Rkyv encoding.
pub type WebSocketManager = GenericWebSocketManager<RkyvWebSocketClient>;

// Provide a convenient constructor for WebSocketManager
impl WebSocketManager {
    /// Creates a new WebSocket manager with a UUID.
    pub fn new(uuid: Uuid) -> Self {
        RkyvWebSocketClient::new(uuid).create_manager()
    }
}
