set shell := ["bash", "-cu"]

{% if style != "default" -%}
# Install NPM package
install:
    bun install

# TailWind CSS
build-tailwind:
    npx @tailwindcss/cli build -i style/tailwind.scss -o target/tailwind.css

build-tailwind-minify:
    npx @tailwindcss/cli build -i style/tailwind.scss -o target/tailwind.css --minify

# SCSS
build-scss:
    bunx sass style/main.scss target/scss.css

build-scss-minify:
    bunx sass style/main.scss target/scss.css --style=compressed

# CSS merge
build-merge:
    cat target/tailwind.css target/scss.css > public/style.css

{% else -%}
# SCSS
build-scss:
    bunx sass --no-source-map style/main.scss public/style.css

build-scss-minify:
    bunx sass --no-source-map style/main.scss public/style.css --style=compressed

{% endif -%}
# JavaScript
build-js:
    #!/usr/bin/env bash
    rm -rf public/js
    shopt -s nullglob
    files=(js/*.js)
    if (( ${#files[@]} > 0 )); then
        esbuild "${files[@]}" --outdir=public/js
    fi

build-js-minify:
    #!/usr/bin/env bash
    rm -rf public/js
    shopt -s nullglob
    files=(js/*.js)
    if (( ${#files[@]} > 0 )); then
        esbuild "${files[@]}" --outdir=public/js --minify
    fi

# Build
{% if style == "default" -%}
build: build-scss build-js

build-minify: build-scss-minify build-js-minify

{% else -%}
build: install build-tailwind build-scss build-merge build-js

build-minify: install build-tailwind-minify build-scss-minify build-merge build-js-minify

{% endif -%}
# Leptos
dev: build
    cargo leptos serve --split

minify: build-minify
    cargo leptos serve --split
