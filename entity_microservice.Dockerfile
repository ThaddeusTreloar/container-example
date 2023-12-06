
# Stage 1: Build the binary
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /build

# Copy the Cargo.toml  files to the container
COPY Cargo.toml ./

# Copy the source code to the container
COPY entity_microservice ./entity_microservice
COPY shared ./shared

# Build the application
RUN cargo build --release --bin entity_microservice

# Stage 2: Create the output container
FROM rust:latest

# Set the working directory inside the container
WORKDIR /opt/thermite/entity_microservice

# Copy the binary from the builder stage to the output container
COPY --from=builder /build/target/release/entity_microservice /opt/thermite/entity_microservice/app
COPY ./bootstrap.sh /opt/thermite/entity_microservice/bootstrap.sh
RUN chmod 755 /opt/thermite/entity_microservice/bootstrap.sh

# Set the entrypoint command for the container
CMD ["./bootstrap.sh", "./app"]