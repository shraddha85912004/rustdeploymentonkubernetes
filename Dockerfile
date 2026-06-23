# Build stage
FROM rust:1.81-bookworm AS builder

WORKDIR /usr/src/app
# Copy the manifests
COPY Cargo.toml ./

# Create a dummy src/main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/rustdeployment*

# Copy actual source code
COPY src ./src
# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /usr/src/app
# Install OpenSSL, needed by some Rust crates depending on features
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/rustdeployment /usr/local/bin/rustdeployment

# Run as non-root user
RUN useradd -m appuser
USER appuser

EXPOSE 8080

CMD ["rustdeployment"]
