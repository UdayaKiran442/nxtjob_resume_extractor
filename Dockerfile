# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app



# Copy the Cargo.toml and src directory to the container
COPY Cargo.toml .
COPY src ./src

# Build the Rust project in release mode
RUN cargo build --release

# Specify the command to run the application

