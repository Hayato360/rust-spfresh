# SPFresh Integration with Rust Backend

This project integrates SPFresh (a high-performance vector database) with a Rust backend using FastEmbed for text embeddings.

## Architecture

- **Backend**: Rust (axum) with FastEmbed-rs for embeddings
- **Vector Store**: SPFresh C++ library with C ABI wrapper
- **Frontend**: Leptos (Rust WebAssembly)
- **Deployment**: Docker Compose

## Requirements

### System Dependencies
- CMake 3.10+
- GCC 9+ (for C++14 support)
- Boost libraries
- Intel TBB
- Jemalloc
- Snappy
- Gflags

### For Ubuntu/Debian:
```bash
sudo apt install cmake build-essential
sudo apt install libjemalloc-dev libsnappy-dev libgflags-dev
sudo apt install pkg-config libboost-all-dev libtbb-dev
sudo apt install libgoogle-perftools-dev gcc-9 g++-9
```

## Features

### 3.1 Data Append Flow (ข้อมูลต่อเนื่อง)
- Users can insert new review data continuously through the web interface
- Each insert operation:
  1. Frontend sends data to Backend
  2. Backend creates embedding vectors from review fields using fastembed-rs
  3. Vector and metadata are appended to SPFresh index (append-only)
  4. No deletion/overwrite - all inserts are appends
- Index grows continuously (append-only)

### 3.2 Search Flow (การค้นหา)
- Frontend provides search interface
- Search process:
  1. Frontend sends query to backend
  2. Backend creates embedding from query
  3. Backend searches using SPFresh for similar vectors
  4. Returns list of semantically similar reviews

## Building and Running

### Method 1: Docker Compose (Recommended)
```bash
# Build and run everything
docker-compose up --build

# Backend will be available at: http://localhost:8000
# Frontend will be available at: http://localhost:3000
```

### Method 2: Manual Build
```bash
# Build SPFresh integration
chmod +x build_spfresh_integration.sh
./build_spfresh_integration.sh

# Set library path and run backend
export LD_LIBRARY_PATH=$(pwd)/SPFresh-main/build_c_api:$LD_LIBRARY_PATH
cd backend
./target/release/review-search-backend --port 8000 --data-dir ./data
```

## API Endpoints

### Insert Review
```
POST /reviews
Content-Type: application/json

{
  "review_title": "Great product!",
  "review_body": "I love this product. It works perfectly.",
  "product_id": "product-123",
  "review_rating": 5
}
```

### Search Reviews
```
POST /search
Content-Type: application/json

{
  "query": "great product quality",
  "limit": 10
}
```

### Get Stats
```
GET /stats
```

### Health Check
```
GET /health
```

## SPFresh Configuration

The SPFresh index is configured with:
- Algorithm: BKT (Balanced K-means Tree)
- Vector Type: Float32
- Dimension: 384 (FastEmbed default)
- Normalized: true
- Thread Count: 4
- Max Check: 8192

## Data Storage

- **Metadata**: `./data/reviews.jsonl` (append-only JSONL file)
- **Vector Index**: `./data/spfresh_index/` (SPFresh index files)
- **Embeddings**: Generated on-the-fly using FastEmbed, stored in SPFresh

## Development

### Project Structure
```
├── backend/
│   ├── src/
│   │   ├── main.rs
│   │   ├── spfresh_bindings.rs      # Rust bindings for SPFresh C API
│   │   ├── spfresh_vector_store.rs  # SPFresh integration
│   │   ├── fastembed_service.rs     # FastEmbed wrapper
│   │   └── handlers.rs              # HTTP handlers
│   ├── build.rs                     # Build script for linking SPFresh
│   └── Dockerfile
├── frontend/
│   └── src/
├── SPFresh-main/
│   ├── spfresh_c_api.h             # C API header
│   ├── spfresh_c_api.cpp           # C API implementation
│   ├── build_c_api.sh              # Build script for C API
│   └── CMakeLists_c_api.txt        # CMake configuration
└── docker-compose.yml
```

### Adding New Features

1. **New Search Parameters**: Modify `SearchRequest` in `models.rs`
2. **Custom Embeddings**: Update `fastembed_service.rs`
3. **Index Configuration**: Modify `SPFreshIndexConfig` in `spfresh_bindings.rs`

## Troubleshooting

### Common Issues

1. **Library not found**: Ensure `LD_LIBRARY_PATH` includes SPFresh build directory
2. **Build failures**: Check that all system dependencies are installed
3. **Docker build fails**: Ensure Docker has enough memory (4GB+)

### Debugging
```bash
# Check SPFresh library
ldd backend/target/release/review-search-backend

# Run with debug logs
RUST_LOG=debug ./target/release/review-search-backend

# Check SPFresh index status
curl http://localhost:8000/stats
```

## Performance Considerations

- SPFresh supports high-throughput append operations
- Search performance depends on index size and configuration
- Consider adjusting `max_check` and `thread_count` for your workload
- Index will grow continuously - monitor disk space

## License

This project combines:
- SPFresh (MIT License)
- Rust backend code (project-specific)
- FastEmbed-rs (Apache 2.0)
