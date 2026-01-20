use futures::StreamExt;
use futures::channel::mpsc::UnboundedSender;
use leptos::prelude::*;
use leptos::server_fn::BoxedStream;

use super::{Request, Response};

pub struct WebsocketBackend {
    input: BoxedStream<Request, ServerFnError>,
    tx: UnboundedSender<Result<Response, ServerFnError>>,
}

impl WebsocketBackend {
    pub fn new(
        input: BoxedStream<Request, ServerFnError>,
        tx: UnboundedSender<Result<Response, ServerFnError>>,
    ) -> Self {
        Self { input, tx }
    }

    pub async fn serve(mut self) {
        loop {
            tokio::select! {
                input_result = self.input.next() => {
                    if !self.handle_input_result(input_result) {
                        break;
                    }
                }
            }
        }
    }

    fn handle_input_result(&self, input_result: Option<Result<Request, ServerFnError>>) -> bool {
        match input_result {
            Some(Ok(request)) => {
                match request {
                    Request::Handshake { uuid } => {
                        {%- if tracing == "yes" %}
                        tracing::info!("User connected: {uuid}");
                        {%- else %}
                        leptos::logging::info!("User connected: {uuid}");
                        {%- endif %}

                        if let Err(e) = self.tx.unbounded_send(Ok(Response::HandshakeResponse)) {
                            {%- if tracing == "yes" %}
                            tracing::info!("Failed send `Response` to client: {e}");
                            {%- else %}
                            leptos::logging::info!("Failed send `Response` to client: {e}");
                            {%- endif %}
                        }
                    }

                    Request::Disconnect { uuid } => {
                        {%- if tracing == "yes" %}
                        tracing::info!("User disconnect: {uuid}");
                        {%- else %}
                        leptos::logging::info!("User disconnect: {uuid}");
                        {%- endif %}
                        return false;
                    }
                }
                true
            }

            Some(Err(e)) => {
                {%- if tracing == "yes" %}
                tracing::error!("Error receiving message: {e}");
                {%- else %}
                leptos::logging::error!("Error receiving message: {e}");
                {%- endif %}
                false
            }

            None => {
                {%- if tracing == "yes" %}
                tracing::info!("Input stream closed");
                {%- else %}
                leptos::logging::info!("Input stream closed");
                {%- endif %}
                false
            }
        }
    }
}
