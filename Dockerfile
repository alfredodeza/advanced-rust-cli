# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory to /app
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Copy the source code to the container
COPY src ./src

# Build the dependencies
RUN cargo build --release

# Set the entrypoint to the binary
ENTRYPOINT ["./target/release/blkrs"]

# Set the default command to --help
CMD ["--help"]