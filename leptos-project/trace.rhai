let is_tracing = variable::get("tracing");

if !is_tracing {
    file::delete("src/trace.rs");
    file::delete("Cargo-tracing.toml");
    file::rename("Cargo-normal.toml", "Cargo.toml");

    file::delete("src/lib-tracing.rs");
    file::delete("src/main-tracing.rs");
    file::rename("src/lib-normal.rs", "src/lib.rs");
    file::rename("src/main-normal.rs", "src/main.rs");
} else {
    file::delete("Cargo-normal.toml");
    file::rename("Cargo-tracing.toml", "Cargo.toml");

    file::delete("src/lib-normal.rs");
    file::delete("src/main-normal.rs");
    file::rename("src/lib-tracing.rs", "src/lib.rs");
    file::rename("src/main-tracing.rs", "src/main.rs")
}
