use futures::StreamExt;
use futures::channel::mpsc::{self, UnboundedSender};
use leptos::prelude::*;
use uuid::Uuid;

use super::{Request, Response, rkyv_websocket};

#[derive(Clone)]
pub struct WebSocketManager {
    tx: StoredValue<Option<UnboundedSender<Result<Request, ServerFnError>>>>,
    pub is_connected: RwSignal<bool>,
    pub uuid: StoredValue<Uuid>,
}

impl WebSocketManager {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            tx: StoredValue::new(None),
            is_connected: RwSignal::new(false),
            uuid: StoredValue::new(uuid),
        }
    }

    /// Establishes WebSocket connection and starts listening for responses
    pub fn connect(&self) {
        let (tx, rx) = mpsc::unbounded();
        let uuid = self.uuid.get_value();

        if let Err(e) = tx.unbounded_send(Ok(Request::Handshake { uuid })) {
            leptos::logging::error!("Failed to send `Request::HandShake`: {e}");
            return;
        }

        self.tx.set_value(Some(tx));

        let is_connected = self.is_connected;

        leptos::task::spawn_local(async move {
            let mut stream = match rkyv_websocket(rx.into()).await {
                Ok(stream) => stream,
                Err(e) => {
                    leptos::logging::error!("Failed to connect websocket: {e}");
                    is_connected.set(false);
                    return;
                }
            };

            while let Some(response) = stream.next().await {
                let response = match response {
                    Ok(response) => response,
                    Err(e) => match e.to_string().as_ref() {
                        "error reaching server to call server function: WebSocket Closed: code: 1005, reason: " =>
                        {
                            leptos::logging::log!("Websocket closed: {e}");
                            is_connected.set(false);
                            return;
                        }
                        error => {
                            leptos::logging::error!("{error}");
                            continue;
                        }
                    },
                };

                match response {
                    Response::HandshakeResponse => {
                        is_connected.set(true);
                        leptos::logging::log!("Received: FrontendResponse::HandshakeResponse");
                    }
                }
            }
        });
    }

    /// Sends a request through the WebSocket connection
    pub fn send(&self, request: Request) -> Result<(), String> {
        match self.tx.get_value() {
            Some(tx) => tx
                .unbounded_send(Ok(request))
                .map_err(|e| format!("Failed to send request: {e}")),
            None => {
                leptos::logging::error!("`tx` value is None");
                self.is_connected.set(false);
                Err("Connection not available".to_string())
            }
        }
    }

    /// Gracefully disconnects the WebSocket
    pub fn disconnect(&self) {
        let uuid = self.uuid.get_value();
        if let Err(e) = self.send(Request::Disconnect { uuid }) {
            leptos::logging::error!("{e}");
        }
        self.is_connected.set(false);
    }
}
