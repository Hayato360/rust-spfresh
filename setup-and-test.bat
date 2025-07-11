@echo off
echo ============================================
echo  Review Semantic Search Platform - Test
echo ============================================
echo.

echo Step 1: Testing Backend...
cd backend

echo Building backend...
cargo build
if %errorlevel% neq 0 (
    echo ❌ Backend build failed!
    goto :error
)

echo ✅ Backend built successfully!
echo.

echo Starting backend server...
start "Backend Server" cmd /k "cargo run -- --port 8000 --data-dir ./data"

echo Waiting for backend to start...
timeout /t 5 >nul

echo Testing backend health...
curl -s http://localhost:8000/health >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Backend health check failed! Make sure it's running on port 8000
    goto :error
)

echo ✅ Backend is running!
echo.

echo Step 2: Testing Simple Frontend...
cd ..\frontend-simple

echo Starting frontend server...
start "Frontend Server" cmd /k "python server.py 3000"

echo.
echo ============================================
echo  🎉 SETUP COMPLETE!
echo ============================================
echo.
echo ✅ Backend running at: http://localhost:8000
echo ✅ Frontend running at: http://localhost:3000
echo.
echo 🧪 Quick Test Commands:
echo.
echo Test API directly:
echo   curl http://localhost:8000/health
echo   curl http://localhost:8000/stats
echo.
echo Add a sample review:
echo   curl -X POST http://localhost:8000/reviews -H "Content-Type: application/json" -d "{\"review_title\":\"Test Review\",\"review_body\":\"Great product with excellent quality\",\"product_id\":\"TEST-001\",\"review_rating\":5}"
echo.
echo Search reviews:
echo   curl -X POST http://localhost:8000/search -H "Content-Type: application/json" -d "{\"query\":\"excellent quality\",\"limit\":5}"
echo.
echo 🌐 Web Interface:
echo   Open http://localhost:3000 in your browser
echo.
echo Press any key to run automatic API tests...
pause >nul

echo.
echo Running automatic tests...
echo.

echo Testing API endpoints...
echo.

echo 1. Health Check:
curl -s http://localhost:8000/health | jq . 2>nul
if %errorlevel% neq 0 (
    curl -s http://localhost:8000/health
)
echo.

echo 2. System Stats:
curl -s http://localhost:8000/stats | jq . 2>nul
if %errorlevel% neq 0 (
    curl -s http://localhost:8000/stats
)
echo.

echo 3. Adding Sample Review:
curl -s -X POST http://localhost:8000/reviews -H "Content-Type: application/json" -d "{\"review_title\":\"Amazing Product\",\"review_body\":\"This product has excellent quality and great performance. Highly recommended!\",\"product_id\":\"DEMO-001\",\"review_rating\":5}" | jq . 2>nul
if %errorlevel% neq 0 (
    curl -s -X POST http://localhost:8000/reviews -H "Content-Type: application/json" -d "{\"review_title\":\"Amazing Product\",\"review_body\":\"This product has excellent quality and great performance. Highly recommended!\",\"product_id\":\"DEMO-001\",\"review_rating\":5}"
)
echo.

echo 4. Searching Reviews:
curl -s -X POST http://localhost:8000/search -H "Content-Type: application/json" -d "{\"query\":\"excellent quality\",\"limit\":5}" | jq . 2>nul
if %errorlevel% neq 0 (
    curl -s -X POST http://localhost:8000/search -H "Content-Type: application/json" -d "{\"query\":\"excellent quality\",\"limit\":5}"
)
echo.

echo ============================================
echo  ✅ ALL TESTS COMPLETED!
echo ============================================
echo.
echo The system is ready for use:
echo • Backend API: http://localhost:8000
echo • Web Interface: http://localhost:3000
echo.
echo Check the web interface to add more reviews and test semantic search!
echo.
goto :end

:error
echo.
echo ❌ Setup failed! Please check the error messages above.
echo Make sure you have:
echo • Rust installed (rustc --version)
echo • Python installed (python --version)
echo • curl available for testing
echo.

:end
pause
