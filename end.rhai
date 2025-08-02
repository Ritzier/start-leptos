let template = variable::get("template");
let tracing = variable::get("tracing");

switch template {
    "CSR" => {}

    "Project" => {

        switch tracing {
            "yes" => {
                file::delete("src/ssr/trace.rs");
                file::delete("Cargo-tracing.toml");
                file::rename("Cargo-normal.toml", "Cargo.toml");

                file::delete("src/lib-tracing.rs");
                file::delete("src/main-tracing.rs");
                file::rename("src/lib-normal.rs", "src/lib.rs");
                file::rename("src/main-normal.rs", "src/main.rs");

                file::delete("src/ssr-tracing.rs");
                file::rename("src/ssr-normal.rs", "src/ssr.rs");
            }
            _ => {
                file::delete("Cargo-normal.toml");
                file::rename("Cargo-tracing.toml", "Cargo.toml");

                file::delete("src/lib-normal.rs");
                file::delete("src/main-normal.rs");
                file::rename("src/lib-tracing.rs", "src/lib.rs");
                file::rename("src/main-tracing.rs", "src/main.rs");

                file::delete("src/ssr-normal.rs");
                file::rename("src/ssr-tracing.rs", "src/ssr.rs");
            }
        }
    }

    "Workspace" => {
        switch tracing {
            "yes" => {
                file::delete("Cargo-tracing.toml");
                file::rename("Cargo-normal.toml", "Cargo.toml");

                file::delete("server/src/trace.rs");
                file::delete("server/src/main-tracing.rs");
                file::rename("server/src/main-normal.rs", "server/src/main.rs");

                file::delete("server/Cargo-tracing.toml");
                file::rename("server/Cargo-normal.toml", "server/Cargo.toml");

                file::delete("frontend/src/lib-tracing.rs");
                file::rename("frontend/src/lib-normal.rs", "frontend/src/lib.rs");

                file::delete("frontend/Cargo-tracing.toml");
                file::rename("frontend/Cargo-normal.toml", "frontend/Cargo.toml");

                file::delete("app/Cargo-tracing.toml");
                file::rename("app/Cargo-normal.toml", "app/Cargo.toml");
            }
            _ => {
                file::delete("Cargo-normal.toml");
                file::rename("Cargo-tracing.toml", "Cargo.toml");

                file::delete("server/src/main-normal.rs");
                file::rename("server/src/main-tracing.rs", "server/src/main.rs");

                file::delete("server/Cargo-normal.toml");
                file::rename("server/Cargo-tracing.toml", "server/Cargo.toml");

                file::delete("frontend/src/lib-normal.rs");
                file::rename("frontend/src/lib-tracing.rs", "frontend/src/lib.rs");

                file::delete("frontend/Cargo-normal.toml");
                file::rename("frontend/Cargo-tracing.toml", "frontend/Cargo.toml");

                file::delete("app/Cargo-normal.toml");
                file::rename("app/Cargo-tracing.toml", "app/Cargo.toml");
            }
        } 
    } 
} 
