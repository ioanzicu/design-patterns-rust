# --- Stage 1: Build WASM Client ---
FROM rust:latest as wasm-builder
RUN apt-get update && apt-get install -y curl binaryen && rm -rf /var/lib/apt/lists/*
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
WORKDIR /usr/src/app
COPY wasm_client ./wasm_client
WORKDIR /usr/src/app/wasm_client
RUN wasm-pack build --target web --out-dir www/pkg

# --- Stage 2: Build Rust Backend ---
FROM rust:1.94-slim-bullseye as backend-builder
RUN apt-get update && apt-get install -y libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/app

# If you chose Option 2 (Offline Mode), uncomment the next line:
# COPY sqlx-data.json .
# ENV SQLX_OFFLINE=true

COPY Cargo.toml Cargo.lock ./

# Ensure tower-http has "fs" feature before this runs!
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src ./src
COPY migrations ./migrations
RUN cargo build --release

# --- Stage 3: Runtime ---
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Find the binary and copy it
RUN --mount=type=bind,from=backend-builder,source=/usr/src/app/target/release,target=/release \
    find /release -maxdepth 1 -type f ! -name "*.*" -executable -exec cp {} ./server \;

COPY --from=wasm-builder /usr/src/app/wasm_client/www ./www

ENV STATIC_DIR=/app/www
ENV PORT=8080
EXPOSE 8080

CMD ["./server"]