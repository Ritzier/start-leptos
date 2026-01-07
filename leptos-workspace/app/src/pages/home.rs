{%- if websocket == "yes" %}use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::{LazyRoute, lazy_route};
use uuid::Uuid;

use crate::structs::WebSocketManager;

pub struct HomePage {
    websocket_manager: WebSocketManager,
}

#[lazy_route]
impl LazyRoute for HomePage {
    fn data() -> Self {
        let uuid = Uuid::new_v4();
        let websocket_manager = WebSocketManager::new(uuid);

        Self { websocket_manager }
    }

    fn view(this: Self) -> AnyView {
        view! {
            {move || match this.websocket_manager.is_connected.get() {
                false => {
                    Either::Left(
                        view! {
                            <DisconnectedComponent websocket_manager=this
                                .websocket_manager
                                .clone() />
                        }
                            .into_any(),
                    )
                }
                true => {
                    Either::Right(
                        view! {
                            <ConnectedComponent websocket_manager=this.websocket_manager.clone() />
                        }
                            .into_any(),
                    )
                }
            }}
        }
        .into_any()
    }
}

#[component]
fn DisconnectedComponent(websocket_manager: WebSocketManager) -> impl IntoView {
    view! {
        <button on:click=move |_| {
            websocket_manager.connect();
        }>"Connect"</button>
    }
}

#[component]
fn ConnectedComponent(websocket_manager: WebSocketManager) -> impl IntoView {
    view! {
        <button on:click=move |_| {
            websocket_manager.disconnect();
        }>"Diconnect"</button>
    }
}
{%- else -%}
use leptos::prelude::*;
use leptos_router::{LazyRoute, lazy_route};

pub struct HomePage;

#[lazy_route]
impl LazyRoute for HomePage {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        let (count, set_count) = signal(0);
        let on_click = move |_| set_count.update(|count| *count += 1);

        view! {
            <h1>"Welcome to Leptos!"</h1>
            <button on:click=on_click>"Click Me: "{count}</button>
        }
        .into_any()
    }
}
{%- endif %}
