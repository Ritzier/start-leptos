use leptos::prelude::*;
use leptos_meta::{Link, MetaTags, Stylesheet};

use crate::app::App;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico" />
                {% if style == "unocss" %}<Stylesheet id="uno" href="/uno.css" />{%else%}<Stylesheet id="leptos" href="/pkg/{{project-name}}.css" />{% endif %}
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}
