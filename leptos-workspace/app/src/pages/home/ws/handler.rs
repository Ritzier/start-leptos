use futures::channel::mpsc::UnboundedSender;
use leptos::prelude::*;

use crate::ws_core::server::{ResponseSender, WebSocketMessage};

use super::message::{Request, Response};

pub struct RkyvWebSocketMessage;

impl WebSocketMessage for RkyvWebSocketMessage {
    type Request = Request;
    type Response = Response;

    async fn handle_request(
        &mut self,
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
                tx.send_response(Response::HandshakeResponse);

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
