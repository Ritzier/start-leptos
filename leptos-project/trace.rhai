let is_tracing = variable::get("tracing");

if !is_tracing {
    file::delete("src/ssr/trace.rs");
    file::delete("Cargo-tracing.toml");
    file::rename("Cargo-normal.toml", "Cargo.toml");

    file::delete("src/lib-tracing.rs");
    file::delete("src/main-tracing.rs");
    file::rename("src/lib-normal.rs", "src/lib.rs");
    file::rename("src/main-normal.rs", "src/main.rs");

    file::delete("src/ssr-tracing.rs");
    file::rename("src/ssr-normal.rs", "src/ssr.rs");
} else {
    file::delete("Cargo-normal.toml");
    file::rename("Cargo-tracing.toml", "Cargo.toml");

    file::delete("src/lib-normal.rs");
    file::delete("src/main-normal.rs");
    file::rename("src/lib-tracing.rs", "src/lib.rs");
    file::rename("src/main-tracing.rs", "src/main.rs");

    file::delete("src/ssr-normal.rs");
    file::rename("src/ssr-tracing.rs", "src/ssr.rs");
}

let is_e2e = variable::get("end2end");

if !is_e2e {
    file::delete("end2end");
}

let is_style = variable::get("style");

if is_style {
    file::delete("style/main.scss");
    file::rename("style/main-full.scss", "style/main.scss");
} else {
    file::delete("style/main-full.scss");
    file::delete("style/abstracts");
    file::delete("style/base");
    file::delete("style/components");
    file::delete("style/pages");
    file::delete("style/themes");
    file::delete("style/vendors");
    file::delete("style/layout");
}

let stylelint = variable::get("stylelint");

if !stylelint {
    file::delete("package.json");
    file::delete("stylelint.config.mjs");
}
