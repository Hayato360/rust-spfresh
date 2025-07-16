#ifndef SPFRESH_C_API_H
#define SPFRESH_C_API_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

// Opaque pointer to SPFresh index
typedef struct SPFreshIndex SPFreshIndex;

// Error codes
typedef enum {
    SPFRESH_SUCCESS = 0,
    SPFRESH_ERROR_INVALID_PARAM = -1,
    SPFRESH_ERROR_MEMORY_ALLOCATION = -2,
    SPFRESH_ERROR_INDEX_NOT_READY = -3,
    SPFRESH_ERROR_BUILD_FAILED = -4,
    SPFRESH_ERROR_SEARCH_FAILED = -5,
    SPFRESH_ERROR_UNKNOWN = -6
} SPFreshErrorCode;

// Vector data structure
typedef struct {
    float* data;
    size_t dimension;
} SPFreshVector;

// Search result structure
typedef struct {
    int32_t* ids;
    float* distances;
    size_t count;
    size_t capacity;
} SPFreshSearchResult;

// Index configuration
typedef struct {
    const char* algo_type;      // "BKT" or "KDT"
    const char* value_type;     // "Float"
    int32_t dimension;
    const char* index_directory;
    bool normalized;
    int32_t thread_count;
    int32_t max_check;
    int32_t knn;
} SPFreshConfig;

// Create a new SPFresh index
SPFreshIndex* spfresh_create_index(const SPFreshConfig* config);

// Destroy SPFresh index
void spfresh_destroy_index(SPFreshIndex* index);

// Build index from existing data
SPFreshErrorCode spfresh_build_index(SPFreshIndex* index);

// Add vectors to index (append-only)
SPFreshErrorCode spfresh_add_vectors(SPFreshIndex* index, 
                                     const SPFreshVector* vectors, 
                                     size_t count,
                                     const char** metadata);

// Search for similar vectors
SPFreshErrorCode spfresh_search(SPFreshIndex* index,
                                const SPFreshVector* query,
                                int32_t k,
                                SPFreshSearchResult* result);

// Check if index is ready to serve
bool spfresh_is_ready(SPFreshIndex* index);

// Save index to disk
SPFreshErrorCode spfresh_save_index(SPFreshIndex* index, const char* filepath);

// Load index from disk
SPFreshErrorCode spfresh_load_index(SPFreshIndex* index, const char* filepath);

// Set index parameter
SPFreshErrorCode spfresh_set_build_param(SPFreshIndex* index, 
                                         const char* name, 
                                         const char* value,
                                         const char* section);

SPFreshErrorCode spfresh_set_search_param(SPFreshIndex* index,
                                          const char* name,
                                          const char* value,
                                          const char* section);

// Cleanup search result
void spfresh_free_search_result(SPFreshSearchResult* result);

#ifdef __cplusplus
}
#endif

#endif // SPFRESH_C_API_H
