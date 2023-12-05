
# Stage 1: Build the binary
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /build

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Copy the source code to the container
COPY logging-processor ./logging-processor

# Build the application
RUN cargo build --release --bin logging-processor

# Stage 2: Create the output container
FROM rust:latest

# Set the working directory inside the container
WORKDIR /opt/thermite/logging-processor

# Copy the binary from the builder stage to the output container
COPY --from=builder /build/target/release/logging-processor /opt/thermite/logging-processor/app

# Set the entrypoint command for the container
CMD ["sh", "-c", " ls . && tail -n+1 -F $LOG_PATH | ./app"]