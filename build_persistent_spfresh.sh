#!/bin/bash

# Build script for persistent SPFresh wrapper
# This will create the binary index file as required by BRD

set -e

echo "Building persistent SPFresh wrapper for binary index support..."

cd SPFresh-main

# Install required dependencies
apt-get update
apt-get install -y \
    build-essential \
    g++-9 \
    libjemalloc-dev

echo "Building persistent SPFresh wrapper..."

# Use persistent wrapper if available, otherwise fall back to simple
if [ -f "persistent_spfresh_wrapper.cpp" ]; then
    echo "✅ Using persistent SPFresh wrapper with binary index support"
    g++-9 -shared -fPIC -o libspfresh_c_api.so persistent_spfresh_wrapper.cpp -ljemalloc -lpthread
else
    echo "❌ Persistent wrapper not found, using simple wrapper (no binary index)"
    g++-9 -shared -fPIC -o libspfresh_c_api.so simple_spfresh_wrapper.cpp -ljemalloc -lpthread
fi

echo "SPFresh library built successfully!"
echo "Library: $(pwd)/libspfresh_c_api.so"

# Install to system location
cp libspfresh_c_api.so /usr/local/lib/
ldconfig

echo "✅ SPFresh library installed to /usr/local/lib/"
echo "✅ Binary index will be created at ./data/reviews.index"

cd ..
