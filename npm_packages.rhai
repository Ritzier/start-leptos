let stylelint = variable::get("stylelint");
let tailwind = variable::get("tailwind");

if stylelint == "yes" {
    file::write("stylelint.config.mjs", "/** @type {import('stylelint').Config} */export default {\n    extends: [\"stylelint-config-standard-scss\"],\n};");
}

if stylelint == "yes" && tailwind == "yes" {
    file::write("package.json", "{\n    \"devDependencies\": {\n        \"stylelint\": \"^16\",\n        \"stylelint-config-standard-scss\": \"^15\",\n        \"@tailwindcss/cli\": \"^4\",\n        \"tailwindcss\": \"^4\"\n    }\n}");
    return;
}

if stylelint == "yes" {
    file::write("package.json", "{\n    \"devDependencies\": {\n        \"stylelint\": \"^16\",\n        \"stylelint-config-standard-scss\": \"^15\"\n    }\n}");
    return;
}

if tailwind == "yes" {
    file::write("package.json", "{\n    \"devDependencies\": {\n        \"@tailwindcss/cli\": \"^4\",\n        \"tailwindcss\": \"^4\"\n    }\n}");
    return;
}
