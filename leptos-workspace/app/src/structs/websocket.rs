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
                        {%- if tracing == "yes" %}
                        tracing::info!("User connected: {uuid}");
                        {%- else %}
                        leptos::logging::log!("User connected: {uuid}");
                        {%- endif %}

                        if let Err(e) = tx.unbounded_send(Ok(Response::HandshakeResponse)) {
                            {%- if tracing == "yes" %}
                            tracing::error!("Failed to send: {e}");
                            {%- else %}
                            leptos::logging::error!("Failed to send: {e}");
                            {%- endif %}
                        }
                    }

                    Request::Disconnect { uuid } => {
                        {%- if tracing == "yes" %}
                        tracing::info!("User disconnect: {uuid}");
                        {%- else %}
                        leptos::logging::log!("User disconnect: {uuid}");
                        {%- endif %}
                    }
                },

                Err(e) => {
                    {%- if tracing == "yes" %}
                    tracing::error!("Received error: {e}");
                    {%- else %}
                    leptos::logging::error!("Received error: {e}");
                    {%- endif %}
                }
            }
        }
    });

    Ok(rx.into())
}
