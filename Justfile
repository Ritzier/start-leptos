set shell := ["bash", "-cu"]

# =============================================================================
# Style
# =============================================================================
{% if style != "default" -%}
# Insall JS dependencies via Bun
install:
    bun install

# TailWind CSS: compile `style/tailwind.scss` into `target/tailwind.css`
build-tailwind:
    bunx @tailwindcss/cli build -i style/tailwind.scss -o target/tailwind.css

# Same as `build-tailwind`, but minified for producation
build-tailwind-minify:
    bunx @tailwindcss/cli build -i style/tailwind.scss -o target/tailwind.css --minify

# Compile SCSS into CSS
build-scss:
    bunx sass style/main.scss target/scss.css

# Compile and compress SCSS for production
build-scss-minify:
    bunx sass style/main.scss target/scss.css --style=compressed

# Merge `TailWind CSS` and `SCSS` output into a single stylesheet
build-merge:
    cat target/tailwind.css target/scss.css > public/style.css

{% else -%}
# Compile `SCSS` directly into the public stylesheet
build-scss:
    bunx sass --no-source-map style/main.scss public/style.css

# Compile and minify SCSS for production
build-scss-minify:
    bunx sass --no-source-map style/main.scss public/style.css --style=compressed

{% endif -%}
# =============================================================================
# JAVASCRIPT BUILD CONFIGURATION
# =============================================================================
build-js:
    #!/usr/bin/env bash

    # Remove previously generated `JavaScript` files
    rm -rf public/js

    # Enable nullglob so unmatched patterns expand to an empty array
    shopt -s nullglob

    # Collect all `JavaScript` files from the source directory
    files=(js/*.js)

    # Only run esbuild when `JavaScript` files are available
    if (( ${#files[@]} > 0 )); then
        esbuild "${files[@]}" --outdir=public/js
    fi

# Compile and minify all `JavaScript` files for production
build-js-minify:
    #!/usr/bin/env bash
    # Remove previously generated `JavaScript` files
    rm -rf public/js

    # Enable nullglob so unmatched patterns expand to an empty array
    shopt -s nullglob

    # Collect all `JavaScript` files from the source directory
    files=(js/*.js)

    # Only run esbuild when `JavaScript` files are available
    if (( ${#files[@]} > 0 )); then
        esbuild "${files[@]}" --outdir=public/js --minify
    fi

# =============================================================================
# BUILD TASKS
# =============================================================================
{% if style == "default" -%}
# Development build: compile `CSS` and `JavaScript` without minification
build: build-scss build-js

# Production build: compile and minify `CSS` and `JavaScript`
build-minify: build-scss-minify build-js-minify

{% else -%}
# Development build:
# 1. Install dependencies
# 2. Compile Tailwind CSS
# 3. Compile SCSS
# 4. Merge both CSS outputs
# 5. Compile JavaScript
build: install build-tailwind build-scss build-merge build-js

# Production build:
# Perform the same steps with `CSS` and `JavaScript` minification enabled
build-minify: install build-tailwind-minify build-scss-minify build-merge build-js-minify

{% endif -%}
# =============================================================================
# LEPTOS DEVELOPMENT AND RELEASE COMMANDS
# =============================================================================

# Start the `Leptos` development server after building assets
dev: build
    cargo leptos serve --split

# Start the `Leptos` development server using minified assets
dev-minify: build-minify
    cargo leptos serve --split

# Run the `Leptos` application in release mode
# -r: Enables release mode
# -P: Precompress static assets
release: build-minify
    cargo leptos serve --split -rP

# Build the `Leptos` application for production deployment
release-build: build-minify
    cargo leptos build --split -rP
