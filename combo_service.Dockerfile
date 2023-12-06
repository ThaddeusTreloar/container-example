
# Stage 1: Build the binary
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /build

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Copy the source code to the container
COPY combo_service ./combo_service
COPY shared ./shared

# Build the application
RUN cargo build --bin combo_service

# Stage 2: Create the output container
FROM rust:latest

# Set the working directory inside the container
WORKDIR /opt/thermite/combo_service

# Copy the binary from the builder stage to the output container
COPY --from=builder /build/target/debug/combo_service /opt/thermite/combo_service/app
COPY ./bootstrap.sh /opt/thermite/combo_service/bootstrap.sh
RUN chmod 755 /opt/thermite/combo_service/bootstrap.sh

# Set the entrypoint command for the container
CMD ["./bootstrap.sh", "./app" ]