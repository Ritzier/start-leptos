use leptos::prelude::*;
use leptos::server_fn::codec::RkyvEncoding;
use leptos::server_fn::{BoxedStream, Websocket};

use super::{Request, Response};

#[server(protocol = Websocket<RkyvEncoding, RkyvEncoding>)]
#[lazy]
pub async fn rkyv_websocket(
    input: BoxedStream<Request, ServerFnError>,
) -> Result<BoxedStream<Response, ServerFnError>, ServerFnError> {
    use super::WebsocketBackend;
    use futures::channel::mpsc;

    let (tx, rx) = mpsc::unbounded();

    let websocket_backend = WebsocketBackend::new(input, tx);
    tokio::spawn(async move { websocket_backend.serve().await });

    Ok(rx.into())
}
