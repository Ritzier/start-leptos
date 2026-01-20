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
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
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

# Install Dart (changes rarely)
ARG DART_VERSION=3.10.7
RUN curl -o dart-sdk.zip https://storage.googleapis.com/dart-archive/channels/stable/release/${DART_VERSION}/sdk/dartsdk-linux-x64-release.zip && \
    unzip dart-sdk.zip -d /usr/lib && \
    rm dart-sdk.zip

# Setup Dart environment
ENV PATH="/usr/lib/dart-sdk/bin:/root/.pub-cache/bin:${PATH}"

# Verify installations
RUN dart --version && cargo-leptos --version

{% if style == "unocss" %}
# Copy package.json and install Node dependencies (for better caching)
COPY package.json ./
RUN npm install

# Copy UnoCSS config
COPY uno.config.ts ./

{% endif -%}
# Copy only dependency manifests first (for better caching)
COPY Cargo.toml ./
COPY app/Cargo.toml ./app/
COPY frontend/Cargo.toml ./frontend/
COPY server/Cargo.toml ./server/

# Create dummy source files to cache dependencies
RUN mkdir -p app/src frontend/src server/src && \
    echo "fn main() {}" > server/src/main.rs && \
    echo "pub fn main() {}" > app/src/lib.rs && \
    echo "pub fn main() {}" > frontend/src/lib.rs

# Pre-build dependencies (this layer will be cached)
RUN cargo build --release || true

# Remove dummy files
RUN rm -rf app/src frontend/src server/src

# Copy actual source code (changes frequently)
COPY app/src ./app/src
COPY frontend/src ./frontend/src
COPY server/src ./server/src

# Copy static resources
COPY style ./style
COPY public ./public

{% if style == "unocss" %}
# Build UnoCSS before Leptos build (generates public/uno.css)
RUN npm run build

{% endif -%}
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
