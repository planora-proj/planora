# ===== BUILDER STAGE =====
FROM rust:latest AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY api/Cargo.toml api/
RUN mkdir api/src && echo "fn main() {}" > api/src/main.rs
RUN cargo build --release
RUN rm -rf api/src

# Copy full source
COPY api /app/api

# Building binary
RUN cargo build --release

# ===== RUNTIME STAGE =====
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/arx-gatehouse /app/arx-gatehouse
EXPOSE 8080
CMD ["./arx-gatehouse"]
