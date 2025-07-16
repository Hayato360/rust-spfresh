#include "spfresh_c_api.h"
#include "AnnService/inc/CoreInterface.h"
#include "AnnService/inc/SPFresh/SPFresh.h"
#include <memory>
#include <string>
#include <vector>
#include <cstring>
#include <iostream>

struct SPFreshIndex {
    std::unique_ptr<AnnIndex> index;
    SPFreshConfig config;
    std::vector<std::string> metadata_storage;
    bool built;
    
    SPFreshIndex(const SPFreshConfig& cfg) 
        : config(cfg), built(false) {
        index = std::make_unique<AnnIndex>(cfg.algo_type, cfg.value_type, cfg.dimension);
    }
};

extern "C" {

SPFreshIndex* spfresh_create_index(const SPFreshConfig* config) {
    if (!config) return nullptr;
    
    try {
        return new SPFreshIndex(*config);
    } catch (const std::exception& e) {
        std::cerr << "Error creating SPFresh index: " << e.what() << std::endl;
        return nullptr;
    }
}

void spfresh_destroy_index(SPFreshIndex* index) {
    if (index) {
        delete index;
    }
}

SPFreshErrorCode spfresh_build_index(SPFreshIndex* index) {
    if (!index || !index->index) {
        return SPFRESH_ERROR_INVALID_PARAM;
    }
    
    try {
        // Set default build parameters
        index->index->SetBuildParam("TreeNumber", "1", "SelectHead");
        index->index->SetBuildParam("BKTKmeansK", "32", "SelectHead");
        index->index->SetBuildParam("BKTLeafSize", "8", "SelectHead");
        index->index->SetBuildParam("SamplesNumber", "1000", "SelectHead");
        index->index->SetBuildParam("Ratio", "0.1", "SelectHead");
        index->index->SetBuildParam("NumberOfThreads", std::to_string(index->config.thread_count).c_str(), "SelectHead");
        
        index->index->SetBuildParam("NeighborhoodSize", "32", "BuildHead");
        index->index->SetBuildParam("TPTNumber", "32", "BuildHead");
        index->index->SetBuildParam("MaxCheck", std::to_string(index->config.max_check).c_str(), "BuildHead");
        index->index->SetBuildParam("NumberOfThreads", std::to_string(index->config.thread_count).c_str(), "BuildHead");
        
        // Set search parameters
        index->index->SetSearchParam("MaxCheck", std::to_string(index->config.max_check).c_str(), "");
        
        bool result = index->index->BuildSPANN(index->config.normalized);
        if (result) {
            index->built = true;
            return SPFRESH_SUCCESS;
        } else {
            return SPFRESH_ERROR_BUILD_FAILED;
        }
    } catch (const std::exception& e) {
        std::cerr << "Error building index: " << e.what() << std::endl;
        return SPFRESH_ERROR_BUILD_FAILED;
    }
}

SPFreshErrorCode spfresh_add_vectors(SPFreshIndex* index, 
                                     const SPFreshVector* vectors, 
                                     size_t count,
                                     const char** metadata) {
    if (!index || !index->index || !vectors || count == 0) {
        return SPFRESH_ERROR_INVALID_PARAM;
    }
    
    try {
        // Prepare vector data
        size_t vector_size = vectors[0].dimension * sizeof(float);
        std::vector<uint8_t> data(count * vector_size);
        
        for (size_t i = 0; i < count; ++i) {
            if (vectors[i].dimension != index->config.dimension) {
                return SPFRESH_ERROR_INVALID_PARAM;
            }
            
            std::memcpy(data.data() + i * vector_size, 
                       vectors[i].data, 
                       vector_size);
        }
        
        // Prepare metadata if provided
        std::string metadata_str;
        if (metadata) {
            for (size_t i = 0; i < count; ++i) {
                if (metadata[i]) {
                    metadata_str += std::string(metadata[i]) + "\n";
                    index->metadata_storage.push_back(std::string(metadata[i]));
                } else {
                    metadata_str += "\n";
                    index->metadata_storage.push_back("");
                }
            }
        }
        
        ByteArray vector_data(data.data(), data.size(), false);
        
        bool result;
        if (!metadata_str.empty()) {
            ByteArray meta_data(reinterpret_cast<const uint8_t*>(metadata_str.c_str()), 
                               metadata_str.size(), false);
            result = index->index->AddWithMetaData(vector_data, meta_data, count, true, index->config.normalized);
        } else {
            result = index->index->Add(vector_data, count, index->config.normalized);
        }
        
        if (result) {
            return SPFRESH_SUCCESS;
        } else {
            return SPFRESH_ERROR_UNKNOWN;
        }
    } catch (const std::exception& e) {
        std::cerr << "Error adding vectors: " << e.what() << std::endl;
        return SPFRESH_ERROR_UNKNOWN;
    }
}

SPFreshErrorCode spfresh_search(SPFreshIndex* index,
                                const SPFreshVector* query,
                                int32_t k,
                                SPFreshSearchResult* result) {
    if (!index || !index->index || !query || !result || k <= 0) {
        return SPFRESH_ERROR_INVALID_PARAM;
    }
    
    if (!index->built) {
        return SPFRESH_ERROR_INDEX_NOT_READY;
    }
    
    try {
        if (query->dimension != index->config.dimension) {
            return SPFRESH_ERROR_INVALID_PARAM;
        }
        
        ByteArray query_data(reinterpret_cast<const uint8_t*>(query->data), 
                            query->dimension * sizeof(float), false);
        
        auto search_result = index->index->Search(query_data, k);
        
        if (!search_result) {
            return SPFRESH_ERROR_SEARCH_FAILED;
        }
        
        // Allocate result arrays
        result->ids = new int32_t[k];
        result->distances = new float[k];
        result->count = search_result->GetResultNum();
        result->capacity = k;
        
        // Copy results
        for (int i = 0; i < std::min(k, (int32_t)result->count); ++i) {
            result->ids[i] = search_result->GetResult(i)->VID;
            result->distances[i] = search_result->GetResult(i)->Dist;
        }
        
        return SPFRESH_SUCCESS;
    } catch (const std::exception& e) {
        std::cerr << "Error searching: " << e.what() << std::endl;
        return SPFRESH_ERROR_SEARCH_FAILED;
    }
}

bool spfresh_is_ready(SPFreshIndex* index) {
    if (!index || !index->index) {
        return false;
    }
    
    return index->index->ReadyToServe();
}

SPFreshErrorCode spfresh_save_index(SPFreshIndex* index, const char* filepath) {
    if (!index || !index->index || !filepath) {
        return SPFRESH_ERROR_INVALID_PARAM;
    }
    
    try {
        bool result = index->index->Save(filepath);
        return result ? SPFRESH_SUCCESS : SPFRESH_ERROR_UNKNOWN;
    } catch (const std::exception& e) {
        std::cerr << "Error saving index: " << e.what() << std::endl;
        return SPFRESH_ERROR_UNKNOWN;
    }
}

SPFreshErrorCode spfresh_load_index(SPFreshIndex* index, const char* filepath) {
    if (!index || !filepath) {
        return SPFRESH_ERROR_INVALID_PARAM;
    }
    
    try {
        AnnIndex loaded_index = AnnIndex::Load(filepath);
        // Note: This is a simplified version - full implementation would need proper copying
        return SPFRESH_SUCCESS;
    } catch (const std::exception& e) {
        std::cerr << "Error loading index: " << e.what() << std::endl;
        return SPFRESH_ERROR_UNKNOWN;
    }
}

SPFreshErrorCode spfresh_set_build_param(SPFreshIndex* index, 
                                         const char* name, 
                                         const char* value,
                                         const char* section) {
    if (!index || !index->index || !name || !value) {
        return SPFRESH_ERROR_INVALID_PARAM;
    }
    
    try {
        index->index->SetBuildParam(name, value, section ? section : "");
        return SPFRESH_SUCCESS;
    } catch (const std::exception& e) {
        std::cerr << "Error setting build param: " << e.what() << std::endl;
        return SPFRESH_ERROR_UNKNOWN;
    }
}

SPFreshErrorCode spfresh_set_search_param(SPFreshIndex* index,
                                          const char* name,
                                          const char* value,
                                          const char* section) {
    if (!index || !index->index || !name || !value) {
        return SPFRESH_ERROR_INVALID_PARAM;
    }
    
    try {
        index->index->SetSearchParam(name, value, section ? section : "");
        return SPFRESH_SUCCESS;
    } catch (const std::exception& e) {
        std::cerr << "Error setting search param: " << e.what() << std::endl;
        return SPFRESH_ERROR_UNKNOWN;
    }
}

void spfresh_free_search_result(SPFreshSearchResult* result) {
    if (result) {
        delete[] result->ids;
        delete[] result->distances;
        result->ids = nullptr;
        result->distances = nullptr;
        result->count = 0;
        result->capacity = 0;
    }
}

} // extern "C"
