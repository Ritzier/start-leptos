# =============================================================================
# Stage 1: base
# Setup toolchain and system dependencies
# =============================================================================
FROM lukemathwalker/cargo-chef:latest-rust-bookworm AS base

WORKDIR /app

ENV BUN_INSTALL=/usr/local/bun \
    PATH="/usr/local/bun/bin:/usr/local/cargo/bin:${PATH}" \
    CARGO_NET_GIT_FETCH_WITH_CLI=true

# -----------------------------------------------------------------------------
# System packages required for building and fetching crates.
# Ket as a single layer and cleaned up to minimize image size.
# -----------------------------------------------------------------------------
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    bash \
    unzip \
    pkg-config \
    libssl-dev \
    ca-certificates \
    git \
    && rm -rf /var/lib/apt/lists/*

# -----------------------------------------------------------------------------
# Rust `WebAssembly` target used to compile the frontend crate
# -----------------------------------------------------------------------------
RUN rustup target add wasm32-unknown-unknown

# -----------------------------------------------------------------------------
# `cargo-binstall` fetches a prebuilt `cargo-leptos` binary,
# avoiding a from-source compile (which is slow).
# Pin the version so builds stay reproducible.
# -----------------------------------------------------------------------------
RUN curl -L --proto '=https' --tlsv1.2 -sSf \
    https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh \
    | bash \
    && cargo binstall cargo-leptos@0.3.12 --no-confirm \
    && cargo leptos --version

# -----------------------------------------------------------------------------
# `just` drives the build (see Justfile); install the latest
# release binary to /usr/local/bin.
# -----------------------------------------------------------------------------
RUN curl --proto '=https' --tlsv1.2 -sSf \
    https://just.systems/install.sh \
    | bash -s -- --to /usr/local/bin \
    && just --version

# -----------------------------------------------------------------------------
# Bun is used to run Tailwind/Sass tooling and to install the
# project's JS dependencies (see package.json).
# -----------------------------------------------------------------------------
RUN curl -fsSL https://bun.com/install | bash \
    && bun --version \
    && bunx --version

# =============================================================================
# Stage 2: planner
# `cargo chef prepare` inspects the manifests only (no source) and emits
# `recipe.json` describing all dependencies. Cache layers from this stage are
# reused untial a manifest changes, so dependency builds aren't repeated on 
# source-only edits.
# =============================================================================
FROM base AS planner
COPY Cargo.toml ./
COPY app ./app
COPY frontend ./frontend
COPY server ./server
{% if websocket == true %}COPY websocket_trait ./websocket_trait
{% endif -%}
{% if cucumber == true %}COPY e2e-tests ./e2e-tests
{% endif -%}
{% if benchmark == true %}COPY benchmark ./benchmark
{% endif -%}
RUN cargo chef prepare --recipe-path recipe.json

# =============================================================================
# Stage 3: builder
# Builds the server binary and the WASM frontend, plus the  static assets (CSS)
# produced by the Justfile
# =============================================================================
FROM base AS builder

# JS deps first: this layer only invalidates when `package.json`
# or the `lockfile` changes, not on Rust source edits
COPY package.json bun.lock* ./
RUN bun install --frozen-lockfile || bun install

# Dependency recipe from the planner stage
COPY --from=planner /app/recipe.json recipe.json

# Build server (native) dependencies only. Uses `BuildKit` cache mounts so the
# cargo registry/git caches survive across builds without bloating image
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    cargo chef cook \
        --recipe-path recipe.json \
        --package=server \
        --no-default-features \
        --features=ssr \
        --release

# Build frontend (WASM) dependencies. The target and feature flags must match
# what `cargo-leptos` uses, or the cached deps won't be reused
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    cargo chef cook \
        --recipe-path recipe.json \
        --package=frontend \
        --target=wasm32-unknown-unknown \
        --no-default-features \
        --features=hydrate \
        --profile=wasm-release

# Application sources.
# Everything below this point is the only set of layers that invalidates on a
# normal code edit
COPY Cargo.toml Justfile ./
COPY app ./app
COPY frontend ./frontend
COPY server ./server
{% if websocket == true %}COPY websocket_trait ./websocket_trait
{% endif -%}
{% if cucumber == true %}COPY e2e-tests ./e2e-tests
{% endif -%}
{% if benchmark == true %}COPY benchmark ./benchmark
{% endif -%}
COPY style ./style
COPY public ./public

# Full build via the `Justfile`. Intentionally *no* target cache mount here:
# compiled artifacts must remain in the image layers so the runtime stage can
# COPY the server binary and site dir.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    just release-build

# =============================================================================
# Stage 4: runtime
# Minimal image containing only the server binary and the generated site assets
# running as a non-root user
# =============================================================================
FROM debian:bookworm-slim AS runtime
WORKDIR /app

# Runtime system deps only (TLS for outbound HTTPS).
# Create an unprivileged user to run the server.
RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --system appuser \
    && useradd --system --gid appuser --home-dir /app --no-create-home appuser

# Copy the compiled server and the site output produced by `just build-minify`
COPY --from=builder --chown=appuser:appuser /app/target/release/server ./server
COPY --from=builder --chown=appuser:appuser /app/target/site ./site

# Bind to all interfaces so the container is reachable,
# and point `Leptos` at the copied site directory.
ENV LEPTOS_SITE_ADDR=0.0.0.0:3000 \
    LEPTOS_SITE_ROOT=site

USER appuser
EXPOSE 3000
CMD ["./server"]
