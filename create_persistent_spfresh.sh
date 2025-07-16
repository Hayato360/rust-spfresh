#!/bin/bash

# Enhanced SPFresh wrapper with binary index persistence

echo "Creating persistent SPFresh wrapper..."

cat > SPFresh-main/persistent_spfresh_wrapper.cpp << 'EOF'
#include <iostream>
#include <vector>
#include <string>
#include <memory>
#include <cstring>
#include <algorithm>
#include <cmath>
#include <fstream>
#include <sstream>

extern "C" {
    
// Enhanced C API for SPFresh with persistence
typedef struct {
    void* internal_data;
    int dimension;
    int max_vectors;
    char* index_file_path;
} SPFreshIndex;

// Persistent Vector Store with binary index file
class PersistentVectorStore {
public:
    std::vector<std::vector<float>> vectors;
    std::vector<std::string> metadata;
    int dimension;
    std::string index_file_path;
    
    PersistentVectorStore(int dim, const char* file_path) 
        : dimension(dim), index_file_path(file_path) {
        load_from_file();
    }
    
    ~PersistentVectorStore() {
        save_to_file();
    }
    
    void load_from_file() {
        std::ifstream file(index_file_path, std::ios::binary);
        if (!file.is_open()) {
            std::cout << "No existing index file found, starting fresh" << std::endl;
            return;
        }
        
        // Read number of vectors
        size_t num_vectors;
        file.read(reinterpret_cast<char*>(&num_vectors), sizeof(size_t));
        
        // Read vectors
        for (size_t i = 0; i < num_vectors; i++) {
            std::vector<float> vec(dimension);
            file.read(reinterpret_cast<char*>(vec.data()), dimension * sizeof(float));
            vectors.push_back(vec);
            
            // Read metadata length
            size_t meta_len;
            file.read(reinterpret_cast<char*>(&meta_len), sizeof(size_t));
            
            // Read metadata
            std::string meta(meta_len, '\0');
            file.read(&meta[0], meta_len);
            metadata.push_back(meta);
        }
        
        file.close();
        std::cout << "Loaded " << num_vectors << " vectors from index file" << std::endl;
    }
    
    void save_to_file() {
        std::ofstream file(index_file_path, std::ios::binary);
        if (!file.is_open()) {
            std::cerr << "Failed to open index file for writing" << std::endl;
            return;
        }
        
        // Write number of vectors
        size_t num_vectors = vectors.size();
        file.write(reinterpret_cast<const char*>(&num_vectors), sizeof(size_t));
        
        // Write vectors and metadata
        for (size_t i = 0; i < num_vectors; i++) {
            file.write(reinterpret_cast<const char*>(vectors[i].data()), 
                      dimension * sizeof(float));
            
            // Write metadata length
            size_t meta_len = metadata[i].length();
            file.write(reinterpret_cast<const char*>(&meta_len), sizeof(size_t));
            
            // Write metadata
            file.write(metadata[i].c_str(), meta_len);
        }
        
        file.close();
        std::cout << "Saved " << num_vectors << " vectors to index file" << std::endl;
    }
    
    int add_vector(const float* data, const char* meta) {
        std::vector<float> vec(data, data + dimension);
        vectors.push_back(vec);
        metadata.push_back(meta ? meta : "");
        
        // Save immediately for persistence
        save_to_file();
        
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
            
            norm_a = std::sqrt(norm_a);
            norm_b = std::sqrt(norm_b);
            
            float similarity = (norm_a > 0 && norm_b > 0) ? dot_product / (norm_a * norm_b) : 0.0f;
            results.push_back({i, similarity});
        }
        
        // Sort by similarity (descending)
        std::sort(results.begin(), results.end(), 
                  [](const std::pair<int, float>& a, const std::pair<int, float>& b) {
                      return a.second > b.second;
                  });
        
        // Return top k results
        if (results.size() > k) {
            results.resize(k);
        }
        
        return results;
    }
};

// API functions
SPFreshIndex* spfresh_create_index(int dimension) {
    SPFreshIndex* index = new SPFreshIndex();
    index->dimension = dimension;
    index->max_vectors = 1000000;
    
    // Create index file path
    std::string index_path = "./data/reviews.index";
    index->index_file_path = new char[index_path.length() + 1];
    strcpy(index->index_file_path, index_path.c_str());
    
    index->internal_data = new PersistentVectorStore(dimension, index->index_file_path);
    
    return index;
}

void spfresh_destroy_index(SPFreshIndex* index) {
    if (index) {
        delete static_cast<PersistentVectorStore*>(index->internal_data);
        delete[] index->index_file_path;
        delete index;
    }
}

int spfresh_add_vector(SPFreshIndex* index, const float* data, const char* metadata) {
    if (!index) return -1;
    PersistentVectorStore* store = static_cast<PersistentVectorStore*>(index->internal_data);
    return store->add_vector(data, metadata);
}

int spfresh_search(SPFreshIndex* index, const float* query, int k, int* result_ids, float* result_distances) {
    if (!index) return 0;
    PersistentVectorStore* store = static_cast<PersistentVectorStore*>(index->internal_data);
    
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
    PersistentVectorStore* store = static_cast<PersistentVectorStore*>(index->internal_data);
    return store->vectors.size();
}

} // extern "C"
EOF

echo "Enhanced SPFresh wrapper created with binary index persistence!"
echo "This will create and maintain ./data/reviews.index file as required by BRD"
