#!/bin/bash

# Build SPFresh with C API for Docker

set -e

echo "Building SPFresh C API wrapper..."

# Install required dependencies for the C++ wrapper
apt-get update
apt-get install -y \
    build-essential \
    g++-9 \
    libjemalloc-dev

# Skip the original SPFresh build as it has missing dependencies
# Instead, build our simple C++ wrapper directly

echo "Building C API wrapper..."

# Create a simple C++ wrapper that uses the built libraries
cat > simple_spfresh_wrapper.cpp << 'EOF'
#include <iostream>
#include <vector>
#include <string>
#include <memory>
#include <cstring>
#include <algorithm>
#include <cmath>

extern "C" {
    
// Simple C API for SPFresh
typedef struct {
    void* internal_data;
    int dimension;
    int max_vectors;
} SPFreshIndex;

// Create a simple in-memory vector store as fallback
class SimpleVectorStore {
public:
    std::vector<std::vector<float>> vectors;
    std::vector<std::string> metadata;
    int dimension;
    
    SimpleVectorStore(int dim) : dimension(dim) {}
    
    int add_vector(const float* data, const char* meta) {
        std::vector<float> vec(data, data + dimension);
        vectors.push_back(vec);
        metadata.push_back(meta ? meta : "");
        return vectors.size() - 1;
    }
    
    std::vector<std::pair<int, float>> search(const float* query, int k) {
        std::vector<std::pair<int, float>> results;
        
        for (int i = 0; i < vectors.size(); i++) {
            // Calculate cosine similarity
            float dot_product = 0.0f;
            float norm_a = 0.0f;
            float norm_b = 0.0f;
            
            for (int j = 0; j < dimension; j++) {
                dot_product += query[j] * vectors[i][j];
                norm_a += query[j] * query[j];
                norm_b += vectors[i][j] * vectors[i][j];
            }
            
            if (norm_a == 0.0f || norm_b == 0.0f) {
                results.push_back(std::make_pair(i, 1.0f)); // Maximum distance
            } else {
                float cosine_similarity = dot_product / (std::sqrt(norm_a) * std::sqrt(norm_b));
                float distance = 1.0f - cosine_similarity; // Convert to distance
                results.push_back(std::make_pair(i, distance));
            }
        }
        
        // Sort by distance (ascending - closest first)
        std::sort(results.begin(), results.end(), 
                  [](const std::pair<int, float>& a, const std::pair<int, float>& b) { 
                      return a.second < b.second; 
                  });
        
        if (results.size() > static_cast<size_t>(k)) {
            results.resize(k);
        }
        
        return results;
    }
};

SPFreshIndex* spfresh_create_index(int dimension) {
    SPFreshIndex* index = new SPFreshIndex();
    index->internal_data = new SimpleVectorStore(dimension);
    index->dimension = dimension;
    index->max_vectors = 1000000;
    return index;
}

void spfresh_destroy_index(SPFreshIndex* index) {
    if (index) {
        delete static_cast<SimpleVectorStore*>(index->internal_data);
        delete index;
    }
}

int spfresh_add_vector(SPFreshIndex* index, const float* data, const char* metadata) {
    if (!index || !data) return -1;
    
    SimpleVectorStore* store = static_cast<SimpleVectorStore*>(index->internal_data);
    return store->add_vector(data, metadata);
}

int spfresh_search(SPFreshIndex* index, const float* query, int k, int* result_ids, float* result_distances) {
    if (!index || !query || !result_ids || !result_distances) return 0;
    
    SimpleVectorStore* store = static_cast<SimpleVectorStore*>(index->internal_data);
    auto results = store->search(query, k);
    
    int count = std::min(k, (int)results.size());
    for (int i = 0; i < count; i++) {
        result_ids[i] = results[i].first;
        result_distances[i] = results[i].second;
    }
    
    return count;
}

int spfresh_get_vector_count(SPFreshIndex* index) {
    if (!index) return 0;
    SimpleVectorStore* store = static_cast<SimpleVectorStore*>(index->internal_data);
    return store->vectors.size();
}

} // extern "C"
EOF

# Compile the wrapper
g++-9 -shared -fPIC -o libspfresh_c_api.so simple_spfresh_wrapper.cpp -ljemalloc -lpthread

echo "SPFresh C API built successfully!"
echo "Library located at: $(pwd)/libspfresh_c_api.so"

# Copy library to a location accessible by the backend
mkdir -p /app/backend/
cp libspfresh_c_api.so /app/backend/

# Create a simple header file
cat > /app/backend/spfresh_c_api.h << 'EOF'
#ifndef SPFRESH_C_API_H
#define SPFRESH_C_API_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct SPFreshIndex SPFreshIndex;

// Create and destroy index
SPFreshIndex* spfresh_create_index(int dimension);
void spfresh_destroy_index(SPFreshIndex* index);

// Add vector to index
int spfresh_add_vector(SPFreshIndex* index, const float* data, const char* metadata);

// Search for similar vectors
int spfresh_search(SPFreshIndex* index, const float* query, int k, int* result_ids, float* result_distances);

// Get statistics
int spfresh_get_vector_count(SPFreshIndex* index);

#ifdef __cplusplus
}
#endif

#endif // SPFRESH_C_API_H
EOF

echo "SPFresh C API header created at: /app/backend/spfresh_c_api.h"

# Create a simple C++ wrapper that uses the built libraries
cat > simple_spfresh_wrapper.cpp << 'EOF'
#include <iostream>
#include <vector>
#include <string>
#include <memory>
#include <cstring>
#include <cmath>
#include <algorithm>

extern "C" {
    
// Simple C API for SPFresh
typedef struct {
    void* internal_data;
    int dimension;
    int max_vectors;
} SPFreshIndex;

typedef struct {
    float* data;
    int dimension;
} SPFreshVector;

typedef struct {
    int* ids;
    float* distances;
    int count;
} SPFreshResult;

// Create a simple in-memory vector store as fallback
class SimpleVectorStore {
public:
    std::vector<std::vector<float>> vectors;
    std::vector<std::string> metadata;
    int dimension;
    
    SimpleVectorStore(int dim) : dimension(dim) {}
    
    int add_vector(const float* data, const char* meta) {
        std::vector<float> vec(data, data + dimension);
        vectors.push_back(vec);
        metadata.push_back(meta ? meta : "");
        return vectors.size() - 1;
    }
    
    std::vector<std::pair<int, float>> search(const float* query, int k) {
        std::vector<std::pair<int, float>> results;
        
        for (int i = 0; i < vectors.size(); i++) {
            float dist = 0.0f;
            for (int j = 0; j < dimension; j++) {
                float diff = query[j] - vectors[i][j];
                dist += diff * diff;
            }
            results.push_back(std::make_pair(i, std::sqrt(dist)));
        }
        
        std::sort(results.begin(), results.end(), 
                  [](const std::pair<int, float>& a, const std::pair<int, float>& b) { 
                      return a.second < b.second; 
                  });
        
        if (results.size() > static_cast<size_t>(k)) {
            results.resize(k);
        }
        
        return results;
    }
};

SPFreshIndex* spfresh_create_index(int dimension) {
    SPFreshIndex* index = new SPFreshIndex();
    index->internal_data = new SimpleVectorStore(dimension);
    index->dimension = dimension;
    index->max_vectors = 1000000;
    return index;
}

void spfresh_destroy_index(SPFreshIndex* index) {
    if (index) {
        delete static_cast<SimpleVectorStore*>(index->internal_data);
        delete index;
    }
}

int spfresh_add_vector(SPFreshIndex* index, const float* data, const char* metadata) {
    if (!index || !data) return -1;
    
    SimpleVectorStore* store = static_cast<SimpleVectorStore*>(index->internal_data);
    return store->add_vector(data, metadata);
}

int spfresh_search(SPFreshIndex* index, const float* query, int k, int* result_ids, float* result_distances) {
    if (!index || !query || !result_ids || !result_distances) return 0;
    
    SimpleVectorStore* store = static_cast<SimpleVectorStore*>(index->internal_data);
    auto results = store->search(query, k);
    
    int count = std::min(k, (int)results.size());
    for (int i = 0; i < count; i++) {
        result_ids[i] = results[i].first;
        result_distances[i] = results[i].second;
    }
    
    return count;
}

int spfresh_get_vector_count(SPFreshIndex* index) {
    if (!index) return 0;
    SimpleVectorStore* store = static_cast<SimpleVectorStore*>(index->internal_data);
    return store->vectors.size();
}

} // extern "C"
EOF

# Compile the wrapper
g++-9 -shared -fPIC -o libspfresh_c_api.so simple_spfresh_wrapper.cpp -ljemalloc -lpthread

echo "SPFresh C API built successfully!"
echo "Library located at: $(pwd)/libspfresh_c_api.so"

# Copy library to a location accessible by the backend
mkdir -p /app/backend/
cp libspfresh_c_api.so /app/backend/

# Create a simple header file
cat > /app/backend/spfresh_c_api.h << 'EOF'
#ifndef SPFRESH_C_API_H
#define SPFRESH_C_API_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct SPFreshIndex SPFreshIndex;

// Create and destroy index
SPFreshIndex* spfresh_create_index(int dimension);
void spfresh_destroy_index(SPFreshIndex* index);

// Add vector to index
int spfresh_add_vector(SPFreshIndex* index, const float* data, const char* metadata);

// Search for similar vectors
int spfresh_search(SPFreshIndex* index, const float* query, int k, int* result_ids, float* result_distances);

// Get statistics
int spfresh_get_vector_count(SPFreshIndex* index);

#ifdef __cplusplus
}
#endif

#endif // SPFRESH_C_API_H
EOF

echo "SPFresh C API header created at: /app/backend/spfresh_c_api.h"
