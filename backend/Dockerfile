# Use Ubuntu base image for better compatibility with FastEmbed-rs
FROM ubuntu:22.04

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set working directory
WORKDIR /app

# Copy backend files (paths adjusted since Dockerfile is now in backend dir)
COPY Cargo.toml Cargo.lock ./
COPY src ./src/
COPY data ./data/

# Build the application
RUN cargo build --release

# Expose port
EXPOSE 8000

# Run the application
CMD ["./target/release/review-search-backend", "--port", "8000", "--data-dir", "./data"]
