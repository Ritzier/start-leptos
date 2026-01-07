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
