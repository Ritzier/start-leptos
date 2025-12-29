# Leptos Axum Starter Template

Leptos template with Axum SSR, optional testing, and automated workflows

## Quick Start

Prerequisites

```bash
# Install Rust WASM target
rustup target add wasm32-unknown-unknown

# Install tools
cargo install cargo-leptos cargo-generate cargo-make
```

Create Project

```bash
# Method 1: cargo-generate (interactive)
cargo generate --git https://github.com/ritzier/start-leptos-workspace

# Method 2: cargo-leptos
cargo leptos new --git https://github.com/ritzier/start-leptos-workspace my-app
```

### Interacitve Prompts

````
? What is the project name? my-leptos-app
? Lazy loading(--split)? yes
? Style? default
? Makefile? yes
? Makefile: (Choose with space, confirm with Enter)
[x] Cucumber
[ ] Playwright```

### Commands

```bash
# Development (hot reload)
cargo leptos watch

# Build production
cargo leptos build --release

# Run tests
cargo make chrome        # Cucumber Chrome
cargo make firefox       # Cucumber Firefox
cargo make playwright    # Playwright E2E

# Full test suite
cargo make both
````

## Structure

```text
my-leptos-app/
├── Cargo.toml          # Workspace config
├── Makefile.toml       # Task runner (optional)
├── uno.config.ts       # UnoCSS config (if selected)
├── package.json        # UnoCSS deps (if selected)
├── app/                # Shared app logic
├── frontend/           # WASM library
├── server/             # Axum SSR server
├── style/              # SCSS styles
├── public/
│ └── uno.css           # Generated UnoCSS (if selected)
├── makefile/           # Task definitions (optional)
│ ├── leptos.toml
│ ├── cucumber.toml     # If Cucumber selected
│ └── playwright.toml   # If Playwright selected
└── tests/
    ├── cucumber_test/  # BDD tests (if selected)
    └── playwright/     # E2E tests (if selected)
```

## Styling Options

- **Default**: Uses Leptos built-in CSS bundling (`/pkg/{{project-name}}.css`)

- **UnoCSS**: Atomic CSS engine with:
  - Auto pattern scanning from `src/**/*.rs` (Project) or `app/**/*.rs` (Workspace)
  - Output to `public/uno.css`
  - `npm run watch` for development HMR
  - `npm run build` for production minification

## Testing Frameworks

### Cucumber (BDD)

- WebDriver-based browser automation with `Fantoccini`
- Supports Chrome (`chromedriver`) and Firefox (`geckodriver`)
- Feature files in `tests/cucumber_test/features/`
- Run: `cargo make chrome` or `cargo make both`

### Playwright

- Modern E2E testing with Node.js runtime
- Cross-browser support (Chromium/Firefox/WebKit)
- TypeScript test files in `tests/playwright/`
- Run: `cargo make playwright`

## Template Features

- **Workspace architecture**: Modular `app/frontend/server` separation
- **Lazy loading**: Optional code-splitting with `--split` flag
- **Conditional test setup**: Only includes selected test frameworks
- **Auto-cleanup**: Template removes unused files after generation
- **Hot reload**: Leptos watch mode with live CSS injection
