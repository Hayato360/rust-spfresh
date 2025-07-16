#!/bin/bash

# Comprehensive build script for SPFresh integration with Rust backend

set -e

echo "=== SPFresh + Rust Backend Build Script ==="

# Check if running in Docker
if [ -f /.dockerenv ]; then
    echo "Running in Docker environment..."
    DOCKER_BUILD=true
else
    echo "Running on host system..."
    DOCKER_BUILD=false
fi

echo "Step 1: Building SPFresh C API..."
cd SPFresh-main

# Make build script executable
chmod +x build_c_api.sh

# Build SPFresh C API
if [ "$DOCKER_BUILD" = true ]; then
    ./build_docker.sh
else
    ./build_c_api.sh
fi

echo "Step 2: Copying library files..."
# Copy library and header to backend directory
cp build_c_api/libspfresh_c_api.so ../backend/
cp spfresh_c_api.h ../backend/

echo "Step 3: Building Rust backend..."
cd ../backend

# Set library path
export LD_LIBRARY_PATH="$(pwd)/../SPFresh-main/build_c_api:$LD_LIBRARY_PATH"

# Build the Rust backend
cargo build --release

echo "Step 4: Setup complete!"
echo "SPFresh library: $(pwd)/../SPFresh-main/build_c_api/libspfresh_c_api.so"
echo "Backend binary: $(pwd)/target/release/review-search-backend"
echo ""
echo "To run the backend:"
echo "export LD_LIBRARY_PATH=$(pwd)/../SPFresh-main/build_c_api:\$LD_LIBRARY_PATH"
echo "cd backend && ./target/release/review-search-backend --port 8000 --data-dir ./data"
echo ""
echo "Or use Docker Compose:"
echo "docker-compose up --build"
