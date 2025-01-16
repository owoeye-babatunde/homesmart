# Build stage
FROM rust:1.82-bullseye as builder

WORKDIR /usr/src/app
# COPY . .

# Install required dependencies for building
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev xgboost libclang-dev && \
    rm -rf /var/lib/apt/lists/*

ENV LIBCLANG_PATH=/usr/lib/llvm-11/lib

# Copy the Cargo.toml and Cargo.lock files
# This is done to speed up the build process
# So when we change files that are not Cargo.toml or Cargo.lock,
# we don't need to rebuild the entire project
COPY ./Cargo.toml ./
COPY ./Cargo.lock ./
# Create a dummy src/main.rs to satisfy Cargo
RUN mkdir -p src/bin && echo "fn main() {}" > src/bin/train.rs
RUN cargo build --release --bin train

# Copy the rest of the application code
COPY ./src ./src

# Build the application in release mode
RUN cargo build --release --bin api


# Runtime stage
FROM debian:bullseye-slim

WORKDIR /usr/local/bin

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl1.1 \
    libgomp1 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/api .

# Run the API
ENTRYPOINT ["/bin/sh", "-c", "./api --bucket-name-s3 $BUCKET_NAME_S3 --key-s3 $KEY_S3 --port $PORT"]
