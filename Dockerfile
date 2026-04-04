# Multi-stage Dockerfile for Neo Rust Node
# R3E Network <jimmy@r3e.network>

FROM rust:1.76-bullseye as builder

# Install system dependencies for building
RUN apt-get update && apt-get install -y \
    build-essential \
    gcc \
    g++ \
    cmake \
    make \
    pkg-config \
    llvm-14 \
    libclang-14-dev \
    clang-14 \
    librocksdb-dev \
    libsnappy-dev \
    liblz4-dev \
    libzstd-dev \
    zlib1g-dev \
    libbz2-dev \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set environment variables for libclang
ENV LIBCLANG_PATH=/usr/lib/llvm-14/lib

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY node/ node/
COPY demo/ demo/

# Build release binaries (both neo-cli and neo-node)
RUN cargo build --release --workspace

# Runtime stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    librocksdb-dev \
    libsnappy1v5 \
    liblz4-1 \
    libzstd1 \
    zlib1g \
    libbz2-1.0 \
    libssl1.1 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create neo user
RUN groupadd -r neo && useradd -r -g neo neo

# Create data directories
RUN mkdir -p /data /data/blocks /data/logs && chown -R neo:neo /data

# Copy binaries from builder stage
COPY --from=builder /app/target/release/neo-node /usr/local/bin/neo-node
COPY --from=builder /app/target/release/neo-cli /usr/local/bin/neo-cli

# Set up volumes
VOLUME ["/data"]

# Switch to neo user
USER neo

# Expose ports
# TestNet ports
EXPOSE 20332 20333
# MainNet ports
EXPOSE 10332 10333
# Private network ports
EXPOSE 30332 30333

# Health check - check if RPC is responsive
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:20332/health || exit 1

# Environment variables
ENV NEO_NETWORK=testnet
ENV NEO_DATA_DIR=/data

# Default command for neo-cli
ENTRYPOINT ["neo-cli"]
CMD ["--network", "testnet", "--data-dir", "/data", "--daemon"]

# Metadata
LABEL org.opencontainers.image.title="Neo-Rust-Node"
LABEL org.opencontainers.image.description="Production-ready Rust implementation of the Neo N3 blockchain protocol"
LABEL org.opencontainers.image.url="https://github.com/r3e-network/neo-rs"
LABEL org.opencontainers.image.documentation="https://github.com/r3e-network/neo-rs/blob/master/README.md"
LABEL org.opencontainers.image.source="https://github.com/r3e-network/neo-rs"
LABEL org.opencontainers.image.vendor="R3E Network"
LABEL org.opencontainers.image.licenses="MIT"