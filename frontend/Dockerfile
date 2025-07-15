# Build stage  
FROM rustlang/rust:nightly as builder

# Install dependencies for WebAssembly
RUN rustup target add wasm32-unknown-unknown
# Install trunk with nightly Rust that supports edition2024
RUN cargo install trunk

# Set working directory
WORKDIR /app

# Copy frontend files
COPY Cargo.toml Cargo.lock ./
COPY src ./src/
COPY index.html ./
COPY Trunk.toml ./

# Build the frontend
RUN trunk build --release

# Runtime stage
FROM nginx:alpine

# Copy built files to nginx
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy nginx configuration
COPY nginx.conf /etc/nginx/nginx.conf

# Expose port
EXPOSE 8080

CMD ["nginx", "-g", "daemon off;"]
