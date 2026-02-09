// ====== Websocket =====
let websocket = variable::get("websocket");
if websocket == false {
    file::delete("app/src/ws_core");
}

// ===== Tracing =====
let tracing = variable::get("tracing");
if tracing == false {
    file::delete("server/src/trace.rs");
}

// ===== Style =====
let style = variable::get("style");

// Move files out
if style == "unocss" {
    file::rename("styles/unocss/package.json", "package.json");
    file::rename("styles/unocss/uno.config.ts", "uno.config.ts");
}
// Add to global variable
// variable::set("style", style);
// Cleanup
file::delete("styles");

// ===== Docker =====
let docker = variable::get("docker");
// Move these file out
if docker == true {
    file::rename("Docker/Dockerfile", "Dockerfile");
    file::rename("Docker/docker-compose.yml", "docker-compose.yml");
}
// Cleanup
file::delete("Docker");

// ====== Cucumber =====
let cucumber = variable::get("cucumber");

if (cucumber == false) {
    file::delete("e2e-tests");
}

// ====== Benchmark =====
let benchmark = variable::get("benchmark");
if (benchmark == false) {
    file::delete("benchmark");
}
