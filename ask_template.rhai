let template = variable::get("template");

while switch template {
    "CSR" => {
        system::command("mv", ["leptos-csr/*", "./"]);
        false
    }
    "Project" => {
        system::command("mv", ["leptos-project/*", "./"]);
        false
    }
    "Workspace" => {
        system::command("mv", ["leptos-workspace/*", "./"]);
        false
    }
    _ => true,
} {
    template = variable::prompt("Which template?", "none", ["Project", "Workspace", "CSR"]);
}

file::delete("leptos-csr");
file::delete("leptos-project");
file::delete("leptos-workspace");
