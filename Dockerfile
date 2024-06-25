# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Install dependencies for libtorch and other necessary tools
RUN apt-get update && \
    apt-get install -y wget unzip cmake build-essential && \
    rm -rf /var/lib/apt/lists/*

# Download and extract libtorch to /usr/lib
RUN wget https://download.pytorch.org/libtorch/cpu/libtorch-shared-with-deps-2.3.0%2Bcpu.zip && \
    unzip libtorch-shared-with-deps-2.3.0+cpu.zip && \
    mv libtorch /usr/lib/libtorch && \
    rm libtorch-shared-with-deps-2.3.0+cpu.zip

# Set the environment variables for libtorch
ENV LIBTORCH=/usr/lib/libtorch
ENV LIBTORCH_INCLUDE=${LIBTORCH}/include
ENV LIBTORCH_LIB=${LIBTORCH}/lib

# Copy the Cargo.toml and src directory to the container
COPY Cargo.toml .
COPY src ./src

# Ensure the correct Rust toolchain is installed
RUN rustup override set stable
RUN rustup update stable

# Build the Rust project in release mode
RUN cargo build --release
