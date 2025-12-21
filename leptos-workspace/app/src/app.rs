use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::components::{Route, Router, Routes};
{% if lazy == "no" %}use leptos_router::path;{% else %}use leptos_router::{Lazy, path};{% endif %}

use crate::pages::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes fallback=|| "Page not found".into_view()>
                {% if lazy == "no" %}<Route path=path!("") view=HomePage />{% else %}<Route path=path!("") view={Lazy::<HomePage>::new()} />{% endif %}
            </Routes>
        </Router>
    }
}
