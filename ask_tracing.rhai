let template = variable::get("template");

if template == "Workspace" || template == "Project" {
    let enable_tracing = variable::prompt("Enable `tracing` crate?", "no", ["yes", "no"]);
    variable::set("tracing", enable_tracing);
}
