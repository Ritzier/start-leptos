<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos Axum Starter Template

This is a template for use with the [Leptos](https://github.com/leptos-rs/leptos) web framework and the [cargo-leptos](https://github.com/akesson/cargo-leptos) tool using [Axum](https://github.com/tokio-rs/axum).

## Creating Leptos Workspace

Make sure you have `cargo-leptos` and `wasm32-unknown-unknown` installed before creating your project. You
can install it using:

```bash
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos
```

## Creating the project:

```bash
cargo leptos new --git https://github.com/ritzier/leptos-workspace/
```

This will create a new project directory with the name you specify `{projectname}`

## Runing the project:

Navigate to your project directory and start the development server using:

```bash
cd {projectname}
cargo leptos serve
```

go to the project and start leptos

### Run with Tailwind:

#### Installation

Install package in `package.json`:

```sh
npm i
```

and start the server:

```sh
cargo leptos serve
```

## TODO

- update project `leptos` crate to version 0.7
- add `thaw`
- add `tailwindcss`
