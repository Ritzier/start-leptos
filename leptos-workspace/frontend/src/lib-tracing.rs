use app::*;
use tracing_subscriber::fmt;
use tracing_subscriber_wasm::MakeConsoleWriter;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    fmt()
        .with_writer(MakeConsoleWriter::default().map_trace_level_to(tracing::Level::DEBUG))
        .without_time()
        .init();
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
