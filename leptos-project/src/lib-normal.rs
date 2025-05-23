pub mod app;

#[cfg(feature = "ssr")]
pub mod ssr;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    leptos::mount::hydrate_body(App)
}
