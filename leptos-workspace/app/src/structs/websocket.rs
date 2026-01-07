use leptos::prelude::*;
use leptos::server_fn::codec::RkyvEncoding;
use leptos::server_fn::{BoxedStream, Websocket};

use super::{Request, Response};

#[server(protocol = Websocket<RkyvEncoding, RkyvEncoding>)]
#[lazy]
pub async fn rkyv_websocket(
    input: BoxedStream<Request, ServerFnError>,
) -> Result<BoxedStream<Response, ServerFnError>, ServerFnError> {
    use futures::StreamExt;
    use futures::channel::mpsc;

    let (tx, rx) = mpsc::unbounded();

    tokio::spawn(async move {
        let mut input = input;

        while let Some(msg) = input.next().await {
            match msg {
                Ok(request) => match request {
                    Request::Handshake { uuid } => {
                        leptos::logging::log!("User connected: {uuid}");

                        if let Err(e) = tx.unbounded_send(Ok(Response::HandshakeResponse)) {
                            leptos::logging::error!("Failed to send: {e}");
                        }
                    }

                    Request::Disconnect { uuid } => {
                        leptos::logging::log!("User disconnect: {uuid}");
                    }
                },

                Err(e) => {
                    leptos::logging::error!("Received error: {e}");
                }
            }
        }
    });

    Ok(rx.into())
}
