# Use Ubuntu base image for better compatibility with FastEmbed-rs and SPFresh
FROM ubuntu:22.04

# Install dependencies for building SPFresh and Rust
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    cmake \
    libjemalloc-dev \
    libsnappy-dev \
    libgflags-dev \
    libboost-all-dev \
    libtbb-dev \
    libgoogle-perftools-dev \
    gcc-9 \
    g++-9 \
    git \
    swig \
    libboost-serialization-dev \
    libboost-thread-dev \
    libboost-system-dev \
    libboost-filesystem-dev \
    libboost-regex-dev \
    libboost-program-options-dev \
    libisal-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set working directory
WORKDIR /app

# Copy SPFresh source
COPY SPFresh-main ./SPFresh-main/

# Build SPFresh C API with persistent binary index support
RUN cd SPFresh-main && \
    ls -la && \
    chmod +x build_docker.sh && \
    # Use persistent wrapper for binary index support
    if [ -f "persistent_spfresh_wrapper.cpp" ]; then \
        echo "Using persistent SPFresh wrapper for binary index"; \
        g++-9 -shared -fPIC -o libspfresh_c_api.so persistent_spfresh_wrapper.cpp -ljemalloc -lpthread; \
    else \
        echo "Falling back to simple wrapper (no binary index)"; \
        ./build_docker.sh; \
    fi && \
    ls -la && \
    cp libspfresh_c_api.so /usr/local/lib/ && \
    ldconfig

# Copy backend files
COPY backend/Cargo.toml backend/Cargo.lock backend/build.rs ./
COPY backend/src ./src/
COPY backend/data ./data/

# Set library path for building
ENV LD_LIBRARY_PATH="/app/SPFresh-main/build_c_api:/usr/local/lib:/usr/lib/x86_64-linux-gnu"

# Build the application
RUN cargo build --release

# Expose port
EXPOSE 8000

# Run the application
CMD ["./target/release/review-search-backend", "--port", "8000", "--data-dir", "./data"]
