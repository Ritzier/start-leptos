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
    "Workspace" => {
        file::delete("makefile/leptos.toml");
        file::rename("makefile/leptos-workspace.toml", "makefile/leptos.toml");
        file::delete("makefile/cucumber.toml");
        file::rename("makefile/cucumber-workspace.toml", "makefile/cucumber.toml");

        file::delete("tests/cucumber_test");
        file::rename("tests/cucumber_test_workspace", "tests/cucumber_test");

        if playwright == "yes" && cucumber == "yes" {
            file::write("Makefile.toml", "extend = [\n  { path = \"makefile/leptos.toml\" },\n  { path = \"makefile/cucumber.toml\" },\n  { path = \"makefile/playwright.toml\" },\n]\n[config]\ndefault_to_workspace = false\n\n[env]\nLEPTOS_SITE_ADDR = \"127.0.0.1:3000\"");
            return;
        }

        if cucumber == "yes" {
            file::write("Makefile.toml", "extend = [\n  { path = \"makefile/leptos.toml\" },\n  { path = \"makefile/cucumber.toml\" }\n]\n[config]\ndefault_to_workspace = false\n\n[env]\nLEPTOS_SITE_ADDR = \"127.0.0.1:3000\"");
            file::delete("makefile/playwright.toml");
            file::delete("tests/playwright.toml");
            return;
        }

        if playwright == "yes" {
            file::write("Makefile.toml", "extend = [\n  { path = \"makefile/leptos.toml\" },\n  { path = \"makefile/playwright.toml\" }\n]\n[config]\ndefault_to_workspace = false\n\n[env]\nLEPTOS_SITE_ADDR = \"127.0.0.1:3000\"");
            file::delete("makefile/cucumber.toml");
            file::delete("tests/cucumber_test");
            return;
        }
    }

    _ => {
        file::delete("makefile/leptos-workspace.toml");
        file::delete("makefile/cucumber-workspace.toml");

        file::delete("tests/cucumber_test_workspace");

        if playwright == "yes" && cucumber == "yes" {
            file::write("Makefile.toml", "extend = [\n  { path = \"makefile/leptos.toml\" },\n  { path = \"makefile/cucumber.toml\" },\n  { path = \"makefile/playwright.toml\" },\n]\n\n[env]\nLEPTOS_SITE_ADDR = \"127.0.0.1:3000\"");
            return;
        }

        if cucumber == "yes" {
            file::write("Makefile.toml", "extend = [\n  { path = \"makefile/leptos.toml\" },\n  { path = \"makefile/cucumber.toml\" }\n]\n\n[env]\nLEPTOS_SITE_ADDR = \"127.0.0.1:3000\"");
            file::delete("makefile/playwright.toml");
            file::delete("tests/playwright.toml");
            return;
        }

        if playwright == "yes" {
            file::write("Makefile.toml", "extend = [\n  { path = \"makefile/leptos.toml\" },\n  { path = \"makefile/playwright.toml\" }\n]\n\n[env]\nLEPTOS_SITE_ADDR = \"127.0.0.1:3000\"");
            file::delete("makefile/cucumber.toml");
            file::delete("tests/cucumber_test");
            return;
        }
    }
}

