#[wasm_bindgen::prelude::wasm_bindgen]
#[cfg(feature = "hydrate")]
pub fn hydrate() {
    use app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
