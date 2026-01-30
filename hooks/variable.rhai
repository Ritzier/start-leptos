// ====== Websocket =====
let websocket = variable::get("websocket");
if websocket == false {
    file::delete("app/src/structs");
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

// ====== Cucumber && Playwright =====
let cucumber = variable::get("cucumber");
let playwright = variable::get("playwright");

if (cucumber == false && playwright == false) {
    file::delete("tests");
} else {
    // Selective delete if one/both false
    if cucumber == false {
        file::delete("tests/cucumber_test");
    }
    if playwright == false {
        file::delete("tests/playwright");
    }
}
