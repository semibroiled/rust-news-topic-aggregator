# Use the official Rust image as a base
FROM rust:latest

# Install dependencies
RUN apt-get update && \
    apt-get install -y libtorch-dev

# Create a new directory for the project
WORKDIR /usr/src/app

# Copy the project files
COPY . .

# Set the target architecture to ARM64
RUN rustup target add aarch64-unknown-linux-gnu

# Build the project for ARM64
RUN cargo build --target=aarch64-unknown-linux-gnu

# Set the command to run the built binary
CMD ["./target/aarch64-unknown-linux-gnu/debug/"]