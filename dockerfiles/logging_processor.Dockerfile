
# Stage 1: Build the binary
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /build

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml ./

# Copy the source code to the container
COPY logging_processor ./logging_processor
COPY shared ./shared

# Build the application
RUN cargo build --release --bin logging_processor

# Stage 2: Create the output container
FROM rust:latest

# Set the working directory inside the container
WORKDIR /opt/thermite/logging_processor

# Copy the binary from the builder stage to the output container
COPY --from=builder /build/target/release/logging_processor /opt/thermite/logging_processor/app

COPY ./bootstrap/logging_processor_bootstrap.sh /opt/thermite/logging_processor/bootstrap.sh
RUN chmod +x /opt/thermite/logging_processor/bootstrap.sh

# Set the entrypoint command for the container
CMD ["./bootstrap.sh"]