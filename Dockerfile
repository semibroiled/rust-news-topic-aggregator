# Use the official Rust image from the Docker Hub
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to the working directory
COPY Cargo.toml Cargo.lock ./

# Create a new empty project with the same dependencies as our project to cache dependencies
RUN cargo build --release || true

# Copy the source code into the container
COPY . .

# Build the application
RUN cargo build --release

# Set the startup command to run the application
CMD ["./target/release/rust-news-topic-aggregator"]