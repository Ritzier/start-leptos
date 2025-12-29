let ask_makefile = variable::get("ask_makefile");
let makefile = variable::get("makefile");

print(`${makefile}`);

switch ask_makefile {
    true => {
        switch true {
            true if ("Cucumber" in makefile) && ("Playwright" in makefile) => {}

            false if "Cucumber" in makefile => {
                file::delete("makefile/cucumber.toml");
                file::delete("tests/cucumber_test");
            }

            false if "Playwright" in makefile => {
                file::delete("makefile/playwright.toml");
                file::delete("tests/playwright");
            }

            _ => {}
        }
    }

    false => {
        file::delete("makefile");
        file::delete("tests");
    }
}

if ask_makefile {
    if makefile.len() == 0 {
        file::delete("makefile/cucumber.toml");
        file::delete("makefile/playwright.toml");
        file::delete("tests");

        let content = "extend = [{ path = \"makefile/leptos.toml\" }]\n\n[config]\ndefault_to_workspace = false\n\n[env]\nLEPTOS_SITE_ADDR = \"127.0.0.1:3000\"\nPID_FILE = \"target/.leptos.pid\"";
        file::write("Makefile.toml", content);
    } else {
        // Content start
        let content = "extend = [\n  { path = \"makefile/leptos.toml\" },";

        if "Cucumber" in makefile {
            content += "\n  { path = \"makefile/cucumber.toml\" },";
        } else {
            file::delete("makefile/cucumber.toml");
            file::delete("tests/cucumber_test");
        }

        if "Playwright" in makefile {
            content += "\n  { path = \"makefile/playwright.toml\" },"
        } else {
            file::delete("makefile/playwright.toml");
            file::delete("tests/playwright");
        }

        // Content end
        content += "\n]\n\n[config]\ndefault_to_workspace = false\n\n[env]\nLEPTOS_SITE_ADDR = \"127.0.0.1:3000\"\nPID_FILE = \"target/.leptos.pid\"";
        file::write("Makefile.toml", content);
    }
} else {
    file::delete("makefile");
    file::delete("tests");
}

// Cleanup `hooks` folder
file::delete("hooks");
