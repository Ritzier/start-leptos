use leptos::prelude::*;
use leptos_meta::{HashedStylesheet, Link, MetaTags, Stylesheet};

use crate::app::App;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HashedStylesheet options=options.clone() />
                <HydrationScripts options />
                <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico" />
                <Stylesheet id="leptos" href="/pkg/application.css" />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}
