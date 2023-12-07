
# Stage 1: Build the binary
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /build

# Copy the Cargo.toml  files to the container
COPY Cargo.toml ./

# Copy the source code to the container
COPY proxy_handler ./proxy_handler
COPY shared ./shared

# Build the application
RUN cargo build --release --bin proxy_handler

# Stage 2: Create the output container
FROM rust:latest

# Set the working directory inside the container
WORKDIR /opt/thermite/proxy_handler

# Copy the binary from the builder stage to the output container
COPY --from=builder /build/target/release/proxy_handler /opt/thermite/proxy_handler/app
COPY ./bootstrap/bootstrap.sh /opt/thermite/proxy_handler/bootstrap.sh
RUN chmod 755 /opt/thermite/proxy_handler/bootstrap.sh

# Set the entrypoint command for the container
CMD ["./bootstrap.sh", "./app"]