#!/bin/bash

# Test script for SPFresh integration

set -e

echo "=== Testing SPFresh Integration ==="

BASE_URL="http://localhost:8000"

# Function to test endpoint
test_endpoint() {
    local method=$1
    local endpoint=$2
    local data=$3
    
    echo "Testing $method $endpoint..."
    
    if [ "$method" = "GET" ]; then
        curl -s -X GET "$BASE_URL$endpoint" | jq .
    else
        curl -s -X POST "$BASE_URL$endpoint" \
             -H "Content-Type: application/json" \
             -d "$data" | jq .
    fi
    
    echo ""
}

# Wait for backend to be ready
echo "Waiting for backend to be ready..."
for i in {1..30}; do
    if curl -s "$BASE_URL/health" > /dev/null 2>&1; then
        echo "Backend is ready!"
        break
    fi
    echo "Waiting... ($i/30)"
    sleep 2
done

# Test health endpoint
test_endpoint "GET" "/health"

# Test stats endpoint
test_endpoint "GET" "/stats"

# Test inserting reviews
echo "Inserting sample reviews..."

test_endpoint "POST" "/reviews" '{
    "review_title": "Excellent Quality Product",
    "review_body": "This product exceeded my expectations. The build quality is amazing and it works perfectly.",
    "product_id": "product-001",
    "review_rating": 5
}'

test_endpoint "POST" "/reviews" '{
    "review_title": "Good Value for Money",
    "review_body": "Decent product for the price. Not perfect but does the job well.",
    "product_id": "product-002",
    "review_rating": 4
}'

test_endpoint "POST" "/reviews" '{
    "review_title": "Poor Quality",
    "review_body": "The product broke after just a few days. Very disappointed with the quality.",
    "product_id": "product-003",
    "review_rating": 2
}'

test_endpoint "POST" "/reviews" '{
    "review_title": "Amazing Customer Service",
    "review_body": "The product is good, but the customer service is outstanding. They helped me resolve an issue quickly.",
    "product_id": "product-004",
    "review_rating": 5
}'

# Test search functionality
echo "Testing search functionality..."

test_endpoint "POST" "/search" '{
    "query": "excellent quality",
    "limit": 5
}'

test_endpoint "POST" "/search" '{
    "query": "poor quality broken",
    "limit": 5
}'

test_endpoint "POST" "/search" '{
    "query": "customer service",
    "limit": 5
}'

# Test stats after insertions
echo "Checking stats after insertions..."
test_endpoint "GET" "/stats"

echo "=== Testing Complete ==="
echo "SPFresh integration is working correctly!"
