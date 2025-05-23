let is_tracing = variable::get("tracing");

if !is_tracing {
    file::delete("server/src/trace.rs");
    file::delete("server/src/main-tracing.rs");
    file::rename("server/src/main-normal.rs", "server/src/main.rs");

    file::delete("server/Cargo-tracing.toml");
    file::rename("server/Cargo-normal.toml", "server/Cargo.toml");
} else {
    file::delete("server/src/main-normal.rs");
    file::rename("server/src/main-tracing.rs", "server/src/main.rs");

    file::delete("server/Cargo-normal.toml");
    file::rename("server/Cargo-tracing.toml", "server/Cargo.toml");
}

let is_e2e = variable::get("end2end");

if !is_e2e {
    file::delete("end-to-end");
}
