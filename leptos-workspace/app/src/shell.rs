use leptos::prelude::*;
use leptos_meta::{Link, Meta, MetaTags, Stylesheet};

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
                <Meta name="color-scheme" content="dark" />
                <Stylesheet href="/style.css" />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}
