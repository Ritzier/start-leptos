let is_tracing = variable::get("tracing");

if !is_tracing {
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
} else {
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

}

let is_e2e = variable::get("end2end");

if !is_e2e {
    file::delete("end-to-end");
}
