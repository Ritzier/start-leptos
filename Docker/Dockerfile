
# ===== Builder Stage =====
FROM rust:bookworm AS builder

WORKDIR /app

# Install system dependencies (changes rarely)
RUN apt-get update && apt-get install -y \
    curl \
    bash \
    unzip \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

{% if style == "unocss" %}
# Install Node.js and npm for UnoCSS (changes rarely)
RUN curl -fsSL https://deb.nodesource.com/setup_22.x | bash - && \
    apt-get install -y nodejs && \
    rm -rf /var/lib/apt/lists/*

# Verify Node.js installation
RUN node --version && npm --version

{% endif -%}
# Install toolchain (changes rarely)
RUN rustup target add wasm32-unknown-unknown

# Install cargo-binstall (changes rarely)
RUN curl -L --proto '=https' --tlsv1.2 -sSf \
    https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Install cargo-leptos (changes rarely)
RUN cargo binstall cargo-leptos --no-confirm

# Verify installations
RUN cargo-leptos --version

{% if style == "unocss" %}
# Copy package.json and install Node dependencies (for better caching)
COPY package.json ./
RUN npm install

# Copy UnoCSS config
COPY uno.config.ts ./

{% endif -%}
# Copy only dependency manifests and lockfile first (for better caching)
COPY Cargo.toml Cargo.lock ./
COPY app/Cargo.toml ./app/
COPY frontend/Cargo.toml ./frontend/
COPY server/Cargo.toml ./server/

# Create dummy source files so cargo fetch can parse manifests
RUN mkdir -p app/src frontend/src server/src && \
    echo "fn main() {}" > server/src/main.rs && \
    echo "" > app/src/lib.rs && \
    echo "" > frontend/src/lib.rs

# Pre-fetch all dependencies (no compilation, just download)
RUN cargo fetch

# Remove dummy files
RUN rm -rf app/src frontend/src server/src

# Copy actual source code (changes frequently)
COPY app/src ./app/src
COPY frontend/src ./frontend/src
COPY server/src ./server/src

# Copy static resources
COPY style ./style
COPY public ./public

# Build the actual application
RUN cargo leptos build -rP --split

# ===== Runtime Stage =====
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 appuser && \
    chown -R appuser:appuser /app

# Copy binary and resources from builder
COPY --from=builder --chown=appuser:appuser /app/target/release/server ./server
COPY --from=builder --chown=appuser:appuser /app/target/site ./site

# Set environment variables
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000" \
    LEPTOS_SITE_ROOT="site"

USER appuser
EXPOSE 3000

CMD ["./server"]

