@echo off
echo Cleaning project for production deployment...

echo.
echo Stopping Docker containers...
docker-compose down

echo.
echo Removing Docker images...
docker rmi rust-spfresh-backend rust-spfresh-frontend 2>nul
docker system prune -f

echo.
echo Cleaning backend data directory...
del /q backend\data\reviews.index 2>nul
del /q backend\data\reviews.jsonl 2>nul
del /q backend\data\spfresh_reviews.jsonl 2>nul
echo Data files cleared from backend\data\

echo.
echo Cleaning backend build artifacts...
rmdir /s /q backend\target 2>nul
rmdir /s /q backend\target2 2>nul
echo Backend build artifacts cleared

echo.
echo Cleaning frontend build artifacts...
rmdir /s /q frontend\target 2>nul
rmdir /s /q frontend\dist 2>nul
echo Frontend build artifacts cleared

echo.
echo Cleaning temporary files...
del /q nul 2>nul
del /q *.tmp 2>nul
del /q *.log 2>nul

echo.
echo Cleaning complete! Your project is ready for production deployment.
echo.
echo Next steps:
echo 1. Run: docker-compose build --no-cache
echo 2. Run: docker-compose up -d
echo 3. Your application will start with clean data
