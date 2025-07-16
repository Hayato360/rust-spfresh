#!/bin/bash

echo "Cleaning project for production deployment..."

echo
echo "Stopping Docker containers..."
docker-compose down

echo
echo "Removing Docker images..."
docker rmi rust-spfresh-backend rust-spfresh-frontend 2>/dev/null || true
docker system prune -f

echo
echo "Cleaning backend data directory..."
rm -f backend/data/reviews.index
rm -f backend/data/reviews.jsonl
rm -f backend/data/spfresh_reviews.jsonl
echo "Data files cleared from backend/data/"

echo
echo "Cleaning backend build artifacts..."
rm -rf backend/target
rm -rf backend/target2
echo "Backend build artifacts cleared"

echo
echo "Cleaning frontend build artifacts..."
rm -rf frontend/target
rm -rf frontend/dist
echo "Frontend build artifacts cleared"

echo
echo "Cleaning temporary files..."
rm -f nul
rm -f *.tmp
rm -f *.log

echo
echo "Cleaning complete! Your project is ready for production deployment."
echo
echo "Next steps:"
echo "1. Run: docker-compose build --no-cache"
echo "2. Run: docker-compose up -d"
echo "3. Your application will start with clean data"
