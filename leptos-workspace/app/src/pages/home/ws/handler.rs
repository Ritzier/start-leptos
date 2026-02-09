//! Rkyv WebSocket backend implementation.

use futures::channel::mpsc::UnboundedSender;
use leptos::prelude::*;

use crate::ws_core::server::WebSocketMessage;

use super::message::{Request, Response};

/// Rkyv WebSocket message handler.
pub struct RkyvWebSocketMessage;

impl WebSocketMessage for RkyvWebSocketMessage {
    type Request = Request;
    type Response = Response;

    fn handle_request(
        request: Self::Request,
        tx: &UnboundedSender<Result<Self::Response, ServerFnError>>,
    ) -> bool {
        match request {
            Request::Handshake { uuid } => {
                {%- if tracing == true %}
                tracing::info!("User connected: {uuid}");
                {%- else %}
                leptos::logging::log!("User connected: {uuid}");
                {%- endif %}
                if let Err(e) = tx.unbounded_send(Ok(Response::HandshakeResponse)) {
                    {%- if tracing == true %}
                    tracing::info!("Failed send Response to client: {e}");
                    {%- else %}
                    leptos::logging::log!("Failed send Response to client: {e}");
                    {%- endif %}
                }
                true
            }
            Request::Disconnect { uuid } => {
                {%- if tracing == true %}
                tracing::info!("User disconnect: {uuid}");
                {%- else %}
                leptos::logging::log!("User disconnected: {uuid}");
                {%- endif %}
                false
            }
        }
    }
}
