// Ask for the project name
let project_name = variable::prompt("What is the project name?");
variable::set("project-name", project_name);

// Project type
let template = variable::get("template");
while switch template {
    "Project" => {
        // Delete other template
        file::delete("leptos-workspace");

        // Delete `tests/cucumber_test_workspace`
        file::delete("tests/cucumber_test_workspace");
                file::delete("makefile/cucumber-workspace.toml");

        // Move these file out
        file::rename("leptos-project/src", "src");
        file::rename("leptos-project/style", "style");
        file::rename("leptos-project/.gitignore", ".gitignore");

        // Cleanup
        file::delete("leptos-project");

        false
    }
    "Workspace" => {
        // Delete other template
        file::delete("leptos-project");

        // Delete `tests/cucumber_test`
        file::delete("tests/cucumber_test");
        file::rename("tests/cucumber_test_workspace", "tests/cucumber_test");

        file::delete("makefile/cucumber.toml");
        file::rename("makefile/cucumber-workspace.toml", "makefile/cucumber.toml");

        // Move these file out
        file::rename("leptos-workspace/app", "app");
        file::rename("leptos-workspace/frontend", "frontend");
        file::rename("leptos-workspace/server", "server");
        file::rename("leptos-workspace/style", "style");
        file::rename("leptos-workspace/.gitignore", ".gitignore");

        // Cleanup
        file::delete("leptos-workspace");

        false
    }
    _ => true,
} {
    template = variable::prompt("Which template?", "none", ["Project", "Workspace"]);
}

// ===== Lazy Loading(--split) =====
let lazy = variable::prompt("Lazy Loading(--split)?", "no", ["yes", "no"]); 
variable::set("lazy", lazy);

// ===== Style =====
let style = variable::prompt("Style?", "default", ["default", "unocss"]);
switch style {
    "unocss" => {
        file::rename("styles/unocss/package.json", "package.json");

        // `uno.config.ts`
        let content = "import { defineConfig, presetMini } from \"unocss\";\n\nexport default defineConfig({\n  cli: {\n    entry: {\n      patterns: [\"";
        if template == "Workspace" {
            content += "app/**/*.rs";
        } else {
            content += "src/**/*.rs";
        }
        content += "\"],\n      outFile: \"public/uno.css\",\n    },\n  },\n  presets: [presetMini()],\n});";
        file::write("uno.config.ts", content);
    }

    _ => {}
}

/// Add to global variable
variable::set("style", style);
// Cleanup
file::delete("styles");

// ===== Makefile =====
// Makefile (optional)
let makefile = variable::prompt("Makefile (leptos startup, tests)?", "no", ["yes", "no"]); 
let cucumber = if makefile == "yes" { 
    variable::prompt("Cucumber test?", "no", ["yes", "no"]) 
} else { "no" };
let playwright = if makefile == "yes" { 
    variable::prompt("Playwright test?", "no", ["yes", "no"]) 
} else { "no" };


// Generate Cargo.toml ALWAYS (moved outside makefile block)
switch template {
    "Project" => {
        let content = "[package]\nname = \"{{project-name}}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[lib]\ncrate-type = [\"cdylib\", \"rlib\"]\n\n[dependencies]\nleptos = { version = \"0.8\" }\nleptos_meta = { version = \"0.8\" }\nleptos_router = { version = \"0.8\" }\nleptos_axum = { version = \"0.8\", optional = true }\n\naxum = { version = \"0.8\", optional = true }\ntokio = { version = \"1\", features = [\"rt-multi-thread\"], optional = true }\nwasm-bindgen = \"0.2\"\n\n# Logging\nconsole_error_panic_hook = \"0.1\"\n\n[dev-dependencies]\nanyhow = \"1.0.98\"";

        if makefile == "yes" {
            if cucumber == "yes" {
                content += "\ncucumber = \"0.21.1\"\nfantoccini = \"0.22.0\"\nserde_json = \"1.0.141\"\ntokio = { version = \"1.47.0\", features = [\"macros\", \"rt-multi-thread\"] }\n\n[[test]]\nname = \"cucumber_test\"\nharness = false";
            } else {
                content += "\ntokio = { version = \"1.47.0\", features = [\"macros\", \"rt-multi-thread\"] }";
            }
        }

        content += "\n\n[features]\nhydrate = [\"leptos/hydrate\"]\nssr = [\n  \"dep:axum\",\n  \"dep:leptos_axum\",\n  \"dep:tokio\",\n  \"leptos/ssr\",\n  \"leptos_meta/ssr\",\n  \"leptos_router/ssr\",\n]\n\n[profile.wasm-release]\ninherits = \"release\"\nopt-level = 'z'\nlto = true\ncodegen-units = 1\npanic = \"abort\"\n\n[package.metadata.leptos]\noutput-name = \"{{project-name}}\"\n\nstyle-file = \"style/main.scss\"\n# tailwind-input-file = \"style/tailwind.css\"\n\nsite-root = \"target/site\"\nsite-pkg-dir = \"pkg\"\nassets-dir = \"public\"\n\nsite-addr = \"127.0.0.1:3000\"\nreload-port = 3001\n\nend2end-cmd = \"npx playwright test\"\nend2end-dir = \"end2end\"\n\nbrowserquery = \"defaults\"\nenv = \"DEV\"\nwatch = false\n\nbin-features = [\"ssr\"]\nbin-default-features = false\nlib-features = [\"hydrate\"]\nlib-default-features = false\nlib-profile-release = \"wasm-release\"";
        file::write("Cargo.toml", content);
    }

    "Workspace" => {
        let content = "[workspace]\nresolver = \"2\"\nmembers = [\"app\", \"frontend\", \"server\"";
        if makefile == "yes" {
            if cucumber == "yes" {
                content += ", \"tests/cucumber_test\"";
            }
        }
        content += "]\n\n[profile.wasm-release]\ninherits = \"release\"\ncodegen-units = 1\nlto = true\nopt-level = 'z'\npanic = \"abort\"\n\n[workspace.dependencies]\nleptos = { version = \"0.8\" }\nleptos_meta = { version = \"0.8\" }\nleptos_router = { version = \"0.8\" }\nleptos_axum = { version = \"0.8\" }\n\naxum = \"0.8\"\ntokio = { version = \"1\", features = [\"rt-multi-thread\"] }\nwasm-bindgen = \"0.2\"\n\n# Logging\nconsole_error_panic_hook = \"0.1\"\n\n[[workspace.metadata.leptos]]\nname = \"{{project-name}}\"\n\nbin-package = \"server\"\nlib-package = \"frontend\"\n\nstyle-file = \"style/main.scss\"\n# tailwind-input-file = \"style/tailwind.css\"\n\nsite-root = \"target/site\"\nsite-pkg-dir = \"pkg\"\nassets-dir = \"public\"\n\nsite-addr = \"127.0.0.1:3000\"\nreload-port = 3001\n\nend2end-cmd = \"npx playwright test\"\nend2end-dir = \"end2end\"\n\nbrowserquery = \"defaults\"\nenv = \"DEV\"\nwatch = false\n\nbin-features = [\"ssr\"]\nbin-default-features = false\nlib-features = [\"hydrate\"]\nlib-default-features = false\nlib-profile-release = \"wasm-release\"";
        file::write("Cargo.toml", content);
    }
    _ => {}
}

// Makefile details (tests)
if makefile == "yes" {

    let content = "extend = [\n  { path = \"makefile/leptos.toml\" }";
    if cucumber == "yes" { content += ",\n  { path = \"makefile/cucumber.toml\" }" }
    if playwright == "yes" { content += ",\n  { path = \"makefile/playwright.toml\" }" }

    content += "\n]\n[config]\ndefault_to_workspace = false\n\n[env]\nLEPTOS_SITE_ADDR = \"127.0.0.1:3000\"\nPID_FILE = \"target/.leptos.pid\"";
    file::write("Makefile.toml", content);

    if cucumber != "yes" {
        file::delete("makefile/cucumber.toml");
        file::delete("tests/cucumber_test");
    }
    if playwright != "yes" {
        file::delete("makefile/playwright.toml");
        file::delete("tests/playwright");
    }
    if playwright != "yes" && cucumber != "yes" {
        file::delete("tests");
    }

} else {
  file::delete("makefile");
  file::delete("tests");
}



// clean up
file::delete("hooks");
