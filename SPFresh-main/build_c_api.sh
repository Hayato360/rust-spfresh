#!/bin/bash

# Build script for SPFresh C API

set -e

echo "Building SPFresh C API..."

# Create build directory
mkdir -p build_c_api
cd build_c_api

# Configure CMake
cmake -DCMAKE_BUILD_TYPE=Release \
      -DCMAKE_C_COMPILER=gcc \
      -DCMAKE_CXX_COMPILER=g++ \
      -DCMAKE_CXX_FLAGS="-fPIC" \
      -f ../CMakeLists_c_api.txt \
      ..

# Build
make -j$(nproc)

# Install to local directory
make install DESTDIR=../install

echo "SPFresh C API built successfully!"
echo "Library located at: $(pwd)/libspfresh_c_api.so"
echo "Header located at: $(pwd)/../spfresh_c_api.h"
