// Ask for the project name
let project_name = variable::prompt("What is the project name?");
variable::set("project-name", project_name);

// Initialize
file::rename("leptos-workspace/app", "app");
file::rename("leptos-workspace/frontend", "frontend");
file::rename("leptos-workspace/server", "server");
file::rename("leptos-workspace/style", "style");
file::rename("leptos-workspace/.gitignore", ".gitignore");
file::rename("leptos-workspace/Cargo.toml", "Cargo.toml");
// Cleanup
file::delete("leptos-workspace");

// ===== Websocket =====
let websocket = variable::prompt("Websocket?", "no", ["yes", "no"]);
variable::set("websocket", websocket);
// Cleanup
if websocket == "no" {
    file::delete("app/src/structs");
}

// ===== Tracing =====
let tracing = variable::get("Tracing?", "no", ["yes", "no"]);
variable::set("tracing", tracing);

// ===== Style =====
let style = variable::prompt("Style?", "default", ["default", "unocss"]);
switch style {
    "unocss" => {
        file::rename("styles/unocss/package.json", "package.json");
        file::rename("styles/unocss/uno.config.ts", "uno.config.ts");
    }

    _ => {}
}

// Add to global variable
variable::set("style", style);
// Cleanup
file::delete("styles");
