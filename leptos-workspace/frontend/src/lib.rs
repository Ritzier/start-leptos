use app::*;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    {% if lazy == "yes" %}leptos::mount::hydrate_lazy(App);{%else%}leptos::mount::hydrate_body(App);{% endif %}
}
