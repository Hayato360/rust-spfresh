@echo off
REM Script to load sample data into the review search system
REM Make sure the backend is running before executing this script

set BACKEND_URL=http://localhost:8000

echo Loading sample reviews into the system...

REM Check if backend is running
curl -s "%BACKEND_URL%/health" >nul 2>&1
if errorlevel 1 (
    echo ❌ Backend is not running. Please start the backend first:
    echo    cd backend && cargo run
    pause
    exit /b 1
)

REM Add sample reviews one by one
echo Adding sample review 1...
curl -s -X POST "%BACKEND_URL%/reviews" ^
    -H "Content-Type: application/json" ^
    -d "{\"review_title\":\"Excellent smartphone\",\"review_body\":\"This phone has amazing battery life and the camera quality is outstanding. I can easily get through a full day of heavy usage.\",\"product_id\":\"PHONE-001\",\"review_rating\":5}" >nul

echo Adding sample review 2...
curl -s -X POST "%BACKEND_URL%/reviews" ^
    -H "Content-Type: application/json" ^
    -d "{\"review_title\":\"Decent laptop for work\",\"review_body\":\"Good performance for office tasks and video calls. The keyboard is comfortable but the screen could be brighter.\",\"product_id\":\"LAPTOP-002\",\"review_rating\":4}" >nul

echo Adding sample review 3...
curl -s -X POST "%BACKEND_URL%/reviews" ^
    -H "Content-Type: application/json" ^
    -d "{\"review_title\":\"Poor quality headphones\",\"review_body\":\"Sound quality is disappointing and build feels cheap. Not worth the price point.\",\"product_id\":\"HEADPHONE-003\",\"review_rating\":2}" >nul

echo Adding sample review 4...
curl -s -X POST "%BACKEND_URL%/reviews" ^
    -H "Content-Type: application/json" ^
    -d "{\"review_title\":\"Amazing camera features\",\"review_body\":\"The night mode is incredible and photo quality is professional grade. Perfect for photography enthusiasts.\",\"product_id\":\"PHONE-001\",\"review_rating\":5}" >nul

echo Adding sample review 5...
curl -s -X POST "%BACKEND_URL%/reviews" ^
    -H "Content-Type: application/json" ^
    -d "{\"review_title\":\"Good value for money\",\"review_body\":\"Solid performance and reliable build quality. Great choice for students and professionals alike.\",\"product_id\":\"LAPTOP-002\",\"review_rating\":4}" >nul

echo.
echo ✅ Sample data loading complete!
echo 📊 Check stats: curl %BACKEND_URL%/stats
echo 🔍 Try searching: curl -X POST %BACKEND_URL%/search -H "Content-Type: application/json" -d "{\"query\":\"battery life\"}"
pause
