let ask_for_makefile = variable::prompt("Makefile?", "no", ["yes", "no"]);

if ask_for_makefile == "no" {
    file::delete("makefile");
    file::delete("Makefile.toml");
    file::delete("tests");
    return;
}

let cucumber = variable::prompt("Cucumber test?", "no", ["yes", "no"]);
variable::set("cucumber", cucumber);
let playwright = variable::prompt("Playwright test?", "no", ["yes", "no"]);

let template = variable::get("template");

switch template {
    // Toml include `[config]\ndefault_to_workspace=false`
    "Workspace" => {
        file::delete("tests/cucumber_test");
        file::rename("tests/cucumber_test_workspace", "tests/cucumber_test");
        file::delete("makefile/cucumber.toml");
        file::rename("makefile/cucumber-workspace.toml", "makefile/cucumber.toml");

        if playwright == "yes" && cucumber == "yes" {
            file::write("Makefile.toml", "extend = [\n  { path = \"makefile/leptos.toml\" },\n  { path = \"makefile/cucumber.toml\" },\n  { path = \"makefile/playwright.toml\" },\n]\n[config]\ndefault_to_workspace = false\n\n[env]\nLEPTOS_SITE_ADDR = \"127.0.0.1:3000\"\nPID_FILE = \"target/.leptos.pid\"");
            return;
        }

        if cucumber == "yes" {
            file::write("Makefile.toml", "extend = [\n  { path = \"makefile/leptos.toml\" },\n  { path = \"makefile/cucumber.toml\" }\n]\n[config]\ndefault_to_workspace = false\n\n[env]\nLEPTOS_SITE_ADDR = \"127.0.0.1:3000\"\nPID_FILE = \"target/.leptos.pid\"");
            file::delete("makefile/playwright.toml");
            file::delete("tests/playwright.toml");
            return;
        }

        if playwright == "yes" {
            file::write("Makefile.toml", "extend = [\n  { path = \"makefile/leptos.toml\" },\n  { path = \"makefile/playwright.toml\" }\n]\n[config]\ndefault_to_workspace = false\n\n[env]\nLEPTOS_SITE_ADDR = \"127.0.0.1:3000\"\nPID_FILE = \"target/.leptos.pid\"");
            file::delete("makefile/cucumber.toml");
            file::delete("tests/cucumber_test");
            return;
        }

        file::delete("tests");
        file::delete("makefile/playwright.toml");
        file::delete("makefile/cucumber.toml");
        file::write("Makefile.toml", "extend = [{ path = \"makefile/leptos.toml\" }]\n\n[config]\ndefault_to_workspace = false\n\n[env]\nLEPTOS_SITE_ADDR = \"127.0.0.1:3000\"\nPID_FILE = \"target/.leptos.pid\"");
    }

    _ => {
        file::delete("tests/cucumber_test_workspace");
        file::delete("makefile/cucumber-workspace.toml");

        if playwright == "yes" && cucumber == "yes" {
            file::write("Makefile.toml", "extend = [\n  { path = \"makefile/leptos.toml\" },\n  { path = \"makefile/cucumber.toml\" },\n  { path = \"makefile/playwright.toml\" },\n]\n\n[env]\nLEPTOS_SITE_ADDR = \"127.0.0.1:3000\"\nPID_FILE = \"target/.leptos.pid\"");
            return;
        }

        if cucumber == "yes" {
            file::write("Makefile.toml", "extend = [\n  { path = \"makefile/leptos.toml\" },\n  { path = \"makefile/cucumber.toml\" }\n]\n\n[env]\nLEPTOS_SITE_ADDR = \"127.0.0.1:3000\"\nPID_FILE = \"target/.leptos.pid\"");
            file::delete("makefile/playwright.toml");
            file::delete("tests/playwright.toml");
            return;
        }

        if playwright == "yes" {
            file::write("Makefile.toml", "extend = [\n  { path = \"makefile/leptos.toml\" },\n  { path = \"makefile/playwright.toml\" }\n]\n\n[env]\nLEPTOS_SITE_ADDR = \"127.0.0.1:3000\"\nPID_FILE = \"target/.leptos.pid\"");
            file::delete("makefile/cucumber.toml");
            file::delete("tests/cucumber_test");
            return;
        }


        file::delete("tests");
        file::delete("makefile/playwright.toml");
        file::delete("makefile/cucumber.toml");
        file::write("Makefile.toml", "extend = [{ path = \"makefile/leptos.toml\" }]\n\n[env]\nLEPTOS_SITE_ADDR = \"127.0.0.1:3000\"\nPID_FILE = \"target/.leptos.pid\"");
    }
}

