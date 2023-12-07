
# Stage 1: Build the binary
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /build

# Copy the Cargo.toml files to the container
COPY Cargo.toml ./

# Copy the source code to the container
COPY property_microservice ./property_microservice
COPY shared ./shared

# Build the application
RUN cargo build --release --bin property_microservice

# Stage 2: Create the output container
FROM rust:latest

# Set the working directory inside the container
WORKDIR /opt/thermite/property_microservice

# Copy the binary from the builder stage to the output container
COPY --from=builder /build/target/release/property_microservice /opt/thermite/property_microservice/app
COPY ./bootstrap/bootstrap.sh /opt/thermite/property_microservice/bootstrap.sh
RUN chmod 755 /opt/thermite/property_microservice/bootstrap.sh

# Set the entrypoint command for the container
CMD ["./bootstrap.sh", "./app" ]