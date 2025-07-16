#!/bin/bash

echo "=== Production Deployment Script ==="
echo "This script will build and deploy a clean version of the application"
echo

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "Error: Docker is not running. Please start Docker and try again."
    exit 1
fi

echo "Step 1: Building Docker images with no cache..."
echo "Note: SPFresh library will be built automatically as part of the backend build process"
docker-compose build --no-cache

if [ $? -eq 0 ]; then
    echo "✓ Docker images built successfully"
else
    echo "✗ Docker build failed"
    exit 1
fi

echo
echo "Step 2: Starting services..."
docker-compose up -d

if [ $? -eq 0 ]; then
    echo "✓ Services started successfully"
    echo
    echo "Production deployment complete!"
    echo
    echo "Access your application at:"
    echo "- Frontend: http://localhost:3000"
    echo "- Backend API: http://localhost:8000"
    echo
    echo "To check service status: docker-compose ps"
    echo "To view logs: docker-compose logs -f"
    echo "To stop services: docker-compose down"
else
    echo "✗ Failed to start services"
    exit 1
fi
