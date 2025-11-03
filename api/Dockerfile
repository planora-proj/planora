FROM rust:latest AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

COPY api /app/api
COPY Cargo.* /app
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/arx-gatehouse /app/arx-gatehouse
EXPOSE 8080
CMD ["./arx-gatehouse"]
