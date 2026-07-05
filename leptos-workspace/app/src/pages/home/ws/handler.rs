use futures::channel::mpsc::UnboundedSender;
use leptos::prelude::*;
use websocket_trait::server::{ResponseSender, WebSocketMessage};

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
                tracing::info!("User connected: {uuid}");
                tx.send_response(Response::HandshakeResponse);

                true
            }
            Request::Disconnect { uuid } => {
                tracing::info!("User disconnect: {uuid}");
                false
            }
        }
    }
}
