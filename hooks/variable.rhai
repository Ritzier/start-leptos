// ====== Websocket =====
let websocket = variable::get("websocket");
if websocket == false {
    file::delete("websocket_trait");
    file::delete("app/src/pages/home/ws");
}

// ===== Style =====
let style = variable::get("style");

if style == "tailwind" {
    file::rename("styles/tailwind/package.json", "package.json");
    file::rename("styles/tailwind/style", "style");
}

if style == "default" {
    file::rename("styles/default/style", "style");
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
if benchmark == "" {
    variable::set("benchmark", false);
    benchmark = false;
}
if (benchmark == false) {
    file::delete("benchmark");
}
