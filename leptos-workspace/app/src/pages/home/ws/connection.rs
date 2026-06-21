use leptos::prelude::*;
use leptos::server_fn::codec::RkyvEncoding;
use leptos::server_fn::{BoxedStream, Websocket};

use super::message::{Request, Response};

#[server(protocol = Websocket<RkyvEncoding, RkyvEncoding>)]
#[lazy]
pub async fn rkyv_websocket(
    input: BoxedStream<Request, ServerFnError>,
) -> Result<BoxedStream<Response, ServerFnError>, ServerFnError> {
    use futures::channel::mpsc;
    use websocket_trait::server::GenericWebsocketBackend;

    use super::handler::RkyvWebSocketMessage;

    let (tx, rx) = mpsc::unbounded();
    let websocket_backend =
        GenericWebsocketBackend::<RkyvWebSocketMessage>::new(input, tx, RkyvWebSocketMessage);

    tokio::spawn(async move {
        websocket_backend.serve().await;
    });

    Ok(rx.into())
}
