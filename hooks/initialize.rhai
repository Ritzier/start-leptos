// Initialize
file::rename("leptos-workspace/app", "app");
file::rename("leptos-workspace/frontend", "frontend");
file::rename("leptos-workspace/server", "server");
file::rename("leptos-workspace/style", "style");
file::rename("leptos-workspace/.gitignore", ".gitignore");
file::rename("leptos-workspace/Cargo.toml", "Cargo.toml");
file::rename("leptos-workspace/websocket_trait", "websocket_trait");
// Cleanup
file::delete("leptos-workspace");
