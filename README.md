# Review Semantic Search Platform

A semantic search system for product reviews built with Rust, featuring:
- **Backend**: Rust (axum) + simple TF-IDF style embeddings
- **Frontend**: Leptos (Rust WASM/SSR)
- **Storage**: File-based (no database) with append-only architecture
- **Search**: Vector similarity search using bag-of-words embeddings

## Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Frontend      │    │   Backend       │    │   File Storage  │
│   (Leptos)      │───▶│   (axum +       │───▶│   reviews.jsonl  │
│                 │    │   fastembed-rs) │    │   reviews.index (SPfresh)│
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## Features

- **Add Reviews**: Insert new product reviews through a web interface
- **Semantic Search**: Find similar reviews using AI-powered embeddings
- **No Database**: All data stored in files (JSON Lines + vector index)
- **Append-Only**: Reviews are never deleted, only added
- **Real-time**: Immediate search after adding reviews

## Quick Start

### Prerequisites

- Rust 1.75+ (Note: Some frontend tools may require newer versions)
- Python 3.x (for simple frontend server)
- Docker & Docker Compose (optional)

### Local Development

#### Backend

```bash
cd backend
cargo run -- --port 8000 --data-dir ./data
```

The backend will start on `http://localhost:8000` with these endpoints:
- `GET /health` - Health check
- `GET /stats` - Get system statistics
- `POST /reviews` - Add a single review
- `POST /reviews/bulk` - Add multiple reviews
- `POST /search` - Search reviews

#### Frontend

**Option 1: Simple HTML Frontend (Recommended for testing)**
```bash
cd frontend-simple
# Open index.html in your browser
# Or serve with Python: python server.py 3000
# Or on Windows: start-frontend.bat
```

**Option 2: Leptos Frontend (Advanced)**
```bash
cd frontend
# Note: Requires compatible Rust toolchain
# Try with --locked flag if trunk installation fails
cargo install trunk --locked
rustup target add wasm32-unknown-unknown

# Serve the frontend
trunk serve --port 3000
```

**Option 3: Alternative frontend server**
```bash
cd frontend-simple
npx serve . -p 3000
# Or use any static file server
```

The frontend will be available at `http://localhost:3000`

### Troubleshooting

**Frontend Issues:**
- If `trunk` installation fails due to Rust version conflicts, use the simple HTML frontend in `frontend-simple/`
- The simple frontend provides the same functionality without complex dependencies
- For CORS issues, make sure both backend and frontend are running on specified ports

**Backend Issues:**
- Ensure Rust 1.75+ is installed: `rustc --version`
- Check if port 8000 is available: `netstat -an | grep 8000`
- Review logs for dependency compilation issues

### Docker Deployment

```bash
# Build and run everything
docker-compose up --build

# Access the application
# Frontend: http://localhost:3000
# Backend: http://localhost:8000
```

## Usage

### 1. Add Reviews

Navigate to the "Add Review" page and fill in:
- **Product ID**: Unique identifier for the product
- **Review Title**: Short title for the review
- **Review Content**: Detailed review text
- **Rating**: 1-5 star rating

### 2. Search Reviews

Use the "Search" page to find similar reviews:
- Enter natural language queries like "great battery life" or "poor quality"
- Results are ranked by semantic similarity
- Each result shows similarity score and review details

## Data Storage

### File Structure

```
backend/data/
├── reviews.jsonl    # Metadata (one JSON object per line)
└── reviews.index    # Vector embeddings (binary format)
```

### Example Review Data

Each line in `reviews.jsonl`:
```json
{
  "id": "uuid",
  "review_title": "Great phone",
  "review_body": "Battery lasts all day and camera is excellent",
  "product_id": "PHONE-123",
  "review_rating": 5,
  "timestamp": "2025-01-01T00:00:00Z"
}
```

## API Reference

### Insert Review

```bash
curl -X POST http://localhost:8000/reviews \
  -H "Content-Type: application/json" \
  -d '{
    "review_title": "Amazing product",
    "review_body": "This product exceeded my expectations in every way",
    "product_id": "PROD-456",
    "review_rating": 5
  }'
```

### Search Reviews

```bash
curl -X POST http://localhost:8000/search \
  -H "Content-Type: application/json" \
  -d '{
    "query": "excellent battery life",
    "limit": 10
  }'
```

## Technology Stack

- **Backend Framework**: [axum](https://github.com/tokio-rs/axum)
- **AI Embeddings**: Simple TF-IDF style bag-of-words embeddings
- **Frontend Framework**: [Leptos](https://leptos.dev/)
- **Language**: Rust
- **Styling**: Tailwind CSS

## Configuration

### Backend Configuration

Environment variables:
- `RUST_LOG`: Log level (default: info)
- Command line arguments:
  - `--port`: Server port (default: 8000)
  - `--data-dir`: Data directory (default: ./data)

### Frontend Configuration

- Backend URL can be configured in `src/api.rs`
- Default: `http://localhost:8000`

## Development

### Project Structure

```
project-root/
├── backend/                 # Rust backend (axum + fastembed-rs)
│   ├── src/
│   │   ├── main.rs         # Application entry point
│   │   ├── handlers.rs     # HTTP handlers
│   │   ├── models.rs       # Data models
│   │   └── vector_store.rs # Vector storage logic
│   ├── data/               # Data files (created at runtime)
│   └── Cargo.toml
├── frontend/               # Leptos frontend
│   ├── src/
│   │   ├── main.rs        # Frontend entry point
│   │   ├── app.rs         # Main app component
│   │   ├── pages.rs       # Page components
│   │   ├── api.rs         # API client
│   │   └── models.rs      # Shared models
│   ├── index.html
│   └── Cargo.toml
├── docker-compose.yml
└── README.md
```

### Adding Features

1. **New Review Fields**: Update models in both frontend and backend
2. **Search Filters**: Extend search API and frontend components
3. **Bulk Import**: Use the `/reviews/bulk` endpoint
4. **Export Data**: Add new endpoints to read stored data

## Performance

- **Embedding Model**: Simple TF-IDF style vectors (50+ dimensions)
- **Search Speed**: O(n) linear search through vectors
- **Memory Usage**: All vectors loaded in memory for fast search
- **Storage**: Append-only files, no database overhead

**Note**: This implementation uses a simplified bag-of-words embedding approach for compatibility and ease of setup. For production use, consider integrating with more sophisticated embedding models like:
- OpenAI embeddings API
- Sentence-BERT models
- Local transformer models via `candle-rs`

## Limitations

- **Search Algorithm**: Currently uses simple cosine similarity
- **Scalability**: Linear search may become slow with many reviews (>100k)
- **Memory**: All embeddings loaded in RAM
- **Concurrent Writes**: File writes are not atomic across multiple instances

## Future Improvements

- [ ] Implement proper vector index (e.g., HNSW, IVF)
- [ ] Add pagination for search results
- [ ] Support for review images/attachments
- [ ] Bulk CSV import functionality
- [ ] Search filters (rating, date, product)
- [ ] Review analytics dashboard
- [ ] Multi-language support

## License

This project is open source and available under the MIT License.
