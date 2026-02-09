{% if websocket == true -%}
mod page;
mod ws;

pub use page::HomePage;
{% else -%}
mod page;
pub use page::HomePage;
{% endif -%}
